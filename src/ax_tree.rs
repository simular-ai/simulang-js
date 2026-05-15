use napi::Error;
use napi_derive::napi;
use simulang_rs::ax_attribute::attr;
use simulang_rs::traits::{AXNodeActions, AXNodeSynthetic, AXNodeTrait, WindowTrait};
use simulang_rs::{AXNode, TreeIter, Window as SimulangWindow};

use crate::aria_role::AriaRole;

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

#[napi(object)]
#[derive(Clone)]
pub struct AccessibilityNodeJs {
  pub role: AriaRole,
  pub name: String,
  pub class_name: String,
  pub control_type: i32,
  pub localized_control_type: String,
  pub description: String,
  pub overall_description: String,
  pub help_text: String,
  pub value: String,
  pub automation_id: String,
  pub is_enabled: bool,
  pub bounding_box: BoundingBox,
  pub children: Vec<AccessibilityNodeJs>,
  pub ref_id: Option<u32>,
}

/// Axis-aligned rectangle. `right` and `bottom` are exclusive (Playwright /
/// DOM convention), so the box covers `[left, right) × [top, bottom)`.
#[napi(object)]
#[derive(Clone)]
pub struct BoundingBox {
  pub left: i32,
  pub top: i32,
  pub right: i32,
  pub bottom: i32,
}

impl From<simulang_rs::BoundingBox> for BoundingBox {
  fn from(b: simulang_rs::BoundingBox) -> Self {
    Self {
      left: b.left(),
      top: b.top(),
      right: b.right(),
      bottom: b.bottom(),
    }
  }
}

fn snapshot_node(
  node: &AXNode,
  refs: &mut Vec<AXNode>,
  visible_only: bool,
) -> Option<AccessibilityNodeJs> {
  if visible_only && node.get_attribute_boolean(attr::VISIBLE).ok() == Some(false) {
    return None;
  }
  let ref_id = u32::try_from(refs.len()).ok()?;
  refs.push(node.clone());
  let children = collect_collapsed_children(node, refs, visible_only);
  let bounding_box = node.bounding_box().map_or(
    BoundingBox {
      left: 0,
      top: 0,
      right: 0,
      bottom: 0,
    },
    BoundingBox::from,
  );
  Some(AccessibilityNodeJs {
    role: node.aria_role().into(),
    name: node.title(),
    class_name: node.class_name(),
    control_type: node.control_type_id(),
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
/// `AXNodeTrait::children` returns `Result<Vec<Self>, String>`; per the
/// trait docs (and the convention used by simulang-rs's own search
/// walkers) we treat failure as "no children" so a single torn-down
/// subtree cannot abort the whole snapshot.
fn collect_collapsed_children(
  node: &AXNode,
  refs: &mut Vec<AXNode>,
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

#[napi]
/// Accessibility tree bound to a specific window. Provides snapshot and
/// ref-based actions for desktop automation (Windows UIA).
pub struct AccessibilityTree {
  root: AXNode,
  refs: Vec<AXNode>,
}

impl AccessibilityTree {
  fn get_ref(&self, ref_id: u32) -> napi::Result<&AXNode> {
    self
      .refs
      .get(ref_id as usize)
      .ok_or_else(|| Error::from_reason(format!("Unknown ref_id {ref_id}")))
  }
}

#[napi]
impl AccessibilityTree {
  #[napi(factory)]
  /// Create an accessibility tree bound to the current foreground window.
  pub fn from_foreground() -> napi::Result<Self> {
    AXNode::from_focused_application()
      .map(|root| Self {
        root,
        refs: Vec::new(),
      })
      .map_err(Error::from_reason)
  }

  #[napi(factory)]
  /// Create an accessibility tree bound to the first visible window of a process.
  pub fn from_pid(pid: u32) -> napi::Result<Self> {
    #[allow(clippy::cast_possible_wrap)]
    AXNode::from_pid(pid as i32)
      .map(|root| Self {
        root,
        refs: Vec::new(),
      })
      .map_err(Error::from_reason)
  }

  #[napi(factory)]
  /// Create an accessibility tree from a platform-specific window identifier.
  /// On Windows this is an HWND; on macOS it is a PID.
  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  pub fn from_hwnd(hwnd: i64) -> napi::Result<Self> {
    #[cfg(target_os = "macos")]
    let result = AXNode::from_pid(hwnd as i32);
    #[cfg(target_os = "windows")]
    let result = SimulangWindow::from_hwnd_raw(hwnd as isize).node();
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let result: Result<AXNode, String> = Err("from_hwnd is not supported on this platform".into());

    result
      .map(|root| Self {
        root,
        refs: Vec::new(),
      })
      .map_err(Error::from_reason)
  }

  #[napi(getter)]
  #[must_use]
  /// Get the window title.
  pub fn window_title(&self) -> String {
    SimulangWindow::try_from_ax_node(&self.root).map_or_else(String::new, |w| w.title())
  }

  #[napi(getter)]
  #[must_use]
  /// Get the window handle as an integer ID.
  pub fn window_id(&self) -> i64 {
    #[cfg(target_os = "windows")]
    {
      SimulangWindow::try_from_ax_node(&self.root).map_or(0, |w| w.window_id() as i64)
    }
    #[cfg(target_os = "macos")]
    {
      self.root.window_id() as i64
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
      0
    }
  }

  #[napi]
  /// Take a snapshot of the window's accessibility tree.
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
    snapshot_node(&self.root, &mut self.refs, visible_only.unwrap_or(false))
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
    self.refs.clear();
    let visible_only = visible_only.unwrap_or(false);
    let collapse_structural = collapse_structural.unwrap_or(false);
    // Convert the JS-facing enum into the simulang-rs enum once, so
    // the per-node filter is a cheap discriminant compare instead of
    // a string comparison.
    let role: Option<simulang_rs::AriaRole> = role.map(Into::into);

    let iter = TreeIter::new(self.root.clone(), order.into(), collapse_structural).map(|(n, _)| n);

    iter
      .filter(|node| !visible_only || node.get_attribute_boolean(attr::VISIBLE).ok() != Some(false))
      .filter(|node| role.is_none_or(|r| node.aria_role() == r))
      .filter(|node| {
        name.as_ref().is_none_or(|n| {
          node.title().contains(n.as_str()) || node.description_text().contains(n.as_str())
        })
      })
      .take(max_results.map_or(usize::MAX, |n| n as usize))
      .map(|node| {
        let ref_id = u32::try_from(self.refs.len()).unwrap_or(u32::MAX);
        self.refs.push(node.clone());
        node_to_js(&node, ref_id)
      })
      .collect()
  }
}

/// Convert a single `AXNode` into a flat `AccessibilityNodeJs` (no children).
fn node_to_js(node: &AXNode, ref_id: u32) -> AccessibilityNodeJs {
  let bounding_box = node.bounding_box().map_or(
    BoundingBox {
      left: 0,
      top: 0,
      right: 0,
      bottom: 0,
    },
    BoundingBox::from,
  );
  AccessibilityNodeJs {
    role: node.aria_role().into(),
    name: node.title(),
    class_name: node.class_name(),
    control_type: node.control_type_id(),
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
