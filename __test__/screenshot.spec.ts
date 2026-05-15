import { test, expect } from 'vitest'

import { Screen, screenshotCropped, screenshotFull, hasScreenCapturePermission } from '../index'

test('screenshot_cropped_rejects_zero_width', () => {
  expect(() => screenshotCropped(0, 0, 0, 100, true)).toThrow(/greater than 0/)
})

test('screenshot_cropped_rejects_zero_height', () => {
  expect(() => screenshotCropped(0, 0, 100, 0, true)).toThrow(/greater than 0/)
})

test('screenshot_cropped_rejects_zero_width_and_height', () => {
  expect(() => screenshotCropped(0, 0, 0, 0, false)).toThrow(/greater than 0/)
})

const canCapture = hasScreenCapturePermission()

test.skipIf(!canCapture)('screenshot_cropped_small_region', () => {
  const screenshot = screenshotCropped(10, 10, 50, 50, true)
  expect(screenshot.base64().length).toBeGreaterThan(0)
})

test.skipIf(!canCapture)('screenshot_full_with_main_screen', () => {
  const screenshot = screenshotFull(true, Screen.mainScreen())
  expect(screenshot.base64().length).toBeGreaterThan(0)
})
