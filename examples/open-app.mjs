// Run: node examples/open-app.mjs [appName]
// Tests the full pipeline: find app -> open -> list windows -> bind tree -> snapshot.

import { Machine, AccessibilityTree, FocusPolicy, Visibility, TraversalOrder, AriaRole } from '@simular-ai/simulang-js'

const appName = process.argv[2] || 'Chrome'
// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

// 1. List installed apps
console.log('=== Installed apps (first 20) ===')
const apps = machine.apps()
apps.slice(0, 20).forEach((a) => console.log(`  ${a.canonicalName} -> ${a.launchTarget}`))
console.log(`  ... (${apps.length} total)\n`)

// 2. Fuzzy search
console.log(`=== Searching for: ${appName} ===`)
const app = machine.fuzzyApp(appName)
console.log(`Found: ${app.canonicalName} -> ${app.launchTarget}\n`)

// 3. Open the app
console.log(`=== Opening ${appName} ===`)
const instance = app.open('https://www.google.com', FocusPolicy.Steal, Visibility.Show, true)
console.log('PID:', instance.pid)
console.log()

// 4. List windows for this instance
console.log('=== Windows for instance ===')
const windows = instance.windows()
windows.forEach((w) => console.log(`  pid=${w.pid} "${w.title}"`))
console.log()

// 5. Bind and snapshot
if (windows.length > 0) {
  console.log(`=== Snapshot of ${appName} ===`)
  const tree = AccessibilityTree.fromInstance(instance)
  console.log(`Bound to: "${tree.windowTitle}"`)

  const root = tree.snapshot(true)
  console.log('Role:', root.role)
  console.log('Children:', root.children.length)
  console.log('First 10 children:')
  root.children.slice(0, 10).forEach((c) => {
    const name = c.name ? `"${c.name}"` : ''
    console.log(`  - ${c.role} ${name} [ref=${c.refId}]`)
  })

  // 6. Find the first button using depth-first search
  const [firstButton] = tree.find(TraversalOrder.DepthFirst, AriaRole.Button, undefined, true, 1)
  if (firstButton && firstButton.refId != null) {
    console.log(`\nFirst button: "${firstButton.name}" [ref=${firstButton.refId}]`)
    console.log('Actions:', tree.getSupportedActions(firstButton.refId))
    await new Promise((resolve) => setTimeout(resolve, 2000))
    console.log(`Activating: "${firstButton.name}"`)
    tree.activate(firstButton.refId)
  }
}
