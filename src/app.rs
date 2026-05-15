use napi::Error;
use napi_derive::napi;
use simulang_rs::App as SimulangApp;
use simulang_rs::traits::{
  AppTrait, FocusPolicy as SimulangFocusPolicy, Visibility as SimulangVisibility,
};

use crate::instance::Instance;

#[napi]
/// Represents an application that can be opened.
pub struct App {
  pub(crate) inner: SimulangApp,
}

#[napi]
/// Focus behavior when opening an application.
pub enum FocusPolicy {
  /// Request launch without forcing activation/frontmost focus; some
  /// applications may still steal focus.
  DoNotSteal,
  /// Allow the launched app to become active/frontmost.
  Steal,
}

impl From<FocusPolicy> for SimulangFocusPolicy {
  fn from(value: FocusPolicy) -> Self {
    match value {
      FocusPolicy::DoNotSteal => SimulangFocusPolicy::DoNotSteal,
      FocusPolicy::Steal => SimulangFocusPolicy::Steal,
    }
  }
}

#[napi]
/// Visibility behavior when opening an application.
pub enum Visibility {
  /// Launch hidden when supported by the platform.
  Hidden,
  /// Launch in a visible state.
  Show,
}

impl From<Visibility> for SimulangVisibility {
  fn from(value: Visibility) -> Self {
    match value {
      Visibility::Hidden => SimulangVisibility::Hidden,
      Visibility::Show => SimulangVisibility::Show,
    }
  }
}

#[napi]
impl App {
  #[napi(factory)]
  #[allow(clippy::needless_pass_by_value)]
  /// Get the app by exact name. No fuzzy search is performed.
  pub fn exact_name(name: String) -> napi::Result<Self> {
    SimulangApp::exact_name(&name)
      .map(|inner| Self { inner })
      .map_err(Error::from_reason)
  }

  #[napi(getter)]
  #[must_use]
  /// Returns the canonical app name used for fuzzy matching.
  pub fn canonical_name(&self) -> Option<String> {
    self.inner.canonical_name().map(ToOwned::to_owned)
  }

  #[napi(getter)]
  #[must_use]
  /// Returns the launch target used to open the app (name or path).
  pub fn launch_target(&self) -> Option<String> {
    self.inner.launch_target().map(ToOwned::to_owned)
  }

  #[napi(factory)]
  /// Returns a handle to the system's default browser.
  pub fn default_browser() -> napi::Result<Self> {
    SimulangApp::default_browser()
      .map(|inner| Self { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Checks if the app exists.
  pub fn exists(app: String) -> napi::Result<bool> {
    SimulangApp::exists(&app).map_err(Error::from_reason)
  }

  #[napi]
  /// Opens or switches to an application. If a URL is provided, the URL is
  /// opened with the specified app.
  ///
  /// `focus_policy` controls whether the app is allowed to become
  /// active/frontmost. Some applications (e.g. Chrome, Notes) ignore this
  /// request and steal focus regardless.
  ///
  /// `visibility` controls whether the app is launched hidden or shown.
  /// Some applications (e.g. Chrome and other Chromium/Electron apps) ignore
  /// the hidden flag and launch visibly, potentially stealing focus. In that
  /// case the app is hidden explicitly after launch.
  ///
  /// When `wait_for_load_complete` is true, this blocks for a short, fixed
  /// delay to allow the app or URL to become responsive before returning.
  pub fn open(
    &self,
    url: Option<String>,
    focus_policy: FocusPolicy,
    visibility: Visibility,
    wait_for_load_complete: bool,
  ) -> napi::Result<Instance> {
    let url = url
      .map(|url| {
        simulang_rs::Url::parse(&url)
          .map_err(|err| Error::from_reason(format!("Invalid url '{url}': {err}")))
      })
      .transpose()?;
    self
      .inner
      .open(
        url,
        focus_policy.into(),
        visibility.into(),
        wait_for_load_complete,
      )
      .map(|inner| Instance { inner })
      .map_err(Error::from_reason)
  }
}
