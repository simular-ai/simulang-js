import { test, expect } from 'vitest'

import { Machine, screen } from '../index'

test('primary_screen_has_non_zero_bounding_box', () => {
  const screen = Machine.local().primaryScreen()
  const bb = screen.boundingBox()
  expect(bb.right).toBeGreaterThan(bb.left)
  expect(bb.bottom).toBeGreaterThan(bb.top)
})

test('bounding_box_is_stable_for_same_screen_instance', () => {
  const screen = Machine.local().primaryScreen()
  const first = screen.boundingBox()
  const second = screen.boundingBox()
  expect(first.equals(second)).toBe(true)
})

test('all_returns_at_least_one_screen', () => {
  const screens = Machine.local().screens()
  expect(screens.length).toBeGreaterThan(0)
  for (const s of screens) {
    const bb = s.boundingBox()
    expect(bb.right).toBeGreaterThan(bb.left)
    expect(bb.bottom).toBeGreaterThan(bb.top)
  }
})

test.skip('mouse_location_screen_returns_non_zero_bounding_box', () => {
  // screenFromMouse panics without accessibility permission
})

test('screens_are_numbered_from_the_primary', () => {
  const machine = Machine.local()
  const screens = machine.screens()
  expect(screens.length).toBeGreaterThan(0)
  expect(screens[0].index()).toBe(0)
  expect(screens[0].id).toBe(0)
  expect(machine.primaryScreen().index()).toBe(0)
  for (const [i, screen] of screens.entries()) expect(screen.index()).toBe(i)
})

test('a_screen_is_found_by_its_index_and_compares_by_it', () => {
  const machine = Machine.local()
  const first = screen(machine, 0)
  expect(first).not.toBeNull()
  expect(first!.equals(machine.primaryScreen())).toBe(true)
  expect(first!.equals(machine.screens()[0])).toBe(true)
  expect(String(first)).toBe('Screen (index 0)')
  expect(screen(machine, 9999)).toBeNull()
})

test('a_screen_is_found_by_its_unique_id', () => {
  const machine = Machine.local()
  expect(screen(machine, 'nothing-reports-this-id')).toBeNull()
  for (const byIndex of machine.screens()) {
    const unique = byIndex.uniqueId()
    if (unique === null) continue
    const found = screen(machine, unique)
    expect(found).not.toBeNull()
    expect(found!.id).toBe(unique)
    expect(found!.index()).toBe(byIndex.index())
    // A unique id handle and an index handle name the same display but
    // aren't equal: equality is by the stored id.
    expect(found!.equals(byIndex)).toBe(false)
    expect(String(found)).toBe(`Screen (unique id ${unique})`)
  }
})
