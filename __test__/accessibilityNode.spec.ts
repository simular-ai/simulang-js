import { test, expect } from 'vitest'

import { AccessibilityNode, AccessibilitySnapshot, Window } from '../index'

// Live accessibility trees need OS permissions and a foreground UI, so these
// only assert the binding surface (same pattern as the Image prototype test).
test('accessibility node binding exposes handle-walk APIs', () => {
  expect(typeof AccessibilityNode.prototype.children).toBe('function')
  expect(typeof AccessibilityNode.prototype.childrenCollapsed).toBe('function')
  expect(Object.getOwnPropertyDescriptor(AccessibilityNode.prototype, 'isVisible')?.get).toBeTypeOf('function')
})

test('window binding exposes accessibility subtree root', () => {
  expect(typeof Window.prototype.node).toBe('function')
})

test('accessibility snapshot binding mirrors indexed snapshot APIs', () => {
  expect(typeof AccessibilitySnapshot.fromNode).toBe('function')
  expect(typeof AccessibilitySnapshot.fromWindow).toBe('function')
  expect(typeof AccessibilitySnapshot.prototype.len).toBe('function')
  expect(typeof AccessibilitySnapshot.prototype.isEmpty).toBe('function')
  expect(typeof AccessibilitySnapshot.prototype.toStringWith).toBe('function')
  expect(typeof AccessibilitySnapshot.prototype.node).toBe('function')
  expect(typeof AccessibilitySnapshot.prototype.boundingBox).toBe('function')
  expect(typeof AccessibilitySnapshot.prototype.setFocus).toBe('function')
})
