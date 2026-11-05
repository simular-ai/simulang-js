use napi::Error;
use napi::bindgen_prelude::Either;
use napi_derive::napi;
use simulang_rs::Screen as SimulangScreen;
use simulang_rs::ScreenId;
use simulang_rs::traits::BoundingBoxTrait;

use crate::ax_tree::BoundingBox;
use crate::machine::Machine;
use crate::screenshot::Screenshot;

#[napi]
/// A display of a [`Machine`].
///
/// Obtain via [`Machine.screens`], [`Machine.primaryScreen`],
/// [`Machine.screenFromMouse`], [`screen`], or [`Window.screen`]. A
/// handle pins work to one display: two [`Screen.screenshot`] calls on the
/// same handle always capture the same display, unlike re-resolving the
/// cursor's screen between captures.
///
/// A handle stores the id it was opened with (see [`Screen.id`]) and looks
/// the display up on every call. An index handle follows a place in the
/// arrangement; a unique id handle follows one physical monitor.
pub struct Screen {
  pub(crate) inner: SimulangScreen,
}

#[napi]
impl Screen {
  #[napi(getter, catch_unwind)]
  #[must_use]
  /// The id this handle was opened with: the display's index (a number)
  /// or a monitor's unique id (a string).
  pub fn id(&self) -> Either<u32, String> {
    match self.inner.id() {
      ScreenId::Index(index) => Either::A(*index),
      ScreenId::Unique(unique) => Either::B(unique.clone()),
    }
  }

  #[napi(catch_unwind)]
  /// The display's index now. `0` is the display the OS calls primary; the
  /// rest follow by their top-left corner, left to right and then top to
  /// bottom, the same on every platform. Displays mirrored onto one another
  /// share an index. For an index handle this is the stored index.
  pub fn index(&self) -> napi::Result<u32> {
    self.inner.index().map_err(Error::from_reason)
  }

  #[napi(catch_unwind)]
  /// The display's unique id now, or `null` when it reports none: an
  /// Android screen, or a monitor without an identity in its EDID.
  ///
  /// The id follows one physical monitor whatever port it is plugged into
  /// and however the displays are arranged: the PNP manufacturer and
  /// product codes, then the serial number, e.g. `DELA0B1-CN0F1234`.
  /// Identical monitors that report the same identity are told apart in
  /// index order with `#2`, `#3`. Every monitor of a mirror or clone group
  /// has its own id.
  pub fn unique_id(&self) -> napi::Result<Option<String>> {
    self.inner.unique_id().map_err(Error::from_reason)
  }

  /// `true` if `other` is a handle to the same machine opened with the
  /// same id. This doesn't ask the OS, so an index handle and a unique id
  /// handle aren't equal even when they name the same display. `===` is
  /// still object identity.
  #[napi(catch_unwind)]
  #[must_use]
  pub fn equals(&self, other: &Screen) -> bool {
    self.inner == other.inner
  }

  /// The handle's id, e.g. `Screen (index 0)`.
  #[napi(js_name = "toString", catch_unwind)]
  #[must_use]
  pub fn as_js_string(&self) -> String {
    format!("Screen ({})", self.inner.id())
  }

  #[napi(catch_unwind)]
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

  #[napi(catch_unwind)]
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

/// The display of `machine` with this id, or `null` when no connected
/// display has it.
///
/// `id` is an index (a number) or a unique id (a string); see [`Screen.id`].
/// The handle stores it as given: an index handle follows a place in the
/// arrangement, a unique id handle follows one physical monitor whatever
/// port it is plugged into.
#[napi(catch_unwind)]
pub fn screen(machine: &Machine, id: Either<u32, String>) -> napi::Result<Option<Screen>> {
  let id: ScreenId = match id {
    Either::A(index) => index.into(),
    Either::B(unique) => unique.into(),
  };
  SimulangScreen::from(id, &machine.inner)
    .map(|screen| screen.map(|inner| Screen { inner }))
    .map_err(Error::from_reason)
}
