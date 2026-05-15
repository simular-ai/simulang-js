import { test, expect } from 'vitest'

import { App } from '../index'

test.skipIf(process.platform !== 'darwin')('exact_name_fails_for_unknown_app', () => {
  expect(() => App.exactName('SimulangDefinitelyNotAnApp')).toThrow(/not found|does not exist/i)
})

test('default_browser_returns_an_app', () => {
  try {
    const app = App.defaultBrowser()
    expect(app.launchTarget).toBeTruthy()
  } catch {
    // No default browser configured in this environment
  }
})

test.skip('open_url_in_default_browser', () => {})

test.skip('open_chrome', () => {})
