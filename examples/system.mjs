// Run: node examples/system.mjs
// This example opens a URL in the default browser.

import { Machine, FocusPolicy, Visibility } from '@simular-ai/simulang-js'

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

// Open a URL in the default browser.
machine.defaultBrowser().open('https://example.com', FocusPolicy.Steal, Visibility.Show, true)

// Open a specific app by exact name (macOS example).
// machine.app('Safari').open(null, FocusPolicy.Steal, Visibility.Show, false)
