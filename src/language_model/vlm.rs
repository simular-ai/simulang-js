use napi::Error;
use napi::bindgen_prelude::Either;
use napi_derive::napi;

use crate::image::Image;
use crate::screenshot::Screenshot;

#[napi]
/// A visual-language grounding model that can locate concepts on images.
pub struct GroundingModel {
  pub(crate) inner: simulang_rs::language_model::GroundingModel,
}

#[napi]
impl GroundingModel {
  #[napi(factory)]
  /// First VLM model advertised by the first VLM provider in the loaded
  /// config. Throws if no provider in the loaded config advertises a VLM
  /// service.
  #[allow(clippy::should_implement_trait)]
  pub fn default() -> napi::Result<Self> {
    simulang_rs::language_model::GroundingModel::default()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Resolve a model alias against the loaded config (e.g. `"ui_venus_30b"`,
  /// `"ui_tars_7b"`, or any alias declared by a user provider). Throws if
  /// the alias is unknown.
  #[allow(clippy::needless_pass_by_value)]
  pub fn by_alias(alias: String) -> napi::Result<Self> {
    simulang_rs::language_model::GroundingModel::by_alias(&alias)
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Convenience shim for the bundled `ui_tars_7b` alias. Throws if no
  /// provider advertises that alias.
  pub fn ui_tars_7b() -> napi::Result<Self> {
    simulang_rs::language_model::GroundingModel::ui_tars_7b()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Convenience shim for the bundled `ui_venus_30b` alias. Throws if no
  /// provider advertises that alias.
  pub fn ui_venus_30b() -> napi::Result<Self> {
    simulang_rs::language_model::GroundingModel::ui_venus_30b()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  #[must_use]
  /// Every VLM model alias accepted by `byAlias` on this machine, deduplicated
  /// and sorted alphabetically. Use to discover what aliases the loaded config
  /// (bundled defaults plus any user provider files advertises.
  pub fn available_aliases() -> Vec<String> {
    simulang_rs::language_model::GroundingModel::available_aliases()
  }

  #[napi(getter)]
  #[must_use]
  /// The wire-level model identifier sent in the request body.
  pub fn name(&self) -> String {
    self.inner.name().to_string()
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Locate `concept` on `target` and return zero-based pixel coordinates
  /// `[x, y]`:
  ///
  /// * If `target` is an `Image`, coordinates are in **image-space**.
  ///   Normalized model outputs are scaled linearly onto
  ///   `[0, width - 1]` and `[0, height - 1]`.
  /// * If `target` is a `Screenshot`, coordinates are in **global physical
  ///   screen space** — suitable to feed directly into primitives that
  ///   expect global screen coordinates (e.g. `moveTo`, `clickAt`).
  ///
  /// Equivalent to `target.ground(model, concept)`.
  pub fn ground(
    &self,
    target: Either<&Image, &Screenshot>,
    concept: String,
  ) -> napi::Result<(i32, i32)> {
    match target {
      Either::A(image) => self.inner.ground(&image.inner, &concept),
      Either::B(screenshot) => self.inner.ground(&screenshot.inner, &concept),
    }
    .map_err(|e| Error::from_reason(e.to_string()))
  }
}
