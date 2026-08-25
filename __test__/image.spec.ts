import { test, expect } from 'vitest'
import { mkdtempSync, statSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'

import { Image } from '../index'

import {
  INVALID_BASE64,
  NON_IMAGE_BASE64,
  SCREENSHOT_CROPPED_BASE64,
  TINY_GIF_BASE64,
  TINY_GIF_DATA_URL,
  TINY_JPEG_BASE64,
  TINY_JPEG_DATA_URL,
  TINY_PNG_BASE64,
  TINY_PNG_DATA_URL,
  TINY_WEBP_BASE64,
  TINY_WEBP_DATA_URL,
} from './consts'

test('image_base64_roundtrip', () => {
  const img1 = Image.fromBase64(SCREENSHOT_CROPPED_BASE64)
  const b64 = img1.base64()
  const img2 = Image.fromBase64(b64)
  expect(img2.base64()).toBe(b64)
})

test('base64_returns_raw_payload_without_data_url_prefix', () => {
  const img = Image.fromBase64(TINY_PNG_DATA_URL)
  expect(img.base64()).toBe(TINY_PNG_BASE64)
})

test('base64_data_url_returns_mime_prefixed_payload', () => {
  const img = Image.fromBase64(TINY_PNG_DATA_URL)
  expect(img.base64DataUrl()).toBe(TINY_PNG_DATA_URL)
})

test('from_base64_accepts_data_url_prefix', () => {
  const img = Image.fromBase64(TINY_PNG_DATA_URL)
  expect(img.base64()).toBe(TINY_PNG_BASE64)
})

test('from_base64_accepts_raw_payload_with_whitespace', () => {
  const padded = `  \n ${TINY_PNG_BASE64} \n  `
  const img = Image.fromBase64(padded)
  expect(img.base64()).toBe(TINY_PNG_BASE64)
})

test('from_base64_accepts_jpeg_data_url_prefix', () => {
  const img = Image.fromBase64(TINY_JPEG_DATA_URL)
  expect(img.base64()).toBe(TINY_JPEG_BASE64)
})

test('from_base64_accepts_jpg_data_url_prefix', () => {
  const jpgDataUrl = TINY_JPEG_DATA_URL.replace('image/jpeg', 'image/jpg')
  const img = Image.fromBase64(jpgDataUrl)
  expect(img.base64()).toBe(TINY_JPEG_BASE64)
})

test('from_base64_accepts_raw_jpeg_payload', () => {
  const img = Image.fromBase64(TINY_JPEG_BASE64)
  expect(img.base64()).toBe(TINY_JPEG_BASE64)
})

test('from_base64_accepts_gif_data_url_prefix', () => {
  const img = Image.fromBase64(TINY_GIF_DATA_URL)
  expect(img.base64()).toBe(TINY_GIF_BASE64)
})

test('from_base64_accepts_webp_data_url_prefix', () => {
  const img = Image.fromBase64(TINY_WEBP_DATA_URL)
  expect(img.base64()).toBe(TINY_WEBP_BASE64)
})

test('base64_data_url_preserves_gif_format_after_mutation', () => {
  const img = Image.fromBase64(TINY_GIF_DATA_URL)
  img.drawDot(0, 0, 0, 255, 0, 0)
  expect(img.base64DataUrl().startsWith('data:image/gif;base64,')).toBe(true)
})

test('base64_data_url_preserves_webp_format_after_mutation', () => {
  const img = Image.fromBase64(TINY_WEBP_DATA_URL)
  img.drawDot(0, 0, 0, 255, 0, 0)
  expect(img.base64DataUrl().startsWith('data:image/webp;base64,')).toBe(true)
})

test('from_base64_rejects_invalid_base64_payload', () => {
  expect(() => Image.fromBase64(INVALID_BASE64)).toThrow()
})

test('from_base64_rejects_non_image_payload', () => {
  expect(() => Image.fromBase64(NON_IMAGE_BASE64)).toThrow()
})

test('from_base64_rejects_empty_input', () => {
  expect(() => Image.fromBase64('')).toThrow()
})

test('compress_switches_encoding_to_jpeg', () => {
  const img = Image.fromBase64(SCREENSHOT_CROPPED_BASE64)
  img.compress(80)
  expect(img.base64()).not.toMatch(/^data:image\//)
  expect(img.base64DataUrl().startsWith('data:image/jpeg;base64,')).toBe(true)
})

test('compress_rejects_zero_quality', () => {
  const img = Image.fromBase64(SCREENSHOT_CROPPED_BASE64)
  expect(() => img.compress(0)).toThrow(/greater than 0/)
})

test('draw_grid_rejects_zero_dimensions', () => {
  const img = Image.fromBase64(SCREENSHOT_CROPPED_BASE64)
  expect(() => img.drawGrid(0, 1)).toThrow()
  expect(() => img.drawGrid(1, 0)).toThrow()
  expect(() => img.drawGrid(0, 0)).toThrow()
})

test('draw_grid_modifies_image_data', () => {
  const img = Image.fromBase64(SCREENSHOT_CROPPED_BASE64)
  const before = img.base64()
  img.drawGrid(32, 32)
  expect(img.base64()).not.toBe(before)
})

test('save_writes_nonempty_file', () => {
  const dir = mkdtempSync(join(tmpdir(), 'simulang-js-image-'))
  const filePath = join(dir, 'test.png')
  const img = Image.fromBase64(TINY_PNG_BASE64)
  img.save(filePath)
  const stat = statSync(filePath)
  expect(stat.size).toBeGreaterThan(0)
})
