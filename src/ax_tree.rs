use napi::Error;
use napi_derive::napi;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use simulang_rs::Window as SimulangWindow;
use simulang_rs::ax_attribute::attr;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use simulang_rs::traits::WindowTrait;
use simulang_rs::traits::{
  AXNodeActions, AXNodeAncestry, AXNodeSynthetic, AXNodeTrait, BoundingBoxTrait,
};
use simulang_rs::{AXNode, TreeIter};

use crate::aria_role::AriaRole;
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
/// `AXNodeAncestry::children` returns `Result<Vec<Self>, String>`; per the
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

/// macOS root scoping for an [`AccessibilityTree`].
///
/// - `App(pid)` keeps the historical application-wide scope: the macOS AX
///   root is the application element keyed by PID (every window plus the
///   app menu bar). Storing the PID lets us reattach a fresh `AXUIElement`
///   on every snapshot.
/// - `Window(node)` scopes to a single `AXWindow` subtree, used by
///   [`AccessibilityTree::from_window`]. The `AXUIElement` is held directly
///   (it can go stale if the window is destroyed and recreated, mirroring
///   how an `HWND` can on Windows).
#[cfg(target_os = "macos")]
enum MacRoot {
  App(i32),
  Window(AXNode),
}

#[napi]
/// Accessibility tree bound to a specific window. Provides snapshot and
/// ref-based actions for desktop automation (Windows UIA).
pub struct AccessibilityTree {
  /// Native window handle (Windows) — opaque `HWND` as `isize`. Resolved
  /// once at construction time and re-attached via
  /// `Window::from_hwnd_raw(...).node()` on every snapshot.
  #[cfg(target_os = "windows")]
  hwnd: isize,
  /// Root scoping (macOS): application-wide (PID) or a single window — see
  /// [`MacRoot`].
  #[cfg(target_os = "macos")]
  root: MacRoot,
  refs: Vec<AXNode>,
}

impl AccessibilityTree {
  /// Re-resolve the tree's root node from its stored platform identifier.
  ///
  /// On Windows this calls `Window::node()`, which materialises a fresh
  /// cached subtree (single `BuildUpdatedCache` IPC) so the subsequent
  /// walk costs zero additional IPCs. On macOS this rebinds the
  /// `AXUIElement` for the PID — cheap, no IPC.
  // Linux (and other not-yet-supported targets) compile only the stub
  // arm below, which doesn't touch `self`. Keep `clippy::pedantic`
  // enabled on Windows / macOS where the method actually uses `self`.
  #[cfg_attr(
    not(any(target_os = "windows", target_os = "macos")),
    allow(clippy::unused_self)
  )]
  fn resolve_root(&self) -> napi::Result<AXNode> {
    #[cfg(target_os = "windows")]
    {
      SimulangWindow::from_hwnd_raw(self.hwnd)
        .node()
        .map_err(Error::from_reason)
    }
    #[cfg(target_os = "macos")]
    {
      match &self.root {
        MacRoot::App(pid) => AXNode::from_pid(*pid).map_err(Error::from_reason),
        MacRoot::Window(node) => Ok(node.clone()),
      }
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
      Err(Error::from_reason(
        "AccessibilityTree is not supported on this platform".to_owned(),
      ))
    }
  }

  fn get_ref(&self, ref_id: u32) -> napi::Result<&AXNode> {
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
    predicate: impl Fn(&AXNode) -> bool,
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
  /// Create an accessibility tree bound to the current foreground window.
  ///
  /// The foreground window is resolved **once** at construction time and
  /// the resulting identifier (HWND on Windows, PID on macOS) is stored
  /// — subsequent snapshots target that same window even if the user
  /// alt-tabs away.
  pub fn from_foreground() -> napi::Result<Self> {
    #[cfg(target_os = "windows")]
    {
      // Use the existing AXNode entry point + Window::try_from_ax_node
      // to extract the HWND, so we stay on simulang-rs's public surface
      // and don't reach for Win32 directly.
      let live = AXNode::from_focused_application().map_err(Error::from_reason)?;
      let window = SimulangWindow::try_from_ax_node(&live)
        .ok_or_else(|| Error::from_reason("Foreground element has no associated native window"))?;
      Ok(Self {
        hwnd: window.window_id(),
        refs: Vec::new(),
      })
    }
    #[cfg(target_os = "macos")]
    {
      let (_, _, pid) = simulang_rs::get_frontmost_application().map_err(Error::from_reason)?;
      Ok(Self {
        root: MacRoot::App(pid),
        refs: Vec::new(),
      })
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
      Err(Error::from_reason(
        "AccessibilityTree.fromForeground is not supported on this platform".to_owned(),
      ))
    }
  }

  #[napi(factory)]
  /// Create an accessibility tree bound to the first visible window of a
  /// process. The window is selected at construction time; subsequent
  /// snapshots target that same window.
  pub fn from_pid(pid: u32) -> napi::Result<Self> {
    #[allow(clippy::cast_possible_wrap)]
    let pid = pid as i32;
    #[cfg(target_os = "windows")]
    {
      let window = SimulangWindow::all_for_pid(pid)
        .into_iter()
        .next()
        .ok_or_else(|| {
          Error::from_reason(format!(
            "No visible top-level window found for process {pid}."
          ))
        })?;
      Ok(Self {
        hwnd: window.window_id(),
        refs: Vec::new(),
      })
    }
    #[cfg(target_os = "macos")]
    {
      // Validate the PID resolves to an AX application — surfaces a clear
      // error at construction rather than on first snapshot.
      AXNode::from_pid(pid).map_err(Error::from_reason)?;
      Ok(Self {
        root: MacRoot::App(pid),
        refs: Vec::new(),
      })
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
      let _ = pid;
      Err(Error::from_reason(
        "AccessibilityTree.fromPid is not supported on this platform".to_owned(),
      ))
    }
  }

  #[napi(factory)]
  /// Create an accessibility tree from a platform-specific window identifier.
  /// On Windows this is an HWND; on macOS it is a PID.
  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  pub fn from_hwnd(hwnd: i64) -> napi::Result<Self> {
    #[cfg(target_os = "windows")]
    {
      Ok(Self {
        hwnd: hwnd as isize,
        refs: Vec::new(),
      })
    }
    #[cfg(target_os = "macos")]
    {
      Ok(Self {
        root: MacRoot::App(hwnd as i32),
        refs: Vec::new(),
      })
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
      let _ = hwnd;
      Err(Error::from_reason(
        "AccessibilityTree.fromHwnd is not supported on this platform".to_owned(),
      ))
    }
  }

  #[napi(factory)]
  /// Create an accessibility tree scoped to a single window.
  ///
  /// Unlike [`AccessibilityTree.fromForeground`] / [`fromPid`] — which on
  /// macOS scope to the whole application (every window plus the app menu
  /// bar) — this scopes to exactly the given window's subtree on **both**
  /// Windows and macOS. Use it to measure "is this element unique within
  /// this window", or to snapshot / act on one window of a multi-window
  /// app.
  ///
  /// The window is resolved once at construction; subsequent snapshots
  /// target that same window. On macOS the underlying `AXWindow` handle can
  /// go stale if the window is destroyed and recreated (as an `HWND` can on
  /// Windows).
  pub fn from_window(window: &Window) -> napi::Result<Self> {
    #[cfg(target_os = "windows")]
    {
      Ok(Self {
        hwnd: window.inner.window_id(),
        refs: Vec::new(),
      })
    }
    #[cfg(target_os = "macos")]
    {
      let node = window.inner.node().map_err(Error::from_reason)?;
      Ok(Self {
        root: MacRoot::Window(node),
        refs: Vec::new(),
      })
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
      let _ = window;
      Err(Error::from_reason(
        "AccessibilityTree.fromWindow is not supported on this platform".to_owned(),
      ))
    }
  }

  #[napi(getter)]
  #[must_use]
  /// Get the window title.
  pub fn window_title(&self) -> String {
    #[cfg(target_os = "windows")]
    {
      // `Window::title` calls `GetWindowTextW` — no UIA IPC, no cache build.
      SimulangWindow::from_hwnd_raw(self.hwnd).title()
    }
    #[cfg(target_os = "macos")]
    {
      let node = match &self.root {
        MacRoot::App(pid) => AXNode::from_pid(*pid).ok(),
        MacRoot::Window(node) => Some(node.clone()),
      };
      node
        .and_then(|root| SimulangWindow::try_from_ax_node(&root))
        .map_or_else(String::new, |w| w.title())
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
      String::new()
    }
  }

  #[napi(getter)]
  #[must_use]
  /// Get the window handle as an integer ID.
  /// On Windows this is the HWND; on macOS it is the PID.
  pub fn window_id(&self) -> i64 {
    #[cfg(target_os = "windows")]
    {
      self.hwnd as i64
    }
    #[cfg(target_os = "macos")]
    {
      // macOS has no per-window integer handle; report the owning PID for
      // both scopes (the application's PID for `App`, the window's owning
      // PID for `Window`).
      match &self.root {
        MacRoot::App(pid) => i64::from(*pid),
        MacRoot::Window(node) => SimulangWindow::try_from_ax_node(node)
          .and_then(|w| w.pid().ok())
          .map_or(0, i64::from),
      }
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
      0
    }
  }

  #[napi]
  /// Take a snapshot of the window's accessibility tree.
  ///
  /// Re-resolves the root through `Window::node()` so each call sees
  /// current data. On Windows the resolve issues a single
  /// `BuildUpdatedCache` IPC; the recursive walk over children and
  /// properties then stays entirely in-process, ~40× faster than
  /// walking a live `cached: false` root.
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
