// Run: node examples/google_search.mjs
// This example opens Google in Chrome.

import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { pathToFileURL } from 'node:url'
import {
  App,
  KeyboardController,
  Key,
  Direction,
  FocusPolicy,
  Visibility,
  Screen,
  enableAccessibilityForFrontmostApp,
  screenshotFull,
} from '@simular-ai/simulang-js'

const browser = process.platform === 'darwin' ? 'Google Chrome' : 'Chrome'
const imageViewer = process.platform === 'darwin' ? 'Preview' : 'Chrome'
const modifierKey = process.platform === 'darwin' ? Key.Meta : Key.Control

// Open a URL in Chrome
App.exactName(browser).open('https://google.com', FocusPolicy.Steal, Visibility.Show, true)
console.log('Launched Google Chrome')

// Wait two seconds to give Chrome time to start
console.log('Waiting two seconds...')
await new Promise((resolve) => setTimeout(resolve, 2000))

const foreground = new KeyboardController()

// Independent of keyboard layout
foreground.keyUnicode('z', Direction.Click)
await new Promise((resolve) => setTimeout(resolve, 1000))

// Open the accessibility menu
foreground.key(modifierKey, Direction.Press)
foreground.keyUnicode('l', Direction.Click)
foreground.key(modifierKey, Direction.Release)
await new Promise((resolve) => setTimeout(resolve, 500))
foreground.text('chrome://accessibility')
foreground.key(Key.Return, Direction.Click)
await new Promise((resolve) => setTimeout(resolve, 2000))

// Enable the accessibility tree for Chrome
enableAccessibilityForFrontmostApp()
await new Promise((resolve) => setTimeout(resolve, 5000))

// Take a screenshot of the entire screen and save it to a file
const screenshot = screenshotFull(true, Screen.mainScreen())
const path = join(tmpdir(), 'screenshot.png')
screenshot.save(path)
const fileUrl = pathToFileURL(path).toString()

// Open the screenshot in the image viewer
App.exactName(imageViewer).open(fileUrl, FocusPolicy.Steal, Visibility.Show, true)

console.log('Done!')
