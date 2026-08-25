import { test, expect } from 'vitest'

import { StateSatisfiesModel } from '../index'

test('StateSatisfiesModel exposes static factory + introspection methods', () => {
  expect(typeof StateSatisfiesModel.default).toBe('function')
  expect(typeof StateSatisfiesModel.byAlias).toBe('function')
  expect(typeof StateSatisfiesModel.simularStateSatisfies).toBe('function')
  expect(typeof StateSatisfiesModel.availableAliases).toBe('function')
})

test('StateSatisfiesModel.prototype exposes stateSatisfies, checkAuth, and name accessor', () => {
  expect(typeof StateSatisfiesModel.prototype.stateSatisfies).toBe('function')
  expect(typeof StateSatisfiesModel.prototype.checkAuth).toBe('function')
  expect(Object.getOwnPropertyDescriptor(StateSatisfiesModel.prototype, 'name')?.get).toBeTypeOf('function')
})

test('StateSatisfiesModel.byAlias rejects unknown aliases', () => {
  expect(() => StateSatisfiesModel.byAlias('definitely-not-a-real-state-alias')).toThrow(/alias/)
})

test('StateSatisfiesModel.availableAliases returns a sorted, deduplicated string array', () => {
  const aliases = StateSatisfiesModel.availableAliases()
  expect(Array.isArray(aliases)).toBe(true)
  for (const alias of aliases) expect(typeof alias).toBe('string')
  const sorted = [...aliases].sort()
  expect(aliases).toEqual(sorted)
  expect(new Set(aliases).size).toBe(aliases.length)
})
