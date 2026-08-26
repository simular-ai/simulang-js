# Changelog

All notable changes to `@simular-ai/simulang-js` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [12.1.0] - 2026-08-26

### Added

- `clickablePoint(allowDescendants?)` on `AccessibilityNode`, `AccessibilityTree` (by ref id), and `AccessibilitySnapshot` (by index) — a screen point `[x, y]` where a pointer click actually lands on the element, verified by hit-testing (Windows: UIA `GetClickablePoint`; macOS/Linux: UIA's probe ladder — center, edge midpoints, sparse grid, diagonal — checked against the platform hit test; Android: bounds center, unverified). `allowDescendants` (default `false`) also accepts a point landing on a node inside the element; a point landing on an ancestor never counts. Throws with the reason (naming the covering element when known) when no verified point exists — obscured, offscreen, or zero-sized — instead of guessing a center that would click something else.

## [12.0.3] - 2026-08-25

### Added

- `showMenu` on `AccessibilityNode`, `AccessibilityTree` (by ref id), and `AccessibilitySnapshot` (by index) — opens an element's context menu semantically (the right-click equivalent) without synthesizing pointer input, so it works on background/obscured windows; note the opened menu itself typically appears topmost/focused. Maps to `IUIAutomationElement3::ShowContextMenu` on Windows, `AXShowMenu` on macOS, the toolkit's advertised show-menu action on Linux, and a long-press on Android. Throws when the element doesn't support it, so callers can fall back to a coordinate right-click at the element's bounds.

## [12.0.2] - 2026-08-25

## [12.0.2-rc1] - 2026-08-19

### Added

- `Window.normal()` — put the window on-screen at its restored size, neither minimized nor maximized. No-op when already normal. Replaces the old `restore()` / `unmaximize()` pair with one named end-state (`ShowWindow(SW_SHOWNORMAL)` on Windows; clear `AXMinimized` then `AXFullScreen` on macOS; remove the EWMH maximized atoms and map the window on Linux).
- `Instance.root()` — root accessibility node of this application (macOS: the app element including the menu bar; Windows/Linux: the process accessibility root; Android: the current screen root, instance must be in the foreground). This is the replacement for the removed `Machine.processRoot(pid)`. It was listed in the 12.0.0-rc1 notes but missing from the bindings until now.

### Changed

- **Breaking:** `Window.restore()` and `Window.unmaximize()` are removed in favor of `Window.normal()`. The old pair were inverses of `minimize` / `maximize` that left the other state alone (`restore()` never un-maximized; `unmaximize()` never un-minimized). `normal()` always ends in the on-screen, neither-minimized-nor-maximized state. Replace both call sites with `normal()`.
- **Breaking:** `Machine.windowsForPid(pid)` is removed. Use `instance.windows()` — windows belong to an instance, and on Windows an instance is app-scoped (it stores no PID). If you only have a pid, filter `machine.windows()` by `window.pid` (this also replaces the `machine.windowsForPid(pid)[0]` migration the 12.0.0 notes suggested for `AccessibilityTree.fromPid`).
- **Breaking:** `Machine.processRoot(pid)` is removed. Use `instance.root()` or `AccessibilityTree.fromInstance(instance)` — accessibility trees are addressed by instance, not by process id (Android never supported process-scoped trees).
- `App.open(..., waitForLoadComplete)` waits until the instance has a window (or the platform reports launch finished), up to twenty seconds, instead of a short fixed delay. Desktop timeouts are logged; Android treats them as errors. Pass `false` for apps that never show a window. Existing windows of an already-running app satisfy the wait immediately — this does not wait for a window or tab the call may add.
- `Window.minimize()` / `Window.maximize()` are named end-states (no-op when already in that state). `maximize()` shows a minimized window maximized; on macOS `minimize()` leaves full screen first.
- `Instance.pid` is diagnostic only. On Windows the instance is app-scoped and the PID is resolved lazily from a window or matching process (may be `null` while the app has no window yet). On Linux it follows a single-instance handoff to the surviving process.

## [12.0.1-rc1] - 2026-08-06

### Added

- macOS `Window` background input methods (`backgroundMoveMouse`, `backgroundButton`, `backgroundClick`, `backgroundScroll`, `backgroundText`, `backgroundKey`, `backgroundKeyUnicode`, `backgroundKeyOther`, `backgroundRaw`) send input to that specific window without moving the system cursor or bringing its application to the foreground. The methods throw as unsupported on other platforms.

## [12.0.0-rc2] - 2026-07-30

### Added

- `Machine.systemRoot()` — root accessibility node spanning everything currently accessible: on desktops the system-wide root (its children are the running applications), on Android a snapshot of everything currently shown on screen (which can span the foreground app, the system bars, and a split-screen neighbor).
- `AndroidExtras.swipe(fromX, fromY, toX, toY, durationMs)` (via `machine.asAndroid()`) — touch swipe between two points in absolute device pixels, holding the finger down for `durationMs`. The cross-platform mouse lowering always synthesizes a fixed 200ms drag, so gesture timing (a brief fling with momentum vs a slow deliberate drag) was previously inexpressible.

### Changed

- **Breaking:** `Machine.root()` is renamed to `Machine.focusedRoot()` and `Machine.rootForPid(pid)` to `Machine.processRoot(pid)`, matching the underlying simulang-rs API and disambiguating from the new `Machine.systemRoot()`. Same signatures and behavior; update call sites.
- **Breaking:** `Screenshot.toGlobalDesktopCoordinates(...)` is renamed to `Screenshot.toMachineCoordinates(...)`, matching the underlying simulang-rs API. The result is in the owning machine's canonical coordinate space — on an Android machine that is the device screen, not a "global desktop", so the old name was wrong there. Same signature and behavior; update call sites.

## [12.0.0-rc1] - 2026-07-22

### Added

- `Machine` — the new single entry point for all automation, unifying the local desktop and ADB-connected Android devices behind one API. Obtain one with `Machine.local()` (infallible, zero-cost) or `Machine.android(endpoint, appiumBase?)` (connects over adb to a device running the uiautomator2/appium server). Identify it with `machine.os` (`'macos' | 'windows' | 'linux' | 'android'`, via the new `Os` string enum) and `machine.id` (hostname / adb serial). Every handle a machine returns stays bound to that machine, so the same script drives any backend:
  - **Apps**: `app(name)` (exact launcher name or Android package name), `fuzzyApp(query)`, `apps()`, `defaultBrowser()`, `foregroundApp()`.
  - **Windows**: `focusedWindow()`, `windows()`, `windowsForPid(pid)`, `windowAtPoint(x, y)`. On Android a "window" is an application task (a recents-overview entry).
  - **Accessibility**: `root()`, `rootForPid(pid)`, `nodeAtPoint(x, y)`, `snapshot()`.
  - **Screens / screenshots**: `screens()`, `mainScreen()`, and `screenFromMouse()` return machine-bound `Screen` handles (on Android all three resolve to the device's single screen); capture with `screen.screenshot(hideCursor)`, or grab a region in global desktop coordinates with `machine.screenshotCropped(x, y, width, height, hideCursor)`.
  - **Files**: `file(path, createMissing)`, `dir(path, createMissing)`, and `tempDir()` return machine-bound `File` / `Directory` handles, so the same file API reads and writes an Android device's filesystem over adb. Locally a relative path resolves against the `SimularFiles` root; on Android paths must be absolute.
  - **Audio**: `loopback(format)`, `microphone(format)`, and `player()` return machine-bound `Loopback`, `Microphone` (new), and `AudioPlayer` handles; `format` is an `AudioFormat` object (`{ sampleRate, channels }`). Not supported on Android yet — the three constructors throw there, but the signatures will not change when support lands.
  - **Mouse** (flat methods, replacing `MouseController`): `mouseButton(button, direction)`, `moveMouse(x, y, coordinate)`, `scroll(deltaX, deltaY)`, `mouseLocation()`. On Android there is no real cursor: a left press+release becomes a tap or swipe at the recorded cursor location.
  - **Keyboard** (flat methods, replacing `KeyboardController`): `typeText(text)`, `key(key, direction)`, `keyUnicode(char, direction)`, `keyOther(value, direction)`, `keyRaw(keycode, direction)`.
  - **Clipboard** (flat methods, replacing `Clipboard`): `getClipboardString()`, `setClipboardString(value)`, `getClipboardImage()`, `setClipboardImage(image)`, `clearClipboard()`, `pasteText(value)`. The two setters return a `ClipboardContent` snapshot of what the clipboard held before the write (`text` and `image` getters); nothing is restored automatically. On Android, `pasteText` commits the text through the device's input channel without touching the host clipboard.
  - **Escape hatch**: `asAndroid()` returns the phone-only surface (`AndroidExtras`), or `null` on the local desktop. Phone-only controls: `serial`, `notifications()`, `quickSettings()`, `collapseShade()`, `landscape()`, `portrait()`, `setPackageEnabled(package, enabled)`.
- Android support throughout the handle classes: `App`, `Instance`, `Window`, and `AccessibilityNode` now work identically against a local desktop or an Android device. Android accessibility nodes come from uiautomator snapshots; their action methods (`activate`, `setValue`, …) resolve back to the live screen.

### Changed

- **Breaking:** `App`, `Instance`, `Window`, `Screen`, `File`, `Directory`, and `AccessibilityNode` are now machine-bound handles obtained from a `Machine` (or from another handle). Instance methods, properties, and semantics are unchanged except where noted below, but the constructors and static factories are gone — see the removals below for the migration map.
- **Breaking:** `AccessibilitySnapshot.fromForeground()` and `AccessibilitySnapshot.fromPid(pid)` were removed; construct via `AccessibilitySnapshot.fromNode(machine.root())` / `fromNode(machine.rootForPid(pid))` or `AccessibilitySnapshot.fromWindow(window)` instead.
- **Breaking:** `Machine.setClipboardString` and `Machine.setClipboardImage` return a `ClipboardContent` snapshot (with `text` and `image` getters) instead of the previous string content, so an image previously on the clipboard is no longer lost. Replace `const prev = machine.setClipboardString(s)` with `const prev = machine.setClipboardString(s).text`.
- **Breaking:** `Instance.pid` is `number | null` (was `number` with `0` meaning unknown), matching simulang-rs's `Option<i32>`.
- **Breaking:** `AccessibilityNode.children()` throws when the subtree has been torn down or the children attribute is unreadable (was: returned an empty array). Tree walks (`snapshot`, `scoredSearch`, `childrenCollapsed`) still treat such failures as "no children".
- **Breaking:** `Window.pid` reads the pid live from the platform on each access and throws when it cannot be resolved (was: cached at enumeration time). Windows whose pid cannot be read are no longer silently dropped from `machine.windows()`, `machine.windowsForPid()`, `machine.focusedWindow()`, `machine.windowAtPoint()`, and `instance.windows()`.
- `Window.screen()` now works for Android windows too (returns the device's screen); previously it threw for windows of an Android machine.
- `AccessibilityTree` is rebuilt on the machine-bound types, extending it beyond Windows/macOS: `fromWindow(window)` accepts any machine's window (including Android tasks and Linux windows), and the new `fromInstance(instance)` works on every backend. Scoping is unchanged: on macOS `fromInstance` covers the whole application (every window plus the app menu bar), everywhere else a single window; `fromWindow` is window-scoped on every platform. `fromWindow` no longer throws (root resolution failures surface on the first snapshot instead).
- `Instance.root()` — root accessibility node of the application (macOS: app element including the menu bar; Windows/Linux: process root; Android: current screen root, instance must be focused).
- **Breaking:** `AccessibilityTree.fromForeground()` and `AccessibilityTree.fromPid(pid)` were replaced by `AccessibilityTree.fromInstance(instance)` — the old factories always targeted the local machine, while an `Instance` carries its machine. Migrate `fromForeground()` to `fromInstance(machine.foregroundApp())`; migrate `fromPid(pid)` to `fromInstance` if you have the instance, or `fromWindow(machine.windowsForPid(pid)[0])` if you only have a pid.
- **Breaking:** `AccessibilityTree.fromHwnd(id)` and the `windowId` getter were removed — both leaked the platform-specific window identity (HWND on Windows, PID on macOS). Obtain a window via `machine.windows()` / `machine.windowAtPoint(x, y)` and use `AccessibilityTree.fromWindow(window)`; read the pid from `window.pid`.
- The `sai` compatibility subpath now drives all primitives through one `Machine.local()` instance instead of per-controller singletons; primitive behavior is unchanged.

### Removed

- **Breaking:** `AccessibilityNode.controlType` (getter) and the `controlType` field on snapshot nodes. The numeric control type id was only meaningful on Windows (UIA `ControlType.Id`); other platforms returned `0` or a platform-specific numbering that wasn't comparable across backends. Use `localizedControlType` (human-readable, populated everywhere) or the normalized ARIA `role` instead.

The remaining removals are covered by `Machine` methods; migration map:

- `MouseController` → `machine.mouseButton` / `machine.moveMouse` / `machine.scroll` / `machine.mouseLocation` (the `button` and `location` methods gained the `mouse` prefix; `moveMouse` and `scroll` kept their names).
- `KeyboardController` → `machine.typeText` / `machine.key` / `machine.keyUnicode` / `machine.keyOther` / `machine.keyRaw` (`text` became `typeText`, `raw` became `keyRaw`; the rest kept their names).
- `Clipboard` → `machine.getClipboardString` / `setClipboardString` / `getClipboardImage` / `setClipboardImage` / `clearClipboard` / `pasteText`.
- `System.listApps()` → `machine.apps()`; `System.fuzzySearch(query)` → `machine.fuzzyApp(query)`.
- `App.exactName(name)` → `machine.app(name)`; `App.defaultBrowser()` → `machine.defaultBrowser()`; `App.exists(name)` → try `machine.app(name)` and catch.
- `Window.all()` → `machine.windows()`; `Window.allForPid(pid)` → `machine.windowsForPid(pid)`; `Window.focused()` → `machine.focusedWindow()`; `Window.fromPoint(x, y)` → `machine.windowAtPoint(x, y)`.
- `AccessibilityNode.fromFocusedApplication()` → `machine.root()`; `AccessibilityNode.fromPid(pid)` → `machine.rootForPid(pid)`; `AccessibilityNode.fromPoint(x, y)` → `machine.nodeAtPoint(x, y)`.
- `Screen.all()` → `machine.screens()`; `Screen.mainScreen()` → `machine.mainScreen()`; `Screen.fromCurrentMouseLocation()` → `machine.screenFromMouse()`; `Screen.fromWindow(window)` → `window.screen()`.
- `new File(path, createMissing)` → `machine.file(path, createMissing)`; `new Directory(path, createMissing)` → `machine.dir(path, createMissing)`; `Directory.temp()` → `machine.tempDir()`.
- `new LoopbackSource(channels, sampleRate)` → `machine.loopback({ sampleRate, channels })` (the handle is now called `Loopback`; `start`/`stop`/`record`/`drain` and the getters are unchanged).
- `AudioOutput.openDefault()` + `output.createPlayer()` → `machine.player()` (the `AudioPlayer` owns its output device connection; `Player`'s methods are unchanged).
- `readFile(path)` → `machine.file(path, false).read()`; `writeFile(path, content, append)` → `machine.file(path, true).write(content, append)` (then `file.path()` if you need the absolute path). The free functions always targeted the local machine and are gone so callers pick an explicit `Machine`.
- `screenshotFull(hideCursor, screen)` → `machine.screenFromMouse().screenshot(hideCursor)` (or capture a specific display via `machine.screens()` / `machine.mainScreen()` / `window.screen()`); `screenshotCropped(x, y, w, h, hideCursor)` → `machine.screenshotCropped(x, y, w, h, hideCursor)`.
- `enableAccessibilityForFrontmostApp()` → `machine.foregroundApp().enableAccessibility()`.

## [11.0.0-rc4] - 2026-07-20

## [11.0.0-rc3] - 2026-07-20

## [11.0.0-rc2] - 2026-07-17

## [11.0.0-rc1] - 2026-07-15

### Added

- `AccessibilityNode.childrenCollapsed()` — direct children with collapsible structural wrappers hoisted out: a collapsible child is skipped and its own recursively collapsed children take its place, preserving sibling order.
- `AccessibilityNode.isVisible` — `true`/`false` when the platform exposes a visibility attribute for the node (UIA `!IsOffscreen` on Windows, AT-SPI `Visible` + `Showing` states on Linux, Chromium's non-standard `AXVisible` on macOS), `null` when it doesn't (native macOS apps). The handle-walk counterpart of `AccessibilityTree.snapshot`'s `visibleOnly` filter, with the same Chromium caveat: compositor-driven visibility means rendered web content can still report `false`.
- `AccessibilityNode.automationId` now reads each platform's native stable identifier: UIA `AutomationId` on Windows, `AXIdentifier` on macOS, and AT-SPI `AccessibleId` on Linux.
- `Window.node()` — root `AccessibilityNode` of the window's accessibility subtree. Windows and Linux return cached subtrees when possible; macOS returns the window's `AXWindow` element.

- `AccessibilitySnapshot` — a materialized, indexed accessibility snapshot. Construct one via `AccessibilitySnapshot.fromNode(node)`, `.fromForeground()`, `.fromPid(pid)`, or `.fromWindow(window)`; every node of the pre-order walk gets a stable ref index that stays valid for the lifetime of the snapshot. `toStringWith(options?)` (also available as `print`) renders Playwright-style `- role "name" #automationId [ref=N]: "value"` lines with structural, interactive, depth, and character filters. Resolve refs with `node(index)`, inspect them with `len()`, `isEmpty()`, and `boundingBox(index)`, or act with `click`, `activate`, `setValue`, `toggle`, `select`, `expandCollapse`, `scrollIntoView`, and `setFocus`. Compatibility aliases remain available as `nodeCount`, `getBounds`, and `focusElement`.

## [10.1.0] - 2026-07-08

### Added

- `Window.restore()` — restore a minimized window to whatever state it had before minimizing (normal or maximized). No-op when the window is not minimized; never un-maximizes a visible maximized window. Backed by `ShowWindow(SW_RESTORE)` (Windows), clearing `AXMinimized` (macOS), and mapping the window (Linux).
- `Window.isMinimized()` and `Window.isMaximized()` — probe the window's current visual state. `isMaximized()` reports `false` for minimized windows even when they would restore to maximized; on macOS it reports the full-screen state (the zoom button `maximize()` presses enters full screen on modern macOS).
- `Window.unmaximize()` — return a maximized window to its normal size and position (inverse of `maximize()`). No-op when the window is not maximized; never touches a minimized window. On macOS it leaves full screen.

## [10.0.0] - 2026-07-03

### Added

- `Window.focused()` — the window that currently has keyboard focus, or `null` when nothing is focused (no windows open, focus on the desktop, or the frontmost app has no focused window). The read-side counterpart to the instance method `Window.focus()`. Backed by `GetForegroundWindow` (Windows), the frontmost app's `AXFocusedWindow` (macOS), and `_NET_ACTIVE_WINDOW` (Linux); throws only on genuine backend failures.
- `ImageGenModel` for generating images from a text prompt through a provider's `image_gen` service. `generate(prompt, quality?, aspectRatio?, images?)` returns a `[image, description]` tuple — the generated `Image` and the model's text description. `quality` (`ImageQuality.Fast` / `.Pro`) and `aspectRatio` (`ImageAspectRatio.Square`, `.Landscape16x9`, …) are type-safe enums defaulting to `Fast` / `Square`. Optional reference images are downscaled / transcoded and size-checked against the model's configured limits before sending. Discover aliases with `ImageGenModel.availableAliases()`; the bundled `simular_cloud` provider advertises `simular_image_gen`.
- `Image.drawBox(bounds, thickness, red, green, blue)` and `Screenshot.drawBox(...)` — paint the outline of an axis-aligned `BoundingBox` (e.g. the result of `Window.boundingBox()` or `AccessibilityNode.boundingBox()`) with an inset border `thickness` pixels wide in an opaque RGB color. On `Image` the box is in image pixels; on `Screenshot` it is in **global desktop** coordinates (converted to image pixels internally), so a grounding or element box can be passed straight in. Out-of-bounds pixels are clipped; throws on zero thickness or a degenerate box (`right <= left` / `bottom <= top`). Companion to `drawDot` for annotating bounding boxes.

### Breaking

- `Screenshot.drawDot(x, y, ...)` now interprets `(x, y)` as **global desktop** coordinates (the same space `Screenshot.ground` returns and `MouseController` consumes) rather than screenshot image pixels; the point is converted to image pixels internally, inverting the capture offset and any resampling. This lets a `Screenshot.ground` result or an element's `boundingBox()` corner be drawn without manual conversion. Update call sites that passed raw image-pixel coordinates to `Screenshot.drawDot` to pass global desktop coordinates instead. `Image.drawDot` is unchanged (still image pixels).

## [9.0.0] - 2026-06-29

### Added

- `Image.fromBase64()` now accepts GIF and WebP data URLs/payloads.
- `Image.base64DataUrl()` and `Screenshot.base64DataUrl()` for returning MIME-prefixed data URLs.
- Bundled OpenRouter config now advertises `openrouter_claude_opus` for both `AskModel` and `GroundingModel`, including Claude-specific grounding coordinate scaling and image size/format limits.
- `StateSatisfiesModel` for evaluating screen state against a natural-language condition through the dedicated state-satisfaction provider.

### Changed

- `Image.base64()` and `Screenshot.base64()` now return raw base64 without a `data:image/...;base64,` prefix, matching the Rust API. Use `base64DataUrl()` when a MIME-prefixed data URL is needed.

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
