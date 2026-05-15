import { test, expect } from 'vitest'

import { System } from '../index'

test('fuzzy_search_rejects_empty_query', () => {
  expect(() => System.fuzzySearch('')).toThrow(/empty/)
})

test('fuzzy_search_rejects_whitespace_query', () => {
  expect(() => System.fuzzySearch('   \t\n  ')).toThrow(/empty/)
})

test('fuzzy_search_returns_err_when_no_close_match', () => {
  expect(() => System.fuzzySearch('zzzz_nonexistent_application_name_123456789')).toThrow()
})

test('list_apps_returns_nonempty_array', () => {
  const apps = System.listApps()
  expect(apps.length).toBeGreaterThan(0)
})

// Google does not ship a Linux arm64 build of Chrome, and the
// `ubuntu-24.04-arm` GitHub runner image explicitly omits it. The fuzzy-search
// smoke test still runs on macOS, Windows, and Linux x86_64, so coverage is
// unaffected.
const chromeUnavailable = process.platform === 'linux' && process.arch === 'arm64'

test.skipIf(chromeUnavailable)('fuzzy_search_matches_exact_name', () => {
  const appName = process.platform === 'darwin' ? 'Google Chrome' : 'Chrome'
  const app = System.fuzzySearch(appName)
  expect(app.canonicalName).toBe('Google Chrome')
})
