import { test, expect } from 'vitest'

import { Image } from '../index'

test('image binding exposes shrink and compress', () => {
  expect(typeof Image.prototype.shrink).toBe('function')
  expect(typeof Image.prototype.compress).toBe('function')
})
