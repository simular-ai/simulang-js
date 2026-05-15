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
/// Contains functions to control the mouse and to get the location of
/// the cursor. A cartesian coordinate system is used for specifying
/// coordinates. The origin is located in the top-left corner of the
/// current screen, with positive values extending along the axes down
/// and to the right of the origin point and it is measured in pixels.
/// The same coordinate system is used on all operating systems.
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
  /// If you use absolute coordinates, the top left corner of your
  /// monitor screen is x=0 y=0. Move the cursor down the screen by
  /// increasing the y and to the right by increasing x coordinate.
  ///
  /// If you use relative coordinates, a positive x value moves the
  /// mouse cursor `x` pixels to the right. A negative value for `x`
  /// moves the mouse cursor to the left. A positive value of y moves
  /// the mouse cursor down, a negative one moves the mouse cursor up.
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
  /// Get the location of the mouse in pixels.
  pub fn location(&self) -> napi::Result<(i32, i32)> {
    self
      .inner
      .location()
      .map_err(|err| Error::from_reason(err.to_string()))
  }
}
