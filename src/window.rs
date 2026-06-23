use napi::Error;
use napi_derive::napi;
use simulang_rs::Window as SimulangWindow;
use simulang_rs::traits::{
  AXNodeSynthetic, AXNodeTrait, BoundedInputTrait, BoundingBoxTrait, WindowTrait,
};

use crate::accessibility_node::AccessibilityNode;
use crate::ax_tree::{BoundingBox, TraversalOrder};
use crate::language_model::vlm::GroundingModel;
use crate::mouse::{Button, Direction};
use crate::screen::Screen;
use crate::screenshot::Screenshot;

/// Handle to an on-screen window. Constructed via [`Window.all`] or
/// [`Window.allForPid`] and exposes read-only metadata (`title`, `pid`)
/// alongside basic actions (`minimize`, `maximize`, `close`).
#[napi]
pub struct Window {
  pub(crate) inner: SimulangWindow,
  cached_pid: i32,
}

#[napi]
impl Window {
  #[napi]
  #[must_use]
  /// All visible top-level windows for a given process.
  pub fn all_for_pid(pid: i32) -> Vec<Window> {
    SimulangWindow::all_for_pid(pid)
      .into_iter()
      .filter_map(|w| Window::try_from(w).ok())
      .collect()
  }

  #[napi]
  #[must_use]
  /// All visible top-level windows across every process.
  pub fn all() -> Vec<Window> {
    SimulangWindow::all()
      .into_iter()
      .filter_map(|w| Window::try_from(w).ok())
      .collect()
  }

  #[napi]
  /// The top-level window at screen coordinates `(x, y)`, or `null` when no
  /// window sits under the point.
  ///
  /// `(x, y)` are global desktop coordinates in the canonical coordinate
  /// space (OS-native units; see [`MouseController`]) — the same space
  /// `Window.boundingBox`, `AccessibilityNode.fromPoint`, and
  /// `MouseController.location` use, so a cursor `location()` can be passed
  /// straight in.
  ///
  /// Platform notes:
  /// - **macOS**: returns the containing `AXWindow` of the element under the
  ///   cursor — the window itself, not the whole application.
  /// - **Windows**: returns the top-level window (`GA_ROOT`) of whatever
  ///   control sits under the cursor.
  /// - **Linux**: always `null` (no per-point window hit-test).
  pub fn from_point(x: i32, y: i32) -> napi::Result<Option<Window>> {
    Ok(
      SimulangWindow::from_point(x, y)
        .map_err(Error::from_reason)?
        .and_then(|w| Window::try_from(w).ok()),
    )
  }

  #[napi(getter)]
  #[must_use]
  /// Window title (may be empty).
  pub fn title(&self) -> String {
    self.inner.title()
  }

  #[napi(getter)]
  #[must_use]
  /// Process ID that owns this window.
  pub fn pid(&self) -> i32 {
    self.cached_pid
  }

  #[napi]
  /// Live bounding box of the window on the global desktop, in the canonical
  /// coordinate space (OS-native units; see
  /// [`MouseController`]).
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
  /// The screen this window currently lives on.
  ///
  /// "Lives on" is the connected display that contains the largest
  /// area of [`Window.boundingBox`] — the same heuristic the OS uses
  /// to decide a window's "owning" screen, so the result matches what
  /// the system considers the window's screen (the one its window
  /// controls render on, the one full-screen mode targets, etc.).
  ///
  /// Throws if the window has no measurable overlap with any connected
  /// display. Callers that prefer a fallback can wrap in
  /// `try` / `catch` and call [`Screen.mainScreen`]. Equivalent to
  /// `Screen.fromWindow(window)`.
  pub fn screen(&self) -> napi::Result<Screen> {
    Screen::from_window(self)
  }

  #[napi]
  /// Minimize the window (hide to taskbar / Dock).
  pub fn minimize(&self) -> napi::Result<()> {
    self.inner.minimize().map_err(Error::from_reason)
  }

  #[napi]
  /// Maximize / zoom the window.
  pub fn maximize(&self) -> napi::Result<()> {
    self.inner.maximize().map_err(Error::from_reason)
  }

  #[napi]
  /// Close the window (user-equivalent to clicking the close button /
  /// pressing Alt+F4 / Cmd+W).
  pub fn close(&self) -> napi::Result<()> {
    self.inner.close().map_err(Error::from_reason)
  }

  #[napi]
  /// Bring this window to the foreground and give it keyboard focus.
  ///
  /// Platform notes:
  /// - **macOS**: activates the owning application and raises the
  ///   window via the accessibility `AXRaise` action.
  /// - **Windows**: `SetForegroundWindow` + `BringWindowToTop`.
  ///   Subject to the foreground-lock rules — may silently downgrade
  ///   to a taskbar flash if the foreground process hasn't granted
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
  pub fn move_mouse(&self, x: i32, y: i32) -> napi::Result<()> {
    self.inner.move_mouse(x, y).map_err(Error::from_reason)
  }

  #[napi]
  /// Press / release a mouse button at the cursor's current location.
  ///
  /// Provided for ergonomic parity with [`Window.moveMouse`] — combine
  /// the two for press / drag / release patterns. Takes no coordinate
  /// argument: pair with `moveMouse` when you need to control where the
  /// press lands.
  pub fn button(&self, button: Button, direction: Direction) -> napi::Result<()> {
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
  pub fn click(&self, x: i32, y: i32, button: Button, direction: Direction) -> napi::Result<()> {
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
  pub fn scroll(&self, delta_x: i32, delta_y: i32) -> napi::Result<()> {
    self
      .inner
      .scroll(delta_x, delta_y)
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
  /// Reads from the window's own backing store, so overlapping windows do
  /// **not** bleed through and the whole window is captured even where it is
  /// occluded or extends past a display edge — partially off-screen and
  /// multi-display-spanning windows are captured in full. The window must be
  /// on-screen at capture time: a minimized or fully off-display window
  /// throws, because there is nothing to capture.
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
  /// multi-monitor setups; see [`MouseController`]), ready to feed straight
  /// into [`MouseController.moveMouse`] / click helpers.
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
  /// Mirrors `simulang_rs::Window::scored_search` (macOS / Linux) — on
  /// Windows we use the same underlying primitive via
  /// `WindowTrait::node().scored_search(...)`, which prebuilds the cached
  /// UIA subtree so the walk costs a single IPC. Returns every node whose
  /// score equals the maximum found and exceeds `threshold`.
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
    let root = self.inner.node().map_err(Error::from_reason)?;
    let matches = root.scored_search(
      order.into(),
      max_nodes as usize,
      collapse_structural,
      |_| true,
      |node| simulang_rs::BowJaccard::score(&node.summary_with_context(), &query).primary,
      threshold,
    );
    Ok(AccessibilityNode::from_nodes(matches))
  }
}

impl TryFrom<SimulangWindow> for Window {
  type Error = String;

  fn try_from(inner: SimulangWindow) -> Result<Self, Self::Error> {
    let cached_pid = inner.pid()?;
    Ok(Self { inner, cached_pid })
  }
}
