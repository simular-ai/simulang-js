use napi::Error;
use napi_derive::napi;
use simulang_rs::traits::MouseTrait;
use simulang_rs::{
  Button as SimulangButton, Coordinate as SimulangCoordinate, Direction as SimulangDirection,
  InputController,
};

#[napi]
/// Represents a mouse button.
pub enum Button {
  Left,
  Middle,
  Right,
  Back,
  Forward,
  ScrollUp,
  ScrollDown,
  ScrollLeft,
  ScrollRight,
}

impl From<Button> for SimulangButton {
  fn from(value: Button) -> Self {
    match value {
      Button::Left => SimulangButton::Left,
      Button::Middle => SimulangButton::Middle,
      Button::Right => SimulangButton::Right,
      Button::Back => SimulangButton::Back,
      Button::Forward => SimulangButton::Forward,
      Button::ScrollUp => SimulangButton::ScrollUp,
      Button::ScrollDown => SimulangButton::ScrollDown,
      Button::ScrollLeft => SimulangButton::ScrollLeft,
      Button::ScrollRight => SimulangButton::ScrollRight,
    }
  }
}

#[napi]
/// The direction of a button event.
pub enum Direction {
  Press,
  Release,
  Click,
}

impl From<Direction> for SimulangDirection {
  fn from(value: Direction) -> Self {
    match value {
      Direction::Press => SimulangDirection::Press,
      Direction::Release => SimulangDirection::Release,
      Direction::Click => SimulangDirection::Click,
    }
  }
}

#[napi]
/// Specifies if coordinates are relative or absolute.
pub enum Coordinate {
  Abs,
  Rel,
}

impl From<Coordinate> for SimulangCoordinate {
  fn from(value: Coordinate) -> Self {
    match value {
      Coordinate::Abs => SimulangCoordinate::Abs,
      Coordinate::Rel => SimulangCoordinate::Rel,
    }
  }
}

#[napi]
/// Contains functions to control the mouse and to get the cursor position.
///
/// # Canonical coordinate space
///
/// This is the reference description of the coordinate space used throughout
/// the library. `Window.boundingBox()`, `AccessibilityNode.boundingBox()`,
/// `AccessibilityNode.fromPoint()`, screenshots, and grounding-model output
/// all live in this same space, so coordinates round-trip between them
/// **without conversion**.
///
/// Absolute coordinates live on the **global desktop**: top-left origin at
/// `(0, 0)` on the primary monitor, with the OS's native units. The unit is
/// **not** the same on every OS:
///
/// - **Windows** and **Linux** use **physical pixels** (raw hardware pixels).
/// - **macOS** uses **logical points** — on a 2× Retina display one point spans
///   two hardware pixels, so coordinates are half the physical-pixel count.
///
/// Within a single OS every function speaks that OS's unit, so the
/// round-trip guarantee holds; only code that crosses into a *different*
/// coordinate system (e.g. an Electron overlay measured in CSS pixels) needs to
/// account for the per-OS unit. These are also the native units the OS
/// input/accessibility APIs expect, so they are *not* the browser logical/CSS
/// pixel.
///
/// Monitors arranged to the left of or above the primary display contribute
/// **negative** coordinates, so callers should not assume `x, y >= 0`. Use
/// [`Screen.all`] / [`Screen.fromWindow`] to discover where the
/// addressable region actually is.
pub struct MouseController {
  inner: InputController,
}

impl Default for MouseController {
  fn default() -> Self {
    Self::new()
  }
}

#[napi]
impl MouseController {
  #[napi(constructor)]
  #[must_use]
  pub fn new() -> Self {
    Self {
      inner: InputController {},
    }
  }

  #[napi]
  /// Sends an individual mouse button event. You can use this for
  /// example to simulate a click of the left mouse key. Some of the
  /// buttons are specific to a platform.
  pub fn button(&mut self, button: Button, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .button(button.into(), direction.into())
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Move the mouse cursor to the specified x and y coordinates.
  ///
  /// You can specify absolute coordinates or relative from the current
  /// position.
  ///
  /// With absolute coordinates, `(x, y)` is in the global desktop space
  /// described on [`MouseController`]: top-left of the primary monitor is
  /// `(0, 0)`, OS-native units (physical pixels on Windows/Linux, logical
  /// points on macOS), and secondary monitors arranged above / to the left
  /// of the primary may have negative coordinates.
  ///
  /// With relative coordinates, a positive `x` moves the cursor `x`
  /// pixels to the right; a positive `y` moves it down.
  pub fn move_mouse(&mut self, x: i32, y: i32, coordinate: Coordinate) -> napi::Result<()> {
    self
      .inner
      .move_mouse(x, y, coordinate.into())
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Send a mouse scroll event.
  ///
  /// A positive length will result in scrolling down/right and negative
  /// ones up/left.
  pub fn scroll(&mut self, delta_x: i32, delta_y: i32) -> napi::Result<()> {
    self
      .inner
      .scroll(delta_x, delta_y)
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Get the location of the mouse in the canonical global-desktop space
  /// (OS-native units; see [`MouseController`]).
  pub fn location(&self) -> napi::Result<(i32, i32)> {
    self
      .inner
      .location()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
}
