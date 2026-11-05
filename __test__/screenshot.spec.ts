import { test, expect } from 'vitest'

import { Machine, canSee } from '../index'

test('screenshot_cropped_rejects_zero_width', () => {
  expect(() => Machine.local().screenshotCropped(0, 0, 0, 100, true)).toThrow(/greater than 0/)
})

test('screenshot_cropped_rejects_zero_height', () => {
  expect(() => Machine.local().screenshotCropped(0, 0, 100, 0, true)).toThrow(/greater than 0/)
})

test('screenshot_cropped_rejects_zero_width_and_height', () => {
  expect(() => Machine.local().screenshotCropped(0, 0, 0, 0, false)).toThrow(/greater than 0/)
})

test.skipIf(canSee() !== null)('screenshot_cropped_small_region', () => {
  const screenshot = Machine.local().screenshotCropped(10, 10, 50, 50, true)
  expect(screenshot.base64().length).toBeGreaterThan(0)
})

test.skipIf(canSee() !== null)('screenshot_of_the_machine_screen', () => {
  const screenshot = Machine.local().screenFromMouse().screenshot(true)
  expect(screenshot.base64().length).toBeGreaterThan(0)
})

test.skipIf(canSee() !== null)('screenshot_of_the_whole_desktop_spans_every_display', () => {
  const machine = Machine.local()
  const desktop = machine
    .screens()
    .map((screen) => {
      const { left, top, right, bottom } = screen.boundingBox()
      return { left, top, right, bottom }
    })
    .reduce((desktop, box) => ({
      left: Math.min(desktop.left, box.left),
      top: Math.min(desktop.top, box.top),
      right: Math.max(desktop.right, box.right),
      bottom: Math.max(desktop.bottom, box.bottom),
    }))
  const screenshot = machine.screenshot(true)
  // The capture is as large as the arrangement, so a point in it maps back
  // to global coordinates like any capture.
  expect(screenshot.dimensions).toEqual([desktop.right - desktop.left, desktop.bottom - desktop.top])
  expect(screenshot.base64().length).toBeGreaterThan(0)
})
