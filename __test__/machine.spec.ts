import { test, expect } from 'vitest'

import { AndroidExtras, Image, Machine } from '../index'

import { SCREENSHOT_CROPPED_BASE64 } from './consts'

test('local_returns_a_machine', () => {
  const m = Machine.local()
  expect(m).toBeTruthy()
  expect(['macos', 'windows', 'linux']).toContain(m.os)
  expect(m.id.length).toBeGreaterThan(0)
})

// ── apps ──

test('fuzzy_app_rejects_empty_query', () => {
  expect(() => Machine.local().fuzzyApp('')).toThrow(/empty/)
})

test('fuzzy_app_rejects_whitespace_query', () => {
  expect(() => Machine.local().fuzzyApp('   \t\n  ')).toThrow(/empty/)
})

test('fuzzy_app_returns_err_when_no_close_match', () => {
  expect(() => Machine.local().fuzzyApp('zzzz_nonexistent_application_name_123456789')).toThrow()
})

test('apps_returns_nonempty_array', () => {
  const apps = Machine.local().apps()
  expect(apps.length).toBeGreaterThan(0)
})

// Google does not ship a Linux arm64 build of Chrome, and the
// `ubuntu-24.04-arm` GitHub runner image explicitly omits it. The fuzzy-search
// smoke test still runs on macOS, Windows, and Linux x86_64, so coverage is
// unaffected.
const chromeUnavailable = process.platform === 'linux' && process.arch === 'arm64'

test.skipIf(chromeUnavailable)('fuzzy_app_matches_exact_name', () => {
  const appName = process.platform === 'darwin' ? 'Google Chrome' : 'Chrome'
  const app = Machine.local().fuzzyApp(appName)
  expect(app.canonicalName).toBe('Google Chrome')
})

// ── input (real input needs OS permissions; assert the binding surface) ──

test('machine binding exposes flat input methods', () => {
  expect(typeof Machine.prototype.mouseButton).toBe('function')
  expect(typeof Machine.prototype.moveMouse).toBe('function')
  expect(typeof Machine.prototype.scroll).toBe('function')
  expect(typeof Machine.prototype.mouseLocation).toBe('function')
  expect(typeof Machine.prototype.typeText).toBe('function')
  expect(typeof Machine.prototype.key).toBe('function')
  expect(typeof Machine.prototype.keyUnicode).toBe('function')
  expect(typeof Machine.prototype.keyRaw).toBe('function')
  expect(typeof Machine.prototype.asAndroid).toBe('function')
})

test('as_android_is_null_for_the_local_machine', () => {
  expect(Machine.local().asAndroid()).toBeNull()
})

test('android_extras binding exposes phone-only methods', () => {
  expect(typeof AndroidExtras.prototype.notifications).toBe('function')
  expect(typeof AndroidExtras.prototype.quickSettings).toBe('function')
  expect(typeof AndroidExtras.prototype.collapseShade).toBe('function')
  expect(typeof AndroidExtras.prototype.landscape).toBe('function')
  expect(typeof AndroidExtras.prototype.portrait).toBe('function')
  expect(typeof AndroidExtras.prototype.setPackageEnabled).toBe('function')
})

// ── clipboard ──

test.sequential('set_clipboard_string_then_get', () => {
  const m = Machine.local()
  m.setClipboardString('clipboard test content')
  expect(m.getClipboardString()).toBe('clipboard test content')
})

test.sequential('set_clipboard_string_then_clear', () => {
  const m = Machine.local()
  m.setClipboardString('clipboard test content')
  expect(m.getClipboardString()).toBe('clipboard test content')
  m.clearClipboard()
  expect(m.getClipboardString()).toBeNull()
})

test.sequential('clear_clipboard', () => {
  const m = Machine.local()
  m.clearClipboard()
  expect(m.getClipboardString()).toBeNull()
})

test.sequential('set_clipboard_string_overwrites_previous', () => {
  const m = Machine.local()
  m.setClipboardString('first')
  const previous = m.setClipboardString('second')
  expect(previous.text).toBe('first')
  expect(m.getClipboardString()).toBe('second')
})

test.sequential('set_clipboard_string_with_unicode_roundtrip', () => {
  const m = Machine.local()
  const text = 'Unicode: 日本語 café naïve'
  m.setClipboardString(text)
  expect(m.getClipboardString()).toBe(text)
})

test.sequential('set_clipboard_image_overwrites_string', () => {
  const m = Machine.local()
  m.setClipboardString('text before image')
  const img = Image.fromBase64(SCREENSHOT_CROPPED_BASE64)
  const previous = m.setClipboardImage(img)
  expect(previous.text).toBe('text before image')
  expect(m.getClipboardString()).toBeNull()
})

test.sequential.skip('paste_text', () => {})

test.sequential.skip('paste_text_restores_previous_content', () => {})
