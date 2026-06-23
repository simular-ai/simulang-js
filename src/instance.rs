use napi::Error;
use napi_derive::napi;
use simulang_rs::Instance as SimulangInstance;
use simulang_rs::traits::{AXNodeSynthetic, InstanceTrait};

use crate::accessibility_node::AccessibilityNode;
use crate::ax_tree::TraversalOrder;
use crate::window::Window;

#[napi]
/// Represents an opened application instance.
pub struct Instance {
  pub(crate) inner: SimulangInstance,
}

#[napi]
impl Instance {
  #[napi(getter)]
  #[must_use]
  /// Get the process ID of the opened application (returns 0 when unknown).
  pub fn pid(&self) -> u32 {
    #[allow(clippy::cast_sign_loss)]
    self.inner.pid().map_or(0, |p| p as u32)
  }

  #[napi]
  /// Minimizes the instance.
  pub fn hide(&self) -> napi::Result<bool> {
    self.inner.hide().map_err(Error::from_reason)
  }

  #[napi]
  /// Shows the instance.
  pub fn show(&self) -> napi::Result<bool> {
    self.inner.show().map_err(Error::from_reason)
  }

  #[napi]
  /// Returns true if the instance has the focus.
  pub fn is_focused(&self) -> napi::Result<bool> {
    self.inner.is_focused().map_err(Error::from_reason)
  }

  #[napi]
  /// Brings the instance to the foreground and gives it focus.
  pub fn focus(&self) -> napi::Result<bool> {
    self.inner.focus().map_err(Error::from_reason)
  }

  #[napi]
  #[must_use]
  /// Returns all visible top-level windows belonging to this instance.
  pub fn windows(&self) -> Vec<Window> {
    self
      .inner
      .windows()
      .into_iter()
      .filter_map(|w| Window::try_from(w).ok())
      .collect()
  }

  #[napi]
  #[doc(alias = "page_content")]
  #[doc(alias = "application_content")]
  pub fn content(&self) -> napi::Result<String> {
    self.inner.content().map_err(Error::from_reason)
  }

  #[napi]
  /// Returns true if the instance has an accessibility tree.
  pub fn is_accessible(&self) -> napi::Result<bool> {
    self.inner.is_accessible().map_err(Error::from_reason)
  }

  #[napi]
  /// Enables the accessibility tree for the instance.
  ///
  /// This also works when the application is already running.
  pub fn enable_accessibility(&mut self) -> napi::Result<()> {
    self
      .inner
      .enable_accessibility()
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Disable the accessibility tree for the instance.
  ///
  /// Creating the accessibility tree is resource-intensive, so many
  /// applications disable it by default. After we are done controlling the
  /// instance, we should disable the accessibility tree to save resources.
  pub fn disable_accessibility(&mut self) -> napi::Result<()> {
    self
      .inner
      .disable_accessibility()
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Request the application to exit gracefully. Returns when the request has
  /// been dispatched, not when the process has actually terminated; poll
  /// [`Self::is_running`] if you need to wait.
  pub fn close(&self) -> napi::Result<()> {
    self.inner.close().map_err(Error::from_reason)
  }

  #[napi]
  /// Force-terminate immediately. The application gets no chance to save state
  /// or run cleanup handlers.
  pub fn kill(&self) -> napi::Result<()> {
    self.inner.kill().map_err(Error::from_reason)
  }

  #[napi]
  /// `true` if the underlying process is still running. Cheap, non-blocking.
  pub fn is_running(&self) -> napi::Result<bool> {
    self.inner.is_running().map_err(Error::from_reason)
  }

  #[napi]
  /// Search this application's accessibility tree by *concept text*, using
  /// bag-of-words paired-Jaccard scoring against each node's
  /// `overallDescription`
  /// (`simulang_rs::AXNodeSynthetic::summary_with_context`).
  ///
  /// Mirrors `simulang_rs::Instance::scored_search` with `BowJaccard` as the
  /// scorer and a permissive filter; returns every node whose score equals
  /// the maximum found and exceeds `threshold`.
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

#[napi]
/// Enables the accessibility tree for the frontmost application.
pub fn enable_accessibility_for_frontmost_app() -> napi::Result<()> {
  simulang_rs::enable_accessibility_for_frontmost_app().map_err(Error::from_reason)
}
