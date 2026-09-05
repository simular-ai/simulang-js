use std::path::Path;

use napi::Error;
use napi_derive::napi;
use simulang_rs::Screenshot as SimulangScreenshot;
use simulang_rs::traits::{
  ImageTrait, ScreenshotCoordinateType as SimulangScreenshotCoordinateType,
};

use crate::ax_tree::BoundingBox;
use crate::language_model::vlm::GroundingModel;

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
  /// Paints a filled disc on the screenshot. Useful for visualizing point
  /// coordinates returned from grounding, element / layout queries, etc.
  ///
  /// `x` / `y` are **global desktop** coordinates (the same space
  /// `Screenshot.ground` returns and [`Machine.moveMouse`] consumes), which are
  /// converted to image pixels — inverting the capture offset and any
  /// resampling — before drawing. So a `ground(...)` result or an element's
  /// `boundingBox()` corner can be passed straight in. `radius` is the disc
  /// radius in pixels (`0` paints a single pixel at the centre).
  /// `(red, green, blue)` is the fill color; alpha is always 255 (opaque
  /// replacement of the underlying pixel).
  ///
  /// Coordinates that map outside the image bounds silently produce no pixel,
  /// so the helper is safe to call even for points that fall outside the
  /// captured region.
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
  /// Draws the outline of the axis-aligned rectangle `bounds` on the
  /// screenshot. Useful for visualizing bounding boxes returned from grounding,
  /// element / window `boundingBox()` queries, ground-truth annotations, etc.
  ///
  /// `bounds` is in **global desktop** coordinates (the same space element /
  /// window `boundingBox()` and grounding results use); its corners are
  /// converted to image pixels — inverting the capture offset and any
  /// resampling — before drawing, so an element's `boundingBox()` can be passed
  /// straight in. It covers `[left, right) × [top, bottom)` (`right` / `bottom`
  /// exclusive). The border is `thickness` pixels wide, drawn inset, in the
  /// opaque RGB `(red, green, blue)` color. Pixels that map outside the image
  /// bounds are silently clipped. Throws when `thickness` is `0`. A
  /// [`BoundingBox`] cannot be degenerate (the constructor rejects those).
  pub fn draw_box(
    &mut self,
    bounds: &BoundingBox,
    thickness: u16,
    red: u8,
    green: u8,
    blue: u8,
  ) -> napi::Result<()> {
    self
      .inner
      .draw_box(bounds.inner, thickness, [red, green, blue])
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
  /// Returns the image encoded as raw base64, without a MIME prefix.
  pub fn base64(&self) -> napi::Result<String> {
    self.inner.base64().map_err(Error::from_reason)
  }

  #[napi]
  /// Returns the image encoded as a base64 data URL.
  ///
  /// The result includes the MIME prefix, for example
  /// `data:image/png;base64,...`, `data:image/jpeg;base64,...`,
  /// `data:image/gif;base64,...`, or `data:image/webp;base64,...`.
  pub fn base64_data_url(&self) -> napi::Result<String> {
    self.inner.base64_data_url().map_err(Error::from_reason)
  }

  #[napi]
  /// Converts a point in this screenshot to the machine's coordinate space.
  ///
  /// The result is in the library's canonical coordinate space (OS-native
  /// units; see [`Machine`]), so it can be fed straight to
  /// [`Machine.moveMouse`] without conversion.
  ///
  /// Screenshots may represent only part of a display, and the captured
  /// region may have been resampled to a different image size, so this
  /// rescales `(x, y)` from image space back to the captured region (for
  /// example, when moving the mouse to the same on-screen point).
  ///
  /// See `ScreenshotCoordinateType` for how `coord_type` affects
  /// interpretation of `(x, y)`.
  pub fn to_machine_coordinates(
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
      .to_machine_coordinates(x, y, coord_type)
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Locate `concept` on this screenshot using the given grounding model and
  /// return the corresponding **global desktop coordinates** `[x, y]` in
  /// OS-native units (may be negative on multi-monitor setups; see
  /// [`Machine`]). The output can be fed
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
