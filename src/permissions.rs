use napi_derive::napi;
use simulang_rs::local::Permissions;
use simulang_rs::traits::PermissionsTrait;

#[napi]
#[must_use]
pub fn has_screen_capture_permission() -> bool {
  Permissions::has_screen_capture_permission()
}
