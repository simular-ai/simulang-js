//! Bridge between the Rust [`log`] facade and either a built-in stderr sink
//! or a user-supplied JS callback.
//!
//! [`init_logger`] registers a single static relay with the `log` crate;
//! subsequent calls swap the inner sink/filter atomically. Filtering uses
//! [`env_filter`], which understands the full `RUST_LOG` mini-language —
//! per-module specs like `simulang_rs::windows=debug,warn` work as-is.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::LazyLock;

use env_filter::{Builder as FilterBuilder, Filter};
use log::{Log, Metadata, Record};
use napi::Status;
use napi::bindgen_prelude::Unknown;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use parking_lot::RwLock;

/// Default filter applied when `RUST_LOG` is unset and no explicit spec is
/// given.
///
/// Diverges intentionally from `env_logger`'s "errors only" convention:
/// `simulang_rs` emits user-facing action records at info level (clipboard
/// reads/writes, key presses, app launches, window state changes, …)to make
/// the actions observable for an llm. Defaulting to
/// info for simulang + warn for everything else surfaces those actions in
/// the terminal without forcing every consumer to discover `RUST_LOG`,
/// while still keeping noisy dependencies (enigo, arboard, x11rb, …) quiet.
const DEFAULT_SPEC: &str = "simulang_rs=info,warn";

/// Threadsafe handle to the JS log callback. The trailing `false, true` flip
/// two napi-rs defaults:
///
/// - `CalleeHandled = false`: the dispatch never carries an error (a record
///   either matches the filter and gets sent, or it doesn't), so the JS
///   callback signature is just `(record)` — no `(err, record)` errback
///   shape with an always-null first arg.
/// - `Weak = true`: the dispatch is unref'd, so installing a logger does not
///   keep the Node event loop alive on its own.
type LogTsfn = ThreadsafeFunction<JsLogRecord, Unknown<'static>, JsLogRecord, Status, false, true>;

/// A single log entry forwarded to the JS callback.
#[napi(object)]
pub struct JsLogRecord {
  /// `"error"`, `"warn"`, `"info"`, `"debug"`, or `"trace"`.
  pub level: String,
  /// Logger target — defaults to the emitting Rust module path
  /// (e.g. `"simulang_rs::windows::app"`). Used by the filter spec to gate
  /// logs per module.
  pub target: String,
  /// The formatted log message.
  pub message: String,
  /// Source file path of the call site, when available.
  pub file: Option<String>,
  /// 1-based line number of the call site, when available.
  pub line: Option<u32>,
  /// Module path of the call site, when available.
  pub module_path: Option<String>,
}

/// Where the relay forwards records to.
enum Sink {
  /// Default: writes one line per record to stderr. Lives entirely on the Rust
  /// side, so no threadsafe-function machinery is involved.
  Console,
  /// User-supplied JS callback dispatched via a (Weak, unref'd) napi
  /// threadsafe function.
  Js(LogTsfn),
}

struct RelayState {
  sink: Sink,
  filter: Filter,
}

struct Relay {
  state: RwLock<RelayState>,
}

static RELAY: LazyLock<Relay> = LazyLock::new(|| Relay {
  state: RwLock::new(RelayState {
    sink: Sink::Console,
    filter: FilterBuilder::new().parse(DEFAULT_SPEC).build(),
  }),
});

impl Log for Relay {
  fn enabled(&self, metadata: &Metadata) -> bool {
    self.state.read().filter.enabled(metadata)
  }

  fn log(&self, record: &Record) {
    use std::io::Write;
    let state = self.state.read();
    if !state.filter.matches(record) {
      return;
    }
    match &state.sink {
      Sink::Console => {
        let _ = writeln!(
          std::io::stderr().lock(),
          "[{}] {}",
          record.level().as_str().to_ascii_lowercase(),
          record.args(),
        );
      }
      Sink::Js(cb) => {
        let payload = JsLogRecord {
          level: record.level().as_str().to_ascii_lowercase(),
          target: record.target().to_owned(),
          message: record.args().to_string(),
          file: record.file().map(str::to_owned),
          line: record.line(),
          module_path: record.module_path().map(str::to_owned),
        };
        // Don't let a panic from the napi machinery abort the Node process.
        let _ = catch_unwind(AssertUnwindSafe(|| {
          cb.call(payload, ThreadsafeFunctionCallMode::NonBlocking);
        }));
      }
    }
  }

  fn flush(&self) {
    // Nothing to drain: stderr writes are line-flushed per record, and the
    // threadsafe-function queue is owned by the Node event loop — we have
    // no way to wait on it from Rust without an `Env`.
  }
}

/// Build a filter from `spec` (falling back to `RUST_LOG`, then the
/// [`DEFAULT_SPEC`] of `"simulang_rs=info,warn"`) and atomically swap both
/// filter and sink into the relay.
fn install(spec: Option<&str>, sink: Sink) {
  let env = std::env::var("RUST_LOG").unwrap_or_default();
  let resolved = match spec {
    Some(s) if !s.is_empty() => s,
    _ if !env.is_empty() => &env,
    _ => DEFAULT_SPEC,
  };
  let filter = FilterBuilder::new().parse(resolved).build();
  let max = filter.filter();
  {
    let mut state = RELAY.state.write();
    state.filter = filter;
    state.sink = sink;
    log::set_max_level(max);
  }
  // `set_logger` only succeeds the first time; subsequent calls are no-ops.
  let _ = log::set_logger(&*RELAY);
}

#[napi]
#[allow(clippy::needless_pass_by_value)]
/// Initialize the logger.
///
/// **You usually don't need to call this.** The package's `main` entry
/// (`wrapped.js`) calls `initLogger()` with no arguments on first
/// evaluation, so simply `import '@simular-ai/simulang-js'` is enough to
/// see `[level] message` records on stderr. Call this directly
/// only when you want to *override* that default — to forward records to
/// a JS callback, change the filter spec, or silence everything.
///
/// - **No callback** (`initLogger()` or `initLogger(null, spec)`): records
///   are formatted as `[level] message` and written to stderr.
/// - **With a callback** (`initLogger(cb, spec)`): records are forwarded to
///   the JS callback via a non-blocking, unref'd threadsafe function. Use
///   this for GUI panels, file appenders, or routing through a logging
///   library like pino. The unref'd dispatch means the logger does not keep
///   the Node event loop alive on its own.
///
/// Omit `spec` to read `RUST_LOG` (falls back to `"simulang_rs=info,warn"`
/// when `RUST_LOG` is unset — info-level records from simulang's own
/// actions, warn-level from every other crate). Pass an explicit `spec`
/// to override `RUST_LOG`. Syntax is identical to `RUST_LOG`:
///
/// - `"info"` — global level
/// - `"simulang_rs=debug,warn"` — per-module override + default
/// - `"simulang_rs::windows=trace,simulang_rs::macos=debug,warn"` — multiple
/// - `"off"` — disable all logging
///
/// Safe to call repeatedly: each call atomically swaps the inner sink and
/// filter — useful for e.g. starting on stderr at boot and switching to a
/// renderer callback once the UI is ready.
///
/// **Don't log from inside the callback.** Whether directly (`console.log`
/// is fine — that's not a `log::*!` call) or transitively via another
/// `simulang-js` function whose Rust side emits a record, you'd feed the
/// relay another record, which queues another callback invocation, and so
/// on forever. The callback itself runs on the Node main thread; the
/// non-blocking dispatch from Rust is what creates the recursion risk.
pub fn init_logger(
  // Generics spelled out (rather than the `LogTsfn` alias) so napi-rs
  // codegen emits the resolved callback signature in `index.d.ts`.
  cb: Option<ThreadsafeFunction<JsLogRecord, Unknown<'static>, JsLogRecord, Status, false, true>>,
  spec: Option<String>,
) {
  install(spec.as_deref(), cb.map_or(Sink::Console, Sink::Js));
}
