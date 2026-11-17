import { describe, test, expect } from 'vitest'

import { canControl, canSee, canUseAccessibility } from '../index'

describe('capabilities', () => {
  test('each capability is null or an error', () => {
    for (const found of [canSee(), canUseAccessibility(), canControl()]) {
      if (found !== null) {
        expect(found).toEqual({ kind: expect.any(String), reason: expect.any(String) })
      }
    }
  })

  test('control needs accessibility', () => {
    // Element actions go through the accessibility API.
    if (canUseAccessibility() !== null) expect(canControl()).not.toBeNull()
  })
})
