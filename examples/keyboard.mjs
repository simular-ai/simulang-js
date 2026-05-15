// Run: node examples/keyboard.mjs
// This example uses the keyboard controller to type text.
// Ensure a text field is focused (e.g. in a text editor or browser) before running.

import { KeyboardController, Key, Direction, keyFromString } from '@simular-ai/simulang-js'

const keyboard = new KeyboardController()

// Wait two seconds
await new Promise((resolve) => setTimeout(resolve, 2000))

// Type a string. Works regardless of keyboard layout; supports Unicode (e.g. ❤️).
keyboard.text('Hello from simulang-js ❤️')

// Send individual key events, e.g. press Enter.
keyboard.key(Key.Return, Direction.Click)

// Or parse a key from a string.
const key = keyFromString('Return')
keyboard.key(key, Direction.Click)

// Wait two seconds
await new Promise((resolve) => setTimeout(resolve, 2000))
