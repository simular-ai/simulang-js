/**
 * Hand-authored entry that re-exports the napi-generated bindings from
 * `index.d.ts` and adds the `setPauseHook` API used by
 * `@simular-ai/simulang-log-viewer` to weave pause-handshake behavior
 * through every simulang-js call.
 *
 * The package's `types` field points here. `index.d.ts` stays as the
 * napi-rs codegen output (and the source of truth for the public API
 * surface that auto-docs and Claude skills consume).
 */

export * from './index'

/**
 * Register a function to be called synchronously before every
 * simulang-js method or free function executes. Used by
 * `@simular-ai/simulang-log-viewer` to make the log window's
 * pause/grab-hotkey state transparently affect every simulang-js call.
 * Pass `null` to clear.
 *
 * The hook runs on the calling thread, before delegating to the
 * native binding. It must not call back into simulang-js — that would
 * recurse.
 */
export declare function setPauseHook(fn: (() => void) | null): void
