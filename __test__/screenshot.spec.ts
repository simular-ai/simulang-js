import { test, expect } from 'vitest'

import { Machine, hasScreenCapturePermission } from '../index'

test('screenshot_cropped_rejects_zero_width', () => {
  expect(() => Machine.local().screenshotCropped(0, 0, 0, 100, true)).toThrow(/greater than 0/)
})

test('screenshot_cropped_rejects_zero_height', () => {
  expect(() => Machine.local().screenshotCropped(0, 0, 100, 0, true)).toThrow(/greater than 0/)
})

test('screenshot_cropped_rejects_zero_width_and_height', () => {
  expect(() => Machine.local().screenshotCropped(0, 0, 0, 0, false)).toThrow(/greater than 0/)
})

const canCapture = hasScreenCapturePermission()

test.skipIf(!canCapture)('screenshot_cropped_small_region', () => {
  const screenshot = Machine.local().screenshotCropped(10, 10, 50, 50, true)
  expect(screenshot.base64().length).toBeGreaterThan(0)
})

test.skipIf(!canCapture)('screenshot_of_the_machine_screen', () => {
  const screenshot = Machine.local().screenFromMouse().screenshot(true)
  expect(screenshot.base64().length).toBeGreaterThan(0)
})
