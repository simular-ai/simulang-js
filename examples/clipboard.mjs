// Run: node examples/clipboard.mjs
// This example reads and writes the local machine's clipboard.

import { Machine } from '@simular-ai/simulang-js'

console.log('Waiting three seconds...')
await new Promise((resolve) => setTimeout(resolve, 3000))

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

const previous = machine.setClipboardString('Hello from simulang-js').text
const current = machine.getClipboardString()

console.log('Previous clipboard:', previous)
console.log('Current clipboard:', current)

// Paste text by simulating Cmd/Ctrl+V.
machine.pasteText('Pasted from simulang-js')

machine.clearClipboard()

const afterClear = machine.getClipboardString()
console.log('After clear:', afterClear)

// Restore previous clipboard content if it existed.
if (previous !== null) {
  console.log('Restored previous clipboard:')
  machine.setClipboardString(previous)
}
