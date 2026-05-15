import { test, expect } from 'vitest'

import { Screen } from '../index'

test('main_screen_returns_non_zero_dimensions', () => {
  const screen = Screen.mainScreen()
  const [, , width, height] = screen.dimensions()
  expect(width).toBeGreaterThan(0)
  expect(height).toBeGreaterThan(0)
})

test('dimensions_are_stable_for_same_screen_instance', () => {
  const screen = Screen.mainScreen()
  const first = screen.dimensions()
  const second = screen.dimensions()
  expect(first).toEqual(second)
})

test.skip('mouse_location_screen_returns_non_zero_dimensions', () => {
  // fromCurrentMouseLocation panics without accessibility permission
})
