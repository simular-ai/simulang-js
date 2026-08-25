use napi::Error;
use napi_derive::napi;

use crate::audio::source::SamplesBuffer;

#[napi]
/// A queue of sounds playing through a [`Machine`]'s default audio
/// output.
///
/// Obtain via [`Machine.player`]. The handle owns its output device
/// connection: when it is garbage-collected playback stops. Appended
/// sources play back to back; playback starts immediately on append.
pub struct AudioPlayer {
  pub(crate) inner: simulang_rs::AudioPlayer,
}

#[napi]
impl AudioPlayer {
  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Decodes an audio file (WAV, MP3, FLAC, Vorbis/OGG, ...) on the
  /// **host** and appends it to the playback queue.
  pub fn append_file(&self, path: String) -> napi::Result<()> {
    self.inner.append_file(&path).map_err(Error::from_reason)
  }

  #[napi]
  /// Appends a sound to the end of the playback queue.
  pub fn append_samples(&self, buffer: &SamplesBuffer) {
    self.inner.append(buffer.inner.clone());
  }

  #[napi]
  /// Resumes playback of a paused player. No effect if not paused.
  pub fn play(&self) {
    self.inner.play();
  }

  #[napi]
  /// Pauses playback; resume with `play()`.
  pub fn pause(&self) {
    self.inner.pause();
  }

  #[napi]
  /// Stops the player by emptying the queue.
  pub fn stop(&self) {
    self.inner.stop();
  }

  #[napi(getter)]
  #[must_use]
  /// Volume multiplier; `1.0` is the unfiltered input.
  pub fn volume(&self) -> f64 {
    f64::from(self.inner.volume())
  }

  #[napi(setter)]
  /// Sets the volume multiplier; `1.0` is the unfiltered input.
  #[allow(clippy::cast_possible_truncation)]
  pub fn set_volume(&self, value: f64) {
    self.inner.set_volume(value as f32);
  }

  #[napi(getter)]
  #[must_use]
  /// Playback speed; changing it changes pitch by the same factor.
  pub fn speed(&self) -> f64 {
    f64::from(self.inner.speed())
  }

  #[napi(setter)]
  /// Sets the playback speed (and with it the pitch).
  #[allow(clippy::cast_possible_truncation)]
  pub fn set_speed(&self, value: f64) {
    self.inner.set_speed(value as f32);
  }

  #[napi(getter)]
  #[must_use]
  /// Whether the player is currently paused.
  pub fn is_paused(&self) -> bool {
    self.inner.is_paused()
  }

  #[napi]
  #[must_use]
  /// Whether the queue has no more sounds to play.
  pub fn empty(&self) -> bool {
    self.inner.empty()
  }

  #[napi]
  #[must_use]
  /// The number of sounds currently in the queue.
  #[allow(clippy::cast_possible_truncation, clippy::len_without_is_empty)]
  pub fn len(&self) -> u32 {
    self.inner.len() as u32
  }

  #[napi]
  /// Blocks the current thread until every queued sound has finished.
  pub fn sleep_until_end(&self) {
    self.inner.sleep_until_end();
  }

  #[napi]
  #[must_use]
  /// Playback position within the current sound in milliseconds,
  /// accounting for speed changes and seeks.
  ///
  /// Example: if you apply a speedup of *2* to a source and
  /// `getPos()` returns *5000* then the position in the recording
  /// is *10 seconds* from its start.
  #[allow(clippy::cast_precision_loss)]
  pub fn get_pos(&self) -> f64 {
    self.inner.get_pos().as_secs_f64() * 1000.0
  }

  #[napi]
  /// Removes all queued sounds and pauses the player.
  pub fn clear(&self) {
    self.inner.clear();
  }

  #[napi]
  /// Skips to the next sound in the queue.
  pub fn skip_one(&self) {
    self.inner.skip_one();
  }

  #[napi]
  /// Seeks within the current sound to the given position (in
  /// milliseconds), saturating at its end when the duration is known.
  /// For example given a source that reports a total duration of 42
  /// seconds, calling `trySeek(60000)` will seek to 42 seconds.
  ///
  /// This blocks between 0 and ~5 milliseconds.
  pub fn try_seek(&self, pos_ms: f64) -> napi::Result<()> {
    let pos = std::time::Duration::from_secs_f64(pos_ms / 1000.0);
    self.inner.try_seek(pos).map_err(Error::from_reason)
  }
}
