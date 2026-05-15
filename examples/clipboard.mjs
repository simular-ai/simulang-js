// Run: node examples/clipboard.mjs
// This example reads and writes the clipboard.

import { Clipboard } from '@simular-ai/simulang-js'

console.log('Waiting three seconds...')
await new Promise((resolve) => setTimeout(resolve, 3000))

const clipboard = new Clipboard()

const previous = clipboard.setString('Hello from simulang-js')
const current = clipboard.getString()

console.log('Previous clipboard:', previous)
console.log('Current clipboard:', current)

// Paste text by simulating Cmd/Ctrl+V.
clipboard.pasteText('Pasted from simulang-js')

clipboard.clear()

const afterClear = clipboard.getString()
console.log('After clear:', afterClear)

// Restore previous clipboard content if it existed.
if (previous !== null) {
  console.log('Restored previous clipboard:')
  clipboard.setString(previous)
}
