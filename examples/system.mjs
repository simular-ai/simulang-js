// Run: node examples/system.mjs
// This example opens a URL in the default browser.

import { App, FocusPolicy, Visibility } from '@simular-ai/simulang-js'

// Open a URL in the default browser.
App.defaultBrowser().open('https://example.com', FocusPolicy.Steal, Visibility.Show, true)

// Open a specific app (macOS example).
// App.exactName('Safari').open(null, FocusPolicy.Steal, Visibility.Show, false)
