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
  /// Providers with missing credentials are skipped; throws if no provider
  /// remains available.
  #[allow(clippy::should_implement_trait)]
  pub fn default() -> napi::Result<Self> {
    simulang_rs::language_model::AskModel::default()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Resolve a model alias against the loaded config (e.g.
  /// `"openrouter_gpt_4o_mini"` / `"openrouter_claude_opus"` from the
  /// bundled `openrouter` provider, or any alias declared by a user provider).
  /// Throws if the alias is unknown.
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
  ///   base64 data URL in a model-supported format and sent as an
  ///   `image_url` chat content part. Pass `null` / omit for none.
  ///
  /// Returns the trimmed assistant response on success.
  #[allow(clippy::needless_pass_by_value)]
  pub fn ask(
    &self,
    prompt: String,
    text: Option<String>,
    images: Option<Vec<&Image>>,
  ) -> napi::Result<String> {
    let images_inner: Vec<&simulang_rs::Image> =
      images.map_or_default(|v| v.iter().map(|img| &img.inner).collect());
    self
      .inner
      .ask(&prompt, text.as_deref(), &images_inner)
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  #[must_use]
  /// LLM aliases whose provider entry sets `json_schema` to true, using the
  /// same auth filter and first-provider-wins rule as [`AskModel.byAlias`].
  /// [`AskModel.availableAliases`] still lists every LLM, including models
  /// that only serve [`AskModel.ask`].
  pub fn json_schema_aliases() -> Vec<String> {
    simulang_rs::language_model::AskModel::json_schema_aliases()
  }

  #[napi]
  /// Ask the model for one JSON value that matches `schema`.
  ///
  /// Same `prompt`, `text`, and `images` as [`AskModel.ask`]. `schema` is a
  /// JSON Schema document. A caller that has it as text parses it before
  /// calling. The request sends that document unchanged. `"answer"` is
  /// only the name the chat API requires on the schema; it is not a key
  /// in the reply. One call carries one schema.
  ///
  /// The schema is compiled locally before the request. The reply is
  /// parsed and checked against that same schema. A reply that is not
  /// JSON, or that does not match, throws. A document that is not valid
  /// JSON Schema throws and no request is sent.
  ///
  /// The provider is also asked for strict mode. The local check is what
  /// decides whether the value is returned. `additionalProperties` and
  /// `required` are sent exactly as the caller wrote them.
  ///
  /// Throws and sends no request when this model does not support it.
  /// [`AskModel.jsonSchemaAliases`] lists the models that do.
  #[allow(clippy::needless_pass_by_value)]
  pub fn ask_json(
    &self,
    prompt: String,
    text: Option<String>,
    images: Option<Vec<&Image>>,
    schema: serde_json::Value,
  ) -> napi::Result<serde_json::Value> {
    let images_inner: Vec<&simulang_rs::Image> =
      images.map_or_default(|v| v.iter().map(|img| &img.inner).collect());
    self
      .inner
      .ask_json(&prompt, text.as_deref(), &images_inner, &schema)
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}
