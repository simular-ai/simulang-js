use napi::Error;
use napi_derive::napi;
use simulang_rs::Window as SimulangWindow;
use simulang_rs::traits::{AXNodeSynthetic, AXNodeTrait, WindowTrait};

use crate::accessibility_node::AccessibilityNode;
use crate::ax_tree::TraversalOrder;

/// Handle to an on-screen window. Constructed via [`Window.all`] or
/// [`Window.allForPid`] and exposes read-only metadata (`title`, `pid`)
/// alongside basic actions (`minimize`, `maximize`, `close`).
#[napi]
pub struct Window {
  inner: SimulangWindow,
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
