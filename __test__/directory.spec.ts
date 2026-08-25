import { test, expect } from 'vitest'
import { mkdtempSync, writeFileSync, mkdirSync, existsSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'

import { Machine } from '../index'

const machine = Machine.local()

function tmpDir(): string {
  return mkdtempSync(join(tmpdir(), 'simulang-js-dir-'))
}

test('new creates directory when create missing is true', () => {
  const parent = tmpDir()
  const path = join(parent, 'new_dir')
  const dir = machine.dir(path, true)
  expect(existsSync(path)).toBe(true)
  expect(dir.path()).toBe(path)
})

test('new succeeds for existing directory', () => {
  const parent = tmpDir()
  const path = join(parent, 'existing')
  mkdirSync(path)
  const dir = machine.dir(path, false)
  expect(dir.path()).toBe(path)
})

test('new fails when directory does not exist', () => {
  const parent = tmpDir()
  const path = join(parent, 'nonexistent')
  expect(() => machine.dir(path, false)).toThrow()
})

test('new fails when path is a file', () => {
  const parent = tmpDir()
  const path = join(parent, 'not_a_dir')
  writeFileSync(path, 'not a directory')
  expect(() => machine.dir(path, false)).toThrow()
})

test('rename rejects invalid names', () => {
  const invalidNames = ['../evil_dir', 'parent/child', 'dir<', ' leading', 'trailing ', 'CON', 'NUL']

  const parent = tmpDir()
  const path = join(parent, 'test_dir')
  const dir = machine.dir(path, true)

  for (const name of invalidNames) {
    expect(() => dir.rename(name), `rename(${JSON.stringify(name)}) should throw`).toThrow()
  }
})

test('rename accepts valid names', () => {
  const validNames = ['my_project', '.hidden', 'my-project', 'my project', '名前']

  for (const name of validNames) {
    const parent = tmpDir()
    const path = join(parent, 'test_dir')
    const dir = machine.dir(path, true)
    expect(() => dir.rename(name), `rename(${JSON.stringify(name)}) should not throw`).not.toThrow()
    expect(dir.path().endsWith(name)).toBe(true)
  }
})

test('rename is noop when name is same', () => {
  const parent = tmpDir()
  const path = join(parent, 'test_dir')
  const dir = machine.dir(path, true)
  dir.rename('test_dir')
  expect(dir.path()).toBe(path)
  expect(existsSync(path)).toBe(true)
})

test('rename fails when destination exists', () => {
  const parent = tmpDir()
  const srcPath = join(parent, 'src')
  const existingPath = join(parent, 'existing')
  const dir = machine.dir(srcPath, true)
  machine.dir(existingPath, true)
  expect(() => dir.rename('existing')).toThrow()
})

test('name returns directory name', () => {
  const parent = tmpDir()
  const dir = machine.dir(join(parent, 'my_project'), true)
  expect(dir.name()).toBe('my_project')
})

test('is readonly returns false for writable directory', () => {
  const parent = tmpDir()
  const dir = machine.dir(join(parent, 'writable'), true)
  expect(dir.isReadonly()).toBe(false)
})

test('temp creates directory', () => {
  const dir = machine.tempDir()
  expect(dir.path()).toBeTruthy()
  expect(existsSync(dir.path())).toBe(true)
  dir.delete()
})

test('list files returns files in directory', () => {
  const parent = tmpDir()
  const path = join(parent, 'with_files')
  mkdirSync(path)
  writeFileSync(join(path, 'a.txt'), 'a')
  writeFileSync(join(path, 'b.txt'), 'b')

  const dir = machine.dir(path, false)
  const files = dir.listFiles()
  expect(files.length).toBe(2)
})

test('list dirs returns subdirectories', () => {
  const parent = tmpDir()
  const path = join(parent, 'with_dirs')
  mkdirSync(path)
  mkdirSync(join(path, 'sub1'))
  mkdirSync(join(path, 'sub2'))

  const dir = machine.dir(path, false)
  const dirs = dir.listDirs()
  expect(dirs.length).toBe(2)
})

test('copy to creates copy with contents', () => {
  const parent = tmpDir()
  const srcPath = join(parent, 'src_dir')
  mkdirSync(srcPath)
  writeFileSync(join(srcPath, 'file.txt'), 'content')

  const dir = machine.dir(srcPath, false)
  const destPath = join(parent, 'dest_dir')
  const copy = dir.copyTo(destPath)
  expect(existsSync(destPath)).toBe(true)

  const files = copy.listFiles()
  expect(files.length).toBe(1)
})

test('move to moves directory', () => {
  const parent = tmpDir()
  const srcPath = join(parent, 'src_dir')
  const destPath = join(parent, 'dest_dir')
  mkdirSync(srcPath)
  writeFileSync(join(srcPath, 'file.txt'), 'content')

  const dir = machine.dir(srcPath, false)
  dir.moveTo(destPath)
  expect(existsSync(srcPath)).toBe(false)
  expect(existsSync(destPath)).toBe(true)
})

test('delete removes directory', () => {
  const parent = tmpDir()
  const path = join(parent, 'to_delete')
  mkdirSync(path)
  expect(existsSync(path)).toBe(true)

  const dir = machine.dir(path, false)
  dir.delete()
  expect(existsSync(path)).toBe(false)
  expect(() => dir.path()).toThrow()
})
