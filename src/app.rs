use napi::Error;
use napi_derive::napi;
use simulang_rs::App as SimulangApp;
use simulang_rs::traits::{FocusPolicy as SimulangFocusPolicy, Visibility as SimulangVisibility};

use crate::instance::Instance;

#[napi]
/// An installed application on a machine, ready to be opened.
///
/// Obtain via [`Machine.app`], [`Machine.fuzzyApp`], [`Machine.apps`], or
/// [`Machine.defaultBrowser`]. The handle stays bound to the machine it
/// came from.
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
  #[napi(getter)]
  #[must_use]
  /// Returns the canonical app name used for fuzzy matching (Android: the
  /// package name).
  pub fn canonical_name(&self) -> Option<String> {
    self.inner.canonical_name().map(ToOwned::to_owned)
  }

  #[napi(getter)]
  #[must_use]
  /// Returns the launch target used to open the app: a name or path
  /// locally, the package name on Android.
  pub fn launch_target(&self) -> Option<String> {
    self.inner.launch_target().map(ToOwned::to_owned)
  }

  #[napi]
  /// Opens or switches to the application. If a URL is provided, the URL
  /// is opened with this app.
  ///
  /// `focus_policy` controls whether the app is allowed to become
  /// active/frontmost. Some applications (e.g. Chrome, Notes) ignore this
  /// request and steal focus regardless. `visibility` controls whether
  /// the app is launched hidden or shown; some applications (e.g. Chrome
  /// and other Chromium/Electron apps) ignore the hidden flag and launch
  /// visibly. Android's activity model always launches visible and
  /// focused.
  ///
  /// When `wait_for_load_complete` is true, blocks until the instance
  /// has a window (or the platform reports launch finished), up to
  /// twenty seconds. Desktop timeouts are logged; Android treats them as
  /// errors. Pass `false` for apps that never show a window. If the app
  /// is already running, its existing windows satisfy the wait
  /// immediately — this does not wait for whatever window or tab the
  /// call may add.
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
