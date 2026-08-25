// TypeDoc config for `npm run docs:mintlify`: emits a JSON artifact that
// Mintlify renders into docs.simular.ai reference pages (via the `sdk`
// property in simular-ai/docs' docs.json; scripts/sync-mintlify.mjs syncs
// it there).
//
// The script pins typedoc@0.27 (+ a compatible typescript) via npx:
// Mintlify's cloud parser silently drops classes/interfaces/functions from
// TypeDoc 0.28's schemaVersion-2 JSON, rendering only enums (observed
// 2026-08-24). Unpin once Mintlify supports the 0.28 format.
//
// Source links point at the public mirror, which CI tags with the identical
// tree on release — so on tag builds they pin the tag, otherwise main.

const tagMatch = (process.env.GITHUB_REF ?? '').match(/^refs\/tags\/(v\d+\.\d+\.\d+)$/)

/** @type {Partial<import('typedoc').TypeDocOptions>} */
export default {
  entryPoints: ['index.d.ts'],
  json: 'docs/simulang-js.api.json',
  // The pinned TS 5.8 trips lib-check conflicts against the repo's newer
  // @types/node; diagnostics don't matter for doc generation.
  skipErrorChecking: true,
  name: '@simular-ai/simulang-js',
  includeVersion: true,
  readme: 'README.md',
  gitRevision: tagMatch ? tagMatch[1] : 'main',
  sourceLinkTemplate: 'https://github.com/simular-ai/simulang-js/blob/{gitRevision}/{path}#L{line}',
}
