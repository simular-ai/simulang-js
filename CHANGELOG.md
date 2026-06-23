# Changelog

All notable changes to `@simular-ai/simulang-js` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [8.1.0] - 2026-06-23

### Added

- `AccessibilityNode.parent()`, `ancestors()`, direct/strict ancestry checks, and `lowestCommonAncestor(other)` for navigating accessibility-tree relationships. `parent()` returns `null` when a node has no parent, `ancestors()` stops only when the parent chain is fully resolved, and both throw on lookup failure; `lowestCommonAncestor` returns a `[node, level]` tuple where `level` is from the reached parentless node, returns `null` only for resolved unrelated trees, and throws if either parent chain cannot be resolved.
- `Window.screenshot(hideCursor)` — captures just this window's pixels from its own backing store (macOS `ScreenCaptureKit` window filter / Windows `Windows.Graphics.Capture`), so occluding windows don't bleed through and hardware-accelerated content (Chrome, Electron, D3D apps) is captured correctly; off-display or minimized windows throw.
- `Window.ground(model, concept)` — locate a concept within this window and return its global desktop coordinates (sugar for `screenshot(true).ground(model, concept)`); restricting the search to the window's bounds is faster and more accurate than grounding a full-screen screenshot.
- `AccessibilityNode.url` — the node's raw hyperlink target as a `string`, or `null` when the node isn't a link, has no target, or the platform doesn't expose it.
- `Image.drawDot(x, y, radius, red, green, blue)` and `Screenshot.drawDot(...)` — paint a filled opaque RGB disc at an image-pixel coordinate (companion to `drawGrid`; `radius` 0 paints a single pixel, out-of-bounds points are no-ops). Handy for annotating grounding results.

### Changed

- Frame-relative input on a `Window` (`moveMouse`, `click`) now throws when the target point maps off every connected display, instead of letting the OS silently clamp the cursor to a screen edge and click the wrong location.

## [8.0.0] - 2026-06-05

### Added

- `AccessibilityNode.fromPoint(x, y)` — element under a screen coordinate via the platform hit-test (UIA `ElementFromPoint` on Windows, `AXUIElementCopyElementAtPosition` on macOS, recursive AT-SPI `GetAccessibleAtPoint` on Linux). Coordinates are global desktop coordinates in OS-native units (physical pixels on Windows/Linux, logical points on macOS), the same space as `boundingBox()` and `MouseController`; returns an uncached, one-shot handle for reading properties such as `.boundingBox()` / `.overallDescription`, or `null` when the point has no accessible element (empty desktop, gaps, or — on Linux — outside the focused app). Throws only on a genuine backend failure.
- `AccessibilityTree.findByDescription(description)` — every node whose `overallDescription` exactly equals `description` (pre-order depth-first)
- `GroundingModel.checkAuth()`, `AskModel.checkAuth()`, and `SttModel.checkAuth()` — probe the provider's auth-check endpoint to validate credentials before launching any UI automation. Throws (and logs a warning) on rejection or transport failure; no-op for providers without an auth-check endpoint. Idiom: `try { model.checkAuth() } catch { process.exit(1) }`.

### Changed

- `GroundingModel.default()` / `AskModel.default()` / `SttModel.default()` now produce a per-candidate diagnostic when no provider has working credentials, naming each provider and explaining exactly why its credentials were unavailable (env var unset, env var empty, credentials file missing, …). The previous error misleadingly said "no provider advertises a VLM service" even when the provider was correctly configured but its API-key env var was unset.
- Corrected the documented coordinate space across the API (`MouseController`, `Window.boundingBox()`, `AccessibilityNode.boundingBox()` / `fromPoint()`, `Screen.boundingBox()`, screenshots, and grounding output). Coordinates are in **OS-native units** — physical pixels on Windows/Linux and **logical points on macOS** (where one point spans two hardware pixels on a 2× display) — not uniformly "physical pixels" as previously stated. Behavior is unchanged; coordinates still round-trip between these APIs without conversion on a given OS.

### Breaking

- `Image.addGrid()` and `Screenshot.addGrid()` are renamed to `drawGrid()` to match the underlying `simulang-rs` API. Same signature and behavior; update call sites from `.addGrid(w, h)` to `.drawGrid(w, h)`.
- `Screenshot.toGlobalPhysicalPixels()` is renamed to `Screenshot.toGlobalDesktopCoordinates()`. The old name implied physical pixels, but the result is in the canonical global-desktop space (OS-native units — logical points on macOS, physical pixels on Windows/Linux). Same signature and behavior; update call sites from `.toGlobalPhysicalPixels(...)` to `.toGlobalDesktopCoordinates(...)`.

## [7.0.1] - 2026-05-22

## [7.0.0] - 2026-05-22

### Added

- `Instance.windows()` — enumerate visible top-level windows belonging to a specific opened application (companion to the global `Window.all()` / `Window.allForPid()` enumerators).
- `Window.boundingBox()` — live bounding box of a window in global physical pixels (`right` / `bottom` exclusive, Playwright / DOM convention).
- `Screen.boundingBox()` — live bounding box of a screen in global physical pixels (matches `Window.boundingBox()` shape; secondary monitors may report negative `left` / `top`).
- `Instance.close()`, `Instance.kill()`, and `Instance.isRunning()` — request graceful exit, force-terminate, or poll whether the underlying process is still running.
- `Screen.all()`, `Screen.fromWindow(window)`, and `Window.screen()` — enumerate connected displays and find the screen a specific window lives on (multi-monitor support). Matches the OS-native algorithm (`NSWindow.screen` on macOS, `MonitorFromWindow` on Windows); throws when the window has no measurable overlap with any display.
- `Window.focus()` — bring a specific window to the foreground.
- `Window.moveMouse()`, `Window.button()`, `Window.click()`, `Window.scroll()` — frame-relative mouse input on a window. Coordinates are relative to the window's `boundingBox()` top-left (i.e. include the title bar / chrome); the caller is responsible for focusing the window first if needed.

### Changed

- Clarified that mouse coordinates in `MouseController` are in the **global virtual desktop** (top-left of the primary monitor at `(0, 0)`, physical pixels, may be **negative** on secondary monitors arranged to the left of / above the primary). The previous "current screen" wording was wrong on multi-monitor setups.

### Breaking

- `Screen.dimensions()` is removed; use `Screen.boundingBox()` and read its `right - left` / `bottom - top` (or the `left` / `top` origin). The previous `[x, y, width, height]` tuple shape was redundant with `Window.boundingBox()` and the accessibility-tree `BoundingBox`, and a single canonical shape is friendlier for AI agents driving the API.

## [6.0.1] - 2026-05-16

### Added

- `AskModel` (`AskModel.default()`, `AskModel.byAlias()`, `AskModel.availableAliases()`, `ask()`) — JS bindings for the `ask` LLM primitive (OpenAI-compatible chat completions, optional vision via `Image` attachments).

## [6.0.0] - 2026-05-15

### Added

- `setPauseHook(fn | null)` on the JS wrapper — synchronous hook invoked before every exported call; used by `@simular-ai/simulang-log-viewer` for pause/grab behavior without wrapping each API by hand.
- Default stderr logging on `import '@simular-ai/simulang-js'` — `initLogger` auto-installs unless overridden; filter follows `RUST_LOG` or `simulang_rs=info,warn`.
