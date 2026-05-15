use napi::Error;
use napi_derive::napi;

use crate::app::{FocusPolicy, Visibility};

#[napi]
#[allow(clippy::needless_pass_by_value)]
/// Legacy helper used by Electron to open an app or URL.
pub fn legacy_open(
  app: Option<String>,
  url: Option<String>,
  focus_policy: FocusPolicy,
  visibility: Visibility,
) -> napi::Result<()> {
  simulang_rs::legacy::open(app, url, focus_policy.into(), visibility.into())
    .map_err(Error::from_reason)
}

#[napi]
/// Legacy helper used by Electron to capture a screenshot as base64.
pub fn legacy_take_screenshot(
  needs_compression: bool,
  shrink_to1080p: bool,
) -> napi::Result<String> {
  simulang_rs::legacy::take_screenshot(needs_compression, shrink_to1080p)
    .map_err(Error::from_reason)
}
