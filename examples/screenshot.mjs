// Run: node examples/screenshot.mjs
// This example takes a screenshot, shrinks it, compresses it, and saves it.

import { Screen, screenshotFull } from '@simular-ai/simulang-js'

try {
  const screenshot = screenshotFull(true, Screen.mainScreen())
  screenshot.shrink(1920, 1080)
  screenshot.compress(80)
  screenshot.save('simulang-js-screenshot.png')
  console.log('Saved simulang-js-screenshot.png')
  console.log('Base64 length:', screenshot.base64().length)
} catch (error) {
  console.error('Screenshot failed:', error instanceof Error ? error.message : error)
}
