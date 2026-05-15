#!/usr/bin/env node
// Wire up Claude Code to read this package's CLAUDE.md by appending one
// idempotent import line to <project>/CLAUDE.md (or ~/.claude/CLAUDE.md
// with --user). Re-running is a no-op.
//
// Usage (after `npm install @simular-ai/simulang-js`):
//   npx simulang-init-claude          # update <project>/CLAUDE.md
//   npx simulang-init-claude --user   # update ~/.claude/CLAUDE.md
//   npx simulang-init-claude --check  # report status, don't write

import { existsSync, readFileSync, writeFileSync, mkdirSync } from 'node:fs'
import { homedir } from 'node:os'
import { dirname, join, relative } from 'node:path'
import { fileURLToPath } from 'node:url'

const PKG = '@simular-ai/simulang-js'
const BEGIN = `<!-- ${PKG}:claude-import:begin -->`
const END = `<!-- ${PKG}:claude-import:end -->`
const BLOCK_RE = new RegExp(`${BEGIN}[\\s\\S]*?${END}\\n?`)

/** @type {{ user: boolean, check: boolean, help: boolean }} */
const flags = { user: false, check: false, help: false }
for (const a of process.argv.slice(2)) {
  if (a === '--user') flags.user = true
  else if (a === '--check') flags.check = true
  else if (a === '--help' || a === '-h') flags.help = true
  else {
    process.stderr.write(`Unknown flag: ${a}\n`)
    process.exit(2)
  }
}

if (flags.help) {
  process.stdout.write(
    `Usage: simulang-init-claude [--user] [--check]\n` +
      `  Point Claude Code at ${PKG}/CLAUDE.md so it knows how to use this library.\n` +
      `  --user   Target ~/.claude/CLAUDE.md instead of <project>/CLAUDE.md.\n` +
      `  --check  Report status without writing.\n`,
  )
  process.exit(0)
}

// We live at <pkgRoot>/bin/init-claude.mjs in the package, so the doc is two
// levels up — no need to walk the filesystem for it.
const docPath = join(dirname(dirname(fileURLToPath(import.meta.url))), 'CLAUDE.md')

/**
 * Walk up from `start` looking for the consumer's package.json (i.e. one that
 * isn't ours). Returns the directory or null if we hit the filesystem root.
 * @param {string} start
 */
function findProjectRoot(start) {
  for (let dir = start; ; dir = dirname(dir)) {
    try {
      /** @type {{ name?: string }} */
      const pkg = JSON.parse(readFileSync(join(dir, 'package.json'), 'utf8'))
      if (pkg.name !== PKG) return dir
    } catch {
      // No readable package.json here — keep walking.
    }
    if (dirname(dir) === dir) return null
  }
}

let targetMd, importLine
if (flags.user) {
  targetMd = join(homedir(), '.claude', 'CLAUDE.md')
  // Absolute path — node_modules location isn't predictable from ~/.claude.
  importLine = `@${docPath}`
} else {
  const projectRoot = findProjectRoot(process.cwd())
  if (!projectRoot) {
    process.stderr.write(`No project package.json found from ${process.cwd()}.\nRun inside a project, or use --user.\n`)
    process.exit(1)
  }
  targetMd = join(projectRoot, 'CLAUDE.md')
  const rel = relative(projectRoot, docPath)
  // Project-relative path keeps the import portable across machines.
  importLine = `@${rel.startsWith('.') ? rel : `./${rel}`}`
}
// Claude Code's @import resolver expects POSIX-style paths. On Windows,
// path.relative / path.join produce backslashes — normalise here so the
// resulting CLAUDE.md is identical on every OS.
importLine = importLine.replaceAll('\\', '/')

const block = `${BEGIN}\n${importLine}\n${END}\n`
const existing = existsSync(targetMd) ? readFileSync(targetMd, 'utf8') : ''

if (existing.includes(block)) {
  process.stdout.write(`${PKG}: ${targetMd} already imports the doc — no change.\n`)
  process.exit(0)
}
if (flags.check) {
  process.stdout.write(`${PKG}: ${targetMd} would be ${existing ? 'updated' : 'created'}.\n`)
  process.exit(0)
}

// Replace an existing block (handles import-line changes) or append a fresh
// one after a blank line. Trimming trailing newlines normalises the spacing.
const trimmed = existing.replace(/\n+$/, '')
const next = BLOCK_RE.test(existing) ? existing.replace(BLOCK_RE, block) : trimmed ? `${trimmed}\n\n${block}` : block

mkdirSync(dirname(targetMd), { recursive: true })
writeFileSync(targetMd, next)
process.stdout.write(`${PKG}: ${existing ? 'updated' : 'created'} ${targetMd}\n`)
