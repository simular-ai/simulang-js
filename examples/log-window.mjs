// Run: node examples/log-window.mjs
// You can configure logging via RUST_LOG:
//   RUST_LOG=simulang_rs=debug,info node examples/log-window.mjs
//
// Pipes every Rust-side `log::*!` record from simulang-rs into a floating,
// always-on-top log window. The window comes from the sibling package
// `@simular-ai/simulang-log-viewer`, which is an optional peer dependency
// — install it explicitly to run this example:
//   npm install @simular-ai/simulang-log-viewer
//
// The window is **click-through by default**, so it never interferes with
// the app you're driving — every mouse click passes straight through. To
// move it or pause execution, press the global grab hotkey shown inside
// the window (Ctrl+Shift+Option+L on macOS, Ctrl+Shift+Alt+L elsewhere).
// Press it once to enter "grab mode" (click-through off, paused, draggable);
// press it again to resume.
//
// Inside an automation loop, `await win.waitIfPaused()` yields cleanly
// while the user has paused; it returns immediately otherwise.

import { Machine, initLogger } from '@simular-ai/simulang-js'
import { LogWindow } from '@simular-ai/simulang-log-viewer'

// Set SIMULANG_ANDROID=<host:port> to drive a connected Android device.
const machine = process.env.SIMULANG_ANDROID ? Machine.android(process.env.SIMULANG_ANDROID) : Machine.local()

const win = new LogWindow()
win.spawn()

// `'info'` mirrors `RUST_LOG=info`. Use e.g. `'simulang_rs=debug,warn'` to
// scope per-module. Omit the second argument to fall back to `RUST_LOG`.
//
// `win.log` already mirrors every line to the host's stderr internally,
// so a single call here gets the record into both the floating viewer
// and the terminal — no extra `console.log` needed.
initLogger((rec) => {
  win.log(`[${rec.level}] ${rec.message}`)
}, 'info')

// Run a few simulang-js calls so there's actually something to look at.
// Each call emits its own `[info]` record via `initLogger` above, so the
// `win.log` lines below stick to script-level milestones rather than
// re-narrating every call.
win.log('clipboard + screenshot demo')
try {
  const previous = machine.setClipboardString('hello from simulang-js').text
  machine.getClipboardString()
  if (previous !== null) machine.setClipboardString(previous)

  const shot = machine.screenFromMouse().screenshot(true)
  shot.shrink(640, 480)
  win.log(`screenshot captured: ${shot.base64().length} bytes base64`)
} catch (err) {
  win.log(`demo failed: ${err instanceof Error ? err.message : String(err)}`)
}

win.log('holding window open for 30s, press the grab hotkey to pause')
await new Promise((resolve) => setTimeout(resolve, 30_000))
// In real automation, scatter `await win.waitIfPaused()` between actions
// so the script yields whenever the user has grabbed the window.
await win.waitIfPaused()

win.close()
