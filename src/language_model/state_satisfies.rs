use napi::Error;
use napi_derive::napi;

#[napi]
/// A dedicated state-satisfaction model that evaluates whether observed screen
/// state satisfies a natural-language condition.
///
/// This speaks Simular's `v1/perception/state_satisfies` endpoint. It is
/// separate from `AskModel`, which remains an OpenAI-compatible
/// chat-completions client.
pub struct StateSatisfiesModel {
  pub(crate) inner: simulang_rs::language_model::StateSatisfiesModel,
}

#[napi]
impl StateSatisfiesModel {
  #[napi(factory)]
  /// First state-satisfaction model advertised by the first capable provider
  /// in the loaded configuration whose credentials are currently available.
  /// Throws if no provider advertises the state-satisfies service.
  #[allow(clippy::should_implement_trait)]
  pub fn default() -> napi::Result<Self> {
    simulang_rs::language_model::StateSatisfiesModel::default()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Resolve a model alias against the loaded config. Throws if the alias is
  /// unknown.
  #[allow(clippy::needless_pass_by_value)]
  pub fn by_alias(alias: String) -> napi::Result<Self> {
    simulang_rs::language_model::StateSatisfiesModel::by_alias(&alias)
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi(factory)]
  /// Convenience shim for the bundled `simular_state_satisfies` alias.
  pub fn simular_state_satisfies() -> napi::Result<Self> {
    simulang_rs::language_model::StateSatisfiesModel::simular_state_satisfies()
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  #[must_use]
  /// Every state-satisfaction model alias accepted by `byAlias` on this
  /// machine, deduplicated and sorted alphabetically.
  pub fn available_aliases() -> Vec<String> {
    simulang_rs::language_model::StateSatisfiesModel::available_aliases()
  }

  #[napi(getter)]
  #[must_use]
  /// The provider-configured model identifier.
  pub fn name(&self) -> String {
    self.inner.name().to_string()
  }

  #[napi]
  /// Check that the provider credentials work. Throws if they do not.
  pub fn check_auth(&self) -> napi::Result<()> {
    self
      .inner
      .check_auth()
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Evaluate `condition` against the supplied accessibility text and raw
  /// screenshot base64.
  pub fn state_satisfies(
    &self,
    condition: String,
    text: String,
    image: String,
  ) -> napi::Result<bool> {
    self
      .inner
      .state_satisfies(&condition, &text, &image)
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}
