use napi::Error;
use napi_derive::napi;
use simulang_rs::Node;
use simulang_rs::traits::{
  AXNodeActions, AXNodeAncestry, AXNodeSynthetic, AXNodeTrait, BoundingBoxTrait,
};

use crate::aria_role::AriaRole;
use crate::ax_tree::{BoundingBox, TraversalOrder};

/// A node in a machine's accessibility tree. Thin binding for
/// `simulang_rs::Node` (macOS `AXUIElement` / Windows UIA element /
/// Linux AT-SPI accessible / Android uiautomator snapshot node).
///
/// Obtain via [`Machine.focusedRoot`], [`Machine.systemRoot`],
/// [`Machine.nodeAtPoint`], [`Instance.root`], [`Window.node`],
/// or via tree-walking methods on another node / `Instance` / `Window`
/// (`children`, `find`, `scoredSearch`). Desktop nodes are live handles
/// whose properties re-resolve from the platform accessibility framework
/// on each access; Android nodes are part of a parsed snapshot whose
/// actions resolve back to the live screen.
#[napi]
pub struct AccessibilityNode {
  pub(crate) inner: Node,
}

impl AccessibilityNode {
  pub(crate) const fn new(inner: Node) -> Self {
    Self { inner }
  }

  pub(crate) fn from_nodes(nodes: Vec<Node>) -> Vec<Self> {
    nodes.into_iter().map(Self::new).collect()
  }
}

#[napi]
impl AccessibilityNode {
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
  /// Platform class name (Windows UIA `ClassName` / macOS subrole /
  /// Android widget class).
  pub fn class_name(&self) -> String {
    self.inner.class_name()
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
  /// Stable element identifier that the inspected application's own source
  /// code assigned to this element (e.g. for its UI tests), or the empty
  /// string when the application didn't set one — most elements don't have
  /// it.
  ///
  /// Each platform reads its native concept: UIA `AutomationId` on Windows,
  /// `AXIdentifier` on macOS, AT-SPI `Accessible.AccessibleId` on Linux,
  /// and the view `resource-id` on Android.
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
  /// Whether the platform reports the node as visible, or `null` when the
  /// visibility attribute cannot be read.
  ///
  /// On Windows this maps to UIA `!IsOffscreen`; on Linux to the AT-SPI
  /// `Visible` + `Showing` states. On macOS it reads the non-standard
  /// `AXVisible` attribute — a Chromium-specific extension that native
  /// apps don't expose (→ `null`). Note Chromium's accessibility-tree
  /// visibility is compositor-driven, not pixel-driven: many
  /// web-content nodes (including links) report `AXVisible = false`
  /// even when rendered, so filtering a browser window on
  /// `isVisible === true` can silently drop most of the page.
  pub fn is_visible(&self) -> Option<bool> {
    self.inner.is_visible()
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
  /// coordinate space (OS-native units; see [`Machine`]).
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
  /// Direct child nodes. Throws when the subtree has been torn down or
  /// the children attribute is unreadable. (Tree walks — `snapshot`,
  /// `scoredSearch`, `childrenCollapsed` — treat such failures as "no
  /// children" instead, so one torn-down subtree cannot abort a whole
  /// walk.)
  pub fn children(&self) -> napi::Result<Vec<AccessibilityNode>> {
    self
      .inner
      .children()
      .map(Self::from_nodes)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Direct children with collapsible structural wrappers hoisted out:
  /// a child that is a collapsible structural wrapper is skipped and its own
  /// (recursively collapsed) children take its place, preserving sibling
  /// order. This is the single-level counterpart of the structural hoisting
  /// applied by tree walks, for callers that drive their own walks.
  ///
  /// An error from this node's `children()` is propagated; errors from a
  /// collapsible wrapper's own `children()` are treated as "no children"
  /// (the walkers' convention), so one torn-down wrapper cannot abort the
  /// whole read.
  pub fn children_collapsed(&self) -> napi::Result<Vec<AccessibilityNode>> {
    self
      .inner
      .children_collapsed()
      .map(Self::from_nodes)
      .map_err(Error::from_reason)
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
  /// Open the element's context menu — the semantic equivalent of a
  /// right-click, without synthesizing pointer input, so it works on
  /// background / obscured windows (Windows `ShowContextMenu`, macOS
  /// `AXShowMenu`, Linux AT-SPI show-menu, Android long-press).
  ///
  /// The opened menu itself typically appears as the topmost / focused
  /// window even when the target window stays in the background — a user
  /// watching the desktop sees a menu pop up without having done
  /// anything.
  ///
  /// Throws when the element does not support opening a menu this way;
  /// callers can fall back to a coordinate right-click at the element's
  /// `boundingBox()` center.
  pub fn show_menu(&self) -> napi::Result<()> {
    self.inner.show_menu().map_err(Error::from_reason)
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
