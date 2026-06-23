use std::path::Path;

use napi::Error;
use napi_derive::napi;
use simulang_rs::Screenshot as SimulangScreenshot;
use simulang_rs::traits::{
  ImageTrait, ScreenshotCoordinateType as SimulangScreenshotCoordinateType, ScreenshotTrait,
};

use crate::language_model::vlm::GroundingModel;
use crate::screen::Screen;

#[napi]
/// Represents a screenshot capture.
pub struct Screenshot {
  pub(crate) inner: SimulangScreenshot,
}

#[napi]
/// How a screenshot-relative `(x, y)` coordinate is expressed: as absolute
/// screenshot-image pixels or normalized to a fixed range. Construct one with
/// [`ScreenshotCoordinateType.absolute`] or [`ScreenshotCoordinateType.normalized`].
pub struct ScreenshotCoordinateType {
  normalized_range: Option<u32>,
}

#[napi]
impl ScreenshotCoordinateType {
  #[napi(factory)]
  /// Coordinates are in screenshot image pixels (valid range:
  /// `0..=image_width-1`).
  #[must_use]
  pub fn absolute() -> Self {
    Self {
      normalized_range: None,
    }
  }

  #[napi(factory)]
  /// Coordinates are normalized to a fixed range. Many VLMs (Qwen-VL,
  /// UI-TARS, etc.) output grounding coordinates in `[0, range]` rather
  /// than in pixel space. `range` specifies the inclusive upper bound
  /// (e.g. 1000, meaning valid coordinates are `0..=1000`).
  #[must_use]
  pub fn normalized(range: u32) -> Self {
    Self {
      normalized_range: Some(range),
    }
  }
}

#[napi]
impl Screenshot {
  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Path includes the file name and the extension.
  pub fn save(&self, path: String) -> napi::Result<()> {
    self
      .inner
      .save(Path::new(&path))
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Draws a cross-hair grid on the image.
  ///
  /// Grid squares have the specified `width` and `height`.
  pub fn draw_grid(&mut self, width: u16, height: u16) -> napi::Result<()> {
    self
      .inner
      .draw_grid(width, height)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Paints a filled disc on the image. Useful for visualizing point
  /// coordinates returned from grounding, layout queries, etc.
  ///
  /// `x` / `y` are image-pixel coordinates of the disc's centre. `radius`
  /// is the disc radius in pixels (`0` paints a single pixel at the
  /// centre). `(red, green, blue)` is the fill color; alpha is always 255
  /// (opaque replacement of the underlying pixel).
  ///
  /// Coordinates that fall outside the image bounds (negative, or past the
  /// width / height) silently produce no pixel, so the helper is safe to
  /// call with the raw output of a grounding model even at the edge of the
  /// captured rect.
  pub fn draw_dot(
    &mut self,
    x: i32,
    y: i32,
    radius: u16,
    red: u8,
    green: u8,
    blue: u8,
  ) -> napi::Result<()> {
    self
      .inner
      .draw_dot(x, y, radius, [red, green, blue])
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Compress the image by converting it to JPEG with the specified quality.
  ///
  /// The quality is a value between 1 and 100.
  /// 1 is the lowest possible quality and 100 is the highest quality.
  pub fn compress(&mut self, quality: u8) -> napi::Result<()> {
    self.inner.compress(quality).map_err(Error::from_reason)
  }

  #[napi]
  /// Resizes this image if it is larger than the desired size. The image's
  /// aspect ratio is preserved. The image is scaled to the maximum
  /// possible size that fits within the bounds specified by nwidth and
  /// nheight.
  ///
  /// This method operates on pixel channel values directly without taking
  /// into account color space data.
  ///
  /// We commonly use this to resize the image to 1920x1080.
  pub fn shrink(&mut self, nwidth: u32, nheight: u32) -> napi::Result<()> {
    self
      .inner
      .shrink(nwidth, nheight)
      .map_err(Error::from_reason)
  }

  #[napi(getter)]
  #[must_use]
  /// Returns the screenshot dimensions as `[width, height]` in pixels.
  pub fn dimensions(&self) -> (u32, u32) {
    self.inner.dimensions()
  }

  #[napi]
  #[must_use]
  /// Returns the image encoded as a base64 data URL.
  ///
  /// The result includes the MIME prefix, for example
  /// `data:image/png;base64,...` or `data:image/jpeg;base64,...`.
  pub fn base64(&self) -> String {
    self.inner.base64()
  }

  #[napi]
  /// Converts a point in this screenshot to global desktop coordinates.
  ///
  /// The result is in the library's canonical coordinate space (OS-native
  /// units; see [`MouseController`]), so it can be fed straight to
  /// `MouseController.moveMouse` without conversion.
  ///
  /// Screenshots may represent only part of a display, and the captured
  /// region may have been resampled to a different image size, so this
  /// rescales `(x, y)` from image space back to the captured region (for
  /// example, when moving the mouse to the same on-screen point).
  ///
  /// See `ScreenshotCoordinateType` for how `coord_type` affects
  /// interpretation of `(x, y)`.
  pub fn to_global_desktop_coordinates(
    &self,
    x: u32,
    y: u32,
    coord_type: &ScreenshotCoordinateType,
  ) -> napi::Result<(i32, i32)> {
    let coord_type = match coord_type.normalized_range {
      Some(range) => SimulangScreenshotCoordinateType::Normalized(range),
      None => SimulangScreenshotCoordinateType::Absolute,
    };
    self
      .inner
      .to_global_desktop_coordinates(x, y, coord_type)
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Locate `concept` on this screenshot using the given grounding model and
  /// return the corresponding **global desktop coordinates** `[x, y]` in
  /// OS-native units (may be negative on multi-monitor setups; see
  /// [`MouseController`]). The output can be fed
  /// directly to primitives that expect global screen coordinates.
  ///
  /// Equivalent to `model.ground(screenshot, concept)`.
  pub fn ground(&self, model: &GroundingModel, concept: String) -> napi::Result<(i32, i32)> {
    model
      .inner
      .ground(&self.inner, &concept)
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}

#[napi]
/// Takes the screenshot of the entire selected screen
pub fn screenshot_full(hide_cursor: bool, screen: &Screen) -> napi::Result<Screenshot> {
  SimulangScreenshot::screenshot_full(hide_cursor, &screen.inner)
    .map(|inner| Screenshot { inner })
    .map_err(Error::from_reason)
}

#[napi]
/// Takes the screenshot of a cropped region of the workspace.
pub fn screenshot_cropped(
  x: i32,
  y: i32,
  width: u16,
  height: u16,
  hide_cursor: bool,
) -> napi::Result<Screenshot> {
  SimulangScreenshot::screenshot_cropped(x, y, width, height, hide_cursor)
    .map(|inner| Screenshot { inner })
    .map_err(Error::from_reason)
}
