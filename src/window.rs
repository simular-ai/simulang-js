use napi::Error;
use napi_derive::napi;
use simulang_rs::Key as SimulangKey;
use simulang_rs::Window as SimulangWindow;
use simulang_rs::traits::AXNodeSynthetic;

use crate::accessibility_node::AccessibilityNode;
use crate::ax_tree::{BoundingBox, TraversalOrder};
use crate::key::Key;
use crate::language_model::vlm::GroundingModel;
use crate::mouse::{Button, Direction};
use crate::screen::Screen;
use crate::screenshot::Screenshot;

/// Handle to an on-screen window of a machine. Obtain via
/// [`Machine.windows`], [`Machine.focusedWindow`], or [`Instance.windows`];
/// exposes read-only metadata (`title`, `pid`), state probes
/// (`isMinimized`, `isMaximized`), named visual-state actions
/// (`minimize`, `maximize`, `normal`, `close`), window-relative input, and
/// accessibility access. On Android a "window" is an application task (a
/// recents-overview entry).
#[napi]
pub struct Window {
  pub(crate) inner: SimulangWindow,
}

#[napi]
impl Window {
  #[napi(getter)]
  #[must_use]
  /// Window title (may be empty).
  pub fn title(&self) -> String {
    self.inner.title()
  }

  #[napi(getter)]
  /// Process ID that owns this window.
  pub fn pid(&self) -> napi::Result<i32> {
    self.inner.pid().map_err(Error::from_reason)
  }

  #[napi]
  /// Root accessibility node for this window.
  pub fn node(&self) -> napi::Result<AccessibilityNode> {
    self
      .inner
      .node()
      .map(AccessibilityNode::new)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Live bounding box of the window on the global desktop, in the canonical
  /// coordinate space (OS-native units; see [`Machine`]).
  /// `right` and `bottom` are exclusive (Playwright / DOM convention).
  /// Coordinates can be negative when the window sits on a monitor that
  /// is arranged to the left of / above the primary display.
  pub fn bounding_box(&self) -> napi::Result<BoundingBox> {
    self
      .inner
      .bounding_box()
      .map(BoundingBox::from)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// The screen this window lives on: locally the connected display
  /// containing the largest area of [`Window.boundingBox`] (the same
  /// heuristic the OS uses to decide a window's "owning" screen), on
  /// Android the device's screen.
  ///
  /// Locally this throws if the window has no measurable overlap with any
  /// connected display — for example when the window is fully off-screen,
  /// minimised to an off-screen state, or on a virtual desktop with no
  /// attached display. There is no "correct" screen to pick in that case;
  /// callers that prefer a fallback can wrap in `try` / `catch` and call
  /// [`Machine.mainScreen`].
  pub fn screen(&self) -> napi::Result<Screen> {
    self
      .inner
      .screen()
      .map(|inner| Screen { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Whether the window is currently minimized (Android: the task is
  /// backgrounded).
  ///
  /// Platform notes:
  /// - **Windows**: `IsIconic`.
  /// - **macOS**: reads the accessibility `AXMinimized` attribute.
  /// - **Linux**: EWMH `_NET_WM_STATE` contains `_NET_WM_STATE_HIDDEN`.
  pub fn is_minimized(&self) -> napi::Result<bool> {
    self.inner.is_minimized().map_err(Error::from_reason)
  }

  #[napi]
  /// Whether the window is currently maximized (visible and zoomed / full
  /// screen). A minimized window reports `false`. Android: the task is on
  /// screen and occupies it fully.
  ///
  /// Platform notes:
  /// - **Windows**: `IsZoomed && !IsIconic`.
  /// - **macOS**: accessibility `AXFullScreen && !AXMinimized`. macOS has
  ///   no native "maximized" concept — [`Window.maximize`] enters full
  ///   screen on modern macOS, so this reports that state. A window
  ///   manually zoomed to fill the screen (Option-click the zoom button)
  ///   reports `false`.
  /// - **Linux**: EWMH `_NET_WM_STATE` contains both `MAXIMIZED_HORZ` and
  ///   `MAXIMIZED_VERT`, and not `HIDDEN`.
  pub fn is_maximized(&self) -> napi::Result<bool> {
    self.inner.is_maximized().map_err(Error::from_reason)
  }

  #[napi]
  /// Put the window into the **minimized** visual state (Dock / taskbar /
  /// backgrounded). No-op when already minimized.
  ///
  /// Platform notes:
  /// - **Windows**: `ShowWindow(SW_MINIMIZE)`.
  /// - **macOS**: sets the accessibility `AXMinimized` attribute. A
  ///   full-screen window is taken out of full screen first; otherwise
  ///   the attribute is ignored.
  /// - **Linux**: ICCCM `WM_CHANGE_STATE` → `IconicState`.
  pub fn minimize(&self) -> napi::Result<()> {
    self.inner.minimize().map_err(Error::from_reason)
  }

  #[napi]
  /// Put the window into the **maximized** visual state: on-screen and
  /// maximized / full screen. A minimized window is shown maximized.
  /// No-op when already maximized.
  ///
  /// Platform notes:
  /// - **Windows**: `ShowWindow(SW_MAXIMIZE)`.
  /// - **macOS**: sets `AXFullScreen`. A minimized window is
  ///   un-minimized first.
  /// - **Linux**: EWMH `_NET_WM_STATE` add of `MAXIMIZED_HORZ` +
  ///   `MAXIMIZED_VERT`, then maps the window if it was iconified.
  pub fn maximize(&self) -> napi::Result<()> {
    self.inner.maximize().map_err(Error::from_reason)
  }

  #[napi]
  /// Put the window into the **normal** visual state: on-screen, neither
  /// minimized nor maximized. A minimized or maximized window is shown at
  /// its restored size. No-op when already normal.
  ///
  /// Platform notes:
  /// - **Windows**: `ShowWindow(SW_SHOWNORMAL)`.
  /// - **macOS**: clears `AXMinimized` if needed, then clears
  ///   `AXFullScreen`.
  /// - **Linux**: EWMH `_NET_WM_STATE` remove of `MAXIMIZED_HORZ` +
  ///   `MAXIMIZED_VERT`, then maps the window if it was iconified.
  pub fn normal(&self) -> napi::Result<()> {
    self.inner.normal().map_err(Error::from_reason)
  }

  #[napi]
  /// Close the window (user-equivalent to clicking the close button /
  /// pressing Alt+F4 / Cmd+W). Android: remove the task, including its
  /// recents card.
  pub fn close(&self) -> napi::Result<()> {
    self.inner.close().map_err(Error::from_reason)
  }

  #[napi]
  /// Bring this window to the foreground and give it keyboard focus.
  ///
  /// Platform notes:
  /// - **macOS**: raises the window via the accessibility `AXRaise`
  ///   action so it becomes the app's key window, then activates the
  ///   owning application with default options so that key window
  ///   surfaces.
  /// - **Windows**: `SetForegroundWindow` + `BringWindowToTop`. Subject
  ///   to the foreground lock — the call may be downgraded to a
  ///   taskbar flash if the foreground process didn't grant
  ///   permission.
  /// - **Linux**: EWMH `_NET_ACTIVE_WINDOW` client message to the WM.
  pub fn focus(&self) -> napi::Result<bool> {
    self.inner.focus().map_err(Error::from_reason)
  }

  #[napi]
  /// Move the cursor to a point inside this window.
  ///
  /// `(x, y)` are **window-frame-relative**, in OS-native units (physical
  /// pixels on Windows/Linux, logical points on macOS) — `(0, 0)` is the
  /// top-left of [`Window.boundingBox`], which includes the title bar and
  /// other OS chrome.
  ///
  /// Two validation passes run before any input is synthesized, and either
  /// failing throws:
  /// 1. `(x, y)` must lie inside the window frame (`[0, width) × [0,
  ///    height)`).
  /// 2. The resulting global point must fall on at least one connected
  ///    display. Windows dragged partly off-screen can map an in-frame
  ///    point to a coordinate the OS would silently clamp to a display
  ///    edge — erroring out is strictly more useful than moving the cursor
  ///    somewhere unintended and clicking the wrong thing.
  ///
  /// Does **not** focus the window. Call [`Window.focus`] (or
  /// [`Instance.focus`]) first if the click target requires the window
  /// to be active.
  ///
  /// On Android there is no real cursor: the point is recorded and the
  /// next [`Window.button`] press/release becomes a tap or swipe there.
  ///
  /// Input is routed to the window's own pointer: the host's global
  /// mouse for local windows, the device's emulated pointer for Android
  /// windows.
  pub fn move_mouse(&mut self, x: i32, y: i32) -> napi::Result<()> {
    self.inner.move_mouse(x, y).map_err(Error::from_reason)
  }

  #[napi]
  /// Press / release a mouse button at the cursor's current location.
  ///
  /// Provided for ergonomic parity with [`Window.moveMouse`] — combine
  /// the two for press / drag / release patterns. Takes no coordinate
  /// argument: pair with `moveMouse` when you need to control where the
  /// press lands.
  pub fn button(&mut self, button: Button, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .button(button.into(), direction.into())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Move the cursor to `(x, y)` inside this window and synthesise a
  /// mouse button event. Sugar for `moveMouse(x, y); button(...)`.
  ///
  /// Coordinates are window-frame-relative (see [`Window.moveMouse`]), and
  /// are subject to the same bounds / on-screen validation: an off-frame or
  /// off-screen target throws instead of clicking the wrong location.
  /// Does not focus the window.
  pub fn click(
    &mut self,
    x: i32,
    y: i32,
    button: Button,
    direction: Direction,
  ) -> napi::Result<()> {
    self
      .inner
      .click(x, y, button.into(), direction.into())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Scroll the wheel by `(deltaX, deltaY)` ticks at the cursor's
  /// current location. Positive `deltaY` scrolls down; positive
  /// `deltaX` scrolls right. Does not reposition the cursor first —
  /// pair with [`Window.moveMouse`] if you need scrolling to happen at
  /// a specific point inside the window.
  pub fn scroll(&mut self, delta_x: i32, delta_y: i32) -> napi::Result<()> {
    self
      .inner
      .scroll(delta_x, delta_y)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Move this window's private background pointer without moving the system
  /// cursor or bringing the application to the foreground.
  ///
  /// `(x, y)` are window-frame-relative. The location is retained for
  /// subsequent [`Window.backgroundButton`] and [`Window.backgroundScroll`]
  /// calls, and stays anchored to the window frame even if the window is
  /// moved in between.
  ///
  /// Supported on macOS only. Throws on other platforms or when macOS
  /// background event delivery is unavailable.
  pub fn background_move_mouse(&mut self, x: i32, y: i32) -> napi::Result<()> {
    self
      .inner
      .background_move_mouse(x, y)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Send a mouse-button event to this window without moving the system
  /// cursor or bringing the application to the foreground.
  ///
  /// Call [`Window.backgroundMoveMouse`] first to choose the location.
  /// Supported on macOS only.
  pub fn background_button(&mut self, button: Button, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .background_button(button.into(), direction.into())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Send a mouse-button event to `(x, y)` in this window without moving the
  /// system cursor or bringing the application to the foreground.
  ///
  /// Coordinates are window-frame-relative. Supported on macOS only.
  pub fn background_click(
    &mut self,
    x: i32,
    y: i32,
    button: Button,
    direction: Direction,
  ) -> napi::Result<()> {
    self
      .inner
      .background_click(x, y, button.into(), direction.into())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Scroll this window at its private background-pointer location without
  /// bringing the application to the foreground.
  ///
  /// Call [`Window.backgroundMoveMouse`] first to choose the location.
  /// Supported on macOS only.
  pub fn background_scroll(&mut self, delta_x: i32, delta_y: i32) -> napi::Result<()> {
    self
      .inner
      .background_scroll(delta_x, delta_y)
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Type text into this window without bringing its application to the
  /// foreground. Supported on macOS only.
  pub fn background_text(&mut self, text: String) -> napi::Result<()> {
    self
      .inner
      .background_text(&text)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Send a key event to this window without bringing its application to the
  /// foreground. Supported on macOS only.
  pub fn background_key(&mut self, key: Key, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .background_key(key.try_into()?, direction.into())
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Send a Unicode key event to this window without bringing its application
  /// to the foreground. Provide a single character. Supported on macOS only.
  pub fn background_key_unicode(
    &mut self,
    value: String,
    direction: Direction,
  ) -> napi::Result<()> {
    let mut chars = value.chars();
    let Some(character) = chars.next() else {
      return Err(Error::from_reason(
        "backgroundKeyUnicode requires a character.",
      ));
    };
    if chars.next().is_some() {
      return Err(Error::from_reason(
        "backgroundKeyUnicode expects a single character.",
      ));
    }
    self
      .inner
      .background_key(SimulangKey::Unicode(character), direction.into())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Send a platform-specific key value to this window without bringing its
  /// application to the foreground. Supported on macOS only.
  pub fn background_key_other(&mut self, value: u32, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .background_key(SimulangKey::Other(value), direction.into())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Send a raw keycode to this window without bringing its application to
  /// the foreground. Supported on macOS only.
  pub fn background_raw(&mut self, keycode: u16, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .background_raw(keycode, direction.into())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Render the window's accessibility subtree as an indented
  /// Playwright-style aria snapshot.
  ///
  /// One line per node, two spaces of indentation per depth level, in
  /// pre-order DFS. Roles are emitted raw (`AXWindow` on macOS,
  /// `UIA.ControlType.*` on Windows), with title and value appended when
  /// non-empty. No refs are assigned — for ref-based interaction use
  /// [`AccessibilityTree.snapshot`] instead.
  pub fn snapshot(&self) -> napi::Result<String> {
    self.inner.snapshot().map_err(Error::from_reason)
  }

  #[napi]
  /// Capture just this window's pixels.
  ///
  /// Local windows read from their own backing store, so overlapping
  /// windows do **not** bleed through and the whole window is captured even
  /// where it is occluded or extends past a display edge — partially
  /// off-screen and multi-display-spanning windows are captured in full.
  /// The window must be on-screen at capture time: a minimized or fully
  /// off-display window throws, because there is nothing to capture.
  ///
  /// On Android the capture is the physical screen, so the task must be
  /// the visible one (`hideCursor` is ignored — a phone has no cursor).
  pub fn screenshot(&self, hide_cursor: bool) -> napi::Result<Screenshot> {
    self
      .inner
      .screenshot(hide_cursor)
      .map(|inner| Screenshot { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Locate `concept` inside this window's pixels and return its **global
  /// desktop coordinates** `[x, y]` in OS-native units (may be negative on
  /// multi-monitor setups; see [`Machine`]), ready to feed straight
  /// into [`Machine.moveMouse`] / click helpers on the owning machine.
  ///
  /// Sugar for `screenshot(true).ground(model, concept)` — the cursor is
  /// hidden because it can occlude or distract the model. Restricts the
  /// model's search to this window's bounds, which is both faster (fewer
  /// pixels to upload) and more accurate (no risk of grounding onto a
  /// concept that happens to be elsewhere on the screen) than grounding a
  /// full-screen screenshot.
  ///
  /// Drop down to [`Window.screenshot`] + [`Screenshot.ground`] directly
  /// when you want to keep the cursor visible, reuse the screenshot across
  /// multiple `ground` calls, or shrink / compress the image before
  /// sending it.
  ///
  /// The screenshot includes any portion of the window that's past the
  /// desktop edge (the capture reads the window's own backing store, not a
  /// display rectangle). A `concept` the model locates in that off-screen
  /// region returns coordinates the OS won't route a click to;
  /// [`Window.moveMouse`] catches this and errors out instead of clicking
  /// the nearest on-screen point.
  pub fn ground(&self, model: &GroundingModel, concept: String) -> napi::Result<(i32, i32)> {
    self
      .inner
      .ground(&model.inner, &concept)
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  /// Search this window's accessibility subtree by *concept text*, using
  /// bag-of-words paired-Jaccard scoring against each node's
  /// `overallDescription`
  /// (`simulang_rs::AXNodeSynthetic::summary_with_context`).
  ///
  /// Mirrors `simulang_rs::Window::scored_search`, which resolves the
  /// window's accessibility root and walks it (on Windows this prebuilds
  /// the cached UIA subtree so the walk costs a single IPC). Returns every
  /// node whose score equals the maximum found and exceeds `threshold`.
  ///
  /// - `order`               – `TraversalOrder.DepthFirst` or
  ///                           `TraversalOrder.BreadthFirst`
  /// - `max_nodes`           – upper bound on nodes visited (uses `.take()`
  ///                           over the walk)
  /// - `collapse_structural` – hoist empty structural wrappers out of the
  ///                           walk before scoring
  /// - `query`               – natural-language concept Jaccard-compared
  ///                           against each node's `overallDescription`
  /// - `threshold`           – minimum score to keep a node
  ///
  /// Returned nodes are full `AccessibilityNode` handles — call action
  /// methods (`activate`, `setValue`, …) directly on them, walk children
  /// with `.children()`, or render a snapshot with `.snapshot()`.
  #[allow(clippy::needless_pass_by_value)]
  pub fn scored_search(
    &self,
    order: TraversalOrder,
    max_nodes: u32,
    collapse_structural: bool,
    query: String,
    threshold: f64,
  ) -> napi::Result<Vec<AccessibilityNode>> {
    let matches = self
      .inner
      .scored_search(
        order.into(),
        max_nodes as usize,
        collapse_structural,
        |_| true,
        |node| simulang_rs::BowJaccard::score(&node.summary_with_context(), &query).primary,
        threshold,
      )
      .map_err(Error::from_reason)?;
    Ok(AccessibilityNode::from_nodes(matches))
  }
}
