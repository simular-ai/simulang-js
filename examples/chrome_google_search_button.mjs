// Run: node examples/chrome_google_search_button.mjs
//
// Open Google Chrome on https://www.google.com and locate the main search
// control by *concept text*, using `Instance.scoredSearch` — a thin binding
// over `simulang_rs::Instance::scored_search` + `BowJaccard` paired-Jaccard
// scoring against each node's `summary_with_context`.
//
// This is the JS analog of
// `simulang-rs/examples/chrome_google_search_button.rs`.
//
// Requirements:
//   - Windows or macOS
//   - Google Chrome installed
//   - On macOS: Accessibility permission granted to the process running Node.

import {
  AccessibilityTree,
  App,
  FocusPolicy,
  TraversalOrder,
  Visibility,
  Window,
  ariaRoleToString,
} from '@simular-ai/simulang-js'

/** Natural-language concept matched against each node's `overallDescription`. */
const CONCEPT = 'Auf gut Glück'

/** Match threshold (same default as the Rust example). */
const MATCH_THRESHOLD = 0.75

/** Upper bound on nodes visited so a pathological Chromium tree can't run unbounded. */
const MAX_NODES_VISITED = 50_000

const browser = process.platform === 'darwin' ? 'Google Chrome' : 'Chrome'

/** @param {number} timeoutMs */
async function waitForChromeForeground(timeoutMs) {
  const deadline = Date.now() + timeoutMs
  while (Date.now() < deadline) {
    try {
      const title = AccessibilityTree.fromForeground().windowTitle.toLowerCase()
      if (title.includes('chrome') || title.includes('chromium') || title.includes('谷歌')) {
        return
      }
    } catch {
      // ignore – the foreground app may not be accessible yet
    }
    await new Promise((r) => setTimeout(r, 200))
  }
  throw new Error(`Chrome did not become the foreground window within ${timeoutMs} ms`)
}

const instance = App.exactName(browser).open('https://www.google.com', FocusPolicy.Steal, Visibility.Show, true)
console.log(`Opened ${browser} (pid=${instance.pid})`)

await new Promise((r) => setTimeout(r, 2000))
await waitForChromeForeground(45_000)

// Chrome ships with its accessibility tree disabled by default for perf. Turn
// it on for this process and give it a moment to populate.
instance.enableAccessibility()
await new Promise((r) => setTimeout(r, 3000))

const matches = instance.scoredSearch(TraversalOrder.BreadthFirst, MAX_NODES_VISITED, false, CONCEPT, MATCH_THRESHOLD)

const button = matches[0]
if (!button) {
  throw new Error(
    `No node scored above ${MATCH_THRESHOLD} on concept ${JSON.stringify(CONCEPT)} ` +
      `within ${MAX_NODES_VISITED} BFS nodes. Try another CONCEPT for your locale, ` +
      `close overlays, or wait for the page to finish loading.`,
  )
}

console.log(`Found accessibility node for concept ${JSON.stringify(CONCEPT)}:`)
console.log(`  role:                ${ariaRoleToString(button.role)}`)
console.log(`  name:                ${button.name}`)
console.log(`  overallDescription:  ${button.overallDescription}`)
console.log(`  supportedActions:    ${JSON.stringify(button.supportedActions())}`)
// Action methods (`button.activate()`, `button.setValue(...)`, …) are
// available directly on the returned `AccessibilityNode`.

console.log('\nMinimizing Chrome window…')
const [chromeWindow] = Window.allForPid(instance.pid)
if (!chromeWindow) throw new Error('no windows found for instance')
chromeWindow.minimize()
console.log('Window should be minimized.')
