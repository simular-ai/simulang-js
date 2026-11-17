use napi_derive::napi;
use simulang_rs::local::Capabilities;
use simulang_rs::traits::CapabilitiesTrait;

/// What stands in the way of a capability.
///
/// Decide on `kind`. `reason` is for a person to read, and its wording
/// changes between releases.
#[napi(object)]
pub struct CapabilityError {
  pub kind: CapabilityErrorKind,
  /// What stands in the way and, where there is one, the fix.
  pub reason: String,
}

impl From<simulang_rs::traits::CapabilityError> for CapabilityError {
  fn from(error: simulang_rs::traits::CapabilityError) -> Self {
    Self {
      kind: error.kind.into(),
      reason: error.reason,
    }
  }
}

/// The kinds of [`CapabilityError`]. Each kind has its own way out.
///
/// - `'permission'` (macOS): a permission is off for the app running this
///   process, Screen Recording or Accessibility. The person has to turn it
///   on.
/// - `'restart-needed'` (macOS): Accessibility was turned on after this
///   process started. The process has to restart.
/// - `'display-asleep'` (macOS): every attached display is asleep. macOS
///   draws nothing then: there is no screen to capture, and windows have no
///   place on one. It passes when a display wakes.
/// - `'no-display'`: no display is attached, as on a headless machine
///   without a virtual display.
/// - `'service-session'` (Windows): this process runs in session 0, where
///   services run.
/// - `'session-disconnected'` (Windows): the session is not connected to a
///   screen, such as a Remote Desktop session whose client went away.
/// - `'other-desktop'` (Windows): input goes to a desktop that is not the
///   user's: the session is locked, a UAC or Ctrl+Alt+Del screen is up, or a
///   screen saver runs. It passes when the user's desktop is back.
/// - `'wayland'` (Linux): the session is Wayland, which is not supported.
/// - `'display-unreachable'` (Linux): the X display can't be reached.
/// - `'accessibility-bus-unreachable'` (Linux): the AT-SPI bus, which
///   carries every app's accessibility tree, can't be reached.
#[napi(string_enum = "kebab-case")]
pub enum CapabilityErrorKind {
  Permission,
  RestartNeeded,
  DisplayAsleep,
  NoDisplay,
  ServiceSession,
  SessionDisconnected,
  OtherDesktop,
  Wayland,
  DisplayUnreachable,
  AccessibilityBusUnreachable,
}

impl From<simulang_rs::traits::CapabilityErrorKind> for CapabilityErrorKind {
  fn from(kind: simulang_rs::traits::CapabilityErrorKind) -> Self {
    use simulang_rs::traits::CapabilityErrorKind as Kind;
    match kind {
      Kind::Permission => Self::Permission,
      Kind::RestartNeeded => Self::RestartNeeded,
      Kind::DisplayAsleep => Self::DisplayAsleep,
      Kind::NoDisplay => Self::NoDisplay,
      Kind::ServiceSession => Self::ServiceSession,
      Kind::SessionDisconnected => Self::SessionDisconnected,
      Kind::OtherDesktop => Self::OtherDesktop,
      Kind::Wayland => Self::Wayland,
      Kind::DisplayUnreachable => Self::DisplayUnreachable,
      Kind::AccessibilityBusUnreachable => Self::AccessibilityBusUnreachable,
    }
  }
}

/// Whether screenshots will show what is on screen: `null`, or the
/// [`CapabilityError`] that stands in the way.
///
/// - macOS: Screen Recording is on for the app running this process.
/// - Windows: the process runs in the user's session (not session 0, where
///   services run), the session is connected (not a disconnected Remote
///   Desktop session), and the user's desktop takes input (the session isn't
///   locked and no UAC or Ctrl+Alt+Del screen is up).
/// - Linux: the X display can be reached and the session isn't Wayland.
///
/// On every OS, at least one display is attached and awake: a headless VM or
/// server has nothing to see (`'no-display'`), and neither has a Mac whose
/// displays all sleep (`'display-asleep'`).
///
/// Nothing here asks for a permission or opens a prompt. A check that can't
/// tell doesn't stop a run: it logs a warning and counts as fine.
#[napi(catch_unwind)]
#[must_use]
pub fn can_see() -> Option<CapabilityError> {
  Capabilities::can_see().err().map(Into::into)
}

/// Whether other apps' accessibility trees can be read and acted on
/// (pressing, setting values, focusing): `null`, or the [`CapabilityError`] that
/// stands in the way. A sleeping display is none.
///
/// - macOS: Accessibility is on for the app running this process.
/// - Windows: the process doesn't run in session 0.
/// - Linux: the AT-SPI bus can be reached.
#[napi(catch_unwind)]
#[must_use]
pub fn can_use_accessibility() -> Option<CapabilityError> {
  Capabilities::can_use_accessibility().err().map(Into::into)
}

/// Whether this process can drive other apps, so mouse and keyboard input
/// reaches them and actions on their elements work: `null`, or the
/// [`CapabilityError`] that stands in the way. Needs everything
/// [`canUseAccessibility`] needs too.
///
/// - macOS: Accessibility is on, as both `AXIsProcessTrusted` and the event
///   system read it. A process granted Accessibility after it started can
///   read accessibility trees but still be refused input; the error is
///   then a `'restart-needed'`.
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
/// On every OS, at least one display is attached and awake.
#[napi(catch_unwind)]
#[must_use]
pub fn can_control() -> Option<CapabilityError> {
  Capabilities::can_control().err().map(Into::into)
}
