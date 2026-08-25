// Run: node examples/accessibility.mjs
// Tests AccessibilityTree: list windows, snapshot foreground, query actions.

import { AccessibilityTree, AriaRole, ariaRoleToString, Machine } from '@simular-ai/simulang-js'

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

// 1. List all windows
const windows = machine.windows()
console.log(`=== All visible windows (${windows.length}) ===`)
windows.slice(0, 5).forEach((w) => console.log(`  pid=${w.pid} "${w.title}"`))
console.log()

// 2. Snapshot the foreground application
const tree = AccessibilityTree.fromInstance(machine.foregroundApp())
console.log(`Bound to: "${tree.windowTitle}"`)

const root = tree.snapshot()
console.log(`Role: ${ariaRoleToString(root.role)}`)
console.log(`Children: ${root.children.length}`)

// Print the whole tree as an indented Playwright-style aria snapshot,
// the same format as `WindowTrait::snapshot()` / the `print_ax_tree`
// example in simulang-rs.
/**
 * @param {import('@simular-ai/simulang-js').AccessibilityNodeJs} node
 * @param {number} [depth]
 */
function printSnapshot(node, depth = 0) {
  const indent = '  '.repeat(depth)
  let line = `${indent}- ${ariaRoleToString(node.role)}`
  if (node.name) line += ` ${JSON.stringify(node.name)}`
  if (node.value) line += `: ${JSON.stringify(node.value)}`
  console.log(line)
  for (const child of node.children) printSnapshot(child, depth + 1)
}

console.log('\n=== Snapshot ===')
printSnapshot(root)

// 3. Test actions on first button
const firstButton = root.children.find((c) => c.role === AriaRole.Button && c.refId != null)
if (firstButton && firstButton.refId != null) {
  console.log(`\nFirst button: "${firstButton.name}" [ref=${firstButton.refId}]`)
  console.log('Actions:', tree.getSupportedActions(firstButton.refId))
  console.log('Bounds:', tree.getBounds(firstButton.refId))
}
