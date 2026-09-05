use napi::Error;
use napi_derive::napi;
use simulang_rs::TreeIter;
use simulang_rs::ax_attribute::attr;
use simulang_rs::traits::{
  AXNodeActions, AXNodeAncestry, AXNodeSynthetic, AXNodeTrait, BoundingBoxTrait,
};
use simulang_rs::{Instance as SimulangInstance, Node as SimulangNode, Window as SimulangWindow};

use crate::aria_role::AriaRole;
use crate::instance::Instance;
use crate::window::Window;

#[napi]
/// Tree traversal order for `AccessibilityTree.find()`.
#[derive(Clone, Copy)]
pub enum TraversalOrder {
  /// Pre-order depth-first (each node before its descendants).
  DepthFirst,
  /// Level-order breadth-first (parents before children).
  BreadthFirst,
}

impl From<TraversalOrder> for simulang_rs::TraversalOrder {
  fn from(o: TraversalOrder) -> Self {
    match o {
      TraversalOrder::DepthFirst => simulang_rs::TraversalOrder::DepthFirst,
      TraversalOrder::BreadthFirst => simulang_rs::TraversalOrder::BreadthFirst,
    }
  }
}

/// A snapshot node: plain data plus a [`BoundingBox`] when the platform
/// reported one (`null` otherwise).
#[napi]
#[derive(Clone)]
pub struct AccessibilityNodeJs {
  role: AriaRole,
  name: String,
  class_name: String,
  localized_control_type: String,
  description: String,
  overall_description: String,
  help_text: String,
  value: String,
  automation_id: String,
  is_enabled: bool,
  bounding_box: Option<BoundingBox>,
  children: Vec<AccessibilityNodeJs>,
  ref_id: Option<u32>,
}

#[napi]
impl AccessibilityNodeJs {
  #[napi(getter)]
  #[must_use]
  pub fn role(&self) -> AriaRole {
    self.role
  }

  #[napi(getter)]
  #[must_use]
  pub fn name(&self) -> String {
    self.name.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn class_name(&self) -> String {
    self.class_name.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn localized_control_type(&self) -> String {
    self.localized_control_type.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn description(&self) -> String {
    self.description.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn overall_description(&self) -> String {
    self.overall_description.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn help_text(&self) -> String {
    self.help_text.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn value(&self) -> String {
    self.value.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn automation_id(&self) -> String {
    self.automation_id.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn is_enabled(&self) -> bool {
    self.is_enabled
  }

  /// Live box when the platform reported one; `null` if it didn't.
  #[napi(getter)]
  #[must_use]
  pub fn bounding_box(&self) -> Option<BoundingBox> {
    self.bounding_box.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn children(&self) -> Vec<AccessibilityNodeJs> {
    self.children.clone()
  }

  #[napi(getter)]
  #[must_use]
  pub fn ref_id(&self) -> Option<u32> {
    self.ref_id
  }
}

/// Axis-aligned rectangle. `right` and `bottom` are exclusive (Playwright /
/// DOM convention), so the box covers `[left, right) × [top, bottom)`.
///
/// Spatial predicates (`isBelow`, `overlapsX`, …) match the Rust methods
/// and are what you compose in `searchRelative`.
#[napi]
#[derive(Clone)]
pub struct BoundingBox {
  pub(crate) inner: simulang_rs::BoundingBox,
}

impl From<simulang_rs::BoundingBox> for BoundingBox {
  fn from(inner: simulang_rs::BoundingBox) -> Self {
    Self { inner }
  }
}

#[napi]
impl BoundingBox {
  /// Construct a box from exclusive-edge coordinates. Throws if the box is
  /// degenerate (`right <= left` or `bottom <= top`).
  #[napi(constructor)]
  pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> napi::Result<Self> {
    simulang_rs::BoundingBox::new(left, top, right, bottom)
      .map(Self::from)
      .map_err(Error::from_reason)
  }

  /// Construct from origin + size. Throws if `width` / `height` are not
  /// positive or if `x + width` / `y + height` overflow `i32`.
  #[napi(factory)]
  pub fn from_xywh(x: i32, y: i32, width: i32, height: i32) -> napi::Result<Self> {
    simulang_rs::BoundingBox::from_xywh(x, y, width, height)
      .map(Self::from)
      .map_err(Error::from_reason)
  }

  #[napi(getter)]
  #[must_use]
  pub fn left(&self) -> i32 {
    self.inner.left()
  }

  #[napi(getter)]
  #[must_use]
  pub fn top(&self) -> i32 {
    self.inner.top()
  }

  #[napi(getter)]
  #[must_use]
  pub fn right(&self) -> i32 {
    self.inner.right()
  }

  #[napi(getter)]
  #[must_use]
  pub fn bottom(&self) -> i32 {
    self.inner.bottom()
  }

  /// Width (`right - left`). Always positive by construction.
  #[napi(getter)]
  #[must_use]
  pub fn width(&self) -> u32 {
    self.inner.width()
  }

  /// Height (`bottom - top`). Always positive by construction.
  #[napi(getter)]
  #[must_use]
  pub fn height(&self) -> u32 {
    self.inner.height()
  }

  /// Geometric center as `[x, y]` (integer-truncated, the pixel midpoint).
  #[napi]
  #[must_use]
  pub fn center(&self) -> (i32, i32) {
    self.inner.center()
  }

  /// Area in pixels.
  #[napi]
  #[must_use]
  #[allow(clippy::cast_precision_loss)]
  pub fn area(&self) -> f64 {
    self.inner.area() as f64
  }

  /// Overlap area with `other` in pixels, `0` when the boxes do not
  /// intersect.
  #[napi]
  #[must_use]
  #[allow(clippy::cast_precision_loss)]
  pub fn overlap_area(&self, other: &BoundingBox) -> f64 {
    self.inner.overlap_area(&other.inner) as f64
  }

  /// Playwright / DOM shape: `{x: …, y: …, width: …, height: …}`.
  #[napi(js_name = "toString")]
  #[must_use]
  pub fn as_js_string(&self) -> String {
    self.inner.to_string()
  }

  /// `true` if both boxes have the same exclusive-edge corners. `===` is
  /// still object identity — two separately constructed boxes with the
  /// same corners need this method.
  #[napi]
  #[must_use]
  pub fn equals(&self, other: &BoundingBox) -> bool {
    self.inner == other.inner
  }

  /// `true` if this box sits entirely above `other` (no vertical overlap;
  /// exclusive edges that touch still count).
  #[napi]
  #[must_use]
  pub fn is_above(&self, other: &BoundingBox) -> bool {
    self.inner.is_above(&other.inner)
  }

  /// `true` if this box sits entirely below `other`. See [`isAbove`].
  #[napi]
  #[must_use]
  pub fn is_below(&self, other: &BoundingBox) -> bool {
    self.inner.is_below(&other.inner)
  }

  /// `true` if this box sits entirely to the left of `other`. See [`isAbove`].
  #[napi]
  #[must_use]
  pub fn is_left_of(&self, other: &BoundingBox) -> bool {
    self.inner.is_left_of(&other.inner)
  }

  /// `true` if this box sits entirely to the right of `other`. See [`isAbove`].
  #[napi]
  #[must_use]
  pub fn is_right_of(&self, other: &BoundingBox) -> bool {
    self.inner.is_right_of(&other.inner)
  }

  /// `true` if the boxes share a row: centers within `tolerance` on `y`.
  /// `tolerance` is in OS-native units (physical pixels on Windows/Linux,
  /// logical points on macOS).
  #[napi]
  #[must_use]
  pub fn same_row(&self, other: &BoundingBox, tolerance: i32) -> bool {
    self.inner.same_row(&other.inner, tolerance)
  }

  /// `true` if the boxes share a column: centers within `tolerance` on `x`.
  #[napi]
  #[must_use]
  pub fn same_column(&self, other: &BoundingBox, tolerance: i32) -> bool {
    self.inner.same_column(&other.inner, tolerance)
  }

  /// `true` if this box fully encloses `other` (equal boxes count).
  #[napi]
  #[must_use]
  pub fn contains_box(&self, other: &BoundingBox) -> bool {
    self.inner.contains_box(&other.inner)
  }

  /// `true` if `other` fully encloses this box. Inverse of [`containsBox`].
  #[napi]
  #[must_use]
  pub fn is_contained_in(&self, other: &BoundingBox) -> bool {
    self.inner.is_contained_in(&other.inner)
  }

  /// `true` if the point `(x, y)` lies inside this box. Exclusive `right` /
  /// `bottom` edges are outside.
  #[napi]
  #[must_use]
  pub fn contains_point(&self, x: i32, y: i32) -> bool {
    self.inner.contains_point(x, y)
  }

  /// `true` if the boxes' interiors overlap. Exclusive edges that merely
  /// touch do not count.
  #[napi]
  #[must_use]
  pub fn intersects(&self, other: &BoundingBox) -> bool {
    self.inner.intersects(&other.inner)
  }

  /// `true` if the x-projections overlap (exclusive edges that touch do not).
  /// A narrow button over a wide field typically matches even when centers
  /// do not — compose with [`isAbove`] for "above in this column".
  #[napi]
  #[must_use]
  pub fn overlaps_x(&self, other: &BoundingBox) -> bool {
    self.inner.overlaps_x(&other.inner)
  }

  /// `true` if the y-projections overlap. See [`overlapsX`].
  #[napi]
  #[must_use]
  pub fn overlaps_y(&self, other: &BoundingBox) -> bool {
    self.inner.overlaps_y(&other.inner)
  }

  /// Shortest edge-to-edge distance to `other`, `0` when they touch or
  /// overlap. Rank matches with `Math.min` / `Math.max` on this value.
  #[napi]
  #[must_use]
  pub fn shortest_distance_to(&self, other: &BoundingBox) -> f64 {
    self.inner.shortest_distance_to(&other.inner)
  }
}

fn snapshot_node(
  node: &SimulangNode,
  refs: &mut Vec<SimulangNode>,
  visible_only: bool,
) -> Option<AccessibilityNodeJs> {
  if visible_only && node.get_attribute_boolean(attr::VISIBLE).ok() == Some(false) {
    return None;
  }
  let ref_id = u32::try_from(refs.len()).ok()?;
  refs.push(node.clone());
  let children = collect_collapsed_children(node, refs, visible_only);
  let bounding_box = node.bounding_box().ok().map(BoundingBox::from);
  Some(AccessibilityNodeJs {
    role: node.aria_role().into(),
    name: node.title(),
    class_name: node.class_name(),
    localized_control_type: node.localized_control_type(),
    description: node.description_text(),
    overall_description: node.summary_with_context(),
    help_text: node.help_text(),
    value: node.value(),
    automation_id: node.automation_id(),
    is_enabled: node.is_enabled(),
    bounding_box,
    children,
    ref_id: Some(ref_id),
  })
}

/// Materialise `node`'s children, **hoisting** any child that
/// [`AXNodeSynthetic::is_collapsible`] reports as `true`: the child itself
/// is skipped and its grandchildren take its place at the same depth.
/// Mirrors the trait-level [`CollapsingDepthFirstIter`] used by
/// `AXNodeTrait::snapshot()`.
///
/// `AXNodeAncestry::children` returns `Result<Vec<Self>, String>`; per the
/// trait docs (and the convention used by simulang-rs's own search
/// walkers) we treat failure as "no children" so a single torn-down
/// subtree cannot abort the whole snapshot.
fn collect_collapsed_children(
  node: &SimulangNode,
  refs: &mut Vec<SimulangNode>,
  visible_only: bool,
) -> Vec<AccessibilityNodeJs> {
  let mut out = Vec::new();
  for child in node.children().unwrap_or_default() {
    if visible_only && child.get_attribute_boolean(attr::VISIBLE).ok() == Some(false) {
      continue;
    }
    if child.is_collapsible() {
      out.extend(collect_collapsed_children(&child, refs, visible_only));
    } else if let Some(materialised) = snapshot_node(&child, refs, visible_only) {
      out.push(materialised);
    }
  }
  out
}

/// Root scoping for an [`AccessibilityTree`], re-resolved on every
/// snapshot so each call sees current data.
///
/// - `Instance` keeps the historical macOS application-wide scope: the
///   root is the application element (every window plus the app menu
///   bar), re-resolved through [`SimulangInstance::root`].
/// - `Window` scopes to a single window subtree. The unified window handle
///   stores a persistent identity (`HWND` / `AXUIElement` / task id), so
///   [`SimulangWindow::node`] re-resolves a fresh root per snapshot — on
///   Windows via a cached subtree build (single `BuildUpdatedCache` IPC,
///   making the subsequent walk ~40× faster than a live root).
enum Root {
  Instance(SimulangInstance),
  Window(SimulangWindow),
}

#[napi]
/// Accessibility tree bound to a window (or, on macOS, an application).
/// Provides snapshot, search, and ref-based actions.
pub struct AccessibilityTree {
  root: Root,
  refs: Vec<SimulangNode>,
}

impl AccessibilityTree {
  fn from_root(root: Root) -> Self {
    Self {
      root,
      refs: Vec::new(),
    }
  }

  /// Re-resolve the tree's root node from its stored scope (see [`Root`]).
  fn resolve_root(&self) -> napi::Result<SimulangNode> {
    match &self.root {
      Root::Instance(instance) => instance.root(),
      Root::Window(window) => window.node(),
    }
    .map_err(Error::from_reason)
  }

  fn get_ref(&self, ref_id: u32) -> napi::Result<&SimulangNode> {
    self
      .refs
      .get(ref_id as usize)
      .ok_or_else(|| Error::from_reason(format!("Unknown ref_id {ref_id}")))
  }

  /// Shared core of `find` / `find_by_description`: clear refs, resolve a
  /// fresh window root, walk it with `TreeIter`, keep nodes matching
  /// `predicate`, cap at `max_results`, and materialize each hit as a
  /// flat `AccessibilityNodeJs` carrying a fresh `refId`.
  ///
  /// `predicate` is a Rust closure chosen by the calling typed method —
  /// we deliberately don't expose a JS callback here, since invoking JS
  /// per node across the napi boundary during a full-tree walk would be
  /// slow.
  ///
  /// KNOWN FOOTGUN: `find` / `find_by_description` read like pure queries
  /// but, via this helper, `clear()` and rebuild the single `refs` table
  /// that `snapshot()` and the action methods (`activate`, `set_value`, …)
  /// also share. A `ref_id` is therefore only valid relative to the *most
  /// recent* traversal call on the tree — a later `find` silently
  /// invalidates `ref_id`s handed out by an earlier `snapshot`/`find`.
  /// Today the app never actions a `find` result's `ref_id` (matches are
  /// flattened to data, and the only ref-based actioning reads from a
  /// `snapshot()` on a tree it never calls `find` on), so this is latent,
  /// not live. The clean fix is to have queries return self-contained
  /// `AccessibilityNode` handles (like `AccessibilityNode.scoredSearch`)
  /// instead of sharing the ref table.
  fn filter_nodes(
    &mut self,
    order: TraversalOrder,
    collapse_structural: bool,
    max_results: Option<u32>,
    predicate: impl Fn(&SimulangNode) -> bool,
  ) -> Vec<AccessibilityNodeJs> {
    self.refs.clear();
    let root = match self.resolve_root() {
      Ok(node) => node,
      Err(e) => {
        log::warn!("AccessibilityTree: failed to resolve window root: {e}");
        return Vec::new();
      }
    };
    TreeIter::new(root, order.into(), collapse_structural)
      .map(|(n, _)| n)
      .filter(|node| predicate(node))
      .take(max_results.map_or(usize::MAX, |n| n as usize))
      .map(|node| {
        let ref_id = u32::try_from(self.refs.len()).unwrap_or(u32::MAX);
        self.refs.push(node.clone());
        node_to_js(&node, ref_id)
      })
      .collect()
  }
}

#[napi]
impl AccessibilityTree {
  #[napi(factory)]
  /// Create an accessibility tree bound to a running application instance
  /// (from [`Machine.foregroundApp`], [`App.open`], …). The instance
  /// carries its machine, so the tree targets whatever machine — local
  /// desktop or Android — the instance came from.
  ///
  /// The target is resolved **once** at construction time — subsequent
  /// snapshots keep targeting it even if the user switches away.
  ///
  /// Scope differs by platform (historical behavior): for a local
  /// instance on macOS the tree covers the whole application (every
  /// window plus the app menu bar); everywhere else it covers the
  /// instance's first visible top-level window (Android: the app's first
  /// live task). Use [`AccessibilityTree.fromWindow`] for guaranteed
  /// window scoping.
  pub fn from_instance(instance: &Instance) -> napi::Result<Self> {
    // macOS keeps the historical app-wide scope (menu bar included).
    // Everywhere else, freeze the first visible window at construction —
    // on Windows that path hits the cached-subtree fast path, and it
    // matches the old fromPid behavior of targeting one window.
    let root = match &instance.inner {
      SimulangInstance::Local(_) if cfg!(target_os = "macos") => {
        Root::Instance(instance.inner.clone())
      }
      inner => {
        let window = inner.windows().into_iter().next().ok_or_else(|| {
          Error::from_reason("Instance has no visible top-level window".to_owned())
        })?;
        Root::Window(window)
      }
    };
    Ok(Self::from_root(root))
  }

  #[napi(factory)]
  #[must_use]
  /// Create an accessibility tree scoped to a single window.
  ///
  /// Unlike [`AccessibilityTree.fromInstance`] — which on macOS scopes to
  /// the whole application (every window plus the app menu bar) — this
  /// scopes to exactly the given window's subtree on every
  /// platform. Use it to measure "is this element unique within this
  /// window", or to snapshot / act on one window of a multi-window app.
  ///
  /// The window handle stores a persistent identity, so each snapshot
  /// re-resolves fresh data for that same window; the handle can go stale
  /// if the window is destroyed and recreated.
  pub fn from_window(window: &Window) -> Self {
    Self::from_root(Root::Window(window.inner.clone()))
  }

  #[napi(getter)]
  #[must_use]
  /// Get the window title (empty when it cannot be resolved).
  ///
  /// For an application-scoped tree (macOS `fromInstance`) this is the
  /// title of the application's first window.
  pub fn window_title(&self) -> String {
    match &self.root {
      Root::Window(window) => window.title(),
      Root::Instance(instance) => instance
        .windows()
        .into_iter()
        .next()
        .map_or_else(String::new, |window| window.title()),
    }
  }

  #[napi]
  /// Take a snapshot of the tree.
  ///
  /// Re-resolves the root so each call sees current data. On Windows the
  /// resolve issues a single `BuildUpdatedCache` IPC; the recursive walk
  /// over children and properties then stays entirely in-process, ~40×
  /// faster than walking a live root.
  ///
  /// `visible_only` (default `false`) controls whether nodes whose
  /// non-standard `AXVisible` attribute reads `false` are dropped from
  /// the result.
  ///
  /// `AXVisible` is a Chromium-specific extension; native macOS apps
  /// don't expose it, so the filter only matters for browser windows.
  /// Chrome reports many web-content nodes (including `AXLink`) as
  /// `AXVisible=false` because its accessibility-tree visibility is
  /// compositor-driven, not pixel-driven — setting `visible_only=true`
  /// on a Chrome window will silently drop most of the page including
  /// links.
  pub fn snapshot(&mut self, visible_only: Option<bool>) -> napi::Result<AccessibilityNodeJs> {
    self.refs.clear();
    let root = self.resolve_root()?;
    snapshot_node(&root, &mut self.refs, visible_only.unwrap_or(false))
      .ok_or_else(|| Error::from_reason("Root element is not visible".to_owned()))
  }

  #[napi]
  /// Invoke/click an element (button, link, menuitem).
  pub fn activate(&self, ref_id: u32) -> napi::Result<()> {
    self.get_ref(ref_id)?.activate().map_err(Error::from_reason)
  }

  #[napi]
  /// Set the text value of an element (textbox, combobox).
  #[allow(clippy::needless_pass_by_value)]
  pub fn set_value(&self, ref_id: u32, value: String) -> napi::Result<()> {
    self
      .get_ref(ref_id)?
      .set_value(&value)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Toggle a checkbox or switch.
  pub fn toggle(&self, ref_id: u32) -> napi::Result<()> {
    self.get_ref(ref_id)?.toggle().map_err(Error::from_reason)
  }

  #[napi]
  /// Select a tab, radio button, or list item.
  pub fn select(&self, ref_id: u32) -> napi::Result<()> {
    self.get_ref(ref_id)?.select().map_err(Error::from_reason)
  }

  #[napi]
  /// Expand or collapse a dropdown or tree item.
  pub fn expand_collapse(&self, ref_id: u32) -> napi::Result<()> {
    self
      .get_ref(ref_id)?
      .expand_collapse()
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Open an element's context menu — the semantic equivalent of a
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
  /// callers can fall back to a coordinate right-click at the center of
  /// [`AccessibilityTree.getBounds`].
  pub fn show_menu(&self, ref_id: u32) -> napi::Result<()> {
    self
      .get_ref(ref_id)?
      .show_menu()
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Scroll an element into view.
  pub fn scroll_into_view(&self, ref_id: u32) -> napi::Result<()> {
    self
      .get_ref(ref_id)?
      .scroll_into_view()
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Focus an element (brings window to foreground).
  pub fn focus_element(&self, ref_id: u32) -> napi::Result<()> {
    self
      .get_ref(ref_id)?
      .set_focus()
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Get the live bounding box of an element.
  pub fn get_bounds(&self, ref_id: u32) -> napi::Result<BoundingBox> {
    self
      .get_ref(ref_id)?
      .bounding_box()
      .map(BoundingBox::from)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// A screen point where a pointer click actually lands on the element,
  /// as `[x, y]` in the canonical global-desktop space. Verified by
  /// hit-testing; throws (saying why) when the element has no such point,
  /// instead of guessing a center that would click something else. See
  /// [`AccessibilityNode.clickablePoint`] for the per-platform behavior
  /// and the `allowDescendants` semantics.
  pub fn clickable_point(
    &self,
    ref_id: u32,
    allow_descendants: Option<bool>,
  ) -> napi::Result<(i32, i32)> {
    self
      .get_ref(ref_id)?
      .clickable_point(allow_descendants.unwrap_or(false))
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Get the list of supported actions for an element.
  pub fn get_supported_actions(&self, ref_id: u32) -> napi::Result<Vec<String>> {
    Ok(self.get_ref(ref_id)?.supported_action_names())
  }

  #[napi]
  /// Clear all stored element refs.
  pub fn clear_refs(&mut self) {
    self.refs.clear();
  }

  #[napi]
  /// Search the tree using the chosen traversal order.
  ///
  /// - `order` – `TraversalOrder.DepthFirst` or `TraversalOrder.BreadthFirst`
  /// - `role`  – keep nodes whose ARIA role equals this value
  ///             (e.g. `AriaRole.Button`, `AriaRole.TabList`,
  ///             `AriaRole.MenuBar`). Cross-platform ARIA roles, not
  ///             the platform-native `UIA.ControlType.*` / `AX*` /
  ///             `AT-SPI.Role.*` vocabulary.
  /// - `name`  – keep nodes whose title or description contains this
  ///             string
  /// - `visible_only` – skip invisible nodes (default `false`)
  /// - `max_results`  – stop after this many matches (uses `.take()`)
  /// - `collapse_structural` – hoist empty structural wrappers (for example `Pane`
  ///   / `Group` / `Custom` / `Document` on Windows, `AXGroup` /
  ///   `AXGenericGroup` / `AXUnknown` on macOS, AT-SPI `Panel` /
  ///   `Filler` / `Section` on Linux) out of the walk so role searches
  ///   do not hit unnamed containers (default `false`)
  ///
  /// Clears existing refs; returned nodes carry `refId` values usable
  /// with action methods (`activate`, `setValue`, …).
  #[allow(clippy::needless_pass_by_value)]
  pub fn find(
    &mut self,
    order: TraversalOrder,
    role: Option<AriaRole>,
    name: Option<String>,
    visible_only: Option<bool>,
    max_results: Option<u32>,
    collapse_structural: Option<bool>,
  ) -> Vec<AccessibilityNodeJs> {
    let visible_only = visible_only.unwrap_or(false);
    // Convert the JS-facing enum into the simulang-rs enum once, so
    // the per-node filter is a cheap discriminant compare instead of
    // a string comparison.
    let role: Option<simulang_rs::AriaRole> = role.map(Into::into);
    self.filter_nodes(
      order,
      collapse_structural.unwrap_or(false),
      max_results,
      move |node| {
        (!visible_only || node.get_attribute_boolean(attr::VISIBLE).ok() != Some(false))
          && role.is_none_or(|r| node.aria_role() == r)
          && name.as_ref().is_none_or(|n| {
            node.title().contains(n.as_str()) || node.description_text().contains(n.as_str())
          })
      },
    )
  }

  #[napi]
  /// Find every node whose `overallDescription`
  /// (`AXNodeSynthetic::summary_with_context`) is **exactly** equal to
  /// `description`, walked in pre-order depth-first.
  ///
  /// Clears existing refs; returned nodes carry `refId` values usable
  /// with the action methods (`activate`, `setValue`, …).
  #[allow(clippy::needless_pass_by_value)]
  pub fn find_by_description(&mut self, description: String) -> Vec<AccessibilityNodeJs> {
    self.filter_nodes(TraversalOrder::DepthFirst, false, None, move |node| {
      node.summary_with_context() == description
    })
  }
}

/// Convert a single node into a flat `AccessibilityNodeJs` (no children).
fn node_to_js(node: &SimulangNode, ref_id: u32) -> AccessibilityNodeJs {
  let bounding_box = node.bounding_box().ok().map(BoundingBox::from);
  AccessibilityNodeJs {
    role: node.aria_role().into(),
    name: node.title(),
    class_name: node.class_name(),
    localized_control_type: node.localized_control_type(),
    description: node.description_text(),
    overall_description: node.summary_with_context(),
    help_text: node.help_text(),
    value: node.value(),
    automation_id: node.automation_id(),
    is_enabled: node.is_enabled(),
    bounding_box,
    children: Vec::new(),
    ref_id: Some(ref_id),
  }
}
