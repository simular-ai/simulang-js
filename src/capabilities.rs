use napi_derive::napi;
use simulang_rs::local::Capabilities;
use simulang_rs::traits::CapabilitiesTrait;

/// Whether screenshots will show what is on screen: `null`, or what stands
/// in the way and, where there is one, the fix.
///
/// - macOS: Screen Recording is on for the app running this process.
/// - Windows: the process runs in the user's session (not session 0, where
///   services run), the session is connected (not a disconnected Remote
///   Desktop session), and the user's desktop takes input (the session isn't
///   locked and no UAC or Ctrl+Alt+Del screen is up).
/// - Linux: the X display can be reached and the session isn't Wayland.
///
/// On every OS, a display is attached: a headless VM or server has nothing
/// to see.
///
/// Nothing here asks for a permission or opens a prompt. A check that can't
/// tell doesn't stop a run: it logs a warning and counts as fine.
#[napi(catch_unwind)]
#[must_use]
pub fn can_see() -> Option<String> {
  Capabilities::can_see().err()
}

/// Whether other apps' accessibility trees can be read and acted on
/// (pressing, setting values, focusing): `null`, or what stands in the way.
///
/// - macOS: Accessibility is on for the app running this process.
/// - Windows: the process doesn't run in session 0.
/// - Linux: the AT-SPI bus can be reached.
#[napi(catch_unwind)]
#[must_use]
pub fn can_use_accessibility() -> Option<String> {
  Capabilities::can_use_accessibility().err()
}

/// Whether this process can drive other apps, so mouse and keyboard input
/// reaches them and actions on their elements work: `null`, or what stands
/// in the way. Needs everything [`canUseAccessibility`] needs too.
///
/// - macOS: Accessibility is on, as both `AXIsProcessTrusted` and the event
///   system read it. A process granted Accessibility after it started can
///   read accessibility trees but still be refused input; the reason then
///   says to restart it.
/// - Windows: the same as [`canSee`]. Two things that stand in the way of
///   some runs are logged as warnings on the first call, not returned,
///   because they depend on what the run meets: an app running as
///   administrator refuses input and element actions from a process that is
///   neither elevated nor has `UIAccess` (`uiAccess="true"` in the manifest of
///   a signed binary in a secure location, which must be the exe hosting
///   this module, such as Electron); and where UAC prompts open on the
///   secure desktop for this process (the default, unless it has `UIAccess`
///   and the policy "Allow `UIAccess` applications to prompt for elevation
///   without using the secure desktop" is on), no process can see or control
///   them, so a run pauses there until the person answers.
/// - Linux: the same as [`canSee`], and the AT-SPI bus can be reached.
///
/// On every OS, a display is attached.
#[napi(catch_unwind)]
#[must_use]
pub fn can_control() -> Option<String> {
  Capabilities::can_control().err()
}
