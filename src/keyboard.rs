use napi::Error;
use napi_derive::napi;
use simulang_rs::traits::KeyboardTrait;
use simulang_rs::{InputController, Key as SimulangKey};

use crate::key::Key;
use crate::mouse::Direction;

#[napi]
/// Contains functions to simulate key presses/releases and to input
/// text.
///
/// For entering text, the `text` method is best. If you want to enter
/// a key without having to worry about the layout or the keymap, use
/// the `key` method. If you want a specific (physical) key to be
/// pressed (e.g WASD for games), use the `raw` method. The resulting
/// keysym will depend on the layout/keymap.
pub struct KeyboardController {
  inner: InputController,
}

impl Default for KeyboardController {
  fn default() -> Self {
    Self::new()
  }
}

#[napi]
impl KeyboardController {
  #[napi(constructor)]
  #[must_use]
  pub fn new() -> Self {
    Self {
      inner: InputController {},
    }
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Enter the text. You can use unicode here like: ❤️. This works regardless
  /// of the current keyboard layout. You cannot use this function for entering
  /// shortcuts or something similar. For shortcuts, use the `key` method instead.
  pub fn text(&mut self, text: String) -> napi::Result<()> {
    self
      .inner
      .text(&text)
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Sends an individual key event. It will enter the keysym (virtual
  /// key). Have a look at the `raw` method, if you want to enter a
  /// keycode.
  ///
  /// Some of the keys are specific to a platform.
  pub fn key(&mut self, key: Key, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .key(key.try_into()?, direction.into())
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Sends an individual Unicode key event. Provide a single character.
  pub fn key_unicode(&mut self, value: String, direction: Direction) -> napi::Result<()> {
    let mut chars = value.chars();
    let Some(ch) = chars.next() else {
      return Err(Error::from_reason("key_unicode requires a character."));
    };
    if chars.next().is_some() {
      return Err(Error::from_reason(
        "key_unicode expects a single character.",
      ));
    }
    self
      .inner
      .key(SimulangKey::Unicode(ch), direction.into())
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Sends a key event for a raw, platform-specific key value.
  pub fn key_other(&mut self, value: u32, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .key(SimulangKey::Other(value), direction.into())
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Sends a raw keycode. The keycode may or may not be mapped on the
  /// current layout. You have to make sure of that yourself. This can
  /// be useful if you want to simulate a press regardless of the layout
  /// (WASD on video games). Have a look at the `key` method, if you
  /// just want to enter a specific key and don't want to worry about
  /// the layout/keymap. Windows only: If you want to enter the keycode
  /// (scancode) of an extended key, you need to set the high byte for
  /// the extended key too. You can for example do:
  /// `raw(0xE01D, Direction.Click)` to simulate `RControl`.
  pub fn raw(&mut self, keycode: u16, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .raw(keycode, direction.into())
      .map_err(|err| Error::from_reason(err.to_string()))
  }
}
