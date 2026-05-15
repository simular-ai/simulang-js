# Publishing

`@simular-ai/simulang-js` is published as a **public** package to the npm registry under the `@simular-ai` org.

For consumer install instructions, see [`README.md`](./README.md). This doc only covers cutting a release.

## How releases work

- Releases run **only from CI** — never publish locally.
- Pushing a git tag triggers the release workflow.
- Tag format determines the dist-tag:
  - `x.y.z` → published with the `latest` tag
  - `x.y.z-*` → published with the `next` tag
- The publish job consumes per-platform artifacts uploaded by the build job, so the build must pass first.
- Auth: the workflow uses the `NPM_TOKEN` repository secret (a granular access token from npmjs.com with publish permission on the `@simular-ai` scope).
- Tags that don't match the version rules above are skipped.

## Documentation

- Stable, versioned API reference: https://docs.simular.ai/simulang-js/api/latest/ — refreshed by each tagged release.
- Bleeding-edge preview of `main`: https://simular-ai.github.io/simulang-js/ — refreshed on every push to `main`.

## Cutting a release

```bash
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]
git push
git push --tags
```

`npm version` creates the version-bump commit and the matching tag. CI publishes when the tag is pushed.
