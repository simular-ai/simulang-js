// Run: node examples/loopback.mjs <audio-file>
//
// Plays an audio file through the speakers while capturing system audio
// via loopback, then transcribes the captured audio using Whisper.
//
// Requires screen-recording permission on macOS (for loopback capture).

import { AudioOutput, LoopbackSource, SttModel } from '@simular-ai/simulang-js'

const audioFile = process.argv[2]
if (!audioFile) {
  console.error('Usage: node examples/loopback.mjs <audio-file>')
  process.exit(1)
}

try {
  const loopback = new LoopbackSource(2, 48000)
  loopback.start()

  const output = AudioOutput.openDefault()
  const player = output.createPlayer()

  console.log(`Playing ${audioFile} to speakers ...`)
  player.appendFile(audioFile)
  player.sleepUntilEnd()

  // Give loopback a moment to flush any trailing samples.
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 500)
  loopback.stop()

  console.log('Captured loopback audio, draining ...')
  const audio = loopback.drain()
  console.log(`  ${audio.durationMs.toFixed(0)} ms, ${audio.channels} ch, ${audio.sampleRate} Hz`)

  console.log('Transcribing ...')
  const model = SttModel.whisperLargeV3Turbo()
  const text = model.transcribe(audio)
  console.log(`Transcription: "${text}"`)
} catch (error) {
  console.error('Loopback failed:', error instanceof Error ? error.message : error)
}
