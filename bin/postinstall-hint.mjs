#!/usr/bin/env node
// Postinstall hint for `@simular-ai/simulang-js`. Prints exactly one line if
// the user appears to be a Claude Code user, otherwise silent. Swallows all
// errors — a broken hint must never break `npm install`.

import { existsSync } from 'node:fs'
import { homedir } from 'node:os'
import { join } from 'node:path'

function shouldPrint() {
  // Skip when the maintainer is running `npm install` in this repo itself —
  // `import.meta.url` only contains a `node_modules` segment when this
  // package has been installed as a dependency of another project.
  if (!import.meta.url.includes('/node_modules/')) return false
  if (!process.stdout.isTTY) return false
  if (process.env.CI) return false
  if (process.env.npm_command === 'ci') return false
  const loglevel = (process.env.npm_config_loglevel ?? '').toLowerCase()
  if (loglevel === 'silent' || loglevel === 'error' || loglevel === 'warn') return false
  if (!existsSync(join(homedir(), '.claude'))) return false
  return true
}

try {
  if (shouldPrint()) {
    process.stdout.write(
      '@simular-ai/simulang-js: Claude Code detected. Run `npx @simular-ai/simulang-js init-claude` to point it at the bundled API docs.\n',
    )
  }
} catch {
  // Never propagate failures from the hint.
}
