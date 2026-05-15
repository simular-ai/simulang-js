import { test, expect } from 'vitest'

import { MouseController } from '../index'

test('constructor_creates_instance', () => {
  const mc = new MouseController()
  expect(mc).toBeTruthy()
})

test.skip('location_returns_coordinates', () => {
  // InputController requires accessibility permission; panics without it
})

test.skip('mouse_simulates_input', () => {})
