import { test, expect } from 'vitest'

import { DecisionModel, TokenUsage, choice, noul, score } from '../index'

// `default()` only returns a provider whose credentials are available, and CI
// has none. The bundled alias resolves either way. These tests return before
// any request, so they never need OPENROUTER_API_KEY.
function decisionsModel() {
  return DecisionModel.openrouterJev()
}

test('question names are properties of the answers', () => {
  const answerShape = (model: DecisionModel, email: string) => {
    const answers = model.decide(email, {
      urgent: noul('Does the message convey urgency?'),
      dept: choice('Which team should handle this?', {
        billing: 'Payments, invoicing, refunds',
        technical: null,
      }),
      severity: score('How severe is this?', ['low', 'medium', 'high']),
    })
    const probability: number = answers.urgent.probability
    const picked: string = answers.dept.choice
    const level: string = answers.severity.nearest()
    const [lower, upper]: [string, string] = answers.severity.bounds()
    const usage: TokenUsage | null = answers.usage
    // @ts-expect-error urgent is a noul answer
    const notAChoice: string = answers.urgent.choice
    return { probability, picked, level, lower, upper, usage, notAChoice }
  }
  expect(typeof answerShape).toBe('function')
})

test('DecisionModel exposes factories and decide', () => {
  expect(typeof DecisionModel.default).toBe('function')
  expect(typeof DecisionModel.byAlias).toBe('function')
  expect(typeof DecisionModel.openrouterJev).toBe('function')
  expect(typeof DecisionModel.availableAliases).toBe('function')
  expect(typeof DecisionModel.prototype.decide).toBe('function')
  expect(typeof DecisionModel.prototype.checkAuth).toBe('function')
  expect(typeof DecisionModel.prototype.maxChoiceOptions).toBe('function')
  expect(typeof DecisionModel.prototype.maxScoreLevels).toBe('function')
  expect(Object.getOwnPropertyDescriptor(DecisionModel.prototype, 'name')?.get).toBeTypeOf('function')
})

test('decide reads each answer from the name it was given', () => {
  const model = decisionsModel()
  const questions = {
    dept: choice('Which team should handle this?', { only: '' }),
  }
  const bad: any = 1
  expect(() => model.decide(bad, questions)).toThrow(/string, an object, or an array/)

  const first = model.decide('hello', questions)
  const second = model.decide('hello', questions)
  expect(first.dept.choice).toBe('only')
  expect(second.dept.choice).toBe('only')
  expect(first.usage).toBeNull()
})

test('a rejected question is reported by its name', () => {
  const model = decisionsModel()
  expect(() => model.decide('hello', {})).toThrow(/at least one question/)
  expect(() => model.decide('hello', { urgent: noul('   ') })).toThrow(/"urgent"/)
  expect(() => model.decide('hello', { severity: score('How severe is this?', ['only']) })).toThrow(/"severity"/)
})

test('usage is reserved for token usage', () => {
  const model = decisionsModel()
  expect(() => model.decide('hello', { usage: noul('nope') })).toThrow(/usage/)
})

test('DecisionModel.byAlias rejects unknown aliases', () => {
  expect(() => DecisionModel.byAlias('definitely-not-a-real-alias')).toThrow(/alias/)
})

test('DecisionModel.availableAliases returns a sorted, deduplicated string array', () => {
  const aliases = DecisionModel.availableAliases()
  expect(Array.isArray(aliases)).toBe(true)
  for (const alias of aliases) expect(typeof alias).toBe('string')
  const sorted = [...aliases].sort()
  expect(aliases).toEqual(sorted)
  expect(new Set(aliases).size).toBe(aliases.length)
})
