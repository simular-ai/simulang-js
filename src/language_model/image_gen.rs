use napi::Error;
use napi_derive::napi;

use simulang_rs::language_model::{
  ImageAspectRatio as SimulangImageAspectRatio, ImageQuality as SimulangImageQuality,
};

use crate::image::Image;

#[napi]
/// Output image quality / model tier for [`ImageGenModel.generate`].
///
/// The image-gen endpoint selects the underlying model from this value, so it
/// is a per-request parameter rather than a config-resolved model name.
pub enum ImageQuality {
  /// Faster, cheaper tier. Default.
  Fast,
  /// Higher-fidelity tier.
  Pro,
}

impl From<ImageQuality> for SimulangImageQuality {
  fn from(value: ImageQuality) -> Self {
    match value {
      ImageQuality::Fast => Self::Fast,
      ImageQuality::Pro => Self::Pro,
    }
  }
}

#[napi]
/// Aspect ratio of the generated image.
pub enum ImageAspectRatio {
  /// `1:1` square. Default.
  Square,
  /// `16:9` widescreen landscape.
  Landscape16x9,
  /// `9:16` tall portrait.
  Portrait9x16,
  /// `4:3` landscape.
  Landscape4x3,
  /// `3:4` portrait.
  Portrait3x4,
  /// `3:2` landscape.
  Landscape3x2,
  /// `2:3` portrait.
  Portrait2x3,
}

impl From<ImageAspectRatio> for SimulangImageAspectRatio {
  fn from(value: ImageAspectRatio) -> Self {
    match value {
      ImageAspectRatio::Square => Self::Square,
      ImageAspectRatio::Landscape16x9 => Self::Landscape16x9,
      ImageAspectRatio::Portrait9x16 => Self::Portrait9x16,
      ImageAspectRatio::Landscape4x3 => Self::Landscape4x3,
      ImageAspectRatio::Portrait3x4 => Self::Portrait3x4,
      ImageAspectRatio::Landscape3x2 => Self::Landscape3x2,
      ImageAspectRatio::Portrait2x3 => Self::Portrait2x3,
    }
  }
}

#[napi]
/// A model for generating images from a text prompt (with optional reference
/// images), speaking Simular's `v1/image-gen` wire contract.
///
/// This targets a dedicated Simular endpoint rather than an `OpenAI`-compatible
/// one; the endpoint picks the concrete backend model from the [`ImageQuality`]
/// argument, so the resolved model `name` is nominal (used only for `name`).
/// Reference images are downscaled / transcoded and size-checked against the
/// model's configured image limits before sending, exactly as the VLM/LLM image
/// paths do.
pub struct ImageGenModel {
  pub(crate) inner: simulang_rs::language_model::ImageGenModel,
}

#[napi]
impl ImageGenModel {
  #[napi(factory)]
  /// First image-gen model advertised by the first capable provider in the
  /// loaded configuration whose credentials are currently available.
  /// Providers that advertise the service but lack working credentials are
  /// skipped. Throws only if no provider is left after that filter.
  #[allow(clippy::should_implement_trait)]
  pub fn default() -> napi::Result<Self> {
    simulang_rs::language_model::ImageGenModel::default()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Resolve a model alias against the loaded configuration.
  ///
  /// The alias is looked up across every loaded provider in alphabetical
  /// order; the first provider that advertises this alias wins. Throws if the
  /// alias is unknown.
  #[allow(clippy::needless_pass_by_value)]
  pub fn by_alias(alias: String) -> napi::Result<Self> {
    simulang_rs::language_model::ImageGenModel::by_alias(&alias)
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Convenience shim for the bundled Simular image-gen alias.
  pub fn simular_image_gen() -> napi::Result<Self> {
    simulang_rs::language_model::ImageGenModel::simular_image_gen()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  #[must_use]
  /// Every image-gen model alias advertised by the loaded configuration,
  /// deduplicated and sorted alphabetically. Use to discover what `byAlias`
  /// will accept on the current machine.
  pub fn available_aliases() -> Vec<String> {
    simulang_rs::language_model::ImageGenModel::available_aliases()
  }

  #[napi(getter)]
  #[must_use]
  /// The model identifier from provider configuration. Nominal — the
  /// endpoint selects the concrete model from the [`ImageQuality`] argument.
  pub fn name(&self) -> String {
    self.inner.name().to_string()
  }

  #[napi]
  /// Check that provider credentials work. Throws (and logs a warning) if they
  /// do not.
  ///
  /// Call right after creating the model so a bad key fails fast:
  ///
  /// ```ts
  /// try { model.checkAuth() } catch { process.exit(1) }
  /// ```
  pub fn check_auth(&self) -> napi::Result<()> {
    self
      .inner
      .check_auth()
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  /// Generate an image from `prompt`.
  ///
  /// - `prompt`: a description of the image to generate.
  /// - `quality`: selects the model tier; defaults to `ImageQuality.Fast`.
  /// - `aspectRatio`: the output shape; defaults to `ImageAspectRatio.Square`.
  /// - `images`: reference images (count capped by the model's configured
  ///   `max_images_per_request` image limit) that condition the result; each is
  ///   downscaled / transcoded to a supported format and size-checked before
  ///   sending. Pass `null` / omit for none.
  ///
  /// Returns a `[image, description]` tuple: the first generated [`Image`] and
  /// the model's text description (an empty string when the model returns none).
  #[allow(clippy::needless_pass_by_value)]
  pub fn generate(
    &self,
    prompt: String,
    quality: Option<ImageQuality>,
    aspect_ratio: Option<ImageAspectRatio>,
    images: Option<Vec<&Image>>,
  ) -> napi::Result<(Image, String)> {
    let quality = quality.unwrap_or(ImageQuality::Fast).into();
    let aspect_ratio = aspect_ratio.unwrap_or(ImageAspectRatio::Square).into();
    let references_inner: Vec<&simulang_rs::Image> = images
      .as_ref()
      .map(|v| v.iter().map(|img| &img.inner).collect())
      .unwrap_or_default();
    let (image, description) = self
      .inner
      .generate(&prompt, quality, aspect_ratio, &references_inner)
      .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok((Image { inner: image }, description))
  }
}
