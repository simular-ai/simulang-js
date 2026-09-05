import { test, expect } from 'vitest'

import { Machine } from '../index'

test('main_screen_has_non_zero_bounding_box', () => {
  const screen = Machine.local().mainScreen()
  const bb = screen.boundingBox()
  expect(bb.right).toBeGreaterThan(bb.left)
  expect(bb.bottom).toBeGreaterThan(bb.top)
})

test('bounding_box_is_stable_for_same_screen_instance', () => {
  const screen = Machine.local().mainScreen()
  const first = screen.boundingBox()
  const second = screen.boundingBox()
  expect(first.equals(second)).toBe(true)
})

test('all_returns_at_least_one_screen', () => {
  const screens = Machine.local().screens()
  expect(screens.length).toBeGreaterThan(0)
  for (const s of screens) {
    const bb = s.boundingBox()
    expect(bb.right).toBeGreaterThan(bb.left)
    expect(bb.bottom).toBeGreaterThan(bb.top)
  }
})

test.skip('mouse_location_screen_returns_non_zero_bounding_box', () => {
  // screenFromMouse panics without accessibility permission
})
