use napi_derive::napi;
use simulang_rs::{
  Button as SimulangButton, Coordinate as SimulangCoordinate, Direction as SimulangDirection,
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
