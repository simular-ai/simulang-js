// Run: node examples/ask.mjs
//
// Drive `AskModel` through its four input shapes — prompt-only, text-only,
// image-only, and text + multiple images — using a real accessibility-tree
// snapshot of the foreground window as text context and back-to-back
// screen captures as images.
//
// This is the JS analog of `simulang-rs/examples/ask_about_screenshot.rs`.
//
// Requirements:
//   - A configured ask provider (the bundled `openrouter` provider needs
//     `OPENROUTER_API_KEY`; see PROVIDERS.md for alternatives)
//   - Screen recording permission (for the screenshots)
//   - Accessibility permission (for the ax-tree snapshot)

import { AccessibilityTree, AskModel, Machine, ariaRoleToString } from '@simular-ai/simulang-js'

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

/**
 * Render an `AccessibilityNodeJs` subtree as an indented Playwright-style
 * aria snapshot (mirrors `WindowTrait::snapshot()` in simulang-rs).
 *
 * @param {import('@simular-ai/simulang-js').AccessibilityNodeJs} node
 * @param {number} [depth]
 * @returns {string}
 */
function snapshotToString(node, depth = 0) {
  const indent = '  '.repeat(depth)
  let line = `${indent}- ${ariaRoleToString(node.role)}`
  if (node.name) line += ` ${JSON.stringify(node.name)}`
  if (node.value) line += `: ${JSON.stringify(node.value)}`
  const childLines = node.children.map((c) => snapshotToString(c, depth + 1))
  return [line, ...childLines].join('\n')
}

const aliases = AskModel.availableAliases()
if (aliases.length === 0) {
  console.error(
    'No LLM model is reachable. Set OPENROUTER_API_KEY (or drop a custom provider config under ~/.config/simulang/providers/) and try again.',
  )
  process.exit(1)
}

const model = AskModel.default()
console.log(`Using ask model ${JSON.stringify(model.name)}`)
console.log(`Available aliases: ${aliases.join(', ')}\n`)

// 1. Prompt-only — no context at all.
const pong = model.ask('Reply with just the word PONG.')
console.log(`[prompt only] -> ${JSON.stringify(pong)}\n`)

// 2. Text-only — ground the model on a real accessibility-tree snapshot of
//    whatever application is currently in the foreground.
const tree = AccessibilityTree.fromInstance(machine.foregroundApp())
const axSnapshot = snapshotToString(tree.snapshot())
console.log(`Snapshotted foreground window: ${JSON.stringify(tree.windowTitle)}`)
const summary = model.ask('Summarize this UI in one sentence and list any buttons that look clickable.', axSnapshot)
console.log(`[text only]\n${summary}\n`)

// 3. Image-only — full screenshot of the main display. Resolve the screen
//    handle once so both captures below are pinned to the same display.
const screen = machine.mainScreen()
const shotA = screen.screenshot(true)
shotA.shrink(1024, 1024)
shotA.compress(80)
const description = model.ask('Describe what is on screen in one sentence.', null, [shotA])
console.log(`[image only]\n${description}\n`)

// 4. Text + multiple images — capture a second screenshot back-to-back and
//    ask the model to compare them with the ax-tree snapshot for context.
const shotB = screen.screenshot(true)
shotB.shrink(1024, 1024)
shotB.compress(80)
const diff = model.ask(
  'These two screenshots were taken back-to-back. Did anything change between them? Be specific.',
  axSnapshot,
  [shotA, shotB],
)
console.log(`[text + 2 images]\n${diff}`)
