# Examples

Runnable snippets demonstrating common `@simular-ai/simulang-js` APIs. All files are plain ESM JavaScript — no TypeScript or extra tooling needed.

> Install the package first if you haven't already — see the [main README](../README.md#install).

Run an example out of `node_modules`:

```bash
node node_modules/@simular-ai/simulang-js/examples/system.mjs
```

Or copy a file into your own project to tweak it:

```bash
cp node_modules/@simular-ai/simulang-js/examples/screenshot.mjs ./
node screenshot.mjs
```

Every example creates a `Machine` once at the top and drives everything
through that one handle, so the rest of the code is backend-agnostic. By
default the machine is the local desktop; set `SIMULANG_ANDROID` to an adb
endpoint to drive a connected Android device instead:

```bash
SIMULANG_ANDROID=127.0.0.1:5555 node examples/open-app.mjs
```

## What each example does

| File                              | Description                                                                                                                                                                                  |
| --------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `system.mjs`                      | Opens a URL in the default browser.                                                                                                                                                          |
| `file-trait.mjs`                  | Writes a text file and reads it back.                                                                                                                                                        |
| `clipboard.mjs`                   | Reads, writes, pastes, and clears the system clipboard.                                                                                                                                      |
| `keyboard.mjs`                    | Synthesizes keyboard input (types a string and sends key events).                                                                                                                            |
| `screenshot.mjs`                  | Captures the machine's screen, resizes and compresses the image, saves to disk.                                                                                                              |
| `google_search.mjs`               | End-to-end flow: launches Chrome, navigates, captures a screenshot.                                                                                                                          |
| `accessibility.mjs`               | Lists windows, snapshots the foreground accessibility tree, queries actions.                                                                                                                 |
| `chrome_google_search_button.mjs` | Opens google.com in Chrome and locates the search button by _concept text_, using BoW Jaccard scoring over `overallDescription`.                                                             |
| `open-app.mjs`                    | Opens an app, lists its windows, snapshots its accessibility tree.                                                                                                                           |
| `loopback.mjs`                    | Plays an audio file while capturing system audio, then transcribes it.                                                                                                                       |
| `ask.mjs`                         | Drives the `AskModel` LLM primitive through prompt-only, text-only, image-only, and text + multi-image calls, using a real ax-tree snapshot and back-to-back screenshots as context.         |
| `logger.mjs`                      | Forwards Rust `log` records to a JS callback (env_logger-style filter spec).                                                                                                                 |
| `log-window.mjs`                  | Pipes Rust `log::*!` records into a floating, always-on-top log window. Requires the optional [`@simular-ai/simulang-log-viewer`](../README.md#3-optional--install-the-log-viewer) peer dep. |

## Heads up — these examples affect your system

Unlike a sandboxed library call, most of these examples perform **real actions** on your machine:

- `keyboard.mjs` and `google_search.mjs` synthesize keystrokes — make sure the intended window is focused, or text will land somewhere unexpected.
- `clipboard.mjs` overwrites your current clipboard contents (it restores them at the end).
- `screenshot.mjs`, `google_search.mjs`, `loopback.mjs`, and `ask.mjs` capture your screen or system audio.
- `system.mjs` and `google_search.mjs` launch a browser window.
- `ask.mjs` and `loopback.mjs` send screen captures / audio to a remote LLM provider — make sure the configured provider is one you're OK sharing that content with. Set `OPENROUTER_API_KEY` (or drop a custom provider config under `~/.config/simulang/providers/`) before running `ask.mjs`.

### Required OS permissions

On **macOS**, the first run of certain examples will prompt for permissions. Grant these in **System Settings → Privacy & Security**:

- **Accessibility** — required for keyboard/mouse input and accessibility-tree reads (`keyboard.mjs`, `google_search.mjs`).
- **Screen Recording** — required for screen capture and audio loopback (`screenshot.mjs`, `google_search.mjs`, `loopback.mjs`).

The permission is granted to the process that launches Node (e.g. your terminal application), not to Node itself.
