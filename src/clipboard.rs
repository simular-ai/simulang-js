use napi::Error;
use napi_derive::napi;
use simulang_rs::Clipboard as SimulangClipboard;
use simulang_rs::traits::ClipboardTrait;

use crate::image::Image;

#[napi]
/// Contains functions to read and write the clipboard.
pub struct Clipboard {
  inner: SimulangClipboard,
}

impl Default for Clipboard {
  fn default() -> Self {
    Self {
      inner: SimulangClipboard,
    }
  }
}

#[napi]
impl Clipboard {
  #[napi(constructor)]
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  #[napi]
  /// Gets the string content from the clipboard.
  ///
  /// Returns the clipboard string if available, or `null` if the
  /// clipboard doesn't contain string data or is empty.
  pub fn get_string(&self) -> napi::Result<Option<String>> {
    self.inner.get_string().map_err(Error::from_reason)
  }

  #[napi]
  /// Gets the image content from the clipboard.
  ///
  /// Returns the clipboard image if available, or `null` if the
  /// clipboard doesn't contain image data.
  pub fn get_image(&self) -> napi::Result<Option<Image>> {
    self
      .inner
      .get_image()
      .map(|opt| opt.map(|inner| Image { inner }))
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Replaces the contents of the clipboard with the given string.
  ///
  /// Returns the previous string content, or `null` if the clipboard
  /// was empty or had no string data. The clipboard is not
  /// automatically restored; the caller can use the returned value to
  /// restore it if desired.
  // TODO: align the return type with the Rust API
  pub fn set_string(&self, value: String) -> napi::Result<Option<String>> {
    self
      .inner
      .set_string(&value)
      .map(|content| content.text)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Replaces the contents of the clipboard with the given image. For
  /// example used to copy a screenshot to the clipboard.
  ///
  /// Returns the previous string content, or `null` if the clipboard
  /// was empty or had no string data. The clipboard is not
  /// automatically restored; the caller can use the returned value to
  /// restore it if desired.
  // TODO: align the return type with the Rust API
  pub fn set_image(&self, image: &Image) -> napi::Result<Option<String>> {
    self
      .inner
      .set_image(&image.inner)
      .map(|content| content.text)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Clears all content from the clipboard, regardless of type.
  pub fn clear(&self) -> napi::Result<()> {
    self.inner.clear().map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Types text by pasting it from the clipboard.
  ///
  /// Saves the previous clipboard text and image (when present), sets
  /// the clipboard to the specified string, verifies it was set
  /// correctly, then simulates Command+V (or Ctrl+V) to paste it.
  /// After pasting, reapplies the saved snapshot (image is restored
  /// in preference to text when both were present). Other clipboard
  /// formats are not saved or restored.
  pub fn paste_text(&self, value: String) -> napi::Result<()> {
    self
      .inner
      .paste_text(&value)
      .map_err(|err| Error::from_reason(err.to_string()))
  }
}
