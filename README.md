# `@simular-ai/simulang-js`

![CI](https://github.com/simular-ai/simulang-js/workflows/CI/badge.svg)

Node.js bindings for [`simulang-rs`](https://github.com/simular-ai/simulang-rs), a Rust crate published by [Simular](https://simular.ai), built with [`napi-rs`](https://napi.rs/). Provides high-level primitives for automating desktops and Android devices — keyboard and mouse input, screenshots, clipboard access, audio capture, accessibility-tree inspection, and more — exposed through idiomatic JavaScript behind one unified `Machine` API (`Machine.local()` for the desktop this process runs on, `Machine.android(endpoint)` for a device over adb). Ships TypeScript definitions auto-generated from the Rust source, so the types always match runtime behavior.

## Install

```bash
npm install @simular-ai/simulang-js
```

Prebuilt native binaries are published for:

| Platform | Architecture                        |
| -------- | ----------------------------------- |
| macOS    | `aarch64` (Apple Silicon), `x86_64` |
| Windows  | `x86_64`, `aarch64`                 |
| Linux    | `x86_64`, `aarch64` (glibc)         |

Node.js **20 or newer** is required. If your platform isn't covered, see [Building from source](#building-from-source).

### Optional — install the log viewer

The [`log-window.mjs`](https://github.com/simular-ai/simulang-js/blob/main/examples/log-window.mjs) example uses [`@simular-ai/simulang-log-viewer`](https://github.com/simular-ai/simulang-log-viewer), which is declared as an **optional peer dependency** and **not** installed by default. Install it only if you want to run that example or use the same pattern in your own code:

```bash
npm install @simular-ai/simulang-log-viewer
```

## Usage

Try the included [`google_search.mjs`](https://github.com/simular-ai/simulang-js/blob/main/examples/google_search.mjs) example — it opens Google in Chrome, enables the accessibility tree, takes a screenshot, and displays it:

```bash
# With the simulang CLI (no local install needed):
npm install -g @simular-ai/simulang
simulang run examples/google_search.mjs

# Or directly with Node.js if you have simulang-js installed locally:
node examples/google_search.mjs
```

## API documentation

Browse the [API reference](https://docs.simular.ai/simulang-js/api/latest/) for more details. A bleeding-edge preview of HEAD on `main` is also published to [GitHub Pages](https://simular-ai.github.io/simulang-js/) between releases.

## Using with Claude Code

This package ships a short [`CLAUDE.md`](https://github.com/simular-ai/simulang-js/blob/main/CLAUDE.md) inside the npm tarball that points Claude Code at `index.d.ts` as the source of truth for the API and adds a few cross-cutting notes that types alone can't express (lifetime rules, coordinate system, etc.). Wire it into your project's `CLAUDE.md` with:

```bash
npx simulang-init-claude          # appends to <project>/CLAUDE.md (creates it if missing)
npx simulang-init-claude --user   # appends to ~/.claude/CLAUDE.md instead
npx simulang-init-claude --check  # report status, don't write
```

The added line is a single `@./node_modules/@simular-ai/simulang-js/CLAUDE.md` import inside a sentinel-delimited block — safe to re-run, no-op when already present.

## Relationship to `simulang`

[`@simular-ai/simulang`](https://github.com/simular-ai/simulang) is a CLI that runs desktop automation scripts (`.ts`, `.js`, `.simulang`) using this package. It bundles `simulang-js` and re-exports the full API to scripts, so you can write and run automation scripts without a build step:

```bash
npm install -g @simular-ai/simulang
simulang run my-script.js
```

`simulang-js` is the underlying primitive library; `simulang` is the batteries-included script runner built on top of it. If you are embedding desktop automation into your own Node.js application, depend on `simulang-js` directly. If you just want to run standalone automation scripts, `simulang` is the easier starting point.

You can also point `simulang` at a local `simulang-js` checkout during development:

```bash
simulang run --simulang-js=/path/to/simulang-js my-script.js
```

## Changelog

See [`CHANGELOG.md`](https://github.com/simular-ai/simulang-js/blob/main/CHANGELOG.md) for version history and unreleased API changes.
