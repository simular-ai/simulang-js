use napi::Error;
use napi_derive::napi;
use simulang_rs::AXNode;
use simulang_rs::traits::{
  AXNodeActions, AXNodeAncestry, AXNodeSynthetic, AXNodeTrait, BoundingBoxTrait,
};

use crate::aria_role::AriaRole;
use crate::ax_tree::{BoundingBox, TraversalOrder};

/// A node in the platform accessibility tree. Thin binding for
/// `simulang_rs::AXNode` (macOS `AXUIElement` / Windows UIA element /
/// Linux AT-SPI accessible).
///
/// Construct via the static factories or via tree-walking methods on
/// another node / `Instance` / `Window` (`children`, `find`,
/// `scoredSearch`). Properties are resolved from the underlying
/// accessibility framework on each access; the node itself is just a
/// handle.
#[napi]
pub struct AccessibilityNode {
  pub(crate) inner: AXNode,
}

impl AccessibilityNode {
  pub(crate) const fn new(inner: AXNode) -> Self {
    Self { inner }
  }

  pub(crate) fn from_nodes(nodes: Vec<AXNode>) -> Vec<Self> {
    nodes.into_iter().map(Self::new).collect()
  }
}

#[napi]
impl AccessibilityNode {
  // ---------------------------------------------------------------
  // factories
  // ---------------------------------------------------------------

  #[napi(factory)]
  /// Root node of the currently focused application's accessibility tree.
  pub fn from_focused_application() -> napi::Result<Self> {
    AXNode::from_focused_application()
      .map(Self::new)
      .map_err(Error::from_reason)
  }

  #[napi(factory)]
  /// Root node of the application identified by `pid`.
  #[allow(clippy::cast_possible_wrap)]
  pub fn from_pid(pid: u32) -> napi::Result<Self> {
    AXNode::from_pid(pid as i32)
      .map(Self::new)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Element at screen coordinates (`x`, `y`) via the platform hit-test
  /// (Windows UIA `ElementFromPoint`, macOS
  /// `AXUIElementCopyElementAtPosition`, Linux recursive AT-SPI
  /// `GetAccessibleAtPoint`).
  ///
  /// `(x, y)` are global desktop coordinates in the canonical coordinate space
  /// (OS-native units; see [`MouseController`])
  /// — the same space `.boundingBox()` and `MouseController` use, so a
  /// `boundingBox()` corner or cursor `location()` can be passed straight in.
  /// Returns an **uncached** handle suitable for one-shot reads of properties
  /// such as `.boundingBox()` or `.overallDescription`; it is not registered
  /// for ref-based action methods (`activate`, `setValue`, …).
  ///
  /// Returns `null` when the point has no accessible element (empty
  /// desktop, gaps between controls, or — on Linux — a point outside the
  /// focused application). Throws only on a genuine accessibility-backend
  /// failure.
  pub fn from_point(x: i32, y: i32) -> napi::Result<Option<AccessibilityNode>> {
    Ok(
      AXNode::from_point(x, y)
        .map_err(Error::from_reason)?
        .map(Self::new),
    )
  }

  // ---------------------------------------------------------------
  // properties
  // ---------------------------------------------------------------

  #[napi(getter)]
  #[must_use]
  /// Cross-platform ARIA role.
  pub fn role(&self) -> AriaRole {
    self.inner.aria_role().into()
  }

  #[napi(getter)]
  #[must_use]
  /// Accessible name (title / label).
  pub fn name(&self) -> String {
    self.inner.title()
  }

  #[napi(getter)]
  #[must_use]
  /// Platform class name (Windows UIA `ClassName` / macOS subrole).
  pub fn class_name(&self) -> String {
    self.inner.class_name()
  }

  #[napi(getter)]
  #[must_use]
  /// Numeric control type (`UIA_ControlTypeIds` on Windows, `0` on macOS).
  pub fn control_type(&self) -> i32 {
    self.inner.control_type_id()
  }

  #[napi(getter)]
  #[must_use]
  /// Localized control type string.
  pub fn localized_control_type(&self) -> String {
    self.inner.localized_control_type()
  }

  #[napi(getter)]
  #[must_use]
  /// Short description (`AXDescription` / UIA `Name`-adjacent fields).
  pub fn description(&self) -> String {
    self.inner.description_text()
  }

  #[napi(getter)]
  #[must_use]
  /// Synthetic, query-friendly description used by `scoredSearch` (combines
  /// the node's role, label, value, and a small amount of ancestor context).
  pub fn overall_description(&self) -> String {
    self.inner.summary_with_context()
  }

  #[napi(getter)]
  #[must_use]
  /// Help text / tooltip.
  pub fn help_text(&self) -> String {
    self.inner.help_text()
  }

  #[napi(getter)]
  #[must_use]
  /// Current text value (textbox content, slider value as string, …).
  pub fn value(&self) -> String {
    self.inner.value()
  }

  #[napi(getter)]
  #[must_use]
  /// UIA `AutomationId` (Windows). Empty on macOS / Linux.
  pub fn automation_id(&self) -> String {
    self.inner.automation_id()
  }

  #[napi(getter)]
  #[must_use]
  /// Whether the node accepts user input.
  pub fn is_enabled(&self) -> bool {
    self.inner.is_enabled()
  }

  #[napi(getter)]
  #[must_use]
  /// Hyperlink target of this node, or `null` when the node is not a link,
  /// the link has no target, or the platform backend does not expose the
  /// target through the accessibility API. The string is the raw URL as
  /// reported by the platform (no parsing or normalisation).
  pub fn url(&self) -> Option<String> {
    self.inner.url()
  }

  #[napi]
  /// Live bounding box of the element on the global desktop, in the canonical
  /// coordinate space (OS-native units; see
  /// [`MouseController`]).
  /// `right` and `bottom` are exclusive (Playwright / DOM convention).
  pub fn bounding_box(&self) -> napi::Result<BoundingBox> {
    self
      .inner
      .bounding_box()
      .map(BoundingBox::from)
      .map_err(Error::from_reason)
  }

  // ---------------------------------------------------------------
  // tree navigation
  // ---------------------------------------------------------------

  #[napi]
  #[must_use]
  /// Direct child nodes. Returns an empty array if the subtree has been
  /// torn down or the children attribute is unreadable (matching
  /// simulang-rs's convention of treating walk failures as "no children").
  pub fn children(&self) -> Vec<AccessibilityNode> {
    Self::from_nodes(self.inner.children().unwrap_or_default())
  }

  #[napi]
  /// Direct parent node, or `null` if this node has no parent. Parent lookup
  /// failures throw.
  pub fn parent(&self) -> napi::Result<Option<AccessibilityNode>> {
    self
      .inner
      .parent()
      .map(|parent| parent.map(Self::new))
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Parent chain for this node, nearest parent first, excluding this node.
  /// Stops when a node has no parent; parent lookup failures throw.
  pub fn ancestors(&self) -> napi::Result<Vec<AccessibilityNode>> {
    self
      .inner
      .ancestors()
      .collect::<Result<Vec<_>, _>>()
      .map(Self::from_nodes)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Whether this node is the direct parent of `other`.
  pub fn is_parent_of(&self, other: &AccessibilityNode) -> napi::Result<bool> {
    self
      .inner
      .is_parent_of(&other.inner)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Whether this node is a direct child of `other`.
  pub fn is_child_of(&self, other: &AccessibilityNode) -> napi::Result<bool> {
    self
      .inner
      .is_child_of(&other.inner)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Whether this node is a strict ancestor of `other`.
  pub fn is_ancestor_of(&self, other: &AccessibilityNode) -> napi::Result<bool> {
    self
      .inner
      .is_ancestor_of(&other.inner)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Whether this node is a strict descendant of `other`.
  pub fn is_descendant_of(&self, other: &AccessibilityNode) -> napi::Result<bool> {
    self
      .inner
      .is_descendant_of(&other.inner)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Lowest shared ancestor for this node and `other`, plus that ancestor's
  /// level from the reached parentless node (`parentless = 0`, its children
  /// `= 1`). Returns `null` when both chains resolve and no shared ancestor
  /// exists; lookup failures throw.
  #[allow(clippy::cast_possible_truncation)]
  pub fn lowest_common_ancestor(
    &self,
    other: &AccessibilityNode,
  ) -> napi::Result<Option<(AccessibilityNode, u32)>> {
    self
      .inner
      .lowest_common_ancestor(&other.inner)
      .map(|ancestor| ancestor.map(|(node, level)| (AccessibilityNode::new(node), level as u32)))
      .map_err(Error::from_reason)
  }

  // ---------------------------------------------------------------
  // snapshot / introspection
  // ---------------------------------------------------------------

  #[napi]
  #[must_use]
  /// Render this node's accessibility subtree as an indented
  /// Playwright-style aria snapshot string.
  ///
  /// One line per node, two spaces of indentation per depth level, in
  /// pre-order DFS. Roles are emitted raw (`AXWindow` on macOS,
  /// `UIA.ControlType.*` on Windows), with title and value appended when
  /// non-empty.
  pub fn snapshot(&self) -> String {
    self.inner.snapshot()
  }

  #[napi]
  #[must_use]
  /// Human-readable names of the actions this node currently supports
  /// (e.g. `"activate"`, `"toggle"`, `"scroll_into_view"`).
  pub fn supported_actions(&self) -> Vec<String> {
    self.inner.supported_action_names()
  }

  // ---------------------------------------------------------------
  // search
  // ---------------------------------------------------------------

  #[napi]
  /// Search this subtree by *concept text*, using bag-of-words
  /// paired-Jaccard scoring against each node's `overallDescription`
  /// (`simulang_rs::AXNodeSynthetic::summary_with_context`).
  ///
  /// Returns every node whose score equals the maximum found and exceeds
  /// `threshold` — same semantics as `simulang_rs::scored_search` with
  /// `BowJaccard::score(...).primary` as the scorer and a permissive
  /// filter.
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
  #[must_use]
  #[allow(clippy::needless_pass_by_value)]
  pub fn scored_search(
    &self,
    order: TraversalOrder,
    max_nodes: u32,
    collapse_structural: bool,
    query: String,
    threshold: f64,
  ) -> Vec<AccessibilityNode> {
    let matches = self.inner.scored_search(
      order.into(),
      max_nodes as usize,
      collapse_structural,
      |_| true,
      |node| simulang_rs::BowJaccard::score(&node.summary_with_context(), &query).primary,
      threshold,
    );
    Self::from_nodes(matches)
  }

  // ---------------------------------------------------------------
  // actions (proxy to `AXNodeActions`)
  // ---------------------------------------------------------------

  #[napi]
  /// Invoke / click the element (button, link, menu item).
  pub fn activate(&self) -> napi::Result<()> {
    self.inner.activate().map_err(Error::from_reason)
  }

  #[napi]
  /// Set the text value of the element (textbox, combobox, …).
  #[allow(clippy::needless_pass_by_value)]
  pub fn set_value(&self, value: String) -> napi::Result<()> {
    self.inner.set_value(&value).map_err(Error::from_reason)
  }

  #[napi]
  /// Toggle a checkbox or switch.
  pub fn toggle(&self) -> napi::Result<()> {
    self.inner.toggle().map_err(Error::from_reason)
  }

  #[napi]
  /// Select a tab, radio button, or list item.
  pub fn select(&self) -> napi::Result<()> {
    self.inner.select().map_err(Error::from_reason)
  }

  #[napi]
  /// Expand or collapse a dropdown or tree item.
  pub fn expand_collapse(&self) -> napi::Result<()> {
    self.inner.expand_collapse().map_err(Error::from_reason)
  }

  #[napi]
  /// Scroll the element into view.
  pub fn scroll_into_view(&self) -> napi::Result<()> {
    self.inner.scroll_into_view().map_err(Error::from_reason)
  }

  #[napi]
  /// Move keyboard focus to this element (brings its window to the
  /// foreground as a side effect on most platforms).
  pub fn focus(&self) -> napi::Result<()> {
    self.inner.set_focus().map_err(Error::from_reason)
  }
}
