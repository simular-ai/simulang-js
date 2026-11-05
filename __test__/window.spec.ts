import { test, expect } from 'vitest'

import { Machine, window } from '../index'

test('a_window_is_found_again_by_its_id', () => {
  const machine = Machine.local()
  const one = machine.windows().find((w) => w.id !== null)
  if (one === undefined) {
    console.log('skipping: no window with an id on this machine')
    return
  }
  const found = window(machine, one.id!)
  expect(found).not.toBeNull()
  expect(found!.id).toBe(one.id)
  expect(found!.pid).toBe(one.pid)
  expect(window(machine, 0)).toBeNull()
})
