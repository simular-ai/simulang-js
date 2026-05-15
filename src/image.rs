use std::path::Path;

use napi::Error;
use napi_derive::napi;
use simulang_rs::Image as SimulangImage;
use simulang_rs::traits::ImageTrait;

use crate::language_model::vlm::GroundingModel;

#[napi]
/// Represents an image.
pub struct Image {
  pub(crate) inner: SimulangImage,
}

#[napi]
impl Image {
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
  pub fn add_grid(&mut self, width: u16, height: u16) -> napi::Result<()> {
    self
      .inner
      .add_grid(width, height)
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
  /// Returns the image dimensions as `[width, height]` in pixels.
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

  #[napi(factory)]
  #[allow(clippy::needless_pass_by_value)]
  /// Decodes a base64 image string into an image.
  ///
  /// Accepts either a raw base64 payload or a data URL such as
  /// `data:image/png;base64,...`, `data:image/jpeg;base64,...`, or
  /// `data:image/jpg;base64,...`.
  pub fn from_base64(base64: String) -> napi::Result<Self> {
    SimulangImage::from_base64(&base64)
      .map(|inner| Self { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Locate `concept` on this image using the given grounding model and
  /// return absolute zero-based pixel coordinates `[x, y]`.
  ///
  /// Equivalent to `model.ground(image, concept)`
  pub fn ground(&self, model: &GroundingModel, concept: String) -> napi::Result<(i32, i32)> {
    model
      .inner
      .ground(&self.inner, &concept)
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}
