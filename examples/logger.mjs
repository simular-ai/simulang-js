// Run: node examples/logger.mjs
// Or with a per-module spec:
//   RUST_LOG=simulang_rs::windows=debug,warn node examples/logger.mjs
//
// `initLogger()` accepts an optional JS callback. Without one, records go to
// stderr (analogous to the env_logger crate's default). With a callback, each record is forwarded
// from any Rust thread to your sink — useful for routing logs into a GUI
// panel, a file appender, pino/winston, or an Electron renderer over IPC.
//
// This example shows the callback form. For the env_logger-style default
// sink, just call `initLogger()` with no arguments.

import { initLogger, Machine } from '@simular-ai/simulang-js'

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

initLogger((record) => {
  // Records carry { level, target, message, file?, line?, modulePath? }.
  // Route however you like — here we just prefix and forward to console.
  console.log(`[${record.level}] [${record.target}] ${record.message}`)
})

// Trigger something so we have at least one record to look at.
try {
  machine.file('/tmp/this/file/probably/does/not/exist', false)
} catch {
  // expected — we just want to exercise the code paths that may log.
}

// The JS callback dispatch is unref'd (so the logger never keeps Node alive
// on its own). For a short-lived script that does its work and exits
// immediately, that means pending records may be dropped. A long-running
// app like an Electron GUI or a server doesn't need this; here we yield to
// libuv once so the dispatch queue can drain before exit.
await new Promise((resolve) => setImmediate(resolve))
