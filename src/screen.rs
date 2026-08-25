use napi::Error;
use napi_derive::napi;
use simulang_rs::Screen as SimulangScreen;
use simulang_rs::traits::BoundingBoxTrait;

use crate::ax_tree::BoundingBox;
use crate::screenshot::Screenshot;

#[napi]
/// A display of a [`Machine`].
///
/// Obtain via [`Machine.screens`], [`Machine.mainScreen`],
/// [`Machine.screenFromMouse`], or [`Window.screen`]. A handle pins work to
/// one display: two [`Screen.screenshot`] calls on the same handle always
/// capture the same display, unlike re-resolving the cursor's screen
/// between captures.
pub struct Screen {
  pub(crate) inner: SimulangScreen,
}

#[napi]
impl Screen {
  #[napi]
  /// Capture this screen's pixels.
  ///
  /// On Android `hideCursor` is ignored — a phone has no cursor.
  pub fn screenshot(&self, hide_cursor: bool) -> napi::Result<Screenshot> {
    self
      .inner
      .screenshot(hide_cursor)
      .map(|inner| Screenshot { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Live bounding box of the screen on the global desktop.
  ///
  /// Top-left of the primary monitor is `(0, 0)`, in the canonical
  /// coordinate space (OS-native units; see [`Machine`]). Monitors
  /// arranged to the left of or above the primary display contribute
  /// **negative** `left` / `top`. `right` and `bottom` are exclusive
  /// (Playwright / DOM convention), matching [`Window.boundingBox`].
  ///
  /// For just the size, read `right - left` and `bottom - top` on the
  /// returned box.
  pub fn bounding_box(&self) -> napi::Result<BoundingBox> {
    self
      .inner
      .bounding_box()
      .map(BoundingBox::from)
      .map_err(Error::from_reason)
  }
}
