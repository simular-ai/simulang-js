import { expect, test } from 'vitest'

import { Direction, Key, Machine } from '../index'

// Named keys press the key. On Linux that needs a display connection, the
// same way enigo's own key test does.
const linuxWithDisplay = process.platform === 'linux' && Boolean(process.env.DISPLAY)

test.skipIf(!linuxWithDisplay)('linux named keys do not throw', () => {
  const machine = Machine.local()
  expect(() => machine.key(Key.S, Direction.Click)).not.toThrow()
  expect(() => machine.key(Key.Num1, Direction.Click)).not.toThrow()
  expect(() => machine.key(Key.OEMComma, Direction.Click)).not.toThrow()
})

test.skipIf(process.platform !== 'darwin')('macos named keys do not throw', () => {
  const machine = Machine.local()
  expect(() => machine.key(Key.OEMMinus, Direction.Click)).not.toThrow()
  expect(() => machine.key(Key.LWin, Direction.Click)).not.toThrow()
})
