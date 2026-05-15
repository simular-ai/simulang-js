import { test, expect } from 'vitest'

import { Clipboard, Image } from '../index'

import { SCREENSHOT_CROPPED_BASE64 } from './consts'

test.sequential('set_string_then_get_string', () => {
  const cb = new Clipboard()
  cb.setString('clipboard test content')
  expect(cb.getString()).toBe('clipboard test content')
})

test.sequential('set_string_then_clear', () => {
  const cb = new Clipboard()
  cb.setString('clipboard test content')
  expect(cb.getString()).toBe('clipboard test content')
  cb.clear()
  expect(cb.getString()).toBeNull()
})

test.sequential('clear', () => {
  const cb = new Clipboard()
  cb.clear()
  expect(cb.getString()).toBeNull()
})

test.sequential('set_string_overwrites_previous', () => {
  const cb = new Clipboard()
  cb.setString('first')
  const previous = cb.setString('second')
  expect(previous).toBe('first')
  expect(cb.getString()).toBe('second')
})

test.sequential('set_string_with_unicode_roundtrip', () => {
  const cb = new Clipboard()
  const text = 'Unicode: 日本語 café naïve'
  cb.setString(text)
  expect(cb.getString()).toBe(text)
})

test.sequential('set_image_overwrites_string', () => {
  const cb = new Clipboard()
  cb.setString('text before image')
  const img = Image.fromBase64(SCREENSHOT_CROPPED_BASE64)
  const previous = cb.setImage(img)
  expect(previous).toBe('text before image')
  expect(cb.getString()).toBeNull()
})

test.sequential.skip('paste_text', () => {})

test.sequential.skip('paste_text_restores_previous_content', () => {})
