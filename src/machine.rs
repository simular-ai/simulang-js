use napi::Error;
use napi_derive::napi;
use simulang_rs::traits::{ClipboardTrait, KeyboardTrait, MouseTrait};
use simulang_rs::{AndroidDevice, Key as SimulangKey, Machine as SimulangMachine};

use crate::accessibility_node::AccessibilityNode;
use crate::app::App;
use crate::audio::playback::AudioPlayer;
use crate::audio::source::{AudioFormat, Loopback, Microphone};
use crate::directory::Directory;
use crate::file::File;
use crate::image::Image;
use crate::instance::Instance;
use crate::key::Key;
use crate::mouse::{Button, Coordinate, Direction};
use crate::screen::Screen;
use crate::screenshot::Screenshot;
use crate::window::Window;

#[napi(string_enum = "lowercase")]
/// The operating system a [`Machine`] runs.
pub enum Os {
  MacOs,
  Windows,
  Linux,
  Android,
}

impl From<simulang_rs::Os> for Os {
  fn from(os: simulang_rs::Os) -> Self {
    match os {
      simulang_rs::Os::MacOs => Self::MacOs,
      simulang_rs::Os::Windows => Self::Windows,
      simulang_rs::Os::Linux => Self::Linux,
      simulang_rs::Os::Android => Self::Android,
    }
  }
}

#[napi]
/// A handle to one machine: the local desktop or an Android device over adb.
///
/// This is the entry point for all automation. Obtain a machine with
/// [`Machine.local`] or [`Machine.android`], then get apps, windows, and
/// accessibility nodes from it — every handle stays bound to the machine it
/// came from, and the same code drives any backend.
///
/// Mouse, keyboard, and clipboard access are flat methods on the machine
/// itself (`moveMouse`, `typeText`, `getClipboardString`, …).
///
/// # Canonical coordinate space
///
/// This is the reference description of the coordinate space used throughout
/// the library. `Window.boundingBox()`, `AccessibilityNode.boundingBox()`,
/// screenshots, and grounding-model output all live in this same space, so
/// coordinates round-trip between them **without conversion**.
///
/// Absolute coordinates live on the **global desktop**: top-left origin at
/// `(0, 0)` on the primary monitor (on Android, the device screen), with the
/// OS's native units. The unit is **not** the same on every OS:
///
/// - **Windows**, **Linux**, and **Android** use **physical pixels** (raw
///   hardware pixels).
/// - **macOS** uses **logical points** — on a 2× Retina display one point
///   spans two hardware pixels, so coordinates are half the physical-pixel
///   count.
///
/// Within a single OS every function speaks that OS's unit, so the
/// round-trip guarantee holds; only code that crosses into a *different*
/// coordinate system (e.g. an Electron overlay measured in CSS pixels) needs
/// to account for the per-OS unit. These are also the native units the OS
/// input/accessibility APIs expect, so they are *not* the browser
/// logical/CSS pixel.
///
/// Monitors arranged to the left of or above the primary display contribute
/// **negative** coordinates, so callers should not assume `x, y >= 0`. Use
/// [`Machine.screens`] / [`Window.screen`] to discover where the
/// addressable region actually is.
pub struct Machine {
  inner: SimulangMachine,
}

#[napi]
impl Machine {
  #[napi(factory)]
  #[must_use]
  /// The local desktop this process runs on. Infallible and zero-cost.
  ///
  /// Local machine handles share one global input state: every local
  /// machine drives the same physical mouse, keyboard, and clipboard.
  pub fn local() -> Self {
    Self {
      inner: SimulangMachine::local(),
    }
  }

  #[napi(factory)]
  #[allow(clippy::needless_pass_by_value)]
  /// Connect to an Android device at an adb endpoint (`host:port`).
  ///
  /// The device must be reachable over adb and run the uiautomator2
  /// (appium) server; `appiumBase` is its base URL and defaults to the
  /// standard local forward (`http://localhost:6790`). The appium URL is
  /// per device: when several devices are connected, forward a distinct
  /// local port for each.
  ///
  /// Unlike the local arm, each Android machine owns its emulated cursor
  /// and modifier state, so input state is per-handle.
  pub fn android(endpoint: String, appium_base: Option<String>) -> napi::Result<Self> {
    let inner = match appium_base {
      Some(base) => SimulangMachine::android_at(&endpoint, &base),
      None => SimulangMachine::android(&endpoint),
    };
    inner
      .map(|inner| Self { inner })
      .map_err(Error::from_reason)
  }

  #[napi(getter)]
  #[must_use]
  /// The operating system this machine runs:
  /// `'macos' | 'windows' | 'linux' | 'android'`.
  pub fn os(&self) -> Os {
    self.inner.os().into()
  }

  #[napi(getter)]
  #[must_use]
  /// A human-readable identifier for the machine: the local hostname, or
  /// the adb serial (`host:port`) for an Android device.
  pub fn id(&self) -> String {
    self.inner.id()
  }

  // ---------------------------------------------------------------
  // apps
  // ---------------------------------------------------------------

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// An installed application by exact name (no fuzzy matching).
  ///
  /// Locally this is the launcher name or path; on Android it is the
  /// package name (e.g. `com.android.chrome`).
  pub fn app(&self, name: String) -> napi::Result<App> {
    self
      .inner
      .app(&name)
      .map(|inner| App { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Find an installed app by fuzzy matching `query` against the machine's
  /// app list (launcher names locally, package names on Android).
  pub fn fuzzy_app(&self, query: String) -> napi::Result<App> {
    self
      .inner
      .fuzzy_app(&query)
      .map(|inner| App { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// All installed applications.
  pub fn apps(&self) -> napi::Result<Vec<App>> {
    self
      .inner
      .apps()
      .map(|apps| apps.into_iter().map(|inner| App { inner }).collect())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// The machine's default web browser.
  pub fn default_browser(&self) -> napi::Result<App> {
    self
      .inner
      .default_browser()
      .map(|inner| App { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// The application currently in the foreground.
  pub fn foreground_app(&self) -> napi::Result<Instance> {
    self
      .inner
      .foreground_app()
      .map(|inner| Instance { inner })
      .map_err(Error::from_reason)
  }

  // ---------------------------------------------------------------
  // windows
  // ---------------------------------------------------------------

  #[napi]
  /// The window that currently has keyboard focus, if any.
  ///
  /// Returns `null` when nothing is focused (desktop: no focused window;
  /// Android: no visible task, e.g. the screen is off) rather than
  /// treating that as an error.
  pub fn focused_window(&self) -> napi::Result<Option<Window>> {
    self
      .inner
      .focused_window()
      .map(|window| window.map(|inner| Window { inner }))
      .map_err(Error::from_reason)
  }

  #[napi]
  /// All windows on the machine: visible top-level windows across every
  /// process locally, one window per live task (recents entry) on Android.
  pub fn windows(&self) -> napi::Result<Vec<Window>> {
    self
      .inner
      .windows()
      .map(|windows| windows.into_iter().map(|inner| Window { inner }).collect())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// The top-level window at global desktop coordinates `(x, y)`, or
  /// `null` when no window sits under the point.
  ///
  /// `(x, y)` are in the canonical coordinate space (see [`Machine`]) — the
  /// same space `Window.boundingBox` and `mouseLocation` use, so a cursor
  /// location can be passed straight in.
  ///
  /// Platform notes:
  /// - **macOS**: returns the containing `AXWindow` of the element under
  ///   the cursor — the window itself, not the whole application.
  /// - **Windows**: returns the top-level window (`GA_ROOT`) of whatever
  ///   control sits under the cursor.
  /// - **Linux**: always `null` (no per-point window hit-test).
  /// - **Android**: throws — tasks have no per-point hit-test.
  pub fn window_at_point(&self, x: i32, y: i32) -> napi::Result<Option<Window>> {
    self
      .inner
      .window_at_point(x, y)
      .map(|window| window.map(|inner| Window { inner }))
      .map_err(Error::from_reason)
  }

  // ---------------------------------------------------------------
  // accessibility
  // ---------------------------------------------------------------

  #[napi]
  /// Root accessibility node of the machine's focused application.
  ///
  /// Android has no per-application tree; this is a snapshot of everything
  /// currently shown on screen, the same as [`Machine.systemRoot`].
  pub fn focused_root(&self) -> napi::Result<AccessibilityNode> {
    self
      .inner
      .focused_root()
      .map(AccessibilityNode::new)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Root accessibility node spanning everything currently accessible: on
  /// desktops the system-wide root (its children are the running
  /// applications), on Android a snapshot of everything currently shown on
  /// screen (which can span the foreground app, the system bars, and a
  /// split-screen neighbor).
  pub fn system_root(&self) -> napi::Result<AccessibilityNode> {
    self
      .inner
      .system_root()
      .map(AccessibilityNode::new)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// The accessibility element at global desktop coordinates `(x, y)` via
  /// the platform hit-test, or `null` when the point has no accessible
  /// element (empty desktop, gaps between controls). Throws on Android (no
  /// accessibility hit-test channel) and on genuine backend failures.
  ///
  /// `(x, y)` are in the canonical coordinate space (see [`Machine`]) — the
  /// same space `.boundingBox()` and `mouseLocation` use.
  pub fn node_at_point(&self, x: i32, y: i32) -> napi::Result<Option<AccessibilityNode>> {
    Ok(
      self
        .inner
        .node_at_point(x, y)
        .map_err(Error::from_reason)?
        .map(AccessibilityNode::new),
    )
  }

  #[napi]
  /// Playwright-style aria snapshot of the machine's focused application
  /// (Android: everything currently shown on screen).
  pub fn snapshot(&self) -> napi::Result<String> {
    self.inner.snapshot().map_err(Error::from_reason)
  }

  // ---------------------------------------------------------------
  // screens / screenshots
  // ---------------------------------------------------------------

  #[napi]
  /// Every display connected to the machine.
  ///
  /// The order is platform-defined; do not rely on it. Android: the
  /// device's single physical screen.
  pub fn screens(&self) -> napi::Result<Vec<Screen>> {
    self
      .inner
      .screens()
      .map(|screens| screens.into_iter().map(|inner| Screen { inner }).collect())
      .map_err(Error::from_reason)
  }

  #[napi]
  /// The machine's main display (the primary monitor). Android: the
  /// device screen.
  pub fn main_screen(&self) -> napi::Result<Screen> {
    self
      .inner
      .main_screen()
      .map(|inner| Screen { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// The display the mouse cursor is currently on (the main screen when
  /// the cursor is on none). Android: the device screen — the emulated
  /// pointer is always on it.
  ///
  /// Capture the returned handle once and reuse it when screen identity
  /// matters: resolving per capture can hit a different display whenever
  /// the mouse moves between calls.
  pub fn screen_from_mouse(&self) -> napi::Result<Screen> {
    self
      .inner
      .screen_from_mouse()
      .map(|inner| Screen { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Screenshot of a region of the machine's screen, addressed in the
  /// canonical global desktop coordinates (see [`Machine`]).
  ///
  /// Local: a native cropped capture. Android: the full screen is captured
  /// and cropped to the region, which must lie entirely within the screen
  /// (`hideCursor` is ignored).
  pub fn screenshot_cropped(
    &self,
    x: i32,
    y: i32,
    width: u16,
    height: u16,
    hide_cursor: bool,
  ) -> napi::Result<Screenshot> {
    self
      .inner
      .screenshot_cropped(x, y, width, height, hide_cursor)
      .map(|inner| Screenshot { inner })
      .map_err(Error::from_reason)
  }

  // ---------------------------------------------------------------
  // files
  // ---------------------------------------------------------------

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// A file on the machine's filesystem, opened — or created along with
  /// its parent directories, with `createMissing` — at `path`.
  ///
  /// Local: an absolute `path` is used as-is. A relative `path` is joined
  /// to the `SimularFiles` root (`..` is kept — this is a default base,
  /// not a sandbox).
  /// Android: `path` must be absolute (there is no working directory on
  /// the device to resolve against).
  pub fn file(&self, path: String, create_missing: bool) -> napi::Result<File> {
    self
      .inner
      .file(&path, create_missing)
      .map(File::new)
      .map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// A directory on the machine's filesystem, opened — or created along
  /// with its parents, with `createMissing` — at `path`. Path resolution
  /// as in [`Machine.file`].
  pub fn dir(&self, path: String, create_missing: bool) -> napi::Result<Directory> {
    self
      .inner
      .dir(&path, create_missing)
      .map(Directory::new)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// A new uniquely named directory in the machine's temp location. It is
  /// **not** removed automatically; call [`Directory.delete`] when done.
  pub fn temp_dir(&self) -> napi::Result<Directory> {
    self
      .inner
      .temp_dir()
      .map(Directory::new)
      .map_err(Error::from_reason)
  }

  // ---------------------------------------------------------------
  // audio
  // ---------------------------------------------------------------

  #[napi]
  /// Loopback capture of the machine's audio output (what its speakers
  /// are playing), in the given format.
  ///
  /// Not supported on Android yet: capturing device audio needs a
  /// device-side streamer, so this throws at construction. The signature
  /// will not change when support lands.
  pub fn loopback(&self, format: AudioFormat) -> napi::Result<Loopback> {
    self
      .inner
      .loopback(format.into())
      .map(|inner| Loopback { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// The machine's default microphone, recording in the given format
  /// from the moment it is opened.
  ///
  /// Not supported on Android yet — throws at construction; see
  /// [`Machine.loopback`].
  pub fn microphone(&self, format: AudioFormat) -> napi::Result<Microphone> {
    self
      .inner
      .microphone(format.into())
      .map(|inner| Microphone { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// A player queueing sounds on the machine's default audio output.
  ///
  /// Not supported on Android yet — throws at construction; see
  /// [`Machine.loopback`].
  pub fn player(&self) -> napi::Result<AudioPlayer> {
    self
      .inner
      .player()
      .map(|inner| AudioPlayer { inner })
      .map_err(Error::from_reason)
  }

  // ---------------------------------------------------------------
  // mouse
  // ---------------------------------------------------------------

  #[napi]
  /// Sends an individual mouse button event, e.g. to simulate a click of
  /// the left mouse key. Some of the buttons are specific to a platform.
  ///
  /// On Android there is no real cursor: a left press+release becomes a
  /// tap or swipe at the location recorded by [`Machine.moveMouse`].
  pub fn mouse_button(&mut self, button: Button, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .button(button.into(), direction.into())
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Move the mouse cursor to the specified x and y coordinates.
  ///
  /// You can specify absolute coordinates or relative from the current
  /// position.
  ///
  /// With absolute coordinates, `(x, y)` is in the global desktop space
  /// described on [`Machine`]: top-left origin at `(0, 0)`, OS-native
  /// units, and secondary monitors arranged above / to the left of the
  /// primary may have negative coordinates.
  ///
  /// With relative coordinates, a positive `x` moves the cursor `x`
  /// pixels to the right; a positive `y` moves it down.
  pub fn move_mouse(&mut self, x: i32, y: i32, coordinate: Coordinate) -> napi::Result<()> {
    self
      .inner
      .move_mouse(x, y, coordinate.into())
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Send a mouse scroll event.
  ///
  /// A positive length will result in scrolling down/right and negative
  /// ones up/left.
  pub fn scroll(&mut self, delta_x: i32, delta_y: i32) -> napi::Result<()> {
    self
      .inner
      .scroll(delta_x, delta_y)
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Get the location of the mouse in the canonical global-desktop space
  /// (OS-native units; see [`Machine`]). On Android this is the emulated
  /// cursor's recorded location.
  pub fn mouse_location(&self) -> napi::Result<(i32, i32)> {
    self
      .inner
      .mouse_location()
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  // ---------------------------------------------------------------
  // keyboard
  // ---------------------------------------------------------------

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Enter the text. You can use unicode here like: ❤️. This works
  /// regardless of the current keyboard layout. You cannot use this
  /// function for entering shortcuts or something similar. For shortcuts,
  /// use the [`Machine.key`] method instead.
  pub fn type_text(&mut self, text: String) -> napi::Result<()> {
    self
      .inner
      .text(&text)
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  #[napi]
  /// Sends an individual key event. It will enter the keysym (virtual
  /// key). Have a look at the [`Machine.keyRaw`] method, if you want to
  /// enter a keycode.
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
      return Err(Error::from_reason("keyUnicode requires a character."));
    };
    if chars.next().is_some() {
      return Err(Error::from_reason("keyUnicode expects a single character."));
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
  /// current layout. You have to make sure of that yourself. This can be
  /// useful if you want to simulate a press regardless of the layout (WASD
  /// on video games). Have a look at the [`Machine.key`] method, if you
  /// just want to enter a specific key and don't want to worry about the
  /// layout/keymap. Windows only: If you want to enter the keycode
  /// (scancode) of an extended key, you need to set the high byte for the
  /// extended key too. You can for example do:
  /// `keyRaw(0xE01D, Direction.Click)` to simulate `RControl`.
  pub fn key_raw(&mut self, keycode: u16, direction: Direction) -> napi::Result<()> {
    self
      .inner
      .raw(keycode, direction.into())
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  // ---------------------------------------------------------------
  // clipboard
  // ---------------------------------------------------------------

  #[napi]
  /// Gets the string content from the machine's clipboard.
  ///
  /// Returns the clipboard string if available, or `null` if the clipboard
  /// doesn't contain string data or is empty.
  pub fn get_clipboard_string(&self) -> napi::Result<Option<String>> {
    self.inner.get_string().map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Replaces the contents of the machine's clipboard with the given
  /// string.
  ///
  /// Returns a [`ClipboardContent`] snapshot of what the clipboard held
  /// before this call. Nothing is restored automatically; see the
  /// snapshot's docs for how to write it back (and the
  /// one-format-per-write caveat).
  pub fn set_clipboard_string(&self, value: String) -> napi::Result<ClipboardContent> {
    self
      .inner
      .set_string(&value)
      .map(|inner| ClipboardContent { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Gets the image content from the machine's clipboard.
  ///
  /// Returns the clipboard image if available, or `null` if the clipboard
  /// doesn't contain image data.
  pub fn get_clipboard_image(&self) -> napi::Result<Option<Image>> {
    self
      .inner
      .get_image()
      .map(|opt| opt.map(|inner| Image { inner }))
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Replaces the contents of the machine's clipboard with the given
  /// image. For example used to copy a screenshot to the clipboard.
  ///
  /// Returns a [`ClipboardContent`] snapshot of what the clipboard held
  /// before this call (see [`Machine.setClipboardString`]).
  pub fn set_clipboard_image(&self, image: &Image) -> napi::Result<ClipboardContent> {
    self
      .inner
      .set_image(&image.inner)
      .map(|inner| ClipboardContent { inner })
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Clears all content from the machine's clipboard, regardless of type.
  pub fn clear_clipboard(&self) -> napi::Result<()> {
    self.inner.clear().map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Types text by pasting it from the clipboard.
  ///
  /// Locally this saves the previous clipboard text and image (when
  /// present), sets the clipboard to the specified string, verifies it was
  /// set correctly, then simulates Command+V (or Ctrl+V) to paste it; after
  /// pasting, the saved snapshot is reapplied. On Android the text is
  /// committed through the device's input channel without touching the
  /// host clipboard.
  pub fn paste_text(&self, value: String) -> napi::Result<()> {
    self
      .inner
      .paste_text(&value)
      .map_err(|err| Error::from_reason(err.to_string()))
  }

  // ---------------------------------------------------------------
  // escape hatch
  // ---------------------------------------------------------------

  #[napi]
  #[must_use]
  /// Phone-only operations of the underlying Android device, or `null`
  /// when this machine is the local desktop.
  ///
  /// Everything cross-platform lives on [`Machine`] and the handles it
  /// returns; this escape hatch is only for operations that have no
  /// desktop counterpart.
  pub fn as_android(&self) -> Option<AndroidExtras> {
    self.inner.as_android().map(|device| AndroidExtras {
      inner: device.clone(),
    })
  }
}

#[napi]
/// Snapshot of the clipboard's text and/or image content, as returned by
/// [`Machine.setClipboardString`] and [`Machine.setClipboardImage`] — what
/// the clipboard held *before* that write replaced it.
///
/// Nothing is restored automatically. To put a snapshot back, write its
/// fields with [`Machine.setClipboardImage`] /
/// [`Machine.setClipboardString`] — but note the platform clipboards
/// replace the **whole** selection on every write, so a snapshot holding
/// both formats can only have one of them restored (pick the image if in
/// doubt; that is also what [`Machine.pasteText`]'s internal restore does).
pub struct ClipboardContent {
  inner: simulang_rs::ClipboardContent,
}

#[napi]
impl ClipboardContent {
  #[napi(getter)]
  #[must_use]
  /// The clipboard's string content, when it had string data.
  pub fn text(&self) -> Option<String> {
    self.inner.text.clone()
  }

  #[napi(getter)]
  #[must_use]
  /// The clipboard's image content (8-bit RGBA, see
  /// [`Machine.getClipboardImage`]), when it had image data.
  pub fn image(&self) -> Option<Image> {
    self.inner.image.clone().map(|inner| Image { inner })
  }
}

#[napi]
/// Phone-only operations of an Android [`Machine`]. Obtain via
/// [`Machine.asAndroid`].
///
/// Cross-platform operations (`apps`, mouse, clipboard, …) live on
/// [`Machine`] itself; this escape hatch is only for controls that have
/// no desktop counterpart (status bar, orientation, package enablement,
/// touch gestures with explicit timing).
pub struct AndroidExtras {
  inner: AndroidDevice,
}

#[napi]
impl AndroidExtras {
  #[napi(getter)]
  #[must_use]
  /// The adb serial (`host:port`) this device is connected through.
  pub fn serial(&self) -> String {
    self.inner.serial().to_owned()
  }

  #[napi]
  /// Open the notification shade.
  pub fn notifications(&self) -> napi::Result<()> {
    self.inner.notifications().map_err(Error::from_reason)
  }

  #[napi]
  /// Open the quick-settings panel.
  pub fn quick_settings(&self) -> napi::Result<()> {
    self.inner.quick_settings().map_err(Error::from_reason)
  }

  #[napi]
  /// Collapse the notification shade / quick-settings panel.
  pub fn collapse_shade(&self) -> napi::Result<()> {
    self.inner.collapse_shade().map_err(Error::from_reason)
  }

  #[napi]
  /// Lock the screen to landscape orientation.
  ///
  /// The freeze overrides auto-rotate until the orientation is set again.
  pub fn landscape(&self) -> napi::Result<()> {
    self.inner.landscape().map_err(Error::from_reason)
  }

  #[napi]
  /// Lock the screen to portrait orientation.
  ///
  /// The freeze overrides auto-rotate until the orientation is set again.
  pub fn portrait(&self) -> napi::Result<()> {
    self.inner.portrait().map_err(Error::from_reason)
  }

  #[napi]
  #[allow(clippy::needless_pass_by_value)]
  /// Enable or disable an installed package.
  ///
  /// Disabling uses the unprivileged, per-user reversible form; `enabled:
  /// true` reverses it.
  pub fn set_package_enabled(&self, package: String, enabled: bool) -> napi::Result<()> {
    self
      .inner
      .set_package_enabled(&package, enabled)
      .map_err(Error::from_reason)
  }

  #[napi]
  /// Swipe (touch-drag) from `(fromX, fromY)` to `(toX, toY)`, holding
  /// the finger down for `durationMs` while it travels. Coordinates are
  /// absolute device pixels in the current rotation's space.
  ///
  /// The whole gesture ships to the device as one atomic pointer
  /// sequence, so nothing is held open across calls. The duration decides
  /// what the gesture means to the app: brief (well under ~200ms) reads
  /// as a fling that keeps scrolling with momentum, longer as a
  /// deliberate drag that stops where the finger stops.
  ///
  /// For a plain drag the cross-platform mouse lowering
  /// ([`Machine.moveMouse`] + [`Machine.mouseButton`] press/release) is
  /// equivalent and portable — it synthesizes this same gesture at a
  /// fixed 200ms. Reach for this method only when the timing matters.
  pub fn swipe(
    &self,
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
    duration_ms: u32,
  ) -> napi::Result<()> {
    self
      .inner
      .swipe((from_x, from_y), (to_x, to_y), duration_ms)
      .map_err(Error::from_reason)
  }
}
