use napi::Error;
use napi_derive::napi;

use crate::audio::source::SamplesBuffer;

#[napi]
/// A speech-to-text model that can transcribe audio.
pub struct SttModel {
  pub(crate) inner: simulang_rs::language_model::SttModel,
}

#[napi]
impl SttModel {
  #[napi(factory)]
  /// First STT model advertised by the first STT provider in the loaded
  /// config. Throws if no provider in the loaded config advertises an STT
  /// service.
  #[allow(clippy::should_implement_trait)]
  pub fn default() -> napi::Result<Self> {
    simulang_rs::language_model::SttModel::default()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Resolve a model alias against the loaded config (e.g.
  /// `"whisper_large_v3_turbo"`, `"whisper_large_v3"`, or any alias declared
  /// by a user provider). Throws if the alias is unknown.
  #[allow(clippy::needless_pass_by_value)]
  pub fn by_alias(alias: String) -> napi::Result<Self> {
    simulang_rs::language_model::SttModel::by_alias(&alias)
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Whisper Large V3 Turbo — fast, slightly less accurate. Convenience
  /// shim for the bundled `whisper_large_v3_turbo` alias.
  pub fn whisper_large_v3_turbo() -> napi::Result<Self> {
    simulang_rs::language_model::SttModel::whisper_large_v3_turbo()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Whisper Large V3 — highest accuracy. Convenience shim for the bundled
  /// `whisper_large_v3` alias.
  pub fn whisper_large_v3() -> napi::Result<Self> {
    simulang_rs::language_model::SttModel::whisper_large_v3()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  #[must_use]
  /// Every STT model alias accepted by `byAlias` on this machine, deduplicated
  /// and sorted alphabetically. Use to discover what aliases the loaded config
  /// (bundled defaults plus any user provider files) advertises.
  pub fn available_aliases() -> Vec<String> {
    simulang_rs::language_model::SttModel::available_aliases()
  }

  #[napi(getter)]
  #[must_use]
  /// The wire-level model identifier sent in the request body.
  pub fn name(&self) -> String {
    self.inner.name().to_string()
  }

  #[napi]
  /// Transcribe a single audio chunk and return the recognised text.
  pub fn transcribe(&self, audio: &SamplesBuffer) -> napi::Result<String> {
    self
      .inner
      .transcribe(&audio.inner)
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}
