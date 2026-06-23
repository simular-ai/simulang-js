// Run: node examples/element_at_point.mjs
// Optionally pass coordinates: node examples/element_at_point.mjs 400 300
//
// Demonstrates the element-picker primitives:
//   1. AccessibilityNode.fromPoint(x, y) — hit-test the element under a
//      screen coordinate (used per-poll to draw a hover highlight). The
//      returned node is already actionable (activate / setValue / …).
//   2. AccessibilityTree.findByDescription(desc) — the picker's grounding
//      check: is this element's overallDescription unique in the window?
//      `isUnique` is just `matches.length === 1`.

import {
  AccessibilityNode,
  AccessibilityTree,
  MouseController,
  Window,
  ariaRoleToString,
} from '@simular-ai/simulang-js'

// Coordinates: explicit args, or the live cursor location.
const args = process.argv.slice(2)
const [x, y] = args.length >= 2 ? [Number(args[0]), Number(args[1])] : new MouseController().location()

// 1. Hit-test — cheap, resolves a single element. Returns null when the
//    point has no accessible element (empty desktop, gaps, etc.).
const t0 = Date.now()
const node = AccessibilityNode.fromPoint(x, y)
if (!node) {
  console.log(`=== No accessible element at (${x}, ${y}) (${Date.now() - t0}ms) ===`)
  process.exit(0)
}
const description = node.overallDescription
console.log(`=== Element at (${x}, ${y}) — fromPoint in ${Date.now() - t0}ms ===`)
console.log(`role:        ${ariaRoleToString(node.role)}`)
console.log(`name:        ${JSON.stringify(node.name)}`)
console.log(`value:       ${JSON.stringify(node.value)}`)
console.log(`description: ${JSON.stringify(description)}`)
console.log('bounds:     ', node.boundingBox())
// The fromPoint node is already actionable — no tree round-trip to click it.
console.log('actions:    ', node.supportedActions())

// 2. Uniqueness of the description across the whole window (the picker's
//    grounding check). This walks the *entire* tree computing
//    summary_with_context per node, so it is far heavier than fromPoint —
//    especially on macOS, where every attribute read is a live AX IPC.
console.log('\n=== findByDescription on foreground window (app-scoped) ===')
const t1 = Date.now()
const tree = AccessibilityTree.fromForeground()
const matches = tree.findByDescription(description)
console.log(`matches:  ${matches.length} (in ${Date.now() - t1}ms)`)
console.log(`isUnique: ${matches.length === 1}`)
if (matches.length > 1) {
  console.log(
    'Not a stable grounding key — the cursor is over a generic/nameless ' +
      'container whose description many siblings share.',
  )
}

// 3. Window-scoped uniqueness: resolve the window under the cursor and check
//    uniqueness WITHIN that window only (cross-platform: AXWindow subtree on
//    macOS, the HWND subtree on Windows). This is what the element picker
//    should use so a description shared across other windows of the same app
//    doesn't count against it.
console.log('\n=== findByDescription on hovered window (window-scoped) ===')
const window = Window.fromPoint(x, y)
if (!window) {
  console.log(`No window under (${x}, ${y}).`)
} else {
  console.log(`window:   ${JSON.stringify(window.title)} (pid ${window.pid})`)
  const t2 = Date.now()
  const windowTree = AccessibilityTree.fromWindow(window)
  const windowMatches = windowTree.findByDescription(description)
  console.log(`matches:  ${windowMatches.length} (in ${Date.now() - t2}ms)`)
  console.log(`isUnique: ${windowMatches.length === 1}`)
}
