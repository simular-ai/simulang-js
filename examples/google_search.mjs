// Run: node examples/google_search.mjs
// This example opens Google in Chrome.

import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { pathToFileURL } from 'node:url'
import { Machine, Key, Direction, FocusPolicy, Visibility } from '@simular-ai/simulang-js'

const browser = process.platform === 'darwin' ? 'Google Chrome' : 'Chrome'
const imageViewer = process.platform === 'darwin' ? 'Preview' : 'Chrome'
const modifierKey = process.platform === 'darwin' ? Key.Meta : Key.Control

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

// Open a URL in Chrome
machine.app(browser).open('https://google.com', FocusPolicy.Steal, Visibility.Show, true)
console.log('Launched Google Chrome')

// Wait two seconds to give Chrome time to start
console.log('Waiting two seconds...')
await new Promise((resolve) => setTimeout(resolve, 2000))

// Independent of keyboard layout
machine.keyUnicode('z', Direction.Click)
await new Promise((resolve) => setTimeout(resolve, 1000))

// Open the accessibility menu
machine.key(modifierKey, Direction.Press)
machine.keyUnicode('l', Direction.Click)
machine.key(modifierKey, Direction.Release)
await new Promise((resolve) => setTimeout(resolve, 500))
machine.typeText('chrome://accessibility')
machine.key(Key.Return, Direction.Click)
await new Promise((resolve) => setTimeout(resolve, 2000))

// Enable the accessibility tree for Chrome
machine.foregroundApp().enableAccessibility()
await new Promise((resolve) => setTimeout(resolve, 5000))

// Take a screenshot of the machine's screen and save it to a file
const screenshot = machine.screenFromMouse().screenshot(true)
const path = join(tmpdir(), 'screenshot.png')
screenshot.save(path)
const fileUrl = pathToFileURL(path).toString()

// Open the screenshot in the image viewer
machine.app(imageViewer).open(fileUrl, FocusPolicy.Steal, Visibility.Show, true)

console.log('Done!')
