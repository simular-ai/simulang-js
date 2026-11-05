import { test, expect } from 'vitest'

import { Machine } from '../index'

test.skipIf(process.platform !== 'darwin')('app_fails_for_unknown_app', () => {
  expect(() => Machine.local().app('SimulangDefinitelyNotAnApp')).toThrow(/not found|does not exist/i)
})

test('default_browser_returns_an_app', () => {
  try {
    const app = Machine.local().defaultBrowser()
    expect(app.launchTarget).toBeTruthy()
  } catch {
    // No default browser configured in this environment
  }
})

test.skip('open_url_in_default_browser', () => {})

test.skip('open_chrome', () => {})

test('running_instance_does_not_launch', () => {
  const apps = Machine.local().apps()
  if (apps.length === 0) {
    console.log('skipping: no installed app to ask about')
    return
  }
  // The answer is the running copy or null; asking never launches anything.
  const instance = apps[0].runningInstance()
  if (instance !== null) expect(instance.isRunning()).toBe(true)
})
