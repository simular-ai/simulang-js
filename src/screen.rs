use napi::Error;
use napi_derive::napi;
use simulang_rs::Screen as SimulangScreen;
use simulang_rs::traits::{BoundingBoxTrait, ScreenTrait};

use crate::ax_tree::BoundingBox;
use crate::window::Window;

#[napi]
/// Represents a connected display/screen on the global desktop.
pub struct Screen {
  pub(crate) inner: SimulangScreen,
}

#[napi]
impl Screen {
  #[napi(factory)]
  /// Returns the main / primary screen.
  pub fn main_screen() -> napi::Result<Self> {
    SimulangScreen::main_screen()
      .map(|inner| Self { inner })
      .map_err(Error::from_reason)
  }

  #[napi(factory)]
  /// Returns the screen on which the mouse cursor is located.
  /// Falls back to the main screen if the cursor is not on any
  /// connected display.
  pub fn from_current_mouse_location() -> napi::Result<Self> {
    SimulangScreen::from_current_mouse_location()
      .map(|inner| Self { inner })
      .map_err(Error::from_reason)
  }

  #[napi(factory)]
  /// Returns the screen the given window is on.
  ///
  /// "On" is the connected display that contains the largest area of
  /// [`Window.boundingBox`] — the same heuristic the OS uses to decide
  /// a window's "owning" screen, so the result matches what the system
  /// considers the window's screen (the one its window controls render
  /// on, the one full-screen mode targets, etc.).
  ///
  /// Throws if the window has no measurable overlap with any connected
  /// display — for example when the window is fully off-screen,
  /// minimised to an off-screen state, or on a virtual desktop with no
  /// attached display. There is no "correct" screen to pick in that
  /// case; callers that prefer a fallback can wrap in `try` / `catch`
  /// and call [`Screen.mainScreen`].
  pub fn from_window(window: &Window) -> napi::Result<Self> {
    SimulangScreen::from_window(&window.inner)
      .map(|inner| Self { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  #[must_use]
  /// All currently connected displays.
  ///
  /// Order is platform-defined; do not rely on it. The result is a
  /// snapshot — connect / disconnect events that happen during the
  /// call are not signalled.
  pub fn all() -> Vec<Screen> {
    SimulangScreen::all()
      .map(|screens| screens.into_iter().map(|inner| Self { inner }).collect())
      .unwrap_or_default()
  }

  #[napi]
  /// Live bounding box of the screen on the global desktop.
  ///
  /// Top-left of the primary monitor is `(0, 0)`, in the canonical coordinate
  /// space (OS-native units; see [`MouseController`]).
  /// Monitors arranged to the left of or above the primary display
  /// contribute **negative** `left` / `top`. `right` and `bottom` are
  /// exclusive (Playwright / DOM convention), matching
  /// [`Window.boundingBox`].
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
