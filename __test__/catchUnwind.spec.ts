import { test, expect } from 'vitest'
import { readdirSync, readFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

// A Rust panic in an exported function aborts the Node process unless the
// function opts into `#[napi(catch_unwind)]`; napi-rs has no crate-wide
// switch. This keeps every exported function opted in.

const SRC = fileURLToPath(new URL('../src/', import.meta.url))
const FN = /^\s*(pub(\([^)]*\))?\s+)?(const\s+)?(async\s+)?(unsafe\s+)?fn\s/

/** Index of the last line of the attribute that starts at `lines[start]`. */
function attributeEnd(lines: string[], start: number): number {
  let depth = 0
  for (let i = start; i < lines.length; i++) {
    for (const ch of lines[i]) {
      if (ch === '[') depth++
      else if (ch === ']') depth--
    }
    if (depth === 0) return i
  }
  throw new Error(`unterminated attribute at line ${start + 1}`)
}

/** Index of the first line after `end` that is not blank, a comment, or an attribute. */
function nextItem(lines: string[], end: number): number | undefined {
  let i = end + 1
  while (i < lines.length) {
    const line = lines[i].trim()
    if (line === '' || line.startsWith('//')) i++
    else if (line.startsWith('#[')) i = attributeEnd(lines, i) + 1
    else return i
  }
  return undefined
}

/** `file:line` of every exported function, and whether it catches panics. */
function exportedFunctions(): { at: string; catchesPanics: boolean }[] {
  const found = []
  const files = readdirSync(SRC, { recursive: true, encoding: 'utf8' }).filter((f) => f.endsWith('.rs'))
  for (const file of files) {
    const lines = readFileSync(join(SRC, file), 'utf8').split('\n')
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim()
      if (line !== '#[napi]' && !line.startsWith('#[napi(')) continue
      const end = attributeEnd(lines, i)
      const item = nextItem(lines, end)
      if (item !== undefined && FN.test(lines[item])) {
        const attribute = lines.slice(i, end + 1).join('\n')
        found.push({ at: `src/${file}:${i + 1}`, catchesPanics: /\bcatch_unwind\b/.test(attribute) })
      }
      i = end
    }
  }
  return found
}

test('every exported function turns a panic into a JS error', () => {
  const functions = exportedFunctions()
  expect(functions.length).toBeGreaterThan(300)
  expect(functions.filter((f) => !f.catchesPanics).map((f) => f.at)).toEqual([])
})
