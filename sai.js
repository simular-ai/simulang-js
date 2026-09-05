// Sai / Unified UI compatibility syntax over @simular-ai/simulang-js.
//
// Implementations mirror simular-pro-unified-ui's primitive bodies as closely
// as possible:
//   - free desktop primitives  -> app/src/main/primitives/primitives-core/index.ts
//   - ref-based launch/getApp  -> app/src/main/primitives/primitives-desktop/index.ts
//
// Where unified-ui branches on a capability that has no standalone simulang-js
// equivalent (cloud endpoints, the LLM accessibility element resolver, the Sai
// product runtime), the unimplementable branch is left as a comment marked
// MISSING / NOTE so the gap is visible, and the implementable branch is kept.

let nativeBinding = globalThis.__SIMULANG_JS_SAI_NATIVE__ ?? null

function binding() {
  if (!nativeBinding) nativeBinding = require('./wrapped.js')
  return nativeBinding
}

// All primitives drive the local machine. Constructed lazily, so importing
// this module never forces the native binding to load before a supported
// primitive is actually called.
let localMachine = null
function machine() {
  const { Machine } = binding()
  return (localMachine ??= Machine.local())
}

// unified-ui defaults grounding to 'vision' (BaseSimularPrimitives.defaultGroundingMode).
const DEFAULT_GROUNDING_MODE = 'vision'

class UnsupportedSaiPrimitiveError extends Error {
  constructor(primitive, details = {}) {
    const reason = details.reason ?? 'This Sai primitive is not available in standalone simulang-js.'
    const closest = details.closestNativeApi ? ` Closest native API: ${details.closestNativeApi}.` : ''
    const category = details.category ? ` Category: ${details.category}.` : ''
    super(`${primitive} is unsupported. ${reason}${closest}${category}`)
    this.name = 'UnsupportedSaiPrimitiveError'
    this.primitive = primitive
    this.reason = reason
    this.closestNativeApi = details.closestNativeApi
    this.category = details.category
  }
}

function unsupported(primitive, details) {
  throw new UnsupportedSaiPrimitiveError(primitive, details)
}

function unsupportedFunction(primitive, details) {
  return function unsupportedSaiPrimitive() {
    unsupported(primitive, details)
  }
}

function unsupportedAsyncFunction(primitive, details) {
  return async function unsupportedSaiPrimitive() {
    unsupported(primitive, details)
  }
}

function unsupportedNamespace(primitive, details) {
  return new Proxy(
    {},
    {
      get(_target, prop) {
        if (prop === Symbol.toStringTag) return 'UnsupportedSaiNamespace'
        unsupported(`${primitive}.${String(prop)}`, details)
      },
      apply() {
        unsupported(primitive, details)
      },
    },
  )
}

function stripDataUrl(dataUrl) {
  return dataUrl.replace(/^data:image\/[a-z]+;base64,/, '')
}

function sleep(ms) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, Math.max(0, Math.round(ms)))
}

function isWindows() {
  return process.platform === 'win32'
}

// ── Timing ──

function wait(params) {
  const { unit = 's', waitTime } = params
  sleep(unit === 'ms' ? waitTime : waitTime * 1000)
}

// ── Clipboard ──

function copyToClipboard(params) {
  machine().setClipboardString(params.text)
}

function getFromClipboard() {
  return machine().getClipboardString() ?? ''
}

// ── Keyboard ──

function press(params) {
  const { key, cmd = false, shift = false, option = false, alt = false, ctrl = false } = params
  if (!key) {
    throw new Error('No key argument was provided for press()')
  }
  const { keyFromString, Key, Direction } = binding()
  // keyFromString throws for invalid values (matches unified-ui).
  const keyToPress = keyFromString(key)
  const m = machine()

  if (cmd) m.key(Key.Meta, Direction.Press)
  if (shift) m.key(Key.Shift, Direction.Press)
  if (option || alt) m.key(Key.Alt, Direction.Press)
  if (ctrl) m.key(Key.Control, Direction.Press)

  m.key(keyToPress, Direction.Click)

  if (cmd) m.key(Key.Meta, Direction.Release)
  if (shift) m.key(Key.Shift, Direction.Release)
  if (option || alt) m.key(Key.Alt, Direction.Release)
  if (ctrl) m.key(Key.Control, Direction.Release)
}

function shortCut(params) {
  const { waitTime = 0, ...pressParams } = params
  press(pressParams)
  if (waitTime > 0) wait({ waitTime })
}

function type(params) {
  const { text, withReturn } = params
  const { Key, Direction } = binding()
  // Typing long text via clipboard is much faster than char-by-char; short text
  // goes through the keyboard so paste-blocking fields (passwords) still work.
  if (text.length > 40) {
    machine().pasteText(text)
  } else {
    machine().typeText(text)
  }
  wait({ waitTime: 0.5 })
  if (withReturn) {
    machine().key(Key.Return, Direction.Click)
  }
}

// ── Grounding (concept -> coordinates) ──

// Vision grounding: screenshot + VLM -> global desktop coordinates.
// Mirrors unified-ui getLocationByConceptFromVisualGrounding, which branches on
// whether a crop region is supplied. The native Screenshot.ground performs the
// VLM call and coordinate mapping that unified-ui does by hand against the cloud.
function getLocationByConceptFromVisualGrounding(concept, cropBounds) {
  try {
    const { GroundingModel } = binding()
    // Model selection differs from unified-ui: it picks a specific cloud-configured
    // production model via resolveProductionGroundingModel(this.groundingModel),
    // whereas standalone simulang-js uses whatever the local provider config
    // advertises as the default.
    const model = GroundingModel.default()

    let screenshot
    if (cropBounds) {
      const width = cropBounds.right - cropBounds.left
      const height = cropBounds.bottom - cropBounds.top
      if (width > 0 && height > 0) {
        screenshot = machine().screenshotCropped(cropBounds.left, cropBounds.top, width, height, true)
      } else {
        screenshot = machine().screenFromMouse().screenshot(false)
      }
    } else {
      screenshot = machine().screenFromMouse().screenshot(false)
    }

    const [x, y] = screenshot.ground(model, concept)
    return { x, y }
  } catch {
    // unified-ui logs the provider details internally and throws this generic
    // primitive-level error, so callers don't branch on provider-specific text.
    throw new Error('Failed to get location by concept from visual grounding')
  }
}

// Mirrors unified-ui getElementByConcept across platforms. Returns an
// AutomationElement (same shape unified-ui produces) or null when nothing resolves.
//   - Windows resolves via an exact `overallDescription` match in the active
//     window's AX tree (uia.findElementByOverallDescription, a deterministic
//     string-equality walk — NOT an LLM). The cross-platform native equivalent
//     is AccessibilityTree.findByDescription(concept).
//   - macOS/Linux in unified-ui resolve via vision grounding wrapped in a
//     synthetic element; we keep that behavior even though standalone
//     simulang-js can also read accessibility trees on those platforms.
function getElementByConcept(concept) {
  if (isWindows()) {
    try {
      const { AccessibilityTree } = binding()
      const tree = AccessibilityTree.fromInstance(machine().foregroundApp())
      const [node] = tree.findByDescription(concept)
      if (node?.boundingBox) {
        return { ...node, processId: 0, boundingRect: node.boundingBox, children: [] }
      }
    } catch {
      return null
    }
    return null
  }

  try {
    const { x, y } = getLocationByConceptFromVisualGrounding(concept)
    // Synthetic 10x10 element around the grounded point — mirrors unified-ui macOS/Linux.
    const halfSize = 5
    return {
      name: concept,
      className: '',
      localizedControlType: 'visual-grounding',
      description: `Visual grounding result for: ${concept}`,
      overallDescription: concept,
      helpText: '',
      value: '',
      automationId: '',
      processId: 0,
      isEnabled: true,
      boundingRect: {
        left: x - halfSize,
        top: y - halfSize,
        right: x + halfSize,
        bottom: y + halfSize,
      },
      children: [],
    }
  } catch {
    return null
  }
}

// Mirrors unified-ui getLocationByConcept(concept, mode).
function getLocationByConcept(concept, mode) {
  // If vision mode is requested, skip accessibility and ground directly.
  if (mode === 'vision') {
    return getLocationByConceptFromVisualGrounding(concept)
  }

  // Accessibility-first: exact AX overallDescription match via
  // AccessibilityTree.findByDescription (the standalone equivalent of Windows'
  // findElementByOverallDescription); on macOS/Linux this naturally falls back to
  // vision inside getElementByConcept. Final vision fallback matches unified-ui.
  const element = getElementByConcept(concept)
  if (element) {
    const { left, top, right, bottom } = element.boundingRect
    return { x: (left + right) / 2, y: (top + bottom) / 2 }
  }
  return getLocationByConceptFromVisualGrounding(concept)
}

// Sai ref-based ground({ concept, app? }). Mirrors unified-ui ground(concept, appBounds):
// when an app is supplied, crop to the window bounds for less noise / better accuracy.
async function ground(params) {
  const app = params.app
  const cropBounds = app && typeof app.getWindowBounds === 'function' ? app.getWindowBounds() : undefined
  return getLocationByConceptFromVisualGrounding(params.concept, cropBounds)
}

// ── Mouse ──

function click(params) {
  // Concept-based click (legacy Sai). Mirrors BaseSimularPrimitives.click.
  if ('concept' in params) {
    const { clickType = 'left', withCommand = false } = params
    const { Button, Direction, Key } = binding()
    const m = machine()

    if (withCommand) m.key(Key.Meta, Direction.Press) // Meta == Cmd
    move(params) // concept move to the location
    if (clickType === 'left') {
      m.mouseButton(Button.Left, Direction.Click)
    } else if (clickType === 'right') {
      m.mouseButton(Button.Right, Direction.Click)
    } else if (clickType === 'doubleClick') {
      m.mouseButton(Button.Left, Direction.Click)
      m.mouseButton(Button.Left, Direction.Click)
    }
    if (withCommand) m.key(Key.Meta, Direction.Release)
    wait({ waitTime: 0.5 })
    return
  }

  // Coordinate click. Mirrors unified-ui clickAt(): move + left click only
  // (the Sai coordinate overload carries no clickType / modifiers).
  const { Button, Coordinate, Direction } = binding()
  const m = machine()
  m.moveMouse(params.x, params.y, Coordinate.Abs)
  m.mouseButton(Button.Left, Direction.Click)
}

function move(params) {
  // Concept-based move (legacy Sai). Mirrors BaseSimularPrimitives.move.
  if ('concept' in params) {
    const { concept, mode = DEFAULT_GROUNDING_MODE } = params
    const { x, y } = getLocationByConcept(concept, mode)
    const { Coordinate } = binding()
    machine().moveMouse(x, y, Coordinate.Abs)
    wait({ waitTime: 0.5 })
    return
  }

  // Coordinate move. Mirrors unified-ui moveTo().
  const { Coordinate } = binding()
  machine().moveMouse(params.x, params.y, Coordinate.Abs)
}

// Smooth, interpolated drag — ported from unified-ui smoothDrag so apps detect
// continuous movement (drop zones need real intermediate moves + dwell).
function smoothDrag(fromX, fromY, toX, toY) {
  const { Button, Coordinate, Direction } = binding()
  const m = machine()

  m.moveMouse(fromX, fromY, Coordinate.Abs)
  sleep(200)
  m.mouseButton(Button.Left, Direction.Press)
  sleep(100)
  // Initial jiggle — exceed the Windows drag threshold (SM_CXDRAG = 4px).
  m.moveMouse(fromX + 5, fromY + 5, Coordinate.Abs)
  sleep(50)

  const dx = toX - fromX
  const dy = toY - fromY
  const distance = Math.sqrt(dx * dx + dy * dy)
  const steps = Math.max(5, Math.round(distance / 20))
  for (let i = 1; i <= steps; i++) {
    const t = i / steps
    m.moveMouse(Math.round(fromX + dx * t), Math.round(fromY + dy * t), Coordinate.Abs)
    sleep(15)
  }

  sleep(500) // hover over target so the drop zone activates
  m.mouseButton(Button.Left, Direction.Release)
}

function drag(params) {
  smoothDrag(params.fromX, params.fromY, params.toX, params.toY)
}

function scroll(params = {}) {
  const { direction = 'down', distance = 200 } = params
  const m = machine()
  switch (direction.toLowerCase()) {
    case 'up':
      m.scroll(0, -distance)
      break
    case 'down':
      m.scroll(0, distance)
      break
    case 'left':
      m.scroll(-distance, 0)
      break
    case 'right':
      m.scroll(distance, 0)
      break
    default:
      throw new Error(`Invalid scroll direction: ${direction}`)
  }
  wait({ waitTime: 0.15 })
}

// ── App / window ──

function open(params) {
  const { app, url } = params
  const { FocusPolicy, Visibility, legacyOpen } = binding()
  // TODO (unified-ui parity): allow caller to specify focus policy and visibility.
  legacyOpen(app, url, FocusPolicy.Steal, Visibility.Show)
}

function serializeNode(node, depth = 0, lines = []) {
  const { ariaRoleToString } = binding()
  const indent = '  '.repeat(depth)
  const role = typeof node.role === 'number' ? ariaRoleToString(node.role) : String(node.role)
  const label = [node.name, node.value].filter(Boolean).join(' ')
  lines.push(`${indent}- ${role}${label ? ` "${label}"` : ''}`)
  for (const child of node.children ?? []) {
    serializeNode(child, depth + 1, lines)
  }
  return lines
}

// unified-ui pageContent is abstract and platform-specific; this is the
// cross-platform approximation: foreground AX text + a full-screen screenshot.
function pageContent() {
  let text = ''
  let imageBase64 = ''
  try {
    const { AccessibilityTree } = binding()
    const tree = AccessibilityTree.fromInstance(machine().foregroundApp())
    text = serializeNode(tree.snapshot()).join('\n')
  } catch {
    text = ''
  }
  try {
    imageBase64 = stripDataUrl(machine().screenFromMouse().screenshot(false).base64())
  } catch {
    imageBase64 = ''
  }
  return { text, imageBase64 }
}

/**
 * Proxy for a desktop application window.
 *
 * NOTE: unified-ui's AppProxy (primitives-desktop) is backed by an
 * AccessibilityTree + DesktopRefManager that dispatches UIA/AX patterns
 * (Invoke/Toggle/Selection/ExpandCollapse/Value) for ref-based actions, plus an
 * LLM ElementResolver for find(). That ref subsystem is not ported here, so the
 * ref-driven action methods below throw; observation (snapshot/screenshot),
 * focus, title and window-scoped grounding are implemented.
 */
class SaiApp {
  constructor(window, instance = null) {
    this._window = window
    this._instance = instance
  }

  async snapshot() {
    // Window.snapshot() returns a plain aria-snapshot string (no refs). unified-ui
    // returns a navigable SnapshotValue with refs from DesktopRefManager — not ported.
    return {
      snapshot: this._window.snapshot(),
      refs: {},
      title: this._window.title,
    }
  }

  async screenshot() {
    return stripDataUrl(this._window.screenshot(true).base64())
  }

  async click(_params) {
    unsupported('App.click', {
      reason: 'Ref-based clicks need the DesktopRefManager (UIA/AX pattern dispatch); not ported.',
      closestNativeApi: 'Window.click(x, y, button, direction)',
      category: 'ref-runtime-only',
    })
  }

  async type(_params) {
    unsupported('App.type', {
      reason: 'Ref-based typing needs the DesktopRefManager (ValuePattern / focus); not ported.',
      closestNativeApi: 'Machine.typeText(text)',
      category: 'ref-runtime-only',
    })
  }

  async check() {
    unsupported('App.check', {
      reason: 'Needs DesktopRefManager TogglePattern; not ported.',
      category: 'ref-runtime-only',
    })
  }

  async select() {
    unsupported('App.select', {
      reason: 'Needs DesktopRefManager SelectionItemPattern; not ported.',
      category: 'ref-runtime-only',
    })
  }

  async scroll() {
    unsupported('App.scroll', {
      reason: 'Needs DesktopRefManager scrollIntoView; not ported.',
      closestNativeApi: 'Window.scroll(deltaX, deltaY)',
      category: 'ref-runtime-only',
    })
  }

  async focus() {
    this._window.focus()
  }

  async press(params) {
    press(params)
  }

  selector() {
    unsupported('App.selector', {
      reason: 'Needs DesktopRefManager ref metadata; not ported.',
      category: 'ref-runtime-only',
    })
  }

  async find() {
    unsupported('App.find', {
      reason: 'Needs DesktopRefManager + LLM ElementResolver; not ported.',
      closestNativeApi: 'Window.scoredSearch(...) or AccessibilityTree.find(...)',
      category: 'ref-runtime-only',
    })
  }

  async waitFor() {
    unsupported('App.waitFor', {
      reason: 'Polls find(), which needs the ref subsystem; not ported.',
      category: 'ref-runtime-only',
    })
  }

  async drag() {
    unsupported('App.drag', { reason: 'Needs DesktopRefManager ref bounds; not ported.', category: 'ref-runtime-only' })
  }

  title() {
    return this._window.title
  }

  windowId() {
    return this._window.pid
  }

  getWindowBounds() {
    return this._window.boundingBox()
  }

  ground(model, concept) {
    return this._window.ground(model, concept)
  }
}

async function launch(appName) {
  const { FocusPolicy, Visibility } = binding()
  const app = machine().fuzzyApp(appName)
  // NOTE: unified-ui binds a foreground-scoped AccessibilityTree into an AppProxy
  // here. We bind the launched Instance's first window instead (the ref subsystem
  // that the AX-tree-backed AppProxy needs is not ported — see SaiApp).
  const instance = app.open(null, FocusPolicy.Steal, Visibility.Show, true)
  const [window] = instance.windows()
  if (!window) throw new Error(`No visible windows found after launching ${appName}`)
  return new SaiApp(window, instance)
}

function getAppWindow(windowPidOrTitle) {
  const windows = machine().windows()
  const window =
    typeof windowPidOrTitle === 'number'
      ? windows.find((w) => w.pid === windowPidOrTitle)
      : windows.find((w) => w.title.toLowerCase().includes(windowPidOrTitle.toLowerCase()))
  if (!window) {
    throw new Error(`No window found matching "${windowPidOrTitle}". Use listAppWindows() to see available windows.`)
  }
  return new SaiApp(window)
}

function listAppWindows() {
  return machine()
    .windows()
    .map((window) => ({
      id: window.pid,
      title: window.title,
      pid: window.pid,
    }))
}

function listApps() {
  return machine()
    .apps()
    .map((app) => ({
      name: app.canonicalName ?? '',
      target: app.launchTarget ?? '',
    }))
}

// ── Files ──

function readFile(params) {
  return machine().file(params.path, false).read()
}

function writeToFile(params) {
  const { text, path = 'SimularActionResult.txt', overwrite = false } = params
  const append = !overwrite
  const file = machine().file(path, true)
  file.write(text, append)
  return file.path()
}

// ── VLM-backed primitives (built on the native AskModel / grounding) ──

function ask(params) {
  const { prompt, context } = params
  const { AskModel, Image } = binding()
  const currentDate = new Date().toISOString()

  // Mirror unified-ui prompt shaping (persona header + page content + task).
  let inputPrompt = `I am Sai, a computer use agent. Now is ${currentDate}. My birthday is November 1, 2024. User asks me questions and I need you to provide a response pretending you were me, without prefacing or meta-commentary.`

  const images = []
  if (context?.text) {
    inputPrompt += `\nPage content:\n${context.text}`
  }
  if (context?.imageBase64) {
    images.push(Image.fromBase64(context.imageBase64.replace(/^data:image\/(png|jpeg);base64,/, '')))
  }
  inputPrompt += `\nTask: ${prompt}`

  // NOTE: unified-ui POSTs to the Simular cloud `/v1/chat/completions/ask`
  // endpoint (billing + server-managed model). Standalone simulang-js routes the
  // same shaped prompt through the locally configured AskModel provider instead.
  return AskModel.default()
    .ask(inputPrompt, null, images.length ? images : null)
    .trim()
}

function stateSatisfies(params) {
  const { condition } = params
  try {
    // Mirrors unified-ui: capture the current page (a11y text + screenshot) and evaluate
    // the condition against the dedicated `v1/perception/state_satisfies` perception
    // model
    const { StateSatisfiesModel } = binding()
    const { text, imageBase64 } = pageContent()
    return StateSatisfiesModel.default().stateSatisfies(condition, text, imageBase64)
  } catch (error) {
    throw new Error(`Failed to check state condition: ${error}`, { cause: error })
  }
}

function ConceptsExist(params) {
  const { concepts } = params
  // Mirrors unified-ui: resolve each concept via getElementByConcept (exact AX
  // overallDescription match, with macOS/Linux vision fallback); absent on the
  // first concept that fails to resolve.
  for (const concept of concepts) {
    if (!getElementByConcept(concept)) return false
  }
  return true
}

// ── Google Sheets helpers (pure keyboard + clipboard, ported from unified-ui) ──

function setFocusToCell(cell) {
  // Cmd/Ctrl+J opens the Google Sheets "Go to range" dialog.
  press({ key: 'j', cmd: true })
  wait({ waitTime: 60, unit: 'ms' })
  type({ text: cell, withReturn: true })
  wait({ waitTime: 60, unit: 'ms' })
}

function getGoogleSheetCellValue(params) {
  setFocusToCell(params.cell)
  const previousClipboardValue = getFromClipboard()
  wait({ waitTime: 30, unit: 'ms' })
  press({ key: 'c', cmd: true })
  wait({ waitTime: 60, unit: 'ms' })
  // Google Sheets appends newlines for empty cells; trim trailing whitespace.
  const cellValue = getFromClipboard().replace(/\s+$/g, '')
  press({ key: 'escape' })
  copyToClipboard({ text: previousClipboardValue })
  return cellValue
}

function setGoogleSheetCellValue(params) {
  setFocusToCell(params.cell)
  press({ key: 'delete' })
  wait({ waitTime: 60, unit: 'ms' })
  type({ text: params.value, withReturn: true })
}

// ── Unsupported Sai runtime/product primitives (no standalone equivalent) ──

const unsupportedProduct = { category: 'product-runtime-only' }
const unsupportedCloud = { category: 'cloud-dependent' }
const unsupportedBrowser = {
  reason: 'Sai browser automation is backed by the Sai runtime Playwright/CDP layer, not simulang-js.',
  ...unsupportedProduct,
}

const browser = {
  newtab: unsupportedAsyncFunction('browser.newtab', unsupportedBrowser),
  getTab: unsupportedAsyncFunction('browser.getTab', unsupportedBrowser),
  listTabs: unsupportedAsyncFunction('browser.listTabs', unsupportedBrowser),
  closeTab: unsupportedAsyncFunction('browser.closeTab', unsupportedBrowser),
  disconnect: unsupportedAsyncFunction('browser.disconnect', unsupportedBrowser),
  close: unsupportedAsyncFunction('browser.close', unsupportedBrowser),
}

module.exports = {
  SaiApp,
  UnsupportedSaiPrimitiveError,
  ask,
  // unified-ui browser.* is the Sai runtime Playwright/CDP layer, not simulang-js.
  browser,
  click,
  ConceptsExist,
  copyToClipboard,
  drag,
  // unified-ui exec runs through the exec security manager + approval UI.
  exec: unsupportedAsyncFunction('exec', {
    reason: 'Needs the Sai exec security manager and approval UI.',
    ...unsupportedProduct,
  }),
  // unified-ui generateImage POSTs to the cloud `/v1/image-gen` service; AskModel is chat-only.
  generateImage: unsupportedAsyncFunction('generateImage', {
    reason: 'simulang-js has no image-generation provider (AskModel is chat-completions only).',
    ...unsupportedCloud,
  }),
  getAppWindow,
  getFromClipboard,
  getGoogleSheetCellValue,
  github: unsupportedNamespace('github', unsupportedProduct),
  google: unsupportedNamespace('google', unsupportedProduct),
  ground,
  launch,
  listApps,
  listAppWindows,
  move,
  open,
  pageContent,
  press,
  readFile,
  // unified-ui requestApproval pauses execution for an approval/user-input UI.
  requestApproval: unsupportedAsyncFunction('requestApproval', {
    reason: 'Needs the Sai approval / user-input UI and execution pause.',
    ...unsupportedProduct,
  }),
  // unified-ui respond shows a message (and optional confirm) in the Sai UI.
  respond: unsupportedFunction('respond', {
    reason: 'Needs the Sai message / confirmation UI (showMessageWithChoices).',
    ...unsupportedProduct,
  }),
  sai: unsupportedNamespace('sai', unsupportedProduct),
  scroll,
  setGoogleSheetCellValue,
  shortCut,
  slack: unsupportedNamespace('slack', unsupportedProduct),
  stateSatisfies,
  type,
  wait,
  writeToFile,
}
