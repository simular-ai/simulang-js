import { test, expect } from 'vitest'

import { BoundingBox, Image, Machine, searchRelative, type AccessibilityNode } from '../index'

import { TINY_PNG_BASE64 } from './consts'

const box = (left: number, top: number, right: number, bottom: number): BoundingBox =>
  new BoundingBox(left, top, right, bottom)

test('constructor rejects a degenerate box', () => {
  expect(() => box(8, 8, 8, 12)).toThrow()
  expect(() => box(8, 8, 12, 8)).toThrow()
  expect(() => box(8, 8, 8, 8)).toThrow()
  expect(() => box(3, 7, 3, 11)).toThrow()
})

test('constructor stores exclusive-edge corners', () => {
  const b = box(8, 16, 48, 64)
  expect(b.left).toBe(8)
  expect(b.top).toBe(16)
  expect(b.right).toBe(48)
  expect(b.bottom).toBe(64)
})

test('equals compares corners, not object identity', () => {
  const a = box(8, 16, 48, 64)
  const same = box(8, 16, 48, 64)
  const other = box(8, 16, 48, 65)
  expect(a.equals(a)).toBe(true)
  expect(a.equals(same)).toBe(true)
  expect(a.equals(other)).toBe(false)
  expect(a === same).toBe(false)
})

test('width height center and area match exclusive-edge size', () => {
  const b = box(8, 16, 48, 64)
  expect(b.width).toBe(40)
  expect(b.height).toBe(48)
  expect(b.center()).toEqual([28, 40])
  expect(b.area()).toBe(1920)
})

test('fromXywh is origin plus size', () => {
  expect(BoundingBox.fromXywh(8, 16, 40, 48).equals(box(8, 16, 48, 64))).toBe(true)
  expect(() => BoundingBox.fromXywh(8, 16, 0, 48)).toThrow()
  expect(() => BoundingBox.fromXywh(8, 16, 40, -1)).toThrow()
})

test('overlapArea is the intersecting rectangle or zero', () => {
  expect(box(0, 0, 10, 10).overlapArea(box(5, 5, 15, 15))).toBe(25)
  expect(box(0, 0, 10, 10).overlapArea(box(10, 0, 20, 10))).toBe(0)
})

test('toString uses the Playwright x y width height shape', () => {
  expect(box(0, 0, 1920, 1080).toString()).toBe('{x: 0, y: 0, width: 1920, height: 1080}')
  expect(String(box(-2560, -200, 0, 1240))).toBe('{x: -2560, y: -200, width: 2560, height: 1440}')
})

test('containsPoint covers inclusive origin and exclusive far edges', () => {
  const a = box(20, 30, 50, 70)
  expect(a.containsPoint(20, 30)).toBe(true)
  expect(a.containsPoint(49, 69)).toBe(true)
  expect(a.containsPoint(50, 30)).toBe(false)
  expect(a.containsPoint(20, 70)).toBe(false)
  expect(a.containsPoint(50, 70)).toBe(false)
  expect(a.containsPoint(19, 30)).toBe(false)
  expect(a.containsPoint(20, 29)).toBe(false)

  const secondary = box(-80, -40, 0, 0)
  expect(secondary.containsPoint(-80, -40)).toBe(true)
  expect(secondary.containsPoint(-1, -1)).toBe(true)
  expect(secondary.containsPoint(0, -20)).toBe(false)
  expect(secondary.containsPoint(-40, 0)).toBe(false)
})

test('containsBox is reflexive and directional', () => {
  const outer = box(4, 4, 28, 28)
  const inner = box(8, 10, 16, 18)
  expect(outer.containsBox(inner)).toBe(true)
  expect(inner.isContainedIn(outer)).toBe(true)
  expect(inner.containsBox(outer)).toBe(false)
  expect(outer.isContainedIn(inner)).toBe(false)
  expect(outer.containsBox(outer)).toBe(true)
  expect(outer.isContainedIn(outer)).toBe(true)
  const overlap = box(16, 16, 36, 36)
  expect(outer.containsBox(overlap)).toBe(false)
  expect(overlap.isContainedIn(outer)).toBe(false)
})

test('shortestDistanceTo is zero on overlap or exclusive-edge touch', () => {
  const a = box(4, 4, 24, 24)
  expect(a.shortestDistanceTo(box(12, 12, 32, 32))).toBe(0)
  expect(a.shortestDistanceTo(box(24, 4, 40, 24))).toBe(0)
})

test('shortestDistanceTo reports axis-aligned and diagonal gaps', () => {
  const a = box(4, 4, 24, 24)
  const beside = box(36, 4, 52, 24)
  expect(a.shortestDistanceTo(beside)).toBe(12)
  expect(a.shortestDistanceTo(box(4, 36, 24, 52))).toBe(12)
  expect(a.shortestDistanceTo(box(36, 36, 52, 52))).toBeCloseTo(Math.hypot(12, 12))
  expect(a.shortestDistanceTo(beside)).toBe(beside.shortestDistanceTo(a))
})

test('shortestDistanceTo far-apart boxes does not overflow', () => {
  const low = box(-2_147_483_648, -2_147_483_648, -2_147_483_638, -2_147_483_638)
  const high = box(2_147_483_637, 2_147_483_637, 2_147_483_647, 2_147_483_647)
  const gap = 2_147_483_637 - -2_147_483_638
  const d = low.shortestDistanceTo(high)
  expect(Number.isFinite(d)).toBe(true)
  expect(Math.abs(d - Math.hypot(gap, gap))).toBeLessThan(1)
})

test('directions select the expected side', () => {
  const anchor = box(40, 80, 120, 140)
  const above = box(40, 0, 120, 40)
  const below = box(40, 200, 120, 240)
  const left = box(0, 80, 24, 140)
  const right = box(200, 80, 260, 140)
  expect(above.isAbove(anchor)).toBe(true)
  expect(below.isBelow(anchor)).toBe(true)
  expect(left.isLeftOf(anchor)).toBe(true)
  expect(right.isRightOf(anchor)).toBe(true)
  expect(above.isBelow(anchor)).toBe(false)
  expect(above.isLeftOf(anchor)).toBe(false)
  expect(above.isRightOf(anchor)).toBe(false)
  expect(left.sameRow(anchor, 8)).toBe(true)
  expect(right.sameRow(anchor, 8)).toBe(true)
  expect(above.sameColumn(anchor, 8)).toBe(true)
  expect(below.sameColumn(anchor, 8)).toBe(true)
  expect(left.sameColumn(anchor, 8)).toBe(false)
  expect(above.sameRow(anchor, 8)).toBe(false)
  expect(above.isAbove(anchor) && above.sameColumn(anchor, 8)).toBe(true)
  expect(left.isAbove(anchor)).toBe(false)
  expect(left.sameRow(anchor, 8)).toBe(true)
})

test('adjacent exclusive edges count as beside', () => {
  const anchor = box(40, 80, 120, 140)
  expect(box(40, 40, 120, 80).isAbove(anchor)).toBe(true)
  expect(box(40, 140, 120, 180).isBelow(anchor)).toBe(true)
  expect(box(0, 80, 40, 140).isLeftOf(anchor)).toBe(true)
  expect(box(120, 80, 180, 140).isRightOf(anchor)).toBe(true)
})

test('overlapping boxes are not directional', () => {
  const anchor = box(40, 80, 120, 140)
  const overlappingAbove = box(40, 0, 120, 100)
  const parent = box(0, 0, 200, 180)
  for (const candidate of [overlappingAbove, parent]) {
    expect(candidate.isAbove(anchor)).toBe(false)
    expect(candidate.isBelow(anchor)).toBe(false)
    expect(candidate.isLeftOf(anchor)).toBe(false)
    expect(candidate.isRightOf(anchor)).toBe(false)
  }
  expect(anchor.isContainedIn(parent)).toBe(true)
})

test('sameRow far-apart centers do not overflow', () => {
  const low = box(-2_147_483_648, -2_147_483_648, -2_147_483_638, -2_147_483_638)
  const high = box(2_147_483_637, 2_147_483_637, 2_147_483_647, 2_147_483_647)
  expect(low.sameRow(high, 2_147_483_647)).toBe(false)
  expect(low.sameColumn(high, 2_147_483_647)).toBe(false)
})

test('sameRow uses center y and rejects exact tolerance', () => {
  const anchor = box(40, 80, 120, 140)
  expect(box(0, 92, 32, 142).sameRow(anchor, 8)).toBe(true)
  expect(box(0, 93, 32, 143).sameRow(anchor, 8)).toBe(false)
  expect(box(0, 20, 32, 200).sameRow(anchor, 8)).toBe(true)
})

test('sameColumn uses center x and rejects exact tolerance', () => {
  const anchor = box(40, 80, 120, 140)
  expect(box(47, 0, 127, 24).sameColumn(anchor, 8)).toBe(true)
  expect(box(48, 0, 128, 24).sameColumn(anchor, 8)).toBe(false)
  expect(box(0, 0, 160, 24).sameColumn(anchor, 8)).toBe(true)
})

test('intersects requires interior overlap', () => {
  const a = box(8, 8, 32, 32)
  const beside = box(48, 8, 64, 32)
  expect(a.intersects(box(16, 16, 40, 40))).toBe(true)
  expect(a.intersects(a)).toBe(true)
  expect(box(12, 12, 28, 28).intersects(a)).toBe(true)
  expect(a.intersects(box(32, 8, 48, 32))).toBe(false)
  expect(a.intersects(beside)).toBe(false)
  expect(a.overlapsY(beside)).toBe(true)
  expect(a.overlapsX(beside)).toBe(false)
})

test('overlapsX matches a left-aligned stack whose centers differ', () => {
  const field = box(50, 90, 290, 122)
  const button = box(50, 40, 110, 72)
  expect(button.isAbove(field) && button.overlapsX(field)).toBe(true)
  expect(button.sameColumn(field, 12)).toBe(false)
})

test('below plus overlapsX keeps the stacked control and drops the aside one', () => {
  const field = box(50, 90, 290, 122)
  const stacked = box(90, 140, 170, 172)
  const aside = box(320, 140, 400, 172)
  const above = box(90, 40, 170, 72)
  expect(stacked.isBelow(field) && stacked.overlapsX(field)).toBe(true)
  expect(aside.isBelow(field) && aside.overlapsX(field)).toBe(false)
  expect(above.isBelow(field)).toBe(false)
})

test('searchRelative is a free function', () => {
  expect(typeof searchRelative).toBe('function')
})

test('searchRelative with empty lists does not call relation', () => {
  expect(
    searchRelative([], [], () => {
      throw new Error('should not run')
    }),
  ).toEqual([])
})

function boxedWindowNodes(): AccessibilityNode[] {
  try {
    const out: AccessibilityNode[] = []
    for (const w of Machine.local().windows()) {
      try {
        const n = w.node()
        n.boundingBox()
        out.push(n)
      } catch {
        // window without an accessibility tree or a box
      }
    }
    return out
  } catch {
    return []
  }
}

const windowNodes = boxedWindowNodes()

test.skipIf(windowNodes.length === 0)('searchRelative calls relation with candidate and landmark boxes', () => {
  const [node] = windowNodes
  const expected = node.boundingBox()
  let calls = 0
  const kept = searchRelative([node], [node], (candidate, landmark) => {
    calls += 1
    expect(candidate.equals(expected)).toBe(true)
    expect(landmark.equals(expected)).toBe(true)
    return candidate.containsBox(landmark)
  })
  expect(calls).toBe(1)
  expect(kept).toHaveLength(1)
})

test.skipIf(windowNodes.length === 0)('searchRelative drops when relation is false', () => {
  expect(searchRelative([windowNodes[0]], [windowNodes[0]], () => false)).toEqual([])
})

test.skipIf(windowNodes.length === 0)('searchRelative with empty landmarks skips relation', () => {
  expect(
    searchRelative([windowNodes[0]], [], () => {
      throw new Error('should not run')
    }),
  ).toEqual([])
})

test.skipIf(windowNodes.length === 0)('searchRelative propagates a throwing relation', () => {
  expect(() =>
    searchRelative([windowNodes[0]], [windowNodes[0]], () => {
      throw new Error('relation failed')
    }),
  ).toThrow(/relation failed/)
})

test.skipIf(windowNodes.length < 2)('searchRelative preserves candidate order', () => {
  const [a, b] = windowNodes
  const kept = searchRelative([a, b], [a], () => true)
  expect(kept).toHaveLength(2)
  expect(kept[0].boundingBox().equals(a.boundingBox())).toBe(true)
  expect(kept[1].boundingBox().equals(b.boundingBox())).toBe(true)
})

test('drawBox accepts a BoundingBox', () => {
  const img = Image.fromBase64(TINY_PNG_BASE64)
  img.drawBox(box(0, 0, 1, 1), 1, 255, 0, 0)
})
