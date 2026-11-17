import { execFileSync } from 'node:child_process'
import { setTimeout as sleep } from 'node:timers/promises'

import { expect, test } from 'vitest'

import { type AccessibilityNodeJs, AccessibilityTree, AriaRole, Machine } from '../index'

// Locks the screen by putting the display to sleep, so System Settings >
// Lock Screen must require the password immediately. Someone has to unlock
// the Mac afterwards, so the test runs only when asked for:
//
//   SIMULANG_TEST_INTERACTIVE=1 npx vitest run __test__/lockedScreen.spec.ts
const interactive = process.platform === 'darwin' && process.env.SIMULANG_TEST_INTERACTIVE === '1'

function screenIsLocked(): boolean {
  return execFileSync('ioreg', ['-n', 'Root', '-d1'], { encoding: 'utf8' }).includes('"IOConsoleLocked" = Yes')
}

function applications(node: AccessibilityNodeJs): number {
  const own = node.role === AriaRole.Application ? 1 : 0
  return node.children.reduce((count, child) => count + applications(child), own)
}

// While locked, an application stands in for each of its windows and lists
// itself as a child. A snapshot that follows that overflows the stack, which
// ends this process: the test fails by not finishing.
test.runIf(interactive)('snapshot_shows_an_application_once_while_the_screen_is_locked', async () => {
  execFileSync('pmset', ['displaysleepnow'])
  for (let waited = 0; !screenIsLocked() && waited < 10_000; waited += 200) await sleep(200)
  const locked = screenIsLocked()
  // The windows are looked up while locked: that is when an application
  // stands in for them.
  const snapshots = locked
    ? Machine.local()
        .windows()
        .flatMap((window) => {
          try {
            return [AccessibilityTree.fromWindow(window).snapshot(false)]
          } catch {
            // A window that cannot be read while locked throws. That is not
            // what this test is about, as long as some window can be read.
            return []
          }
        })
    : []
  // Wake the display, so the lock screen shows.
  execFileSync('caffeinate', ['-u', '-t', '1'])

  expect(locked, 'the display sleeps but the screen did not lock').toBe(true)
  expect(snapshots.length, 'no window could be read while locked').toBeGreaterThan(0)
  for (const snapshot of snapshots) expect(applications(snapshot)).toBeLessThanOrEqual(1)
})
