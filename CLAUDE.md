# `@simular-ai/simulang-js`

Node.js bindings for the Rust `simulang-rs` crate (via napi-rs). Cross-platform
desktop automation: apps, windows, accessibility trees, mouse/keyboard,
screenshots, clipboard, audio, and VLM/LLM/STT model access.

This file ships in the npm tarball alongside `index.d.ts` and is versioned
with it.

## Where the API is documented

Read **`index.d.ts`** first — it is the source of truth. Every class,
function, and enum is fully typed (~2100 lines) and carries JSDoc covering
idioms, lifecycle rules, platform quirks, and inter-API trade-offs that types
alone can't express. The JSDoc is generated from doc comments,
so the per-symbol guidance there is authoritative — trust it over any
restatement elsewhere.

## Mental model

- Calls into the native module are **synchronous**. No Promises are returned;
  `Result::Err` on the Rust side is translated to a thrown JS exception by
  napi-rs, so JS callers `try`/`catch` as usual.
- Many objects are **handles to platform resources** (windows, audio devices,
  accessibility trees, file/directory handles). Their lifetime matters;
  dropping them can free the underlying resource.
- Coordinates live on the **global desktop**: top-left origin at `(0, 0)` on
  the primary monitor, in **OS-native units** — the unit is **not** the same
  on every platform:
  - **Windows / Linux**: **physical pixels** (raw hardware pixels).
  - **macOS**: **logical points** — on a 2× Retina display one point spans two
    hardware pixels, so a 1920×1080-logical screen is `1920×1080` here, not
    `3840×2160`.

  These are the native units the OS input/accessibility APIs expect, **not**
  the browser logical/CSS pixel. Within a single OS every API speaks that OS's
  unit, so coordinates round-trip between `MouseController`, the various
  `boundingBox()` methods, `Screenshot.toGlobalDesktopCoordinates()`, and
  grounding output **without conversion**; only code crossing into a different
  coordinate system (e.g. an Electron overlay measured in CSS pixels) must
  account for the per-OS unit. Monitors arranged to the left of / above the
  primary contribute **negative** coordinates, so don't assume `x, y >= 0`.

## Logging is on by default

`import '@simular-ai/simulang-js'` auto-installs a stderr logger sink — no
explicit `initLogger()` call required. Records from simulang-rs (clipboard
reads/writes, keyboard input, app launches, window state changes, …) show
up on the host process's stderr formatted as `[level] message`. The Rust
`target` (module path, e.g. `simulang_rs::common::clipboard`) is omitted
from the default format to keep terminal output readable; install a
custom callback via `initLogger(cb)` to access it on the record payload.

Filter resolution: `RUST_LOG` if set, otherwise `simulang_rs=info,warn` —
info-level records from simulang's own actions, warn-level from every
other crate. Override at any time with `initLogger(...)`:

- `initLogger(null, 'off')` — silence everything
- `initLogger(null, 'simulang_rs=debug,warn')` — bump verbosity
- `initLogger(cb)` — forward records to a JS callback (log window, file
  appender, pino/winston, Electron renderer over IPC, …) instead of stderr

`initLogger` atomically swaps both the sink and the filter on every call,
so it composes cleanly with the auto-install.

## Live log viewer (optional)

`@simular-ai/simulang-log-viewer` is a **human-facing aid**: a floating,
always-on-top log window that lets the person running the script watch
log records in real time when stdout is hidden behind the app
being driven. Suggest it when the user is writing an interactive
automation script and would benefit from live visibility; skip it for
headless / CI / unattended runs (no human is there to watch the window,
and it adds a window-spawning subprocess for no benefit). **Optional peer
dependency — install it explicitly.**

The window is **click-through by default**, so mouse input passes straight
through to whatever is underneath and never interferes with the app being
automated. On macOS it is also excluded from screen captures system-wide
(via `NSWindowSharingNone`), so `screenshotFull` / `screenshotCropped` and
any other capture tool won't include the viewer in the result. A global
hotkey (`Ctrl+Shift+Option+L` on macOS, `Ctrl+Shift+Alt+L` everywhere
else, also shown inside the window) toggles "grab mode": pressing it once
turns click-through off, pauses execution, and makes the window draggable;
pressing it again restores click-through and resumes.

Re-route records into the window via `initLogger` (overriding the
default stderr sink) and respect the pause inside automation loops:

```js
import { initLogger } from '@simular-ai/simulang-js'
import { LogWindow } from '@simular-ai/simulang-log-viewer'

const win = new LogWindow()
win.spawn()
initLogger((rec) => win.log(`[${rec.level}] [${rec.target}] ${rec.message}`), 'info')

// Yield while the user has paused the viewer (e.g. to drag it).
await win.waitIfPaused()
```

(The simpler way is to `import '@simular-ai/simulang-log-viewer'` for its
side effect — the package's auto-install entry point spawns the window,
registers the pause hook, and calls `initLogger` for you.)

The second argument to `initLogger` is a filter spec in the `RUST_LOG`
environment-variable syntax used by Rust's `env_logger` / `log` crates —
e.g. `'info'` for a global level, `'simulang_rs=debug,warn'` for per-module
scoping. See `initLogger`'s JSDoc in `index.d.ts` for the full grammar
and the auto-install default.

Other methods: `clear()` drops displayed messages, `isPaused` getter for a
non-blocking check, `close()` to shut the viewer (also automatic on parent
exit).
