use napi::Error;
use napi_derive::napi;
use simulang_rs::{AXSnapshot, Node, PrintOptions};

use crate::accessibility_node::AccessibilityNode;
use crate::ax_tree::BoundingBox;
use crate::window::Window;

/// Options for [`AccessibilitySnapshot.toStringWith`].
#[napi(object)]
#[derive(Default)]
pub struct SnapshotPrintOptions {
  /// Hoist unnamed structural wrappers out of the output: the wrapper line
  /// is skipped and its children are rendered at the depth the wrapper
  /// would have had. The root is always rendered. Defaults to `true`.
  pub collapse_structural: Option<bool>,
  /// Only render interactive elements (buttons, links, inputs, …) and the
  /// ancestors needed to give them context: a node is dropped when its
  /// role is not interactive **and** no rendered node exists below it.
  /// Defaults to `false`.
  pub interactive_only: Option<bool>,
  /// Maximum rendered depth (post-collapse, `0`-based). Nodes deeper than
  /// this — and their entire subtrees — are dropped. Unset renders all
  /// depths.
  pub max_depth: Option<u32>,
  /// Maximum characters per node name; longer names are cut and suffixed
  /// with `…`. Also caps rendered values, which are capped at 80 characters
  /// when this is unset.
  pub max_chars: Option<u32>,
}

impl From<&SnapshotPrintOptions> for PrintOptions {
  fn from(o: &SnapshotPrintOptions) -> Self {
    let defaults = Self::default();
    Self {
      collapse_structural: o
        .collapse_structural
        .unwrap_or(defaults.collapse_structural),
      interactive_only: o.interactive_only.unwrap_or(defaults.interactive_only),
      max_depth: o.max_depth.map(|d| d as usize),
      max_chars: o.max_chars.map(|c| c as usize),
    }
  }
}

/// A materialized accessibility snapshot: one pre-order walk of a subtree,
/// stored flat so every node is addressable by a stable index (its "ref").
///
/// Construct with [`AccessibilitySnapshot.fromNode`] (e.g. from
/// [`Machine.focusedRoot`]) or [`AccessibilitySnapshot.fromWindow`]. Render it
/// as
/// LLM-facing text with [`AccessibilitySnapshot.toStringWith`], resolve
/// indices back to live nodes with [`AccessibilitySnapshot.node`], and act
/// on elements by index via the action methods.
///
/// Indices are frozen at capture time and cover **every** node of the walk
/// (including collapsible wrappers), so the same snapshot can be printed
/// with different [`SnapshotPrintOptions`] without renumbering. A
/// consequence: rendering with `collapseStructural` produces
/// non-contiguous `[ref=N]` values, because skipped wrappers keep their
/// index.
#[napi]
pub struct AccessibilitySnapshot {
  inner: AXSnapshot<Node>,
}

#[napi]
impl AccessibilitySnapshot {
  #[napi(factory)]
  /// Construct a snapshot of `window`'s accessibility subtree.
  pub fn from_window(window: &Window) -> napi::Result<Self> {
    window
      .inner
      .node()
      .map(|root| Self {
        inner: AXSnapshot::from_node(root),
      })
      .map_err(Error::from_reason)
  }

  #[napi(factory)]
  /// Construct a snapshot of the subtree rooted at `node`.
  ///
  /// This is a full pre-order walk with no structural collapse and no
  /// property reads. Filtering and formatting are deferred to
  /// [`AccessibilitySnapshot.toStringWith`] so one capture can serve
  /// several renderings.
  #[must_use]
  pub fn from_node(node: &AccessibilityNode) -> Self {
    Self {
      inner: AXSnapshot::from_node(node.inner.clone()),
    }
  }

  #[napi]
  #[must_use]
  /// Number of captured nodes.
  pub fn len(&self) -> u32 {
    u32::try_from(self.inner.len()).unwrap_or(u32::MAX)
  }

  #[napi(getter)]
  #[must_use]
  /// Number of captured nodes. Alias for [`AccessibilitySnapshot.len`].
  pub fn node_count(&self) -> u32 {
    self.len()
  }

  #[napi]
  #[must_use]
  /// `true` if the snapshot holds no nodes.
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  /// Render the snapshot as indented, Playwright-style lines with
  /// `[ref=N]` markers:
  ///
  /// ```text
  /// - window "Untitled - Notepad" [ref=0]
  ///   - textbox "Text editor" #editor [ref=2]: "hello"
  ///   - button "Save" [ref=5]
  /// ```
  ///
  /// One line per rendered node: two spaces of indentation per depth
  /// level, the cross-platform ARIA role, the name (escaped and truncated
  /// per `maxChars`), a `#automation-id` marker when the platform reports
  /// one, the ref index, and a `: "value"` suffix for value-bearing roles.
  /// Filtering is controlled by [`SnapshotPrintOptions`]; returns
  /// `"(empty)"` when every node is filtered out.
  ///
  /// Properties are read from the live elements on every call, so the text
  /// reflects current UI state while ref numbering stays frozen.
  #[must_use]
  pub fn to_string_with(&self, options: Option<SnapshotPrintOptions>) -> String {
    let options = options.unwrap_or_default();
    self.inner.to_string_with(&PrintOptions::from(&options))
  }

  #[napi]
  /// Alias for [`AccessibilitySnapshot.toStringWith`].
  #[must_use]
  pub fn print(&self, options: Option<SnapshotPrintOptions>) -> String {
    self.to_string_with(options)
  }

  #[napi]
  /// Live node handle for a ref index.
  ///
  /// An out-of-range index is an error, not an expected miss: refs come
  /// from this snapshot's own rendered output, so an unknown index means
  /// the caller mixed up snapshots (or hallucinated a ref).
  pub fn node(&self, index: u32) -> napi::Result<AccessibilityNode> {
    self
      .inner
      .node(index as usize)
      .map(|node| AccessibilityNode::new(node.clone()))
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Live bounding box of the element at `index` — for example for a
  /// coordinate-click fallback when a semantic action fails.
  pub fn bounding_box(&self, index: u32) -> napi::Result<BoundingBox> {
    self
      .inner
      .bounding_box(index as usize)
      .map(BoundingBox::from)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Alias for [`AccessibilitySnapshot.boundingBox`].
  pub fn get_bounds(&self, index: u32) -> napi::Result<BoundingBox> {
    self.bounding_box(index)
  }

  #[napi]
  /// Click the element at `index`, dispatching to the platform action that
  /// matches its current role:
  ///
  /// - button / link / menuitem / img → `activate`
  /// - tab / radio / option → `select`
  /// - combobox / treeitem → `expandCollapse`
  /// - checkbox / switch → `toggle`
  /// - anything else → `activate`
  ///
  /// On error, callers can fall back to a coordinate click at the centre
  /// of [`AccessibilitySnapshot.boundingBox`].
  pub fn click(&self, index: u32) -> napi::Result<()> {
    self.inner.click(index as usize).map_err(Error::from_reason)
  }

  #[napi]
  /// Invoke / press the element at `index`.
  pub fn activate(&self, index: u32) -> napi::Result<()> {
    self
      .inner
      .activate(index as usize)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Set the text value of the element at `index`.
  #[allow(clippy::needless_pass_by_value)]
  pub fn set_value(&self, index: u32, value: String) -> napi::Result<()> {
    self
      .inner
      .set_value(index as usize, &value)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Toggle the checkbox / switch at `index`.
  pub fn toggle(&self, index: u32) -> napi::Result<()> {
    self
      .inner
      .toggle(index as usize)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Select the tab / radio / list item at `index`.
  pub fn select(&self, index: u32) -> napi::Result<()> {
    self
      .inner
      .select(index as usize)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Expand or collapse the element at `index`.
  pub fn expand_collapse(&self, index: u32) -> napi::Result<()> {
    self
      .inner
      .expand_collapse(index as usize)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Open the context menu of the element at `index` — the semantic
  /// equivalent of a right-click, without synthesizing pointer input, so
  /// it works on background / obscured windows (Windows `ShowContextMenu`,
  /// macOS `AXShowMenu`, Linux AT-SPI show-menu, Android long-press).
  ///
  /// The opened menu itself typically appears as the topmost / focused
  /// window even when the target window stays in the background — a user
  /// watching the desktop sees a menu pop up without having done
  /// anything.
  ///
  /// Throws when the element does not support opening a menu this way;
  /// callers can fall back to a coordinate right-click at the centre of
  /// [`AccessibilitySnapshot.boundingBox`].
  pub fn show_menu(&self, index: u32) -> napi::Result<()> {
    self
      .inner
      .show_menu(index as usize)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Scroll the element at `index` into view.
  pub fn scroll_into_view(&self, index: u32) -> napi::Result<()> {
    self
      .inner
      .scroll_into_view(index as usize)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Focus the element at `index` (may also bring its window to the
  /// foreground).
  pub fn set_focus(&self, index: u32) -> napi::Result<()> {
    self
      .inner
      .set_focus(index as usize)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Alias for [`AccessibilitySnapshot.setFocus`].
  pub fn focus_element(&self, index: u32) -> napi::Result<()> {
    self.set_focus(index)
  }
}
