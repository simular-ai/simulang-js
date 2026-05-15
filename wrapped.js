// Re-export the napi bindings, but with a global pause hook woven into
// every callable. Set the hook via `setPauseHook(fn)`; clear with `null`.
// While set, the hook runs synchronously before every method or
// free-function call delegates to the native binding — that lets
// `@simular-ai/simulang-log-viewer` make its grab/pause state apply to
// every simulang-js call transparently, without forcing user code to
// switch import paths or add awaits.
//
// The package's `main` points here, so `import '@simular-ai/simulang-js'`
// resolves to this wrapper. The napi-rs codegen still owns `index.js`
// and `index.d.ts` verbatim — auto-doc and Claude-skill tooling that
// reads those files keeps working unchanged.
//
// Wrapping happens once at module load against `index.js`'s own
// exports, so any new function or class added to the napi crate is
// picked up automatically as long as `napi build` emits it through
// the generated `index.js`.

module.exports = require('./index.js')
const binding = module.exports

let pauseHook = null

/**
 * Register a function to call synchronously before every simulang-js
 * method or free function executes. Pass `null` to clear. The hook
 * must not call back into simulang-js — that would recurse.
 */
function setPauseHook(fn) {
  pauseHook = typeof fn === 'function' ? fn : null
}

function wrap(orig) {
  return function pauseAware(...args) {
    if (pauseHook) pauseHook()
    return orig.apply(this, args)
  }
}

// Patch instance methods in place on the prototype. napi-rs emits these
// as writable+configurable properties, so direct assignment works and
// every existing or future instance — wherever the class is imported
// from — sees the wrapped method through prototype lookup.
function patchPrototype(klass) {
  for (const name of Object.getOwnPropertyNames(klass.prototype)) {
    if (name === 'constructor') continue
    const desc = Object.getOwnPropertyDescriptor(klass.prototype, name)
    // Skip accessor properties (getters/setters): reading a property
    // shouldn't trigger pause-wait. Only wrap method-valued slots.
    if (typeof desc?.value !== 'function') continue
    klass.prototype[name] = wrap(desc.value)
  }
}

const RESERVED_STATIC_KEYS = new Set(['length', 'name', 'prototype'])

// Static methods on napi-rs classes are non-writable AND non-configurable,
// so direct assignment fails and Proxy invariants forbid returning a
// different value through `get`. We work around it by exporting a thin
// subclass that redefines the statics as writable properties; the
// subclass's own prototype chain delegates everything else to the
// original class.
//
// `[Symbol.hasInstance]` is overridden so that instances returned by
// the underlying napi factories (which carry the original prototype, not
// the subclass's) still satisfy `inst instanceof WrappedKlass`.
function wrapStatics(klass) {
  const statics = []
  for (const name of Object.getOwnPropertyNames(klass)) {
    if (RESERVED_STATIC_KEYS.has(name)) continue
    const desc = Object.getOwnPropertyDescriptor(klass, name)
    if (typeof desc?.value !== 'function') continue
    statics.push([name, wrap(desc.value)])
  }
  if (statics.length === 0) return klass

  class Wrapped extends klass {
    static [Symbol.hasInstance](inst) {
      return inst instanceof klass
    }
  }
  // Plain assignment (`Wrapped[name] = wrapped`) walks the prototype
  // chain and silently fails when it finds the inherited non-writable
  // static. defineProperty bypasses the chain and installs an own,
  // shadowing slot.
  for (const [name, wrapped] of statics) {
    Object.defineProperty(Wrapped, name, {
      value: wrapped,
      writable: true,
      configurable: true,
      enumerable: false,
    })
  }
  // Preserve `name` for stack traces / `toString()` output.
  Object.defineProperty(Wrapped, 'name', { value: klass.name, configurable: true })
  return Wrapped
}

// napi-rs classes carry either instance methods on the prototype (beyond
// `constructor`) or static methods on the class itself, or both. Plain
// free function exports have neither. Enums are plain objects.
//
// The static-method check has to look for function-VALUED own properties
// specifically, not just any own property beyond the reserved set: Node
// 24's `napi_create_function` installs legacy `arguments` and `caller`
// slots on every napi-emitted function object (Node 25+ does not), and
// without the function-value filter those would make every free function
// (`keyFromString`, `ariaRoleToString`, …) misclassify as a class and
// silently bypass the `wrap(fn)` branch — pause hooks never firing for
// free-function calls is exactly what that bug looked like.
function isClass(value) {
  if (typeof value !== 'function' || !value.prototype) return false
  if (Object.getOwnPropertyNames(value.prototype).length > 1) return true
  for (const name of Object.getOwnPropertyNames(value)) {
    if (RESERVED_STATIC_KEYS.has(name)) continue
    const desc = Object.getOwnPropertyDescriptor(value, name)
    if (typeof desc?.value === 'function') return true
  }
  return false
}

for (const key of Object.keys(binding)) {
  const value = binding[key]
  if (isClass(value)) {
    patchPrototype(value)
    module.exports[key] = wrapStatics(value)
  } else if (typeof value === 'function') {
    module.exports[key] = wrap(value)
  }
}
module.exports.setPauseHook = setPauseHook

// Auto-install the stderr logger sink so simulang-rs `log::*!` records
// (clipboard reads/writes, keyboard input, app launches, window state,
// etc.) are visible in the terminal as soon as `import
// '@simular-ai/simulang-js'` evaluates — no explicit `initLogger()` call
// required. Without this, the `log` crate has no logger registered at
// all, so every `log::*!` is a silent no-op regardless of `RUST_LOG`.
binding.initLogger()
