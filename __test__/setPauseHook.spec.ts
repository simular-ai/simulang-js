import { Worker } from 'node:worker_threads'

import { describe, test, expect, afterEach } from 'vitest'

import {
  AriaRole,
  Direction,
  Image,
  SamplesBuffer,
  ScreenshotCoordinateType,
  ariaRoleToString,
  keyFromString,
  setPauseHook,
} from '../wrapped'

// 1x1 transparent PNG. Cheap to decode and sandbox-safe; we use it to
// exercise `Image.fromBase64` (static) and `Image.prototype.compress`
// (instance) without touching any OS resource.
const TINY_PNG =
  'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII='

afterEach(() => {
  setPauseHook(null)
})

describe('setPauseHook', () => {
  test('fires once per free-function call', () => {
    let calls = 0
    setPauseHook(() => {
      calls++
    })
    keyFromString('A')
    ariaRoleToString(AriaRole.Button)
    expect(calls).toBe(2)
  })

  test('fires once per class static method call', () => {
    let calls = 0
    setPauseHook(() => {
      calls++
    })
    ScreenshotCoordinateType.absolute()
    Image.fromBase64(TINY_PNG)
    expect(calls).toBe(2)
  })

  test('fires once per instance method call', () => {
    const img = Image.fromBase64(TINY_PNG)
    let calls = 0
    setPauseHook(() => {
      calls++
    })
    img.compress(50)
    img.shrink(1, 1)
    expect(calls).toBe(2)
  })

  test('does not fire on property-access getters or enum reads', () => {
    let calls = 0
    setPauseHook(() => {
      calls++
    })
    const img = Image.fromBase64(TINY_PNG)
    // Above static call counts as 1.
    void img.dimensions // getter — must not increment
    void Direction.Press // enum — must not increment
    expect(calls).toBe(1)
  })

  test('does not fire on plain constructor calls', () => {
    let calls = 0
    setPauseHook(() => {
      calls++
    })
    new SamplesBuffer(1, 16000, [0])
    expect(calls).toBe(0)
  })

  test('clearing with null silences further calls', () => {
    let calls = 0
    setPauseHook(() => {
      calls++
    })
    keyFromString('A')
    expect(calls).toBe(1)

    setPauseHook(null)
    keyFromString('A')
    ScreenshotCoordinateType.absolute()
    Image.fromBase64(TINY_PNG).compress(50)
    expect(calls).toBe(1)
  })

  test('non-function arguments are coerced to null', () => {
    let calls = 0
    setPauseHook(() => {
      calls++
    })
    keyFromString('A')
    expect(calls).toBe(1)

    // @ts-expect-error — exercising the runtime guard
    setPauseHook('not a function')
    keyFromString('A')
    expect(calls).toBe(1)
  })

  test('instances from wrapped classes preserve instanceof both ways', () => {
    const sct = ScreenshotCoordinateType.absolute()
    expect(sct).toBeInstanceOf(ScreenshotCoordinateType)

    const img = Image.fromBase64(TINY_PNG)
    expect(img).toBeInstanceOf(Image)

    const buf = new SamplesBuffer(1, 16000, [0])
    expect(buf).toBeInstanceOf(SamplesBuffer)
  })

  test('the wrapped call blocks until the synchronous hook returns', () => {
    const PAUSE_MS = 120
    setPauseHook(() => {
      // `Atomics.wait` on an unshared, never-notified buffer is the
      // cleanest way to sleep synchronously on the main thread without
      // burning CPU. Mirrors the real log-viewer flow, which blocks
      // inside `wait_if_paused_sync` on a Rust condvar.
      const buf = new Int32Array(new SharedArrayBuffer(4))
      Atomics.wait(buf, 0, 0, PAUSE_MS)
    })

    const start = performance.now()
    const result = keyFromString('A')
    const elapsed = performance.now() - start

    // Sanity: the native call still ran and returned a real value.
    // `keyFromString('A')` resolves to a stable enum constant.
    expect(typeof result).toBe('number')
    // Generous lower bound; allow ~25ms of scheduling jitter.
    expect(elapsed).toBeGreaterThanOrEqual(PAUSE_MS - 25)
  })

  test('the wrapped call resumes when another thread releases the hook', async () => {
    // Cross-thread wakeup test that mirrors log-viewer's reader thread
    // flipping `paused = false` from outside the main JS thread. The
    // hook blocks on a SharedArrayBuffer slot; the worker writes 1 to
    // the slot after ~80ms and notifies. The wrapped simulang-js call
    // must unblock within a reasonable window after the worker fires.
    const sab = new SharedArrayBuffer(4)
    const slot = new Int32Array(sab)
    const RELEASE_AFTER_MS = 80

    const worker = new Worker(
      `
        const { workerData, parentPort } = require('node:worker_threads')
        setTimeout(() => {
          const slot = new Int32Array(workerData.sab)
          Atomics.store(slot, 0, 1)
          Atomics.notify(slot, 0)
          parentPort.postMessage('released')
        }, ${RELEASE_AFTER_MS})
      `,
      { eval: true, workerData: { sab } },
    )

    try {
      setPauseHook(() => {
        // Bail after 5s as a safety net; on success the worker wakes
        // us up well before that.
        Atomics.wait(slot, 0, 0, 5000)
      })

      const start = performance.now()
      keyFromString('A')
      const elapsed = performance.now() - start

      expect(Atomics.load(slot, 0)).toBe(1)
      expect(elapsed).toBeGreaterThanOrEqual(RELEASE_AFTER_MS - 25)
      // Should unblock shortly after release, not wait the full 5s.
      expect(elapsed).toBeLessThan(1000)

      await new Promise<void>((resolve) => worker.once('message', () => resolve()))
    } finally {
      await worker.terminate()
    }
  })
})
