use napi::Error;
use napi_derive::napi;

use crate::language_model::stt::SttModel;

#[napi(object)]
#[derive(Clone, Copy)]
/// A concrete PCM audio stream configuration.
///
/// All audio crossing the API uses interleaved `f32` samples in
/// `[-1.0, 1.0]`. This object captures the two remaining degrees of
/// freedom (sample rate and channel count) so callers can request a
/// specific configuration when constructing sources or sinks.
pub struct AudioFormat {
  /// Samples per second per channel (e.g. 44100, 48000).
  pub sample_rate: u32,
  /// Number of interleaved channels (1 = mono, 2 = stereo).
  pub channels: u16,
}

impl From<AudioFormat> for simulang_rs::traits::AudioFormat {
  fn from(format: AudioFormat) -> Self {
    Self {
      sample_rate: format.sample_rate,
      channels: format.channels,
    }
  }
}

#[napi]
/// Loopback capture of a [`Machine`]'s audio output (what its speakers
/// are playing).
///
/// Obtain via [`Machine.loopback`]. Call `start()` before capturing with
/// `record()` or `drain()`; samples are interleaved `f32` in the
/// requested `AudioFormat`.
pub struct Loopback {
  pub(crate) inner: simulang_rs::Loopback,
}

#[napi]
impl Loopback {
  #[napi]
  /// Begin capturing. Must be called before `record()` or `drain()`.
  pub fn start(&mut self) -> napi::Result<()> {
    self.inner.start().map_err(Error::from_reason)
  }

  #[napi]
  /// Stop capturing. May be started again with `start()`. After
  /// stopping, call `drain()` to collect remaining buffered samples.
  pub fn stop(&mut self) -> napi::Result<()> {
    self.inner.stop().map_err(Error::from_reason)
  }

  #[napi(getter)]
  #[must_use]
  /// Number of interleaved channels (1 = mono, 2 = stereo).
  pub fn channels(&self) -> u16 {
    use simulang_rs::rodio::Source as _;
    self.inner.channels().get()
  }

  #[napi(getter)]
  #[must_use]
  /// Samples per second per channel (e.g. 44100, 48000).
  pub fn sample_rate(&self) -> u32 {
    use simulang_rs::rodio::Source as _;
    self.inner.sample_rate().get()
  }

  #[napi]
  /// Blocks for exactly `duration_ms` and returns the captured audio as
  /// a `SamplesBuffer`. Useful for streaming fixed-size chunks while
  /// the source is still running.
  pub fn record(&mut self, duration_ms: u32) -> napi::Result<SamplesBuffer> {
    use simulang_rs::SourceExt as _;

    let duration = std::time::Duration::from_millis(u64::from(duration_ms));
    self
      .inner
      .sample_chunks(duration)
      .next()
      .map(|inner| SamplesBuffer { inner })
      .ok_or_else(|| Error::from_reason("no audio samples collected"))
  }

  #[napi]
  /// Drains all buffered samples and returns them as a `SamplesBuffer`.
  ///
  /// Typical usage: call `start()`, do work while audio accumulates in
  /// the background, call `stop()`, then call `drain()` to collect
  /// everything that was captured.
  pub fn drain(&mut self) -> napi::Result<SamplesBuffer> {
    use simulang_rs::rodio::Source as _;

    let channels = self.inner.channels();
    let sample_rate = self.inner.sample_rate();
    let samples: Vec<f32> = self.inner.by_ref().collect();
    if samples.is_empty() {
      return Err(Error::from_reason("no audio samples collected"));
    }
    Ok(SamplesBuffer {
      inner: simulang_rs::rodio::buffer::SamplesBuffer::new(channels, sample_rate, samples),
    })
  }
}

#[napi]
/// A [`Machine`]'s default microphone, recording in a fixed
/// `AudioFormat`.
///
/// Obtain via [`Machine.microphone`]. The stream records from the moment
/// it is opened and stops when the handle is garbage-collected; collect
/// fixed-size chunks with `record()`.
pub struct Microphone {
  pub(crate) inner: simulang_rs::Microphone,
}

#[napi]
impl Microphone {
  #[napi(getter)]
  #[must_use]
  /// Number of interleaved channels (1 = mono, 2 = stereo).
  pub fn channels(&self) -> u16 {
    use simulang_rs::rodio::Source as _;
    self.inner.channels().get()
  }

  #[napi(getter)]
  #[must_use]
  /// Samples per second per channel (e.g. 44100, 48000).
  pub fn sample_rate(&self) -> u32 {
    use simulang_rs::rodio::Source as _;
    self.inner.sample_rate().get()
  }

  #[napi]
  /// Blocks for exactly `duration_ms` and returns the captured audio as
  /// a `SamplesBuffer`. Call repeatedly to stream fixed-size chunks.
  pub fn record(&mut self, duration_ms: u32) -> napi::Result<SamplesBuffer> {
    use simulang_rs::SourceExt as _;

    let duration = std::time::Duration::from_millis(u64::from(duration_ms));
    self
      .inner
      .sample_chunks(duration)
      .next()
      .map(|inner| SamplesBuffer { inner })
      .ok_or_else(|| Error::from_reason("no audio samples collected"))
  }
}

#[napi]
/// A buffer of samples treated as a source.
pub struct SamplesBuffer {
  pub(crate) inner: simulang_rs::rodio::buffer::SamplesBuffer,
}

#[napi]
impl SamplesBuffer {
  #[napi(constructor)]
  #[allow(clippy::needless_pass_by_value, clippy::cast_possible_truncation)]
  /// Creates a new buffer from raw PCM samples.
  ///
  /// `channels` is the number of interleaved channels (1 = mono,
  /// 2 = stereo). `sample_rate` is samples per second per channel
  /// (e.g. 16000, 44100).  `samples` is an array of f32 audio
  /// samples in `[-1.0, 1.0]`.
  pub fn new(channels: u16, sample_rate: u32, samples: Vec<f64>) -> napi::Result<Self> {
    use std::num::NonZero;

    let sr =
      NonZero::new(sample_rate).ok_or_else(|| Error::from_reason("sample_rate must be > 0"))?;
    let ch = NonZero::new(channels).ok_or_else(|| Error::from_reason("channels must be > 0"))?;
    let f32_samples: Vec<f32> = samples.into_iter().map(|s| s as f32).collect();

    Ok(Self {
      inner: simulang_rs::rodio::buffer::SamplesBuffer::new(ch, sr, f32_samples),
    })
  }

  #[napi(getter)]
  #[must_use]
  /// Number of interleaved channels (1 = mono, 2 = stereo).
  pub fn channels(&self) -> u16 {
    use simulang_rs::rodio::Source as _;
    self.inner.channels().get()
  }

  #[napi(getter)]
  #[must_use]
  /// Samples per second per channel (e.g. 16000, 44100).
  pub fn sample_rate(&self) -> u32 {
    use simulang_rs::rodio::Source as _;
    self.inner.sample_rate().get()
  }

  #[napi(getter)]
  #[must_use]
  /// Total duration of the buffer in milliseconds.
  #[allow(clippy::cast_precision_loss)]
  pub fn duration_ms(&self) -> f64 {
    use simulang_rs::rodio::Source as _;
    self
      .inner
      .total_duration()
      .map_or(0.0, |d| d.as_secs_f64() * 1000.0)
  }

  #[napi]
  /// Transcribe this audio buffer with the given speech-to-text model and
  /// return the recognized text.
  ///
  /// Equivalent to `model.transcribe(buffer)`
  pub fn transcribe(&self, model: &SttModel) -> napi::Result<String> {
    model
      .inner
      .transcribe(&self.inner)
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}
