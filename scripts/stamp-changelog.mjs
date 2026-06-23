#!/usr/bin/env node
// Rename the `[Unreleased]` heading in CHANGELOG.md to the version that npm
// just bumped to, then stage the file so it lands in the `npm version` commit.
//
// Wired into the `version` lifecycle script in package.json — runs after
// `package.json` is bumped, before npm creates the commit and tag.
//
// The `[Unreleased]` heading must always be present — the script itself
// keeps a fresh empty one above every stamped version, so its absence means
// the changelog is structurally broken and the bump should stop. An *empty*
// `[Unreleased]` body is fine: internal-only releases (CI, dependency bumps,
// refactors with no API impact) legitimately have nothing to log, and the
// rename still produces a valid (empty) versioned section.

import { readFileSync, writeFileSync } from 'node:fs'
import { execFileSync } from 'node:child_process'

const version = process.env.npm_package_version
if (!version) {
  console.error('stamp-changelog: npm_package_version not set — run via `npm version`.')
  process.exit(1)
}

const path = 'CHANGELOG.md'
const content = readFileSync(path, 'utf8')

const unreleasedHeading = /^## \[Unreleased\][ \t]*$/m
if (!unreleasedHeading.test(content)) {
  console.error(`stamp-changelog: no "## [Unreleased]" heading found in ${path}.`)
  process.exit(1)
}

const versionHeading = new RegExp(`^## \\[${version.replace(/\./g, '\\.')}\\]`, 'm')
if (versionHeading.test(content)) {
  console.error(`stamp-changelog: ${path} already has a "## [${version}]" section.`)
  process.exit(1)
}

const date = new Date().toISOString().slice(0, 10)
const updated = content.replace(unreleasedHeading, `## [Unreleased]\n\n## [${version}] - ${date}`)

writeFileSync(path, updated)
execFileSync('git', ['add', path])

console.log(`stamp-changelog: CHANGELOG.md → [${version}] - ${date}`)
