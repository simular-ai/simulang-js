use napi::Error;
use napi_derive::napi;
use simulang_rs::Screen as SimulangScreen;
use simulang_rs::traits::ScreenTrait;

#[napi]
/// Represents a physical display/screen.
pub struct Screen {
  pub(crate) inner: SimulangScreen,
}

#[napi]
impl Screen {
  #[napi(factory)]
  /// Returns the screen identifier for the main screen.
  pub fn main_screen() -> napi::Result<Self> {
    SimulangScreen::main_screen()
      .map(|inner| Self { inner })
      .map_err(Error::from_reason)
  }

  #[napi(factory)]
  /// Returns the screen identifier for the screen on which the mouse
  /// is located. If the mouse is not on any screen, the main screen
  /// is returned.
  pub fn from_current_mouse_location() -> napi::Result<Self> {
    SimulangScreen::from_current_mouse_location()
      .map(|inner| Self { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Returns the dimensions of the screen in pixels.
  ///
  /// The return value is `[x, y, width, height]`.
  pub fn dimensions(&self) -> napi::Result<(i32, i32, u16, u16)> {
    self.inner.dimensions().map_err(Error::from_reason)
  }
}
