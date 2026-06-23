use napi::Error;
use napi_derive::napi;

use crate::image::Image;

#[napi]
/// A free-form chat-completions LLM (optionally vision-capable) — the JS
/// analogue of the `ask` primitive.
///
/// Given a `prompt`, optional accessibility/structural `text`, and zero or
/// more `images`, [`AskModel.ask`] returns the model's response as a plain
/// string. The wire format is OpenAI-compatible chat completions, so any
/// provider config that points at such an endpoint works.
pub struct AskModel {
  pub(crate) inner: simulang_rs::language_model::AskModel,
}

#[napi]
impl AskModel {
  #[napi(factory)]
  /// First LLM model advertised by the first LLM-capable provider in the
  /// loaded configuration whose credentials are currently available.
  /// Throws if no provider in the loaded config advertises an LLM service.
  #[allow(clippy::should_implement_trait)]
  pub fn default() -> napi::Result<Self> {
    simulang_rs::language_model::AskModel::default()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Resolve a model alias against the loaded config (e.g.
  /// `"openrouter_gpt_4o_mini"` from the bundled `openrouter` provider, or
  /// any alias declared by a user provider). Throws if the alias is unknown.
  #[allow(clippy::needless_pass_by_value)]
  pub fn by_alias(alias: String) -> napi::Result<Self> {
    simulang_rs::language_model::AskModel::by_alias(&alias)
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  #[must_use]
  /// Every LLM model alias accepted by `byAlias` on this machine,
  /// deduplicated and sorted alphabetically. Use to discover what aliases
  /// the loaded config (bundled defaults plus any user provider files)
  /// advertises.
  pub fn available_aliases() -> Vec<String> {
    simulang_rs::language_model::AskModel::available_aliases()
  }

  #[napi(getter)]
  #[must_use]
  /// The wire-level model identifier sent in the request body.
  pub fn name(&self) -> String {
    self.inner.name().to_string()
  }

  #[napi]
  /// Check that the API key works. Throws if it doesn't.
  ///
  /// Call right after creating the model so a bad key fails fast,
  /// before any UI automation has had a chance to steal focus:
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
  /// Ask the model a question, optionally grounded in accessibility text
  /// and/or images.
  ///
  /// - `prompt`: the question or task to answer.
  /// - `text`: optional accessibility-tree (or any) text included as
  ///   structural context. Pass `null` / omit to skip.
  /// - `images`: zero or more images to attach. Each is encoded as a
  ///   base64 data URL and sent as an `image_url` chat content part. Pass
  ///   `null` / omit for none.
  ///
  /// Returns the trimmed assistant response on success.
  #[allow(clippy::needless_pass_by_value)]
  pub fn ask(
    &self,
    prompt: String,
    text: Option<String>,
    images: Option<Vec<&Image>>,
  ) -> napi::Result<String> {
    let images_inner: Vec<&simulang_rs::Image> = images
      .as_ref()
      .map(|v| v.iter().map(|img| &img.inner).collect())
      .unwrap_or_default();
    self
      .inner
      .ask(&prompt, text.as_deref(), &images_inner)
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}
