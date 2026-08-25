import { test, expect } from 'vitest'
import { mkdtempSync, writeFileSync, chmodSync, existsSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'

import { Machine } from '../index'

const machine = Machine.local()

function tmpDir(): string {
  return mkdtempSync(join(tmpdir(), 'simulang-js-file-'))
}

test('read returns content written by write', () => {
  const dir = tmpDir()
  const path = join(dir, 'test.txt')
  const file = machine.file(path, true)
  file.write('hello from test', false)
  expect(file.read()).toBe('hello from test')
})

test('write overwrite replaces existing file', () => {
  const dir = tmpDir()
  const path = join(dir, 'test.txt')
  const file = machine.file(path, true)

  file.write('first', false)
  expect(file.read()).toBe('first')
  file.write('second', false)
  expect(file.read()).toBe('second')
})

test('write no overwrite appends when file exists', () => {
  const dir = tmpDir()
  const path = join(dir, 'test.txt')
  const file = machine.file(path, true)

  file.write('existing', false)
  file.write('new', true)
  expect(file.read()).toBe('existing\nnew')
})

test('write no overwrite succeeds when file does not exist', () => {
  const dir = tmpDir()
  const path = join(dir, 'test.txt')
  const file = machine.file(path, true)

  file.write('content', true)
  expect(file.read()).toBe('content')
})

test('new fails for nonexistent path', () => {
  const dir = tmpDir()
  const path = join(dir, 'does_not_exist.txt')
  expect(() => machine.file(path, false)).toThrow()
})

test('read trims whitespace', () => {
  const dir = tmpDir()
  const path = join(dir, 'test.txt')
  writeFileSync(path, 'hello\n')
  const file = machine.file(path, false)
  expect(file.read()).toBe('hello')
})

test('write no overwrite does not prepend newline for empty file', () => {
  const dir = tmpDir()
  const path = join(dir, 'test.txt')
  writeFileSync(path, '')
  const file = machine.file(path, false)
  file.write('content', true)
  expect(file.read()).toBe('content')
})

test('new fails when parent is file', () => {
  const dir = tmpDir()
  const parent = join(dir, 'not_a_dir')
  writeFileSync(parent, 'not a directory')
  const child = join(parent, 'child.txt')
  expect(() => machine.file(child, true)).toThrow()
})

test('new rejects directory path', () => {
  const dir = tmpDir()
  expect(() => machine.file(dir, false)).toThrow()
})

test('rename rejects invalid names', () => {
  const invalidNames = ['../evil.txt', 'subdir/file.txt', 'file<.txt', ' leading.txt', 'trailing.txt ', 'CON', 'NUL']

  const dir = tmpDir()
  const path = join(dir, 'test.txt')
  const file = machine.file(path, true)

  for (const name of invalidNames) {
    expect(() => file.rename(name), `rename(${JSON.stringify(name)}) should throw`).toThrow()
  }
})

test('rename accepts valid names', () => {
  const validNames = ['report.pdf', '.gitignore', '名前.txt', 'name with spaces.txt']

  for (const name of validNames) {
    const dir = tmpDir()
    const path = join(dir, 'test.txt')
    const file = machine.file(path, true)
    expect(() => file.rename(name), `rename(${JSON.stringify(name)}) should not throw`).not.toThrow()
    expect(file.path().endsWith(name)).toBe(true)
  }
})

test('rename is noop when name is same', () => {
  const dir = tmpDir()
  const path = join(dir, 'test.txt')
  const file = machine.file(path, true)
  file.rename('test.txt')
  expect(file.path()).toBe(path)
  expect(existsSync(path)).toBe(true)
})

test('rename fails when destination exists', () => {
  const dir = tmpDir()
  const srcPath = join(dir, 'src.txt')
  const existingPath = join(dir, 'existing.txt')
  const file = machine.file(srcPath, true)
  machine.file(existingPath, true)
  expect(() => file.rename('existing.txt')).toThrow()
})

test('name returns filename', () => {
  const dir = tmpDir()
  const file = machine.file(join(dir, 'report.pdf'), true)
  expect(file.name()).toBe('report.pdf')
})

test('extension returns extension', () => {
  const dir = tmpDir()
  const file = machine.file(join(dir, 'photo.png'), true)
  expect(file.extension()).toBe('png')
})

test('extension returns null without extension', () => {
  const dir = tmpDir()
  const file = machine.file(join(dir, 'Makefile'), true)
  expect(file.extension()).toBeNull()
})

test('extension returns last for multiple dots', () => {
  const dir = tmpDir()
  const file = machine.file(join(dir, 'archive.tar.gz'), true)
  expect(file.extension()).toBe('gz')
})

test('size returns correct byte count', () => {
  const dir = tmpDir()
  const file = machine.file(join(dir, 'test.txt'), true)
  file.write('hello', false)
  expect(file.size()).toBe(5)
})

test('size returns zero for empty file', () => {
  const dir = tmpDir()
  const file = machine.file(join(dir, 'empty.txt'), true)
  expect(file.size()).toBe(0)
})

test('is readonly returns false for writable file', () => {
  const dir = tmpDir()
  const file = machine.file(join(dir, 'test.txt'), true)
  expect(file.isReadonly()).toBe(false)
})

test('is readonly returns true for readonly file', () => {
  const dir = tmpDir()
  const path = join(dir, 'readonly.txt')
  writeFileSync(path, 'locked')
  chmodSync(path, 0o444)
  try {
    const file = machine.file(path, false)
    expect(file.isReadonly()).toBe(true)
  } finally {
    chmodSync(path, 0o644)
  }
})

test('copy to creates copy with same content', () => {
  const dir = tmpDir()
  const srcPath = join(dir, 'src.txt')
  const destPath = join(dir, 'dest.txt')
  const file = machine.file(srcPath, true)
  file.write('copy me', false)
  const copy = file.copyTo(destPath)
  expect(copy.read()).toBe('copy me')
  expect(existsSync(destPath)).toBe(true)
})

test('move to moves file', () => {
  const dir = tmpDir()
  const srcPath = join(dir, 'src.txt')
  const destPath = join(dir, 'dest.txt')
  const file = machine.file(srcPath, true)
  file.write('move me', false)
  file.moveTo(destPath)
  expect(existsSync(srcPath)).toBe(false)
  expect(existsSync(destPath)).toBe(true)
})

test('delete removes file', () => {
  const dir = tmpDir()
  const path = join(dir, 'test.txt')
  const file = machine.file(path, true)
  file.write('delete me', false)
  expect(existsSync(path)).toBe(true)
  file.delete()
  expect(existsSync(path)).toBe(false)
  expect(() => file.read()).toThrow()
})
