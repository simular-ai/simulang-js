// Run: node examples/screenshot.mjs
// This example takes a screenshot, shrinks it, compresses it, and saves it.

import { Machine } from '@simular-ai/simulang-js'

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

try {
  const screenshot = machine.screenFromMouse().screenshot(true)
  screenshot.shrink(1920, 1080)
  screenshot.compress(80)
  screenshot.save('simulang-js-screenshot.png')
  console.log('Saved simulang-js-screenshot.png')
  console.log('Base64 length:', screenshot.base64().length)
} catch (error) {
  console.error('Screenshot failed:', error instanceof Error ? error.message : error)
}
