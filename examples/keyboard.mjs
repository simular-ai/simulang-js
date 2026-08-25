// Run: node examples/keyboard.mjs
// This example types text on the local machine's keyboard.
// Ensure a text field is focused (e.g. in a text editor or browser) before running.

import { Machine, Key, Direction, keyFromString } from '@simular-ai/simulang-js'

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

// Wait two seconds
await new Promise((resolve) => setTimeout(resolve, 2000))

// Type a string. Works regardless of keyboard layout; supports Unicode (e.g. ❤️).
machine.typeText('Hello from simulang-js ❤️')

// Send individual key events, e.g. press Enter.
machine.key(Key.Return, Direction.Click)

// Or parse a key from a string.
const key = keyFromString('Return')
machine.key(key, Direction.Click)

// Wait two seconds
await new Promise((resolve) => setTimeout(resolve, 2000))
