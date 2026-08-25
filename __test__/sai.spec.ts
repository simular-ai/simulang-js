import { promises as fs } from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { expect, test } from 'vitest'

import type { BoundingBox, GroundingModel } from '../index'
import type { App } from '../sai'

const mockFiles = new Map<string, string>()
const keyboardEvents: Array<{ kind: 'text'; value: string } | { kind: 'key'; key: number; direction: number }> = []
const mouseEvents: Array<
  | { kind: 'move'; x: number; y: number; coordinate: number }
  | { kind: 'button'; button: number; direction: number }
  | { kind: 'scroll'; x: number; y: number }
> = []

// Test-controllable behavior for the foreground AX tree and vision grounding.
let axMatches: Array<{ boundingBox: BoundingBox }> = []
let groundError = false

// One mock machine mirrors the native `Machine.local()` singleton the sai
// module holds: flat mouse/keyboard/clipboard/screenshot methods.
const mockMachine = {
  clipboardValue: '',
  // keyboard
  typeText(value: string): void {
    keyboardEvents.push({ kind: 'text', value })
  },
  key(key: number, direction: number): void {
    keyboardEvents.push({ kind: 'key', key, direction })
  },
  // mouse
  moveMouse(x: number, y: number, coordinate: number): void {
    mouseEvents.push({ kind: 'move', x, y, coordinate })
  },
  mouseButton(button: number, direction: number): void {
    mouseEvents.push({ kind: 'button', button, direction })
  },
  scroll(x: number, y: number): void {
    mouseEvents.push({ kind: 'scroll', x, y })
  },
  // clipboard
  setClipboardString(value: string): { text: string } {
    const previous = this.clipboardValue
    this.clipboardValue = value
    return { text: previous }
  },
  getClipboardString(): string {
    return this.clipboardValue
  },
  pasteText(value: string): void {
    this.clipboardValue = value
  },
  // screens / screenshots
  screenFromMouse(): { screenshot(): { ground(): [number, number]; base64(): string } } {
    return {
      screenshot: () => ({
        ground: (): [number, number] => {
          if (groundError) throw new Error('grounding refused')
          return [12, 34]
        },
        base64: () => 'data:image/png;base64,',
      }),
    }
  },
  screenshotCropped(): { ground(): [number, number]; base64(): string } {
    return {
      ground: (): [number, number] => {
        if (groundError) throw new Error('grounding refused')
        return [99, 88]
      },
      base64: () => 'data:image/png;base64,',
    }
  },
  // apps / windows
  apps(): Array<{ canonicalName: string; launchTarget: string }> {
    return []
  },
  foregroundApp(): Record<string, never> {
    return {}
  },
  fuzzyApp(): never {
    throw new Error('no apps in the mock machine')
  },
  windows(): Array<{ pid: number; title: string }> {
    return []
  },
  // files
  file(
    filePath: string,
    createMissing: boolean,
  ): {
    path(): string
    read(): string
    write(content: string, append: boolean): void
  } {
    if (!createMissing && !mockFiles.has(filePath)) {
      throw new Error(`file not found: ${filePath}`)
    }
    if (createMissing && !mockFiles.has(filePath)) {
      mockFiles.set(filePath, '')
    }
    return {
      path: () => filePath,
      read: () => mockFiles.get(filePath) ?? '',
      write: (content, append) => {
        const next =
          append && mockFiles.has(filePath) && mockFiles.get(filePath)
            ? `${mockFiles.get(filePath)}\n${content}`
            : content
        mockFiles.set(filePath, next)
      },
    }
  },
}

const mockNative = {
  AccessibilityTree: class {
    static fromInstance(): {
      snapshot(): { role: number; name: string; value: string; children: [] }
      findByDescription(): Array<{ boundingBox: BoundingBox }>
    } {
      return {
        snapshot: () => ({ role: 0, name: '', value: '', children: [] }),
        findByDescription: () => axMatches,
      }
    }
  },
  App: class {},
  AskModel: class {
    static response: string | null = null
    static default(): { ask(prompt: string, text?: string | null, images?: unknown[] | null): string } {
      const response = this.response
      return {
        ask: (prompt, text, images) =>
          response ?? JSON.stringify({ prompt, text: text ?? null, images: images ? images.length : 0 }),
      }
    }
  },
  StateSatisfiesModel: class {
    static result = true
    static lastArgs: { condition: string; text: string; image: string } | null = null
    static default(): { stateSatisfies(condition: string, text: string, image: string): boolean } {
      return {
        stateSatisfies: (condition, text, image) => {
          this.lastArgs = { condition, text, image }
          return this.result
        },
      }
    }
  },
  Image: {
    fromBase64: (b64: string): { kind: 'image'; b64: string } => ({ kind: 'image', b64 }),
  },
  Button: { Left: 0, Middle: 1, Right: 2 },
  Coordinate: { Abs: 0, Rel: 1 },
  Direction: { Press: 0, Release: 1, Click: 2 },
  FocusPolicy: { DoNotSteal: 0, Steal: 1 },
  GroundingModel: class {
    static default(): GroundingModel {
      return {} as GroundingModel
    }
  },
  Key: { Meta: 186, Control: 60, Shift: 273, Alt: 40, Return: 262 },
  Machine: class {
    static local(): typeof mockMachine {
      return mockMachine
    }
  },
  Visibility: { Hidden: 0, Show: 1 },
  ariaRoleToString: () => 'generic',
  keyFromString: () => 10,
  legacyOpen: (): void => {},
}

;(globalThis as typeof globalThis & { __SIMULANG_JS_SAI_NATIVE__?: typeof mockNative }).__SIMULANG_JS_SAI_NATIVE__ =
  mockNative

const sai = await import('../sai')

test('sai subpath exposes compatibility primitives', () => {
  expect(typeof sai.click).toBe('function')
  expect(typeof sai.ground).toBe('function')
  expect(typeof sai.open).toBe('function')
  expect(typeof sai.pageContent).toBe('function')
  expect(typeof sai.browser.newtab).toBe('function')
  expect(typeof sai.UnsupportedSaiPrimitiveError).toBe('function')
})

test('sai readFile + writeToFile wrap Machine.local().file()', async () => {
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), 'simulang-js-sai-'))
  const filePath = path.join(dir, 'sample.txt')

  expect(sai.writeToFile({ path: filePath, text: 'hello', overwrite: true })).toBe(filePath)
  expect(sai.readFile({ path: filePath })).toBe('hello')

  expect(sai.writeToFile({ path: filePath, text: 'world' })).toBe(filePath)
  expect(sai.readFile({ path: filePath })).toBe('hello\nworld')
})

test('sai ground without an app grounds against the full screen', async () => {
  // Mirrors unified-ui: no crop region -> full machine screenshot then vision grounding.
  const point = await sai.ground({ concept: 'button' })

  expect(point).toEqual({ x: 12, y: 34 })
})

test('sai ground with an app crops to the window bounds', async () => {
  // Mirrors unified-ui ground(concept, appBounds): crop to the app window, then
  // vision grounding (screenshotCropped). launch()/getAppWindow() return an
  // object exposing getWindowBounds(); cast to exercise that path.
  const app = {
    getWindowBounds: () => ({ left: 0, top: 0, right: 100, bottom: 100 }),
  } as unknown as App

  expect(await sai.ground({ concept: 'button', app })).toEqual({ x: 99, y: 88 })
})

test('sai ground rejects with the Sai primitive-level error on provider failure', async () => {
  groundError = true
  try {
    await expect(sai.ground({ concept: 'missing button' })).rejects.toThrow(
      'Failed to get location by concept from visual grounding',
    )
  } finally {
    groundError = false
  }
})

test('sai ask shapes the prompt and forwards an image to native AskModel.ask', () => {
  const out = JSON.parse(sai.ask({ prompt: 'what is this?', context: { text: 'a button', imageBase64: 'AAAA' } }))

  // Page content + task are folded into the prompt (matching unified-ui); the
  // native `text` arg stays null and the image is forwarded separately.
  expect(out.prompt).toContain('Task: what is this?')
  expect(out.prompt).toContain('Page content:\na button')
  expect(out.text).toBeNull()
  expect(out.images).toBe(1)
})

test('sai ask omits images when no imageBase64 is given', () => {
  const out = JSON.parse(sai.ask({ prompt: 'hello' }))

  expect(out.prompt).toContain('Task: hello')
  expect(out.images).toBe(0)
})

test('sai stateSatisfies delegates to the native StateSatisfiesModel', () => {
  mockNative.StateSatisfiesModel.result = true
  try {
    expect(sai.stateSatisfies({ condition: 'the form was submitted' })).toBe(true)
    // It forwards the condition plus the captured page content (a11y text + screenshot)
    // to the perception model — it does NOT route through AskModel.
    expect(mockNative.StateSatisfiesModel.lastArgs?.condition).toBe('the form was submitted')
    expect(typeof mockNative.StateSatisfiesModel.lastArgs?.text).toBe('string')
    expect(typeof mockNative.StateSatisfiesModel.lastArgs?.image).toBe('string')
  } finally {
    mockNative.StateSatisfiesModel.result = true
  }

  mockNative.StateSatisfiesModel.result = false
  try {
    expect(sai.stateSatisfies({ condition: 'the form was submitted' })).toBe(false)
  } finally {
    mockNative.StateSatisfiesModel.result = true
  }
})

test('sai ConceptsExist uses the platform concept resolver for each concept', () => {
  // getElementByConcept is platform-branched: Windows resolves via the AX tree
  // (findByDescription), macOS/Linux via vision-backed synthetic elements. Provide an
  // AX match so the Windows path resolves; grounding already succeeds (groundError is
  // false) for the macOS/Linux path. Either way every concept exists.
  axMatches = [{ boundingBox: { left: 0, top: 0, right: 10, bottom: 10 } }]
  try {
    expect(sai.ConceptsExist({ concepts: ['login button', 'password field'] })).toBe(true)
  } finally {
    axMatches = []
  }
})

test('sai ConceptsExist returns false when a concept resolves nowhere', () => {
  // Resolve nowhere on every platform: no AX match (axMatches stays empty -> Windows
  // path) and grounding refuses (groundError -> macOS/Linux path).
  groundError = true
  try {
    expect(sai.ConceptsExist({ concepts: ['nonexistent widget'] })).toBe(false)
  } finally {
    groundError = false
  }
})

test('sai press and scroll preserve legacy primitive defaults', () => {
  keyboardEvents.length = 0
  mouseEvents.length = 0

  sai.press({ key: 'a', cmd: true, shift: true, option: true, ctrl: true })
  expect(keyboardEvents).toEqual([
    { kind: 'key', key: mockNative.Key.Meta, direction: mockNative.Direction.Press },
    { kind: 'key', key: mockNative.Key.Shift, direction: mockNative.Direction.Press },
    { kind: 'key', key: mockNative.Key.Alt, direction: mockNative.Direction.Press },
    { kind: 'key', key: mockNative.Key.Control, direction: mockNative.Direction.Press },
    { kind: 'key', key: 10, direction: mockNative.Direction.Click },
    { kind: 'key', key: mockNative.Key.Meta, direction: mockNative.Direction.Release },
    { kind: 'key', key: mockNative.Key.Shift, direction: mockNative.Direction.Release },
    { kind: 'key', key: mockNative.Key.Alt, direction: mockNative.Direction.Release },
    { kind: 'key', key: mockNative.Key.Control, direction: mockNative.Direction.Release },
  ])

  sai.scroll({})
  expect(mouseEvents).toEqual([{ kind: 'scroll', x: 0, y: 200 }])
})

test('unsupported synchronous Sai primitives throw a dedicated error', () => {
  expect(() => sai.respond({ message: 'hello' })).toThrow(sai.UnsupportedSaiPrimitiveError)
})

test('unsupported promise-shaped Sai primitives reject with a dedicated error', async () => {
  await expect(sai.generateImage({ prompt: 'a cat' })).rejects.toMatchObject({
    primitive: 'generateImage',
    category: 'cloud-dependent',
  })
  await expect(sai.exec({ command: 'echo hello' })).rejects.toMatchObject({
    primitive: 'exec',
    category: 'product-runtime-only',
  })
  await expect(sai.requestApproval({ reason: 'send email' })).rejects.toMatchObject({
    primitive: 'requestApproval',
    category: 'product-runtime-only',
  })
})

test('unsupported browser namespace exposes methods that reject', async () => {
  await expect(sai.browser.newtab('https://example.com')).rejects.toMatchObject({
    primitive: 'browser.newtab',
    category: 'product-runtime-only',
  })
  await expect(sai.browser.listTabs()).rejects.toMatchObject({
    primitive: 'browser.listTabs',
    category: 'product-runtime-only',
  })
})

test('unsupported Sai namespaces throw when accessed', () => {
  expect(() => (sai.google as Record<string, unknown>).gmail).toThrow(sai.UnsupportedSaiPrimitiveError)
})
