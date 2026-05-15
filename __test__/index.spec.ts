import { test, expect } from 'vitest'
import { promises as fs } from 'node:fs'
import os from 'node:os'
import path from 'node:path'

import { Image, readFile, writeFile } from '../index'

test('readFile + writeFile round trip', async () => {
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), 'simulang-js-'))
  const filePath = path.join(dir, 'sample.txt')

  const writtenPath = writeFile(filePath, 'hello', false)
  expect(writtenPath).toBe(filePath)
  expect(readFile(filePath)).toBe('hello')

  const appendedPath = writeFile(filePath, 'world', true)
  expect(appendedPath).toBe(filePath)
  expect(readFile(filePath)).toBe('hello\nworld')
})

test('image binding exposes shrink and compress', () => {
  expect(typeof Image.prototype.shrink).toBe('function')
  expect(typeof Image.prototype.compress).toBe('function')
})
