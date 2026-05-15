import { test, expect } from 'vitest'

import { KeyboardController } from '../index'

test('constructor_creates_instance', () => {
  const kc = new KeyboardController()
  expect(kc).toBeTruthy()
})

test.skip('keyboard_simulates_input', () => {})
