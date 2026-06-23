import { test, expect } from 'vitest'

import { AskModel } from '../index'

test('AskModel exposes static factory + introspection methods', () => {
  expect(typeof AskModel.default).toBe('function')
  expect(typeof AskModel.byAlias).toBe('function')
  expect(typeof AskModel.availableAliases).toBe('function')
})

test('AskModel.prototype exposes ask + name accessor', () => {
  expect(typeof AskModel.prototype.ask).toBe('function')
  expect(Object.getOwnPropertyDescriptor(AskModel.prototype, 'name')?.get).toBeTypeOf('function')
})

test('AskModel.byAlias rejects unknown aliases', () => {
  expect(() => AskModel.byAlias('definitely-not-a-real-alias')).toThrow(/alias/)
})

test('AskModel.availableAliases returns a sorted, deduplicated string array', () => {
  const aliases = AskModel.availableAliases()
  expect(Array.isArray(aliases)).toBe(true)
  for (const alias of aliases) expect(typeof alias).toBe('string')
  const sorted = [...aliases].sort()
  expect(aliases).toEqual(sorted)
  expect(new Set(aliases).size).toBe(aliases.length)
})
