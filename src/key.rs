use napi::Error;
use napi_derive::napi;
use simulang_rs::Key as SimulangKey;

#[napi]
#[derive(Debug, Copy, Clone)]
/// Represents a keyboard key.
pub enum Key {
  Num0,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  AbntC1,
  AbntC2,
  Accept,
  Add,
  /// alt key on Linux and Windows (option key on macOS)
  Alt,
  Apps,
  Attn,
  /// backspace key
  Backspace,
  Break,
  Begin,
  BrightnessDown,
  BrightnessUp,
  BrowserBack,
  BrowserFavorites,
  BrowserForward,
  BrowserHome,
  BrowserRefresh,
  BrowserSearch,
  BrowserStop,
  Cancel,
  /// caps lock key
  CapsLock,
  Clear,
  ContrastUp,
  ContrastDown,
  /// control key
  Control,
  Convert,
  Crsel,
  DBEAlphanumeric,
  DBECodeinput,
  DBEDetermineString,
  DBEEnterDLGConversionMode,
  DBEEnterIMEConfigMode,
  DBEEnterWordRegisterMode,
  DBEFlushString,
  DBEHiragana,
  DBEKatakana,
  DBENoCodepoint,
  DBENoRoman,
  DBERoman,
  DBESBCSChar,
  DBESChar,
  Decimal,
  /// delete key
  Delete,
  Divide,
  /// down arrow key
  DownArrow,
  Eject,
  /// end key
  End,
  Ereof,
  /// escape key (esc)
  Escape,
  Execute,
  Exsel,
  /// F1 key
  F1,
  /// F2 key
  F2,
  /// F3 key
  F3,
  /// F4 key
  F4,
  /// F5 key
  F5,
  /// F6 key
  F6,
  /// F7 key
  F7,
  /// F8 key
  F8,
  /// F9 key
  F9,
  /// F10 key
  F10,
  /// F11 key
  F11,
  /// F12 key
  F12,
  /// F13 key
  F13,
  /// F14 key
  F14,
  /// F15 key
  F15,
  /// F16 key
  F16,
  /// F17 key
  F17,
  /// F18 key
  F18,
  /// F19 key
  F19,
  /// F20 key
  F20,
  /// F21 key
  F21,
  /// F22 key
  F22,
  /// F23 key
  F23,
  /// F24 key
  F24,
  F25,
  F26,
  F27,
  F28,
  F29,
  F30,
  F31,
  F32,
  F33,
  F34,
  F35,
  Function,
  Final,
  Find,
  GamepadA,
  GamepadB,
  GamepadDPadDown,
  GamepadDPadLeft,
  GamepadDPadRight,
  GamepadDPadUp,
  GamepadLeftShoulder,
  GamepadLeftThumbstickButton,
  GamepadLeftThumbstickDown,
  GamepadLeftThumbstickLeft,
  GamepadLeftThumbstickRight,
  GamepadLeftThumbstickUp,
  GamepadLeftTrigger,
  GamepadMenu,
  GamepadRightShoulder,
  GamepadRightThumbstickButton,
  GamepadRightThumbstickDown,
  GamepadRightThumbstickLeft,
  GamepadRightThumbstickRight,
  GamepadRightThumbstickUp,
  GamepadRightTrigger,
  GamepadView,
  GamepadX,
  GamepadY,
  Hangeul,
  Hangul,
  Hanja,
  Help,
  /// home key
  Home,
  Ico00,
  IcoClear,
  IcoHelp,
  IlluminationDown,
  IlluminationUp,
  IlluminationToggle,
  IMEOff,
  IMEOn,
  Insert,
  Junja,
  Kana,
  Kanji,
  LaunchApp1,
  LaunchApp2,
  LaunchMail,
  LaunchMediaSelect,
  /// Opens launchpad
  Launchpad,
  LaunchPanel,
  LButton,
  LControl,
  /// left arrow key
  LeftArrow,
  Linefeed,
  LMenu,
  LShift,
  LWin,
  MButton,
  MediaFast,
  MediaNextTrack,
  MediaPlayPause,
  MediaPrevTrack,
  MediaRewind,
  MediaStop,
  /// meta key (also known as "windows", "super", and "command")
  Meta,
  /// Opens mission control
  MissionControl,
  ModeChange,
  Multiply,
  NavigationAccept,
  NavigationCancel,
  NavigationDown,
  NavigationLeft,
  NavigationMenu,
  NavigationRight,
  NavigationUp,
  NavigationView,
  NoName,
  NonConvert,
  None,
  /// Num lock
  Numlock,
  Numpad0,
  Numpad1,
  Numpad2,
  Numpad3,
  Numpad4,
  Numpad5,
  Numpad6,
  Numpad7,
  Numpad8,
  Numpad9,
  NumpadEnter,
  OEM1,
  OEM102,
  OEM2,
  OEM3,
  OEM4,
  OEM5,
  OEM6,
  OEM7,
  OEM8,
  OEMAttn,
  OEMAuto,
  OEMAx,
  OEMBacktab,
  OEMClear,
  OEMComma,
  OEMCopy,
  OEMCusel,
  OEMEnlw,
  OEMFinish,
  OEMFJJisho,
  OEMFJLoya,
  OEMFJMasshou,
  OEMFJRoya,
  OEMFJTouroku,
  OEMJump,
  OEMMinus,
  OEMNECEqual,
  OEMPA1,
  OEMPA2,
  OEMPA3,
  OEMPeriod,
  OEMPlus,
  OEMReset,
  OEMWsctrl,
  /// option key on macOS (alt key on Linux and Windows)
  Option,
  PA1,
  Packet,
  /// page down key
  PageDown,
  /// page up key
  PageUp,
  Pause,
  Play,
  Power,
  /// Print key (e.g. print screen / `SysRq` on some keyboards)
  Print,
  /// Take a screenshot
  PrintScr,
  Processkey,
  RButton,
  RCommand,
  RControl,
  Redo,
  /// return key
  Return,
  /// right arrow key
  RightArrow,
  RMenu,
  ROption,
  RShift,
  RWin,
  Scroll,
  ScrollLock,
  Select,
  ScriptSwitch,
  Separator,
  /// shift key
  Shift,
  /// Lock shift key
  ShiftLock,
  Sleep,
  /// space key
  Space,
  Subtract,
  SysReq,
  /// tab key (tabulator)
  Tab,
  Undo,
  /// up arrow key
  UpArrow,
  VidMirror,
  VolumeDown,
  VolumeMute,
  VolumeUp,
  /// microphone mute toggle on linux
  MicMute,
  XButton1,
  XButton2,
  Zoom,
  /// Use `key_unicode` to provide the character.
  Unicode,
  /// Use `key_other` to provide the raw key value.
  Other,
}

impl TryFrom<String> for Key {
  type Error = Error;

  #[allow(clippy::too_many_lines)]
  fn try_from(value: String) -> Result<Self, Self::Error> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
      return Err(Error::from_reason("Key string cannot be empty."));
    }

    let normalized = trimmed.to_ascii_lowercase();
    let normalized: String = normalized
      .chars()
      .filter(|ch| !matches!(ch, '_' | '-' | ' '))
      .collect();

    match normalized.as_str() {
      "num0" | "0" => Ok(Key::Num0),
      "num1" | "1" => Ok(Key::Num1),
      "num2" | "2" => Ok(Key::Num2),
      "num3" | "3" => Ok(Key::Num3),
      "num4" | "4" => Ok(Key::Num4),
      "num5" | "5" => Ok(Key::Num5),
      "num6" | "6" => Ok(Key::Num6),
      "num7" | "7" => Ok(Key::Num7),
      "num8" | "8" => Ok(Key::Num8),
      "num9" | "9" => Ok(Key::Num9),
      "a" => Ok(Key::A),
      "b" => Ok(Key::B),
      "c" => Ok(Key::C),
      "d" => Ok(Key::D),
      "e" => Ok(Key::E),
      "f" => Ok(Key::F),
      "g" => Ok(Key::G),
      "h" => Ok(Key::H),
      "i" => Ok(Key::I),
      "j" => Ok(Key::J),
      "k" => Ok(Key::K),
      "l" => Ok(Key::L),
      "m" => Ok(Key::M),
      "n" => Ok(Key::N),
      "o" => Ok(Key::O),
      "p" => Ok(Key::P),
      "q" => Ok(Key::Q),
      "r" => Ok(Key::R),
      "s" => Ok(Key::S),
      "t" => Ok(Key::T),
      "u" => Ok(Key::U),
      "v" => Ok(Key::V),
      "w" => Ok(Key::W),
      "x" => Ok(Key::X),
      "y" => Ok(Key::Y),
      "z" => Ok(Key::Z),
      "abntc1" => Ok(Key::AbntC1),
      "abntc2" => Ok(Key::AbntC2),
      "accept" => Ok(Key::Accept),
      "add" => Ok(Key::Add),
      "alt" => Ok(Key::Alt),
      "apps" => Ok(Key::Apps),
      "attn" => Ok(Key::Attn),
      "backspace" => Ok(Key::Backspace),
      "break" => Ok(Key::Break),
      "begin" => Ok(Key::Begin),
      "brightnessdown" => Ok(Key::BrightnessDown),
      "brightnessup" => Ok(Key::BrightnessUp),
      "browserback" => Ok(Key::BrowserBack),
      "browserfavorites" => Ok(Key::BrowserFavorites),
      "browserforward" => Ok(Key::BrowserForward),
      "browserhome" => Ok(Key::BrowserHome),
      "browserrefresh" => Ok(Key::BrowserRefresh),
      "browsersearch" => Ok(Key::BrowserSearch),
      "browserstop" => Ok(Key::BrowserStop),
      "cancel" => Ok(Key::Cancel),
      "capslock" => Ok(Key::CapsLock),
      "clear" => Ok(Key::Clear),
      "contrastup" => Ok(Key::ContrastUp),
      "contrastdown" => Ok(Key::ContrastDown),
      "control" => Ok(Key::Control),
      "convert" => Ok(Key::Convert),
      "crsel" => Ok(Key::Crsel),
      "dbealphanumeric" => Ok(Key::DBEAlphanumeric),
      "dbecodeinput" => Ok(Key::DBECodeinput),
      "dbedeterminestring" => Ok(Key::DBEDetermineString),
      "dbeenterdlgconversionmode" => Ok(Key::DBEEnterDLGConversionMode),
      "dbeenterimeconfigmode" => Ok(Key::DBEEnterIMEConfigMode),
      "dbeenterwordregistermode" => Ok(Key::DBEEnterWordRegisterMode),
      "dbeflushstring" => Ok(Key::DBEFlushString),
      "dbehiragana" => Ok(Key::DBEHiragana),
      "dbekatakana" => Ok(Key::DBEKatakana),
      "dbenocodepoint" => Ok(Key::DBENoCodepoint),
      "dbenoroman" => Ok(Key::DBENoRoman),
      "dberoman" => Ok(Key::DBERoman),
      "dbesbcschar" => Ok(Key::DBESBCSChar),
      "dbeschar" => Ok(Key::DBESChar),
      "decimal" => Ok(Key::Decimal),
      "delete" => Ok(Key::Delete),
      "divide" => Ok(Key::Divide),
      "downarrow" | "down" => Ok(Key::DownArrow),
      "eject" => Ok(Key::Eject),
      "end" => Ok(Key::End),
      "enter" | "return" => Ok(Key::Return),
      "numpadenter" => Ok(Key::NumpadEnter),
      "ereof" => Ok(Key::Ereof),
      "escape" => Ok(Key::Escape),
      "execute" => Ok(Key::Execute),
      "exsel" => Ok(Key::Exsel),
      "f1" => Ok(Key::F1),
      "f2" => Ok(Key::F2),
      "f3" => Ok(Key::F3),
      "f4" => Ok(Key::F4),
      "f5" => Ok(Key::F5),
      "f6" => Ok(Key::F6),
      "f7" => Ok(Key::F7),
      "f8" => Ok(Key::F8),
      "f9" => Ok(Key::F9),
      "f10" => Ok(Key::F10),
      "f11" => Ok(Key::F11),
      "f12" => Ok(Key::F12),
      "f13" => Ok(Key::F13),
      "f14" => Ok(Key::F14),
      "f15" => Ok(Key::F15),
      "f16" => Ok(Key::F16),
      "f17" => Ok(Key::F17),
      "f18" => Ok(Key::F18),
      "f19" => Ok(Key::F19),
      "f20" => Ok(Key::F20),
      "f21" => Ok(Key::F21),
      "f22" => Ok(Key::F22),
      "f23" => Ok(Key::F23),
      "f24" => Ok(Key::F24),
      "f25" => Ok(Key::F25),
      "f26" => Ok(Key::F26),
      "f27" => Ok(Key::F27),
      "f28" => Ok(Key::F28),
      "f29" => Ok(Key::F29),
      "f30" => Ok(Key::F30),
      "f31" => Ok(Key::F31),
      "f32" => Ok(Key::F32),
      "f33" => Ok(Key::F33),
      "f34" => Ok(Key::F34),
      "f35" => Ok(Key::F35),
      "function" => Ok(Key::Function),
      "final" => Ok(Key::Final),
      "find" => Ok(Key::Find),
      "gamepada" => Ok(Key::GamepadA),
      "gamepadb" => Ok(Key::GamepadB),
      "gamepaddpaddown" => Ok(Key::GamepadDPadDown),
      "gamepaddpadleft" => Ok(Key::GamepadDPadLeft),
      "gamepaddpadright" => Ok(Key::GamepadDPadRight),
      "gamepaddpadup" => Ok(Key::GamepadDPadUp),
      "gamepadleftshoulder" => Ok(Key::GamepadLeftShoulder),
      "gamepadleftthumbstickbutton" => Ok(Key::GamepadLeftThumbstickButton),
      "gamepadleftthumbstickdown" => Ok(Key::GamepadLeftThumbstickDown),
      "gamepadleftthumbstickleft" => Ok(Key::GamepadLeftThumbstickLeft),
      "gamepadleftthumbstickright" => Ok(Key::GamepadLeftThumbstickRight),
      "gamepadleftthumbstickup" => Ok(Key::GamepadLeftThumbstickUp),
      "gamepadlefttrigger" => Ok(Key::GamepadLeftTrigger),
      "gamepadmenu" => Ok(Key::GamepadMenu),
      "gamepadrightshoulder" => Ok(Key::GamepadRightShoulder),
      "gamepadrightthumbstickbutton" => Ok(Key::GamepadRightThumbstickButton),
      "gamepadrightthumbstickdown" => Ok(Key::GamepadRightThumbstickDown),
      "gamepadrightthumbstickleft" => Ok(Key::GamepadRightThumbstickLeft),
      "gamepadrightthumbstickright" => Ok(Key::GamepadRightThumbstickRight),
      "gamepadrightthumbstickup" => Ok(Key::GamepadRightThumbstickUp),
      "gamepadrighttrigger" => Ok(Key::GamepadRightTrigger),
      "gamepadview" => Ok(Key::GamepadView),
      "gamepadx" => Ok(Key::GamepadX),
      "gamepady" => Ok(Key::GamepadY),
      "hangeul" => Ok(Key::Hangeul),
      "hangul" => Ok(Key::Hangul),
      "hanja" => Ok(Key::Hanja),
      "help" => Ok(Key::Help),
      "home" => Ok(Key::Home),
      "ico00" => Ok(Key::Ico00),
      "icoclear" => Ok(Key::IcoClear),
      "icohelp" => Ok(Key::IcoHelp),
      "illuminationdown" => Ok(Key::IlluminationDown),
      "illuminationup" => Ok(Key::IlluminationUp),
      "illuminationtoggle" => Ok(Key::IlluminationToggle),
      "imeoff" => Ok(Key::IMEOff),
      "imeon" => Ok(Key::IMEOn),
      "insert" => Ok(Key::Insert),
      "junja" => Ok(Key::Junja),
      "kana" => Ok(Key::Kana),
      "kanji" => Ok(Key::Kanji),
      "launchapp1" => Ok(Key::LaunchApp1),
      "launchapp2" => Ok(Key::LaunchApp2),
      "launchmail" => Ok(Key::LaunchMail),
      "launchmediaselect" => Ok(Key::LaunchMediaSelect),
      "launchpad" => Ok(Key::Launchpad),
      "launchpanel" => Ok(Key::LaunchPanel),
      "lbutton" => Ok(Key::LButton),
      "lcontrol" => Ok(Key::LControl),
      "leftarrow" | "left" => Ok(Key::LeftArrow),
      "linefeed" => Ok(Key::Linefeed),
      "lmenu" => Ok(Key::LMenu),
      "lshift" => Ok(Key::LShift),
      "lwin" => Ok(Key::LWin),
      "mbutton" => Ok(Key::MButton),
      "mediafast" => Ok(Key::MediaFast),
      "medianexttrack" => Ok(Key::MediaNextTrack),
      "mediaplaypause" => Ok(Key::MediaPlayPause),
      "mediaprevtrack" => Ok(Key::MediaPrevTrack),
      "mediarewind" => Ok(Key::MediaRewind),
      "mediastop" => Ok(Key::MediaStop),
      "meta" => Ok(Key::Meta),
      "missioncontrol" => Ok(Key::MissionControl),
      "modechange" => Ok(Key::ModeChange),
      "multiply" => Ok(Key::Multiply),
      "navigationaccept" => Ok(Key::NavigationAccept),
      "navigationcancel" => Ok(Key::NavigationCancel),
      "navigationdown" => Ok(Key::NavigationDown),
      "navigationleft" => Ok(Key::NavigationLeft),
      "navigationmenu" => Ok(Key::NavigationMenu),
      "navigationright" => Ok(Key::NavigationRight),
      "navigationup" => Ok(Key::NavigationUp),
      "navigationview" => Ok(Key::NavigationView),
      "noname" => Ok(Key::NoName),
      "nonconvert" => Ok(Key::NonConvert),
      "none" => Ok(Key::None),
      "numlock" => Ok(Key::Numlock),
      "numpad0" => Ok(Key::Numpad0),
      "numpad1" => Ok(Key::Numpad1),
      "numpad2" => Ok(Key::Numpad2),
      "numpad3" => Ok(Key::Numpad3),
      "numpad4" => Ok(Key::Numpad4),
      "numpad5" => Ok(Key::Numpad5),
      "numpad6" => Ok(Key::Numpad6),
      "numpad7" => Ok(Key::Numpad7),
      "numpad8" => Ok(Key::Numpad8),
      "numpad9" => Ok(Key::Numpad9),
      "oem1" => Ok(Key::OEM1),
      "oem102" => Ok(Key::OEM102),
      "oem2" => Ok(Key::OEM2),
      "oem3" => Ok(Key::OEM3),
      "oem4" => Ok(Key::OEM4),
      "oem5" => Ok(Key::OEM5),
      "oem6" => Ok(Key::OEM6),
      "oem7" => Ok(Key::OEM7),
      "oem8" => Ok(Key::OEM8),
      "oemattn" => Ok(Key::OEMAttn),
      "oemauto" => Ok(Key::OEMAuto),
      "oemax" => Ok(Key::OEMAx),
      "oembacktab" => Ok(Key::OEMBacktab),
      "oemclear" => Ok(Key::OEMClear),
      "oemcomma" => Ok(Key::OEMComma),
      "oemcopy" => Ok(Key::OEMCopy),
      "oemcusel" => Ok(Key::OEMCusel),
      "oemenlw" => Ok(Key::OEMEnlw),
      "oemfinish" => Ok(Key::OEMFinish),
      "oemfjjisho" => Ok(Key::OEMFJJisho),
      "oemfjloya" => Ok(Key::OEMFJLoya),
      "oemfjmasshou" => Ok(Key::OEMFJMasshou),
      "oemfjroya" => Ok(Key::OEMFJRoya),
      "oemfjtouroku" => Ok(Key::OEMFJTouroku),
      "oemjump" => Ok(Key::OEMJump),
      "oemminus" => Ok(Key::OEMMinus),
      "oemnecequal" => Ok(Key::OEMNECEqual),
      "oempa1" => Ok(Key::OEMPA1),
      "oempa2" => Ok(Key::OEMPA2),
      "oempa3" => Ok(Key::OEMPA3),
      "oemperiod" => Ok(Key::OEMPeriod),
      "oemplus" => Ok(Key::OEMPlus),
      "oemreset" => Ok(Key::OEMReset),
      "oemwsctrl" => Ok(Key::OEMWsctrl),
      "option" => Ok(Key::Option),
      "pa1" => Ok(Key::PA1),
      "packet" => Ok(Key::Packet),
      "pagedown" => Ok(Key::PageDown),
      "pageup" => Ok(Key::PageUp),
      "pause" => Ok(Key::Pause),
      "play" => Ok(Key::Play),
      "power" => Ok(Key::Power),
      "print" | "printscreen" | "printscr" => Ok(Key::Print), // TODO: Check if this is correct
      "processkey" => Ok(Key::Processkey),
      "rbutton" => Ok(Key::RButton),
      "rcommand" => Ok(Key::RCommand),
      "rcontrol" => Ok(Key::RControl),
      "redo" => Ok(Key::Redo),
      "rightarrow" | "right" => Ok(Key::RightArrow),
      "rmenu" => Ok(Key::RMenu),
      "roption" => Ok(Key::ROption),
      "rshift" => Ok(Key::RShift),
      "rwin" => Ok(Key::RWin),
      "scroll" => Ok(Key::Scroll),
      "scrolllock" => Ok(Key::ScrollLock),
      "select" => Ok(Key::Select),
      "scriptswitch" => Ok(Key::ScriptSwitch),
      "separator" => Ok(Key::Separator),
      "shift" => Ok(Key::Shift),
      "shiftlock" => Ok(Key::ShiftLock),
      "sleep" => Ok(Key::Sleep),
      "space" => Ok(Key::Space),
      "subtract" => Ok(Key::Subtract),
      "sysreq" => Ok(Key::SysReq),
      "tab" => Ok(Key::Tab),
      "undo" => Ok(Key::Undo),
      "uparrow" | "up" => Ok(Key::UpArrow),
      "vidmirror" => Ok(Key::VidMirror),
      "volumedown" => Ok(Key::VolumeDown),
      "volumemute" => Ok(Key::VolumeMute),
      "volumeup" => Ok(Key::VolumeUp),
      "micmute" => Ok(Key::MicMute),
      "xbutton1" => Ok(Key::XButton1),
      "xbutton2" => Ok(Key::XButton2),
      "zoom" => Ok(Key::Zoom),
      "unicode" => Ok(Key::Unicode),
      "other" => Ok(Key::Other),
      _ => Err(Error::from_reason(format!("Unknown key string: {trimmed}"))),
    }
  }
}

/// Convert a string representation into a `Key`.
#[napi]
pub fn key_from_string(value: String) -> napi::Result<Key> {
  value.try_into()
}

impl TryFrom<Key> for SimulangKey {
  type Error = Error;

  #[allow(clippy::too_many_lines)]
  fn try_from(value: Key) -> Result<Self, Self::Error> {
    match value {
      Key::Unicode => Err(Error::from_reason(
        "Key::Unicode requires key_unicode(char).",
      )),
      Key::Other => Err(Error::from_reason("Key::Other requires key_other(value).")),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num0 => Ok(SimulangKey::Num0),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num1 => Ok(SimulangKey::Num1),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num2 => Ok(SimulangKey::Num2),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num3 => Ok(SimulangKey::Num3),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num4 => Ok(SimulangKey::Num4),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num5 => Ok(SimulangKey::Num5),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num6 => Ok(SimulangKey::Num6),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num7 => Ok(SimulangKey::Num7),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num8 => Ok(SimulangKey::Num8),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Num9 => Ok(SimulangKey::Num9),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::A => Ok(SimulangKey::A),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::B => Ok(SimulangKey::B),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::C => Ok(SimulangKey::C),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::D => Ok(SimulangKey::D),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::E => Ok(SimulangKey::E),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::F => Ok(SimulangKey::F),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::G => Ok(SimulangKey::G),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::H => Ok(SimulangKey::H),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::I => Ok(SimulangKey::I),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::J => Ok(SimulangKey::J),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::K => Ok(SimulangKey::K),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::L => Ok(SimulangKey::L),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::M => Ok(SimulangKey::M),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::N => Ok(SimulangKey::N),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::O => Ok(SimulangKey::O),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::P => Ok(SimulangKey::P),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Q => Ok(SimulangKey::Q),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::R => Ok(SimulangKey::R),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::S => Ok(SimulangKey::S),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::T => Ok(SimulangKey::T),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::U => Ok(SimulangKey::U),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::V => Ok(SimulangKey::V),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::W => Ok(SimulangKey::W),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::X => Ok(SimulangKey::X),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Y => Ok(SimulangKey::Y),
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      Key::Z => Ok(SimulangKey::Z),
      #[cfg(target_os = "windows")]
      Key::AbntC1 => Ok(SimulangKey::AbntC1),
      #[cfg(target_os = "windows")]
      Key::AbntC2 => Ok(SimulangKey::AbntC2),
      #[cfg(target_os = "windows")]
      Key::Accept => Ok(SimulangKey::Accept),
      Key::Add => Ok(SimulangKey::Add),
      Key::Alt => Ok(SimulangKey::Alt),
      #[cfg(target_os = "windows")]
      Key::Apps => Ok(SimulangKey::Apps),
      #[cfg(target_os = "windows")]
      Key::Attn => Ok(SimulangKey::Attn),
      Key::Backspace => Ok(SimulangKey::Backspace),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Break => Ok(SimulangKey::Break),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Begin => Ok(SimulangKey::Begin),
      #[cfg(target_os = "macos")]
      Key::BrightnessDown => Ok(SimulangKey::BrightnessDown),
      #[cfg(target_os = "macos")]
      Key::BrightnessUp => Ok(SimulangKey::BrightnessUp),
      #[cfg(target_os = "windows")]
      Key::BrowserBack => Ok(SimulangKey::BrowserBack),
      #[cfg(target_os = "windows")]
      Key::BrowserFavorites => Ok(SimulangKey::BrowserFavorites),
      #[cfg(target_os = "windows")]
      Key::BrowserForward => Ok(SimulangKey::BrowserForward),
      #[cfg(target_os = "windows")]
      Key::BrowserHome => Ok(SimulangKey::BrowserHome),
      #[cfg(target_os = "windows")]
      Key::BrowserRefresh => Ok(SimulangKey::BrowserRefresh),
      #[cfg(target_os = "windows")]
      Key::BrowserSearch => Ok(SimulangKey::BrowserSearch),
      #[cfg(target_os = "windows")]
      Key::BrowserStop => Ok(SimulangKey::BrowserStop),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Cancel => Ok(SimulangKey::Cancel),
      Key::CapsLock => Ok(SimulangKey::CapsLock),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Clear => Ok(SimulangKey::Clear),
      #[cfg(target_os = "macos")]
      Key::ContrastUp => Ok(SimulangKey::ContrastUp),
      #[cfg(target_os = "macos")]
      Key::ContrastDown => Ok(SimulangKey::ContrastDown),
      Key::Control => Ok(SimulangKey::Control),
      #[cfg(target_os = "windows")]
      Key::Convert => Ok(SimulangKey::Convert),
      #[cfg(target_os = "windows")]
      Key::Crsel => Ok(SimulangKey::Crsel),
      #[cfg(target_os = "windows")]
      Key::DBEAlphanumeric => Ok(SimulangKey::DBEAlphanumeric),
      #[cfg(target_os = "windows")]
      Key::DBECodeinput => Ok(SimulangKey::DBECodeinput),
      #[cfg(target_os = "windows")]
      Key::DBEDetermineString => Ok(SimulangKey::DBEDetermineString),
      #[cfg(target_os = "windows")]
      Key::DBEEnterDLGConversionMode => Ok(SimulangKey::DBEEnterDLGConversionMode),
      #[cfg(target_os = "windows")]
      Key::DBEEnterIMEConfigMode => Ok(SimulangKey::DBEEnterIMEConfigMode),
      #[cfg(target_os = "windows")]
      Key::DBEEnterWordRegisterMode => Ok(SimulangKey::DBEEnterWordRegisterMode),
      #[cfg(target_os = "windows")]
      Key::DBEFlushString => Ok(SimulangKey::DBEFlushString),
      #[cfg(target_os = "windows")]
      Key::DBEHiragana => Ok(SimulangKey::DBEHiragana),
      #[cfg(target_os = "windows")]
      Key::DBEKatakana => Ok(SimulangKey::DBEKatakana),
      #[cfg(target_os = "windows")]
      Key::DBENoCodepoint => Ok(SimulangKey::DBENoCodepoint),
      #[cfg(target_os = "windows")]
      Key::DBENoRoman => Ok(SimulangKey::DBENoRoman),
      #[cfg(target_os = "windows")]
      Key::DBERoman => Ok(SimulangKey::DBERoman),
      #[cfg(target_os = "windows")]
      Key::DBESBCSChar => Ok(SimulangKey::DBESBCSChar),
      #[cfg(target_os = "windows")]
      Key::DBESChar => Ok(SimulangKey::DBESChar),
      Key::Decimal => Ok(SimulangKey::Decimal),
      Key::Delete => Ok(SimulangKey::Delete),
      Key::Divide => Ok(SimulangKey::Divide),
      Key::DownArrow => Ok(SimulangKey::DownArrow),
      #[cfg(target_os = "macos")]
      Key::Eject => Ok(SimulangKey::Eject),
      Key::End => Ok(SimulangKey::End),
      #[cfg(target_os = "windows")]
      Key::Ereof => Ok(SimulangKey::Ereof),
      Key::Escape => Ok(SimulangKey::Escape),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Execute => Ok(SimulangKey::Execute),
      #[cfg(target_os = "windows")]
      Key::Exsel => Ok(SimulangKey::Exsel),
      Key::F1 => Ok(SimulangKey::F1),
      Key::F2 => Ok(SimulangKey::F2),
      Key::F3 => Ok(SimulangKey::F3),
      Key::F4 => Ok(SimulangKey::F4),
      Key::F5 => Ok(SimulangKey::F5),
      Key::F6 => Ok(SimulangKey::F6),
      Key::F7 => Ok(SimulangKey::F7),
      Key::F8 => Ok(SimulangKey::F8),
      Key::F9 => Ok(SimulangKey::F9),
      Key::F10 => Ok(SimulangKey::F10),
      Key::F11 => Ok(SimulangKey::F11),
      Key::F12 => Ok(SimulangKey::F12),
      Key::F13 => Ok(SimulangKey::F13),
      Key::F14 => Ok(SimulangKey::F14),
      Key::F15 => Ok(SimulangKey::F15),
      Key::F16 => Ok(SimulangKey::F16),
      Key::F17 => Ok(SimulangKey::F17),
      Key::F18 => Ok(SimulangKey::F18),
      Key::F19 => Ok(SimulangKey::F19),
      Key::F20 => Ok(SimulangKey::F20),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::F21 => Ok(SimulangKey::F21),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::F22 => Ok(SimulangKey::F22),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::F23 => Ok(SimulangKey::F23),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::F24 => Ok(SimulangKey::F24),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F25 => Ok(SimulangKey::F25),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F26 => Ok(SimulangKey::F26),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F27 => Ok(SimulangKey::F27),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F28 => Ok(SimulangKey::F28),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F29 => Ok(SimulangKey::F29),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F30 => Ok(SimulangKey::F30),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F31 => Ok(SimulangKey::F31),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F32 => Ok(SimulangKey::F32),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F33 => Ok(SimulangKey::F33),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F34 => Ok(SimulangKey::F34),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::F35 => Ok(SimulangKey::F35),
      #[cfg(target_os = "macos")]
      Key::Function => Ok(SimulangKey::Function),
      #[cfg(target_os = "windows")]
      Key::Final => Ok(SimulangKey::Final),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Find => Ok(SimulangKey::Find),
      #[cfg(target_os = "windows")]
      Key::GamepadA => Ok(SimulangKey::GamepadA),
      #[cfg(target_os = "windows")]
      Key::GamepadB => Ok(SimulangKey::GamepadB),
      #[cfg(target_os = "windows")]
      Key::GamepadDPadDown => Ok(SimulangKey::GamepadDPadDown),
      #[cfg(target_os = "windows")]
      Key::GamepadDPadLeft => Ok(SimulangKey::GamepadDPadLeft),
      #[cfg(target_os = "windows")]
      Key::GamepadDPadRight => Ok(SimulangKey::GamepadDPadRight),
      #[cfg(target_os = "windows")]
      Key::GamepadDPadUp => Ok(SimulangKey::GamepadDPadUp),
      #[cfg(target_os = "windows")]
      Key::GamepadLeftShoulder => Ok(SimulangKey::GamepadLeftShoulder),
      #[cfg(target_os = "windows")]
      Key::GamepadLeftThumbstickButton => Ok(SimulangKey::GamepadLeftThumbstickButton),
      #[cfg(target_os = "windows")]
      Key::GamepadLeftThumbstickDown => Ok(SimulangKey::GamepadLeftThumbstickDown),
      #[cfg(target_os = "windows")]
      Key::GamepadLeftThumbstickLeft => Ok(SimulangKey::GamepadLeftThumbstickLeft),
      #[cfg(target_os = "windows")]
      Key::GamepadLeftThumbstickRight => Ok(SimulangKey::GamepadLeftThumbstickRight),
      #[cfg(target_os = "windows")]
      Key::GamepadLeftThumbstickUp => Ok(SimulangKey::GamepadLeftThumbstickUp),
      #[cfg(target_os = "windows")]
      Key::GamepadLeftTrigger => Ok(SimulangKey::GamepadLeftTrigger),
      #[cfg(target_os = "windows")]
      Key::GamepadMenu => Ok(SimulangKey::GamepadMenu),
      #[cfg(target_os = "windows")]
      Key::GamepadRightShoulder => Ok(SimulangKey::GamepadRightShoulder),
      #[cfg(target_os = "windows")]
      Key::GamepadRightThumbstickButton => Ok(SimulangKey::GamepadRightThumbstickButton),
      #[cfg(target_os = "windows")]
      Key::GamepadRightThumbstickDown => Ok(SimulangKey::GamepadRightThumbstickDown),
      #[cfg(target_os = "windows")]
      Key::GamepadRightThumbstickLeft => Ok(SimulangKey::GamepadRightThumbstickLeft),
      #[cfg(target_os = "windows")]
      Key::GamepadRightThumbstickRight => Ok(SimulangKey::GamepadRightThumbstickRight),
      #[cfg(target_os = "windows")]
      Key::GamepadRightThumbstickUp => Ok(SimulangKey::GamepadRightThumbstickUp),
      #[cfg(target_os = "windows")]
      Key::GamepadRightTrigger => Ok(SimulangKey::GamepadRightTrigger),
      #[cfg(target_os = "windows")]
      Key::GamepadView => Ok(SimulangKey::GamepadView),
      #[cfg(target_os = "windows")]
      Key::GamepadX => Ok(SimulangKey::GamepadX),
      #[cfg(target_os = "windows")]
      Key::GamepadY => Ok(SimulangKey::GamepadY),
      #[cfg(target_os = "windows")]
      Key::Hangeul => Ok(SimulangKey::Hangeul),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Hangul => Ok(SimulangKey::Hangul),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Hanja => Ok(SimulangKey::Hanja),
      Key::Help => Ok(SimulangKey::Help),
      Key::Home => Ok(SimulangKey::Home),
      #[cfg(target_os = "windows")]
      Key::Ico00 => Ok(SimulangKey::Ico00),
      #[cfg(target_os = "windows")]
      Key::IcoClear => Ok(SimulangKey::IcoClear),
      #[cfg(target_os = "windows")]
      Key::IcoHelp => Ok(SimulangKey::IcoHelp),
      #[cfg(target_os = "macos")]
      Key::IlluminationDown => Ok(SimulangKey::IlluminationDown),
      #[cfg(target_os = "macos")]
      Key::IlluminationUp => Ok(SimulangKey::IlluminationUp),
      #[cfg(target_os = "macos")]
      Key::IlluminationToggle => Ok(SimulangKey::IlluminationToggle),
      #[cfg(target_os = "windows")]
      Key::IMEOff => Ok(SimulangKey::IMEOff),
      #[cfg(target_os = "windows")]
      Key::IMEOn => Ok(SimulangKey::IMEOn),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Insert => Ok(SimulangKey::Insert),
      #[cfg(target_os = "windows")]
      Key::Junja => Ok(SimulangKey::Junja),
      #[cfg(target_os = "windows")]
      Key::Kana => Ok(SimulangKey::Kana),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Kanji => Ok(SimulangKey::Kanji),
      #[cfg(target_os = "windows")]
      Key::LaunchApp1 => Ok(SimulangKey::LaunchApp1),
      #[cfg(target_os = "windows")]
      Key::LaunchApp2 => Ok(SimulangKey::LaunchApp2),
      #[cfg(target_os = "windows")]
      Key::LaunchMail => Ok(SimulangKey::LaunchMail),
      #[cfg(target_os = "windows")]
      Key::LaunchMediaSelect => Ok(SimulangKey::LaunchMediaSelect),
      #[cfg(target_os = "macos")]
      Key::Launchpad => Ok(SimulangKey::Launchpad),
      #[cfg(target_os = "macos")]
      Key::LaunchPanel => Ok(SimulangKey::LaunchPanel),
      #[cfg(target_os = "windows")]
      Key::LButton => Ok(SimulangKey::LButton),
      Key::LControl => Ok(SimulangKey::LControl),
      Key::LeftArrow => Ok(SimulangKey::LeftArrow),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Linefeed => Ok(SimulangKey::Linefeed),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::LMenu => Ok(SimulangKey::LMenu),
      Key::LShift => Ok(SimulangKey::LShift),
      #[cfg(target_os = "windows")]
      Key::LWin => Ok(SimulangKey::LWin),
      #[cfg(target_os = "windows")]
      Key::MButton => Ok(SimulangKey::MButton),
      #[cfg(target_os = "macos")]
      Key::MediaFast => Ok(SimulangKey::MediaFast),
      Key::MediaNextTrack => Ok(SimulangKey::MediaNextTrack),
      Key::MediaPlayPause => Ok(SimulangKey::MediaPlayPause),
      Key::MediaPrevTrack => Ok(SimulangKey::MediaPrevTrack),
      #[cfg(target_os = "macos")]
      Key::MediaRewind => Ok(SimulangKey::MediaRewind),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::MediaStop => Ok(SimulangKey::MediaStop),
      Key::Meta => Ok(SimulangKey::Meta),
      #[cfg(target_os = "macos")]
      Key::MissionControl => Ok(SimulangKey::MissionControl),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::ModeChange => Ok(SimulangKey::ModeChange),
      Key::Multiply => Ok(SimulangKey::Multiply),
      #[cfg(target_os = "windows")]
      Key::NavigationAccept => Ok(SimulangKey::NavigationAccept),
      #[cfg(target_os = "windows")]
      Key::NavigationCancel => Ok(SimulangKey::NavigationCancel),
      #[cfg(target_os = "windows")]
      Key::NavigationDown => Ok(SimulangKey::NavigationDown),
      #[cfg(target_os = "windows")]
      Key::NavigationLeft => Ok(SimulangKey::NavigationLeft),
      #[cfg(target_os = "windows")]
      Key::NavigationMenu => Ok(SimulangKey::NavigationMenu),
      #[cfg(target_os = "windows")]
      Key::NavigationRight => Ok(SimulangKey::NavigationRight),
      #[cfg(target_os = "windows")]
      Key::NavigationUp => Ok(SimulangKey::NavigationUp),
      #[cfg(target_os = "windows")]
      Key::NavigationView => Ok(SimulangKey::NavigationView),
      #[cfg(target_os = "windows")]
      Key::NoName => Ok(SimulangKey::NoName),
      #[cfg(target_os = "windows")]
      Key::NonConvert => Ok(SimulangKey::NonConvert),
      #[cfg(target_os = "windows")]
      Key::None => Ok(SimulangKey::None),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Numlock => Ok(SimulangKey::Numlock),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::NumpadEnter => Ok(SimulangKey::NumpadEnter),
      Key::Numpad0 => Ok(SimulangKey::Numpad0),
      Key::Numpad1 => Ok(SimulangKey::Numpad1),
      Key::Numpad2 => Ok(SimulangKey::Numpad2),
      Key::Numpad3 => Ok(SimulangKey::Numpad3),
      Key::Numpad4 => Ok(SimulangKey::Numpad4),
      Key::Numpad5 => Ok(SimulangKey::Numpad5),
      Key::Numpad6 => Ok(SimulangKey::Numpad6),
      Key::Numpad7 => Ok(SimulangKey::Numpad7),
      Key::Numpad8 => Ok(SimulangKey::Numpad8),
      Key::Numpad9 => Ok(SimulangKey::Numpad9),
      #[cfg(target_os = "windows")]
      Key::OEM1 => Ok(SimulangKey::OEM1),
      #[cfg(target_os = "windows")]
      Key::OEM102 => Ok(SimulangKey::OEM102),
      #[cfg(target_os = "windows")]
      Key::OEM2 => Ok(SimulangKey::OEM2),
      #[cfg(target_os = "windows")]
      Key::OEM3 => Ok(SimulangKey::OEM3),
      #[cfg(target_os = "windows")]
      Key::OEM4 => Ok(SimulangKey::OEM4),
      #[cfg(target_os = "windows")]
      Key::OEM5 => Ok(SimulangKey::OEM5),
      #[cfg(target_os = "windows")]
      Key::OEM6 => Ok(SimulangKey::OEM6),
      #[cfg(target_os = "windows")]
      Key::OEM7 => Ok(SimulangKey::OEM7),
      #[cfg(target_os = "windows")]
      Key::OEM8 => Ok(SimulangKey::OEM8),
      #[cfg(target_os = "windows")]
      Key::OEMAttn => Ok(SimulangKey::OEMAttn),
      #[cfg(target_os = "windows")]
      Key::OEMAuto => Ok(SimulangKey::OEMAuto),
      #[cfg(target_os = "windows")]
      Key::OEMAx => Ok(SimulangKey::OEMAx),
      #[cfg(target_os = "windows")]
      Key::OEMBacktab => Ok(SimulangKey::OEMBacktab),
      #[cfg(target_os = "windows")]
      Key::OEMClear => Ok(SimulangKey::OEMClear),
      #[cfg(target_os = "windows")]
      Key::OEMComma => Ok(SimulangKey::OEMComma),
      #[cfg(target_os = "windows")]
      Key::OEMCopy => Ok(SimulangKey::OEMCopy),
      #[cfg(target_os = "windows")]
      Key::OEMCusel => Ok(SimulangKey::OEMCusel),
      #[cfg(target_os = "windows")]
      Key::OEMEnlw => Ok(SimulangKey::OEMEnlw),
      #[cfg(target_os = "windows")]
      Key::OEMFinish => Ok(SimulangKey::OEMFinish),
      #[cfg(target_os = "windows")]
      Key::OEMFJJisho => Ok(SimulangKey::OEMFJJisho),
      #[cfg(target_os = "windows")]
      Key::OEMFJLoya => Ok(SimulangKey::OEMFJLoya),
      #[cfg(target_os = "windows")]
      Key::OEMFJMasshou => Ok(SimulangKey::OEMFJMasshou),
      #[cfg(target_os = "windows")]
      Key::OEMFJRoya => Ok(SimulangKey::OEMFJRoya),
      #[cfg(target_os = "windows")]
      Key::OEMFJTouroku => Ok(SimulangKey::OEMFJTouroku),
      #[cfg(target_os = "windows")]
      Key::OEMJump => Ok(SimulangKey::OEMJump),
      #[cfg(target_os = "windows")]
      Key::OEMMinus => Ok(SimulangKey::OEMMinus),
      #[cfg(target_os = "windows")]
      Key::OEMNECEqual => Ok(SimulangKey::OEMNECEqual),
      #[cfg(target_os = "windows")]
      Key::OEMPA1 => Ok(SimulangKey::OEMPA1),
      #[cfg(target_os = "windows")]
      Key::OEMPA2 => Ok(SimulangKey::OEMPA2),
      #[cfg(target_os = "windows")]
      Key::OEMPA3 => Ok(SimulangKey::OEMPA3),
      #[cfg(target_os = "windows")]
      Key::OEMPeriod => Ok(SimulangKey::OEMPeriod),
      #[cfg(target_os = "windows")]
      Key::OEMPlus => Ok(SimulangKey::OEMPlus),
      #[cfg(target_os = "windows")]
      Key::OEMReset => Ok(SimulangKey::OEMReset),
      #[cfg(target_os = "windows")]
      Key::OEMWsctrl => Ok(SimulangKey::OEMWsctrl),
      Key::Option => Ok(SimulangKey::Option),
      #[cfg(target_os = "windows")]
      Key::PA1 => Ok(SimulangKey::PA1),
      #[cfg(target_os = "windows")]
      Key::Packet => Ok(SimulangKey::Packet),
      Key::PageDown => Ok(SimulangKey::PageDown),
      Key::PageUp => Ok(SimulangKey::PageUp),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Pause => Ok(SimulangKey::Pause),
      #[cfg(target_os = "windows")]
      Key::Play => Ok(SimulangKey::Play),
      #[cfg(target_os = "macos")]
      Key::Power => Ok(SimulangKey::Power),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::PrintScr => Ok(SimulangKey::PrintScr),
      #[cfg(target_os = "windows")]
      Key::Processkey => Ok(SimulangKey::Processkey),
      #[cfg(target_os = "windows")]
      Key::RButton => Ok(SimulangKey::RButton),
      #[cfg(target_os = "macos")]
      Key::RCommand => Ok(SimulangKey::RCommand),
      Key::RControl => Ok(SimulangKey::RControl),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Redo => Ok(SimulangKey::Redo),
      Key::Return => Ok(SimulangKey::Return),
      Key::RightArrow => Ok(SimulangKey::RightArrow),
      #[cfg(target_os = "windows")]
      Key::RMenu => Ok(SimulangKey::RMenu),
      #[cfg(target_os = "macos")]
      Key::ROption => Ok(SimulangKey::ROption),
      Key::RShift => Ok(SimulangKey::RShift),
      #[cfg(target_os = "windows")]
      Key::RWin => Ok(SimulangKey::RWin),
      #[cfg(target_os = "windows")]
      Key::Scroll => Ok(SimulangKey::Scroll),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::ScrollLock => Ok(SimulangKey::ScrollLock),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Select => Ok(SimulangKey::Select),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::ScriptSwitch => Ok(SimulangKey::ScriptSwitch),
      #[cfg(target_os = "windows")]
      Key::Separator => Ok(SimulangKey::Separator),
      Key::Shift => Ok(SimulangKey::Shift),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::ShiftLock => Ok(SimulangKey::ShiftLock),
      #[cfg(target_os = "windows")]
      Key::Sleep => Ok(SimulangKey::Sleep),
      Key::Space => Ok(SimulangKey::Space),
      Key::Subtract => Ok(SimulangKey::Subtract),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::SysReq => Ok(SimulangKey::SysReq),
      Key::Tab => Ok(SimulangKey::Tab),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Undo => Ok(SimulangKey::Undo),
      Key::UpArrow => Ok(SimulangKey::UpArrow),
      #[cfg(target_os = "macos")]
      Key::VidMirror => Ok(SimulangKey::VidMirror),
      Key::VolumeDown => Ok(SimulangKey::VolumeDown),
      Key::VolumeMute => Ok(SimulangKey::VolumeMute),
      Key::VolumeUp => Ok(SimulangKey::VolumeUp),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::MicMute => Ok(SimulangKey::MicMute),
      #[cfg(target_os = "windows")]
      Key::XButton1 => Ok(SimulangKey::XButton1),
      #[cfg(target_os = "windows")]
      Key::XButton2 => Ok(SimulangKey::XButton2),
      #[cfg(target_os = "windows")]
      Key::Zoom => Ok(SimulangKey::Zoom),
      other => Err(Error::from_reason(format!(
        "Key {other:?} is not supported on this platform.",
      ))),
    }
  }
}
