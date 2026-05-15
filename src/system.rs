use napi::Error;
use napi_derive::napi;
use simulang_rs::System as SimulangSystem;
use simulang_rs::traits::SystemTrait;

use crate::app::App;

#[napi]
/// Provides system-wide operations.
pub struct System;

#[napi]
impl System {
  #[napi]
  /// Returns all available applications.
  pub fn list_apps() -> napi::Result<Vec<App>> {
    SimulangSystem::list_apps()
      .map(|apps| apps.into_iter().map(|inner| App { inner }).collect())
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Tries to find an installed app by fuzzy matching against the
  /// available app list.
  pub fn fuzzy_search(query: String) -> napi::Result<App> {
    SimulangSystem::fuzzy_search(&query)
      .map(|inner| App { inner })
      .map_err(Error::from_reason)
  }
}
