// Run: node examples/file-trait.mjs
// This example writes a file and reads it back.

import { Machine } from '@simular-ai/simulang-js'

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()
const file = machine.file('simulang-js-example.txt', true)
file.write('hello from simulang-js', false)
console.log('Wrote file:', file.path())
console.log('Read file:', file.read())
