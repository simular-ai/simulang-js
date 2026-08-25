#!/usr/bin/env node
// Open or update a PR on simular-ai/docs that syncs the TypeDoc JSON
// artifact and points the `sdk` navigation groups at it — Mintlify renders
// the reference pages from the artifact (see
// https://mintlify.com/docs/api-playground/sdk-reference-setup).
// Run by the publish job in .github/workflows/CI.yml on tag pushes.
//
// Inputs from env:
//   GITHUB_REF  -- refs/tags/vX.Y.Z (extracted to VERSION)
//   GH_TOKEN    -- token with contents:write + pull-requests:write on
//                  simular-ai/docs
//
// Preconditions (set up by CI):
//   ./docs/simulang-js.api.json  -- output of `npm run docs:mintlify`
//   ./docs-repo/                 -- actions/checkout of simular-ai/docs @ main
//                                   (with persisted credentials so git push works)

import { spawnSync } from 'node:child_process'
import { existsSync } from 'node:fs'
import { copyFile, mkdir, readFile, writeFile } from 'node:fs/promises'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const REPO_ROOT = join(__dirname, '..')
const ARTIFACT = join(REPO_ROOT, 'docs', 'simulang-js.api.json')
const DOCS_REPO = join(REPO_ROOT, 'docs-repo')
// Per-version branch name (computed below after VERSION is parsed). Using
// the version in the branch name means concurrent in-flight tag syncs each
// get their own PR instead of clobbering each other via force-push.
const SYNC_BRANCH_PREFIX = 'simulang-js-api-sync'
const TARGET_REPO = 'simular-ai/docs'
const SUBGROUP_NAME = 'simulang-js Reference'
const PARENT_GROUP = 'Simulang'
const API_BASE = 'simulang-js/api'

function compareVersion(a, b) {
  // Compare two "vMAJOR.MINOR.PATCH" strings. Returns >0 if a is newer.
  // Safe because the regex below rejects anything with a pre-release suffix.
  const parse = (v) => v.slice(1).split('.').map(Number)
  const [a1, a2, a3] = parse(a)
  const [b1, b2, b3] = parse(b)
  return a1 - b1 || a2 - b2 || a3 - b3
}

function run(cmd, args, opts = {}) {
  const res = spawnSync(cmd, args, { stdio: 'inherit', ...opts })
  if (res.status !== 0) {
    throw new Error(`Command failed (exit ${res.status}): ${cmd} ${args.join(' ')}`)
  }
}

function runCapture(cmd, args, opts = {}) {
  const res = spawnSync(cmd, args, { encoding: 'utf8', ...opts })
  return {
    stdout: res.stdout ?? '',
    stderr: res.stderr ?? '',
    status: res.status,
  }
}

function sdkGroup(label, version, directory, extra = {}) {
  return {
    group: label,
    ...extra,
    sdk: {
      format: 'typedoc',
      source: `${API_BASE}/_artifacts/${version}.json`,
      directory,
    },
  }
}

const tagRef = process.env.GITHUB_REF ?? ''
const tagMatch = tagRef.match(/^refs\/tags\/(v\d+\.\d+\.\d+)$/)
if (!tagMatch) {
  // Pre-releases (v1.2.3-rc1) and non-tag refs both fall through here.
  // Don't fail the job; just skip Mintlify sync.
  console.log(`[sync-mintlify] GITHUB_REF=${tagRef} is not a vMAJOR.MINOR.PATCH tag; skipping sync.`)
  process.exit(0)
}
const VERSION = tagMatch[1]
const SYNC_BRANCH = `${SYNC_BRANCH_PREFIX}-${VERSION}`
console.log(`[sync-mintlify] Syncing simulang-js API reference for ${VERSION}`)

if (!existsSync(ARTIFACT)) {
  throw new Error(`Expected ${ARTIFACT} to exist (run \`npm run docs:mintlify\` first).`)
}
if (!existsSync(DOCS_REPO)) {
  throw new Error(`Expected ${DOCS_REPO}/ to exist (checkout simular-ai/docs first).`)
}

// Branch off current main, not the previous sync branch, so each PR shows a
// clean diff against the merge target.
run('git', ['checkout', '-B', SYNC_BRANCH, 'origin/main'], { cwd: DOCS_REPO })

const apiDir = join(DOCS_REPO, ...API_BASE.split('/'))
const artifactsDir = join(apiDir, '_artifacts')
await mkdir(artifactsDir, { recursive: true })

const versionsPath = join(apiDir, '_versions.json')
let manifest = { latest: null, versions: [] }
if (existsSync(versionsPath)) {
  try {
    manifest = JSON.parse(await readFile(versionsPath, 'utf8'))
  } catch (err) {
    throw new Error(`Failed to parse ${versionsPath}: ${err.message}`, { cause: err })
  }
}
// Treat this sync as "latest" only when VERSION is strictly newer than what
// the manifest already records — protects against re-running an older tag
// retroactively after a newer one has shipped.
const isLatest = !manifest.latest || compareVersion(VERSION, manifest.latest) > 0

await copyFile(ARTIFACT, join(artifactsDir, `${VERSION}.json`))

const today = new Date().toISOString().slice(0, 10)
const existingIdx = manifest.versions.findIndex((v) => v.version === VERSION)
if (existingIdx >= 0) {
  manifest.versions[existingIdx].released = today
} else {
  manifest.versions.push({ version: VERSION, released: today })
}
// Sort newest-first so the manifest (and the nav generated from it) stays
// deterministic regardless of insertion order — important when older tags
// are synced after newer ones.
manifest.versions.sort((a, b) => compareVersion(b.version, a.version))
if (isLatest) {
  manifest.latest = VERSION
}
await writeFile(versionsPath, JSON.stringify(manifest, null, 2) + '\n', 'utf8')

const docsJsonPath = join(DOCS_REPO, 'docs.json')
const docsJson = JSON.parse(await readFile(docsJsonPath, 'utf8'))
const tabs = docsJson.navigation?.tabs
if (!Array.isArray(tabs) || tabs.length === 0) {
  throw new Error('Could not find navigation.tabs[] in docs.json')
}
let parentGroup
for (const tab of tabs) {
  parentGroup = tab.groups?.find((g) => g.group === PARENT_GROUP)
  if (parentGroup) break
}
if (!parentGroup) {
  throw new Error(`Could not find "${PARENT_GROUP}" group in docs.json`)
}
parentGroup.pages ??= []

// Preserve any existing `icon` on the subgroup so manual edits in the docs
// repo survive subsequent sync runs (we overwrite the whole subgroup object).
const subgroupIdx = parentGroup.pages.findIndex((p) => typeof p === 'object' && p !== null && p.group === SUBGROUP_NAME)
const existingIcon =
  subgroupIdx >= 0 && typeof parentGroup.pages[subgroupIdx].icon === 'string'
    ? parentGroup.pages[subgroupIdx].icon
    : undefined

const subgroupPages = []
for (const v of manifest.versions) {
  if (!existsSync(join(artifactsDir, `${v.version}.json`))) {
    console.warn(`[sync-mintlify] ${v.version} listed in manifest but its artifact is missing; skipping.`)
    continue
  }
  subgroupPages.push(
    sdkGroup(
      v.version === manifest.latest ? `${v.version} (latest)` : v.version,
      v.version,
      `${API_BASE}/${v.version}`,
      {
        expanded: false,
      },
    ),
  )
}

const newSubgroup = {
  group: SUBGROUP_NAME,
  ...(existingIcon ? { icon: existingIcon } : {}),
  pages: subgroupPages,
}
if (subgroupIdx >= 0) {
  parentGroup.pages[subgroupIdx] = newSubgroup
} else {
  parentGroup.pages.push(newSubgroup)
}

// No rendered "latest" tree — old /api/latest/... URLs redirect to the
// newest version instead.
const latestRedirectSource = `/${API_BASE}/latest/:slug*`
docsJson.redirects = (docsJson.redirects ?? []).filter((r) => r.source !== latestRedirectSource)
docsJson.redirects.push({
  source: latestRedirectSource,
  destination: `/${API_BASE}/${manifest.latest}/:slug*`,
})

await writeFile(docsJsonPath, JSON.stringify(docsJson, null, 2) + '\n', 'utf8')

const diff = runCapture('git', ['diff', '--quiet'], { cwd: DOCS_REPO })
if (diff.status === 0) {
  console.log(`[sync-mintlify] No changes vs. main for ${VERSION}. Exiting cleanly.`)
  process.exit(0)
}

run('git', ['add', '.'], { cwd: DOCS_REPO })
run(
  'git',
  [
    '-c',
    'user.name=simulang-js-bot',
    '-c',
    'user.email=bot@simular.ai',
    'commit',
    '-m',
    `Update simulang-js API reference to ${VERSION}`,
  ],
  { cwd: DOCS_REPO },
)
// Force-push: the sync branch is bot-owned and intentionally a snapshot of
// the latest sync, not a history we care about preserving.
run('git', ['push', '--force', 'origin', SYNC_BRANCH], { cwd: DOCS_REPO })

const prView = runCapture('gh', ['pr', 'view', SYNC_BRANCH, '--repo', TARGET_REPO, '--json', 'number'], {
  cwd: DOCS_REPO,
})
if (prView.status === 0) {
  console.log('[sync-mintlify] PR already open for this branch; force-pushed updates only.')
} else {
  const body = [
    `Automated sync from \`simular-ai/simulang-js\`.`,
    ``,
    `Updates the simulang-js API reference for **${VERSION}**:`,
    ``,
    `- Wrote the TypeDoc JSON artifact \`${API_BASE}/_artifacts/${VERSION}.json\` (rendered by Mintlify via the \`sdk\` navigation property).`,
    isLatest
      ? `- Pointed the \`/${API_BASE}/latest/...\` redirect at ${VERSION} (now the newest release in the manifest).`
      : `- Left the \`/${API_BASE}/latest/...\` redirect at ${manifest.latest}, which is still newer than ${VERSION}.`,
    `- Refreshed \`${API_BASE}/_versions.json\`.`,
    `- Regenerated the \`${SUBGROUP_NAME}\` subgroup of \`${PARENT_GROUP}\` in \`docs.json\`.`,
    ``,
    `Triggered by tag push to https://github.com/simular-ai/simulang-js/tree/${VERSION}.`,
  ].join('\n')
  run(
    'gh',
    [
      'pr',
      'create',
      '--repo',
      TARGET_REPO,
      '--base',
      'main',
      '--head',
      SYNC_BRANCH,
      '--title',
      `Update simulang-js API reference to ${VERSION}`,
      '--body',
      body,
    ],
    { cwd: DOCS_REPO },
  )
}

console.log(`[sync-mintlify] Done syncing ${VERSION}.`)
