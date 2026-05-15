use napi::Error;
use napi_derive::napi;

use crate::audio::source::SamplesBuffer;

#[napi]
/// Handle to an open audio output device. Must be kept alive for
/// playback to continue — when dropped all associated `Player`s stop
/// producing sound.
pub struct AudioOutput {
  inner: simulang_rs::rodio::MixerDeviceSink,
}

#[napi]
impl AudioOutput {
  #[napi(factory)]
  /// Opens the default audio output device with its default
  /// configuration. If that fails, tries alternative configurations
  /// and non-default output devices. Returns the first configuration
  /// that succeeds. If all attempts fail, returns the initial error.
  pub fn open_default() -> napi::Result<Self> {
    let mut handle = simulang_rs::DeviceSinkBuilder::open_default_sink()
      .map_err(|e| Error::from_reason(e.to_string()))?;
    handle.log_on_drop(false);
    Ok(Self { inner: handle })
  }

  #[napi]
  #[must_use]
  /// Creates a new `Player` attached to this output.
  pub fn create_player(&self) -> Player {
    Player {
      inner: simulang_rs::Player::connect_new(self.inner.mixer()),
    }
  }
}

#[napi]
/// Handle to a device that outputs sounds.
///
/// Dropping the `Player` (prevent this by holding a reference) stops
/// all its sounds.
pub struct Player {
  inner: simulang_rs::Player,
}

#[napi]
impl Player {
  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Decodes an audio file and appends it to the playback queue.
  ///
  /// Supports WAV, MP3, FLAC, Vorbis/OGG, and other formats
  /// depending on build features.
  pub fn append_file(&self, path: String) -> napi::Result<()> {
    let file =
      std::fs::File::open(&path).map_err(|e| Error::from_reason(format!("{path}: {e}")))?;
    let source =
      simulang_rs::Decoder::try_from(file).map_err(|e| Error::from_reason(e.to_string()))?;
    self.inner.append(source);
    Ok(())
  }

  #[napi]
  /// Appends a sound to the queue of sounds to play.
  pub fn append_samples(&self, buffer: &SamplesBuffer) {
    self.inner.append(buffer.inner.clone());
  }

  #[napi]
  /// Resumes playback of a paused player. No effect if not paused.
  pub fn play(&self) {
    self.inner.play();
  }

  #[napi]
  /// Pauses playback of this player.
  ///
  /// No effect if already paused. A paused player can be resumed with
  /// `play()`.
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
  /// Volume of the sound.
  ///
  /// The value `1.0` is the "normal" volume (unfiltered input). Any
  /// value other than `1.0` will multiply each sample by this value.
  pub fn volume(&self) -> f64 {
    f64::from(self.inner.volume())
  }

  #[napi(setter)]
  /// Sets the volume of the sound.
  ///
  /// The value `1.0` is the "normal" volume (unfiltered input). Any
  /// value other than `1.0` will multiply each sample by this value.
  #[allow(clippy::cast_possible_truncation)]
  pub fn set_volume(&self, value: f64) {
    self.inner.set_volume(value as f32);
  }

  #[napi(getter)]
  #[must_use]
  /// Playback speed of the sound.
  ///
  /// Increasing the speed will increase the pitch by the same factor.
  /// For example, speed `0.5` halves the frequency (lowering pitch)
  /// and speed `2` doubles it (raising pitch). Changes in speed
  /// affect the total duration inversely.
  pub fn speed(&self) -> f64 {
    f64::from(self.inner.speed())
  }

  #[napi(setter)]
  /// Changes the play speed of the sound. Does not adjust the
  /// samples, only the playback speed.
  #[allow(clippy::cast_possible_truncation)]
  pub fn set_speed(&self, value: f64) {
    self.inner.set_speed(value as f32);
  }

  #[napi(getter)]
  #[must_use]
  /// Whether the player is currently paused. Players can be paused
  /// and resumed using `pause()` and `play()`.
  pub fn is_paused(&self) -> bool {
    self.inner.is_paused()
  }

  #[napi]
  #[must_use]
  /// Returns `true` if this player has no more sounds to play.
  pub fn empty(&self) -> bool {
    self.inner.empty()
  }

  #[napi]
  #[must_use]
  /// Returns the number of sounds currently in the queue.
  #[allow(clippy::cast_possible_truncation, clippy::len_without_is_empty)]
  pub fn len(&self) -> u32 {
    self.inner.len() as u32
  }

  #[napi]
  /// Sleeps the current thread until the sound ends.
  pub fn sleep_until_end(&self) {
    self.inner.sleep_until_end();
  }

  #[napi]
  #[must_use]
  /// Returns the position of the sound that's being played, in
  /// milliseconds.
  ///
  /// This takes into account any speedup or delay applied.
  ///
  /// Example: if you apply a speedup of *2* to a source and
  /// `getPos()` returns *5000* then the position in the recording
  /// is *10 seconds* from its start.
  #[allow(clippy::cast_precision_loss)]
  pub fn get_pos(&self) -> f64 {
    self.inner.get_pos().as_secs_f64() * 1000.0
  }

  #[napi]
  /// Removes all currently loaded sources from the player and pauses
  /// it.
  pub fn clear(&self) {
    self.inner.clear();
  }

  #[napi]
  /// Skips to the next source in the player.
  ///
  /// If there are more sources appended to the player at the time,
  /// it will play the next one. Otherwise, the player will finish as
  /// if it had finished playing a source all the way through.
  pub fn skip_one(&self) {
    self.inner.skip_one();
  }

  #[napi]
  /// Attempts to seek to the given position (in milliseconds) in the
  /// current source.
  ///
  /// This blocks between 0 and ~5 milliseconds.
  ///
  /// As long as the duration of the source is known, seek is
  /// guaranteed to saturate at the end of the source. For example
  /// given a source that reports a total duration of 42 seconds,
  /// calling `trySeek(60000)` will seek to 42 seconds.
  pub fn try_seek(&self, pos_ms: f64) -> napi::Result<()> {
    let pos = std::time::Duration::from_secs_f64(pos_ms / 1000.0);
    self
      .inner
      .try_seek(pos)
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}
