use napi::Error;
use napi_derive::napi;
use simulang_rs::Key as SimulangKey;

#[napi]
#[derive(Debug, Copy, Clone)]
/// Represents a keyboard key.
pub enum Key {
  /// The `0` key in the main row of digits (not the numpad). Alias: `Digit0`.
  Num0,
  /// The `1` key in the main row of digits (not the numpad). Alias: `Digit1`.
  Num1,
  /// The `2` key in the main row of digits (not the numpad). Alias: `Digit2`.
  Num2,
  /// The `3` key in the main row of digits (not the numpad). Alias: `Digit3`.
  Num3,
  /// The `4` key in the main row of digits (not the numpad). Alias: `Digit4`.
  Num4,
  /// The `5` key in the main row of digits (not the numpad). Alias: `Digit5`.
  Num5,
  /// The `6` key in the main row of digits (not the numpad). Alias: `Digit6`.
  Num6,
  /// The `7` key in the main row of digits (not the numpad). Alias: `Digit7`.
  Num7,
  /// The `8` key in the main row of digits (not the numpad). Alias: `Digit8`.
  Num8,
  /// The `9` key in the main row of digits (not the numpad). Alias: `Digit9`.
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
  /// The context menu key. Aliases: `ContextMenu`, `Menu`.
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
  /// The IME `Convert` key (`Henkan` on Japanese keyboards). Alias: `Henkan`.
  Convert,
  Crsel,
  /// The `Alphanumeric` / `Eisu` IME key on Japanese keyboards. Aliases: `Eisu`, `Alphanumeric`.
  DBEAlphanumeric,
  /// The IME `Code Input` key. Alias: `CodeInput`.
  DBECodeinput,
  DBEDetermineString,
  DBEEnterDLGConversionMode,
  DBEEnterIMEConfigMode,
  DBEEnterWordRegisterMode,
  DBEFlushString,
  /// The `Hiragana` IME key on Japanese keyboards. Alias: `Hiragana`.
  DBEHiragana,
  /// The `Katakana` IME key on Japanese keyboards. Alias: `Katakana`.
  DBEKatakana,
  DBENoCodepoint,
  DBENoRoman,
  /// The `Romaji` IME key on Japanese keyboards. Alias: `Romaji`.
  DBERoman,
  /// The `Hankaku` (half-width characters) IME key on Japanese keyboards. Alias: `Hankaku`.
  DBESBCSChar,
  /// The `Zenkaku` (full-width characters) IME key on Japanese keyboards. Alias: `Zenkaku`.
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
  /// The Hangul mode key on Korean keyboards (same key as `Hangul`). Alias: `HangulMode`.
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
  /// The Junja mode key on Korean keyboards. Alias: `JunjaMode`.
  Junja,
  /// The Kana mode (Kana lock) key on Japanese keyboards. Alias: `KanaMode`.
  Kana,
  /// The Kanji mode key on Japanese keyboards (the `kana` key on JIS Macs). Alias: `KanjiMode`.
  Kanji,
  /// The key that launches the first generic application. Alias: `LaunchApplication1`.
  LaunchApp1,
  /// The key that launches the second generic application. Alias: `LaunchApplication2`.
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
  /// The left Alt key (left Option key on macOS). On Linux this presses left Alt; it used to press the context-menu key, which is `Apps`. Alias: `AltLeft`.
  LMenu,
  LShift,
  /// The left Windows key (left Command key on macOS, left Super key on Linux). Alias: `MetaLeft`.
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
  /// The IME `NonConvert` key (`Muhenkan` on Japanese keyboards). Alias: `Muhenkan`.
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
  /// The `;` `:` key on a US layout. Alias: `Semicolon`.
  OEM1,
  /// The key left of `Z` on ISO keyboards. Alias: `IntlBackslash`.
  OEM102,
  /// The `/` `?` key on a US layout. Alias: `Slash`.
  OEM2,
  /// The `` ` `` `~` key on a US layout. Aliases: `Backquote`, `Grave`.
  OEM3,
  /// The `[` `{` key on a US layout. Alias: `BracketLeft`.
  OEM4,
  /// The `\` `|` key on a US layout. Alias: `Backslash`.
  OEM5,
  /// The `]` `}` key on a US layout. Alias: `BracketRight`.
  OEM6,
  /// The `'` `"` key on a US layout. Aliases: `Quote`, `Apostrophe`.
  OEM7,
  OEM8,
  OEMAttn,
  OEMAuto,
  OEMAx,
  OEMBacktab,
  OEMClear,
  /// The `,` `<` key on a US layout. Alias: `Comma`.
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
  /// The `-` `_` key on a US layout. Alias: `Minus`.
  OEMMinus,
  OEMNECEqual,
  OEMPA1,
  OEMPA2,
  OEMPA3,
  /// The `.` `>` key on a US layout. Alias: `Period`.
  OEMPeriod,
  /// The `=` `+` key on a US layout. Alias: `Equal`.
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
  /// The right Alt key (right Option key on macOS). Alias: `AltRight`.
  RMenu,
  ROption,
  RShift,
  /// The right Windows key (right Command key on macOS, right Super key on Linux). Alias: `MetaRight`.
  RWin,
  Scroll,
  ScrollLock,
  Select,
  ScriptSwitch,
  /// The places separator key on the numpad. Alias: `NumpadComma`.
  Separator,
  /// shift key
  Shift,
  /// Lock shift key
  ShiftLock,
  /// The sleep / standby key. Alias: `Standby`.
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
  /// The `AltGr` / `AltGraph` key (ISO level 3 shift). Linux only. Aliases: `AltGr`, `ISO_Level3_Shift`.
  AltGraph,
  /// The `Hyper` modifier key. Linux only. Alias: `Hyper_L`.
  Hyper,
  /// The `Copy` key found on some extended keyboards. Linux only.
  Copy,
  /// The `Cut` key found on some extended keyboards. Linux only.
  Cut,
  /// The `Paste` key found on some extended keyboards. Linux only.
  Paste,
  /// The `Zoom In` key. Linux only. Not the same key as `Zoom`.
  ZoomIn,
  /// The `Zoom Out` key. Linux only.
  ZoomOut,
  /// The `Log Off` key. Linux only.
  LogOff,
  /// The `Hibernate` key. Linux only.
  Hibernate,
  /// The `Wake Up` key. Linux only.
  WakeUp,
  /// The `All Candidates` IME key. Linux only. Alias: `MultipleCandidate`.
  AllCandidates,
  /// The `Compose` key. Linux only. Alias: `Multi_key`.
  Compose,
  /// The IME `Previous Candidate` key. Linux only.
  PreviousCandidate,
  /// The IME `Single Candidate` key. Linux only.
  SingleCandidate,
  /// Switch to the first character group on an ISO/IEC 9995 keyboard. Linux only. Alias: `ISO_First_Group`.
  GroupFirst,
  /// Switch to the last character group on an ISO/IEC 9995 keyboard. Linux only. Alias: `ISO_Last_Group`.
  GroupLast,
  /// Switch to the next character group on an ISO/IEC 9995 keyboard. Linux only. Alias: `ISO_Next_Group`.
  GroupNext,
  /// Switch to the previous character group on an ISO/IEC 9995 keyboard. Linux only. Alias: `ISO_Prev_Group`.
  GroupPrevious,
  /// The `Hiragana/Katakana` toggle key on Japanese keyboards. Linux only. Alias: `Hiragana_Katakana`.
  HiraganaKatakana,
  /// The `Zenkaku/Hankaku` toggle key on Japanese keyboards. Linux only. Alias: `Zenkaku_Hankaku`.
  ZenkakuHankaku,
  /// Media play. Not the play/pause toggle (`MediaPlayPause`) and not the application `Play` key. Linux only.
  MediaPlay,
  /// Media pause. Not the play/pause toggle (`MediaPlayPause`). Linux only.
  MediaPause,
  /// Media record. Linux only.
  MediaRecord,
  /// The `Close` key found on some extended keyboards. Linux only.
  Close,
  /// The `New` (new document) key found on some extended keyboards. Linux only.
  New,
  /// The `Open` (open document) key found on some extended keyboards. Linux only.
  Open,
  /// The `Save` key found on some extended keyboards. Linux only.
  Save,
  /// The `Spell Check` key found on some extended keyboards. Linux only. Alias: `Spell`.
  SpellCheck,
  /// The `Mail Forward` key. Linux only.
  MailForward,
  /// The `Mail Reply` key. Linux only. Alias: `Reply`.
  MailReply,
  /// The `Mail Send` key. Linux only. Alias: `Send`.
  MailSend,
  /// The calculator launcher key. Linux only. Alias: `Calculator`.
  LaunchCalculator,
  /// The calendar launcher key. Linux only. Alias: `Calendar`.
  LaunchCalendar,
  /// The music player launcher key. Linux only. Alias: `Music`.
  LaunchMusicPlayer,
  /// The `My Computer` launcher key. Linux only. Alias: `MyComputer`.
  LaunchMyComputer,
  /// The phone launcher key. Linux only. Alias: `Phone`.
  LaunchPhone,
  /// The screen saver launcher key. Linux only. Alias: `ScreenSaver`.
  LaunchScreenSaver,
  /// The spreadsheet launcher key. Linux only. Alias: `Excel`.
  LaunchSpreadsheet,
  /// The web browser launcher key. Linux only. Alias: `WWW`.
  LaunchWebBrowser,
  /// The webcam launcher key. Linux only. Alias: `WebCam`.
  LaunchWebCam,
  /// The word processor launcher key. Linux only. Alias: `Word`.
  LaunchWordProcessor,
  /// Third generic application launcher key. Linux only. Aliases: `LaunchApplication3`, `Launch2`.
  LaunchApp3,
  /// Fourth generic application launcher key. Linux only. Aliases: `LaunchApplication4`, `Launch3`.
  LaunchApp4,
  /// Fifth generic application launcher key. Linux only. Aliases: `LaunchApplication5`, `Launch4`.
  LaunchApp5,
  /// Sixth generic application launcher key. Linux only. Aliases: `LaunchApplication6`, `Launch5`.
  LaunchApp6,
  /// Seventh generic application launcher key. Linux only. Aliases: `LaunchApplication7`, `Launch6`.
  LaunchApp7,
  /// Eighth generic application launcher key. Linux only. Aliases: `LaunchApplication8`, `Launch7`.
  LaunchApp8,
  /// Ninth generic application launcher key. Linux only. Aliases: `LaunchApplication9`, `Launch8`.
  LaunchApp9,
  /// Tenth generic application launcher key. Linux only. Aliases: `LaunchApplication10`, `Launch9`.
  LaunchApp10,
  /// Eleventh generic application launcher key. Linux only. Aliases: `LaunchApplication11`, `LaunchA`.
  LaunchApp11,
  /// Twelfth generic application launcher key. Linux only. Aliases: `LaunchApplication12`, `LaunchB`.
  LaunchApp12,
  /// Thirteenth generic application launcher key. Linux only. Aliases: `LaunchApplication13`, `LaunchC`.
  LaunchApp13,
  /// Fourteenth generic application launcher key. Linux only. Aliases: `LaunchApplication14`, `LaunchD`.
  LaunchApp14,
  /// Fifteenth generic application launcher key. Linux only. Aliases: `LaunchApplication15`, `LaunchE`.
  LaunchApp15,
  /// Sixteenth generic application launcher key. Linux only. Aliases: `LaunchApplication16`, `LaunchF`.
  LaunchApp16,
}

impl TryFrom<String> for Key {
  type Error = Error;

  #[allow(clippy::too_many_lines)]
  fn try_from(value: String) -> Result<Self, Self::Error> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
      return Err(Error::from_reason("Key string cannot be empty."));
    }

    // A single punctuation character is the key, not text. Stripping
    // `-` below would otherwise turn "-" into an empty name.
    if trimmed.chars().count() == 1 {
      match trimmed {
        "-" => return Ok(Key::OEMMinus),
        "=" => return Ok(Key::OEMPlus),
        "," => return Ok(Key::OEMComma),
        "." => return Ok(Key::OEMPeriod),
        ";" => return Ok(Key::OEM1),
        "/" => return Ok(Key::OEM2),
        "`" => return Ok(Key::OEM3),
        "[" => return Ok(Key::OEM4),
        "\\" => return Ok(Key::OEM5),
        "]" => return Ok(Key::OEM6),
        "'" => return Ok(Key::OEM7),
        _ => {}
      }
    }

    let normalized = trimmed.to_ascii_lowercase();
    let normalized: String = normalized
      .chars()
      .filter(|ch| !matches!(ch, '_' | '-' | ' '))
      .collect();

    match normalized.as_str() {
      "num0" | "0" | "digit0" => Ok(Key::Num0),
      "num1" | "1" | "digit1" => Ok(Key::Num1),
      "num2" | "2" | "digit2" => Ok(Key::Num2),
      "num3" | "3" | "digit3" => Ok(Key::Num3),
      "num4" | "4" | "digit4" => Ok(Key::Num4),
      "num5" | "5" | "digit5" => Ok(Key::Num5),
      "num6" | "6" | "digit6" => Ok(Key::Num6),
      "num7" | "7" | "digit7" => Ok(Key::Num7),
      "num8" | "8" | "digit8" => Ok(Key::Num8),
      "num9" | "9" | "digit9" => Ok(Key::Num9),
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
      "apps" | "contextmenu" | "menu" => Ok(Key::Apps),
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
      "convert" | "henkan" => Ok(Key::Convert),
      "crsel" => Ok(Key::Crsel),
      "dbealphanumeric" | "eisu" | "alphanumeric" => Ok(Key::DBEAlphanumeric),
      "dbecodeinput" | "codeinput" => Ok(Key::DBECodeinput),
      "dbedeterminestring" => Ok(Key::DBEDetermineString),
      "dbeenterdlgconversionmode" => Ok(Key::DBEEnterDLGConversionMode),
      "dbeenterimeconfigmode" => Ok(Key::DBEEnterIMEConfigMode),
      "dbeenterwordregistermode" => Ok(Key::DBEEnterWordRegisterMode),
      "dbeflushstring" => Ok(Key::DBEFlushString),
      "dbehiragana" | "hiragana" => Ok(Key::DBEHiragana),
      "dbekatakana" | "katakana" => Ok(Key::DBEKatakana),
      "dbenocodepoint" => Ok(Key::DBENoCodepoint),
      "dbenoroman" => Ok(Key::DBENoRoman),
      "dberoman" | "romaji" => Ok(Key::DBERoman),
      "dbesbcschar" | "hankaku" => Ok(Key::DBESBCSChar),
      "dbeschar" | "zenkaku" => Ok(Key::DBESChar),
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
      "hangeul" | "hangulmode" => Ok(Key::Hangeul),
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
      "junja" | "junjamode" => Ok(Key::Junja),
      "kana" | "kanamode" => Ok(Key::Kana),
      "kanji" | "kanjimode" => Ok(Key::Kanji),
      "launchapp1" | "launchapplication1" => Ok(Key::LaunchApp1),
      "launchapp2" | "launchapplication2" => Ok(Key::LaunchApp2),
      "launchmail" => Ok(Key::LaunchMail),
      "launchmediaselect" | "launchmediaplayer" => Ok(Key::LaunchMediaSelect),
      "launchpad" => Ok(Key::Launchpad),
      "launchpanel" => Ok(Key::LaunchPanel),
      "lbutton" => Ok(Key::LButton),
      "lcontrol" => Ok(Key::LControl),
      "leftarrow" | "left" => Ok(Key::LeftArrow),
      "linefeed" => Ok(Key::Linefeed),
      "lmenu" | "altleft" => Ok(Key::LMenu),
      "lshift" => Ok(Key::LShift),
      "lwin" | "metaleft" => Ok(Key::LWin),
      "mbutton" => Ok(Key::MButton),
      "mediafast" | "mediafastforward" => Ok(Key::MediaFast),
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
      "nonconvert" | "muhenkan" => Ok(Key::NonConvert),
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
      "oem1" | "semicolon" => Ok(Key::OEM1),
      "oem102" | "intlbackslash" => Ok(Key::OEM102),
      "oem2" | "slash" => Ok(Key::OEM2),
      "oem3" | "backquote" | "grave" => Ok(Key::OEM3),
      "oem4" | "bracketleft" => Ok(Key::OEM4),
      "oem5" | "backslash" => Ok(Key::OEM5),
      "oem6" | "bracketright" => Ok(Key::OEM6),
      "oem7" | "quote" | "apostrophe" => Ok(Key::OEM7),
      "oem8" => Ok(Key::OEM8),
      "oemattn" => Ok(Key::OEMAttn),
      "oemauto" => Ok(Key::OEMAuto),
      "oemax" => Ok(Key::OEMAx),
      "oembacktab" => Ok(Key::OEMBacktab),
      "oemclear" => Ok(Key::OEMClear),
      "oemcomma" | "comma" => Ok(Key::OEMComma),
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
      "oemminus" | "minus" => Ok(Key::OEMMinus),
      "oemnecequal" => Ok(Key::OEMNECEqual),
      "oempa1" => Ok(Key::OEMPA1),
      "oempa2" => Ok(Key::OEMPA2),
      "oempa3" => Ok(Key::OEMPA3),
      "oemperiod" | "period" => Ok(Key::OEMPeriod),
      "oemplus" | "equal" => Ok(Key::OEMPlus),
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
      "rmenu" | "altright" => Ok(Key::RMenu),
      "roption" => Ok(Key::ROption),
      "rshift" => Ok(Key::RShift),
      "rwin" | "metaright" => Ok(Key::RWin),
      "scroll" => Ok(Key::Scroll),
      "scrolllock" => Ok(Key::ScrollLock),
      "select" => Ok(Key::Select),
      "scriptswitch" => Ok(Key::ScriptSwitch),
      "separator" | "numpadcomma" => Ok(Key::Separator),
      "shift" => Ok(Key::Shift),
      "shiftlock" => Ok(Key::ShiftLock),
      "sleep" | "standby" => Ok(Key::Sleep),
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
      "altgraph" | "altgr" | "isolevel3shift" => Ok(Key::AltGraph),
      "hyper" | "hyperl" => Ok(Key::Hyper),
      "copy" => Ok(Key::Copy),
      "cut" => Ok(Key::Cut),
      "paste" => Ok(Key::Paste),
      "zoomin" => Ok(Key::ZoomIn),
      "zoomout" => Ok(Key::ZoomOut),
      "logoff" => Ok(Key::LogOff),
      "hibernate" => Ok(Key::Hibernate),
      "wakeup" => Ok(Key::WakeUp),
      "allcandidates" | "multiplecandidate" => Ok(Key::AllCandidates),
      "compose" | "multikey" => Ok(Key::Compose),
      "previouscandidate" => Ok(Key::PreviousCandidate),
      "singlecandidate" => Ok(Key::SingleCandidate),
      "groupfirst" | "isofirstgroup" => Ok(Key::GroupFirst),
      "grouplast" | "isolastgroup" => Ok(Key::GroupLast),
      "groupnext" | "isonextgroup" => Ok(Key::GroupNext),
      "groupprevious" | "isoprevgroup" => Ok(Key::GroupPrevious),
      "hiraganakatakana" => Ok(Key::HiraganaKatakana),
      "zenkakuhankaku" => Ok(Key::ZenkakuHankaku),
      "mediaplay" => Ok(Key::MediaPlay),
      "mediapause" => Ok(Key::MediaPause),
      "mediarecord" => Ok(Key::MediaRecord),
      "close" => Ok(Key::Close),
      "new" => Ok(Key::New),
      "open" => Ok(Key::Open),
      "save" => Ok(Key::Save),
      "spellcheck" | "spell" => Ok(Key::SpellCheck),
      "mailforward" => Ok(Key::MailForward),
      "mailreply" | "reply" => Ok(Key::MailReply),
      "mailsend" | "send" => Ok(Key::MailSend),
      "launchcalculator" | "calculator" => Ok(Key::LaunchCalculator),
      "launchcalendar" | "calendar" => Ok(Key::LaunchCalendar),
      "launchmusicplayer" | "music" => Ok(Key::LaunchMusicPlayer),
      "launchmycomputer" | "mycomputer" => Ok(Key::LaunchMyComputer),
      "launchphone" | "phone" => Ok(Key::LaunchPhone),
      "launchscreensaver" | "screensaver" => Ok(Key::LaunchScreenSaver),
      "launchspreadsheet" | "excel" => Ok(Key::LaunchSpreadsheet),
      "launchwebbrowser" | "www" => Ok(Key::LaunchWebBrowser),
      "launchwebcam" | "webcam" => Ok(Key::LaunchWebCam),
      "launchwordprocessor" | "word" => Ok(Key::LaunchWordProcessor),
      "launchapp3" | "launchapplication3" | "launch2" => Ok(Key::LaunchApp3),
      "launchapp4" | "launchapplication4" | "launch3" => Ok(Key::LaunchApp4),
      "launchapp5" | "launchapplication5" | "launch4" => Ok(Key::LaunchApp5),
      "launchapp6" | "launchapplication6" | "launch5" => Ok(Key::LaunchApp6),
      "launchapp7" | "launchapplication7" | "launch6" => Ok(Key::LaunchApp7),
      "launchapp8" | "launchapplication8" | "launch7" => Ok(Key::LaunchApp8),
      "launchapp9" | "launchapplication9" | "launch8" => Ok(Key::LaunchApp9),
      "launchapp10" | "launchapplication10" | "launch9" => Ok(Key::LaunchApp10),
      "launchapp11" | "launchapplication11" | "launcha" => Ok(Key::LaunchApp11),
      "launchapp12" | "launchapplication12" | "launchb" => Ok(Key::LaunchApp12),
      "launchapp13" | "launchapplication13" | "launchc" => Ok(Key::LaunchApp13),
      "launchapp14" | "launchapplication14" | "launchd" => Ok(Key::LaunchApp14),
      "launchapp15" | "launchapplication15" | "launche" => Ok(Key::LaunchApp15),
      "launchapp16" | "launchapplication16" | "launchf" => Ok(Key::LaunchApp16),
      _ => Err(Error::from_reason(format!("Unknown key string: {trimmed}"))),
    }
  }
}

/// Convert a string representation into a `Key`.
#[napi(catch_unwind)]
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
      Key::Num0 => Ok(SimulangKey::Num0),
      Key::Num1 => Ok(SimulangKey::Num1),
      Key::Num2 => Ok(SimulangKey::Num2),
      Key::Num3 => Ok(SimulangKey::Num3),
      Key::Num4 => Ok(SimulangKey::Num4),
      Key::Num5 => Ok(SimulangKey::Num5),
      Key::Num6 => Ok(SimulangKey::Num6),
      Key::Num7 => Ok(SimulangKey::Num7),
      Key::Num8 => Ok(SimulangKey::Num8),
      Key::Num9 => Ok(SimulangKey::Num9),
      Key::A => Ok(SimulangKey::A),
      Key::B => Ok(SimulangKey::B),
      Key::C => Ok(SimulangKey::C),
      Key::D => Ok(SimulangKey::D),
      Key::E => Ok(SimulangKey::E),
      Key::F => Ok(SimulangKey::F),
      Key::G => Ok(SimulangKey::G),
      Key::H => Ok(SimulangKey::H),
      Key::I => Ok(SimulangKey::I),
      Key::J => Ok(SimulangKey::J),
      Key::K => Ok(SimulangKey::K),
      Key::L => Ok(SimulangKey::L),
      Key::M => Ok(SimulangKey::M),
      Key::N => Ok(SimulangKey::N),
      Key::O => Ok(SimulangKey::O),
      Key::P => Ok(SimulangKey::P),
      Key::Q => Ok(SimulangKey::Q),
      Key::R => Ok(SimulangKey::R),
      Key::S => Ok(SimulangKey::S),
      Key::T => Ok(SimulangKey::T),
      Key::U => Ok(SimulangKey::U),
      Key::V => Ok(SimulangKey::V),
      Key::W => Ok(SimulangKey::W),
      Key::X => Ok(SimulangKey::X),
      Key::Y => Ok(SimulangKey::Y),
      Key::Z => Ok(SimulangKey::Z),
      #[cfg(target_os = "windows")]
      Key::AbntC1 => Ok(SimulangKey::AbntC1),
      #[cfg(target_os = "windows")]
      Key::AbntC2 => Ok(SimulangKey::AbntC2),
      #[cfg(target_os = "windows")]
      Key::Accept => Ok(SimulangKey::Accept),
      Key::Add => Ok(SimulangKey::Add),
      Key::Alt => Ok(SimulangKey::Alt),
      Key::Apps => Ok(SimulangKey::Apps),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Attn => Ok(SimulangKey::Attn),
      Key::Backspace => Ok(SimulangKey::Backspace),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Break => Ok(SimulangKey::Break),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Begin => Ok(SimulangKey::Begin),
      #[cfg(unix)]
      Key::BrightnessDown => Ok(SimulangKey::BrightnessDown),
      #[cfg(unix)]
      Key::BrightnessUp => Ok(SimulangKey::BrightnessUp),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::BrowserBack => Ok(SimulangKey::BrowserBack),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::BrowserFavorites => Ok(SimulangKey::BrowserFavorites),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::BrowserForward => Ok(SimulangKey::BrowserForward),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::BrowserHome => Ok(SimulangKey::BrowserHome),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::BrowserRefresh => Ok(SimulangKey::BrowserRefresh),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::BrowserSearch => Ok(SimulangKey::BrowserSearch),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::BrowserStop => Ok(SimulangKey::BrowserStop),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Cancel => Ok(SimulangKey::Cancel),
      Key::CapsLock => Ok(SimulangKey::CapsLock),
      Key::Clear => Ok(SimulangKey::Clear),
      #[cfg(target_os = "macos")]
      Key::ContrastUp => Ok(SimulangKey::ContrastUp),
      #[cfg(target_os = "macos")]
      Key::ContrastDown => Ok(SimulangKey::ContrastDown),
      Key::Control => Ok(SimulangKey::Control),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Convert => Ok(SimulangKey::Convert),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Crsel => Ok(SimulangKey::Crsel),
      Key::DBEAlphanumeric => Ok(SimulangKey::DBEAlphanumeric),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
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
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::DBEHiragana => Ok(SimulangKey::DBEHiragana),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::DBEKatakana => Ok(SimulangKey::DBEKatakana),
      #[cfg(target_os = "windows")]
      Key::DBENoCodepoint => Ok(SimulangKey::DBENoCodepoint),
      #[cfg(target_os = "windows")]
      Key::DBENoRoman => Ok(SimulangKey::DBENoRoman),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::DBERoman => Ok(SimulangKey::DBERoman),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::DBESBCSChar => Ok(SimulangKey::DBESBCSChar),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::DBESChar => Ok(SimulangKey::DBESChar),
      Key::Decimal => Ok(SimulangKey::Decimal),
      Key::Delete => Ok(SimulangKey::Delete),
      Key::Divide => Ok(SimulangKey::Divide),
      Key::DownArrow => Ok(SimulangKey::DownArrow),
      #[cfg(unix)]
      Key::Eject => Ok(SimulangKey::Eject),
      Key::End => Ok(SimulangKey::End),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Ereof => Ok(SimulangKey::Ereof),
      Key::Escape => Ok(SimulangKey::Escape),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Execute => Ok(SimulangKey::Execute),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
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
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
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
      #[cfg(unix)]
      Key::IlluminationDown => Ok(SimulangKey::IlluminationDown),
      #[cfg(unix)]
      Key::IlluminationUp => Ok(SimulangKey::IlluminationUp),
      #[cfg(unix)]
      Key::IlluminationToggle => Ok(SimulangKey::IlluminationToggle),
      #[cfg(target_os = "windows")]
      Key::IMEOff => Ok(SimulangKey::IMEOff),
      #[cfg(target_os = "windows")]
      Key::IMEOn => Ok(SimulangKey::IMEOn),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Insert => Ok(SimulangKey::Insert),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Junja => Ok(SimulangKey::Junja),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Kana => Ok(SimulangKey::Kana),
      Key::Kanji => Ok(SimulangKey::Kanji),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::LaunchApp1 => Ok(SimulangKey::LaunchApp1),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::LaunchApp2 => Ok(SimulangKey::LaunchApp2),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::LaunchMail => Ok(SimulangKey::LaunchMail),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
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
      Key::LMenu => Ok(SimulangKey::LMenu),
      Key::LShift => Ok(SimulangKey::LShift),
      Key::LWin => Ok(SimulangKey::LWin),
      #[cfg(target_os = "windows")]
      Key::MButton => Ok(SimulangKey::MButton),
      #[cfg(unix)]
      Key::MediaFast => Ok(SimulangKey::MediaFast),
      Key::MediaNextTrack => Ok(SimulangKey::MediaNextTrack),
      Key::MediaPlayPause => Ok(SimulangKey::MediaPlayPause),
      Key::MediaPrevTrack => Ok(SimulangKey::MediaPrevTrack),
      #[cfg(unix)]
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
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::NonConvert => Ok(SimulangKey::NonConvert),
      #[cfg(target_os = "windows")]
      Key::None => Ok(SimulangKey::None),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Numlock => Ok(SimulangKey::Numlock),
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
      Key::NumpadEnter => Ok(SimulangKey::NumpadEnter),
      Key::OEM1 => Ok(SimulangKey::OEM1),
      Key::OEM102 => Ok(SimulangKey::OEM102),
      Key::OEM2 => Ok(SimulangKey::OEM2),
      Key::OEM3 => Ok(SimulangKey::OEM3),
      Key::OEM4 => Ok(SimulangKey::OEM4),
      Key::OEM5 => Ok(SimulangKey::OEM5),
      Key::OEM6 => Ok(SimulangKey::OEM6),
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
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::OEMJump => Ok(SimulangKey::OEMJump),
      Key::OEMMinus => Ok(SimulangKey::OEMMinus),
      #[cfg(target_os = "windows")]
      Key::OEMNECEqual => Ok(SimulangKey::OEMNECEqual),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::OEMPA1 => Ok(SimulangKey::OEMPA1),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::OEMPA2 => Ok(SimulangKey::OEMPA2),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::OEMPA3 => Ok(SimulangKey::OEMPA3),
      Key::OEMPeriod => Ok(SimulangKey::OEMPeriod),
      Key::OEMPlus => Ok(SimulangKey::OEMPlus),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::OEMReset => Ok(SimulangKey::OEMReset),
      #[cfg(target_os = "windows")]
      Key::OEMWsctrl => Ok(SimulangKey::OEMWsctrl),
      Key::Option => Ok(SimulangKey::Option),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::PA1 => Ok(SimulangKey::PA1),
      #[cfg(target_os = "windows")]
      Key::Packet => Ok(SimulangKey::Packet),
      Key::PageDown => Ok(SimulangKey::PageDown),
      Key::PageUp => Ok(SimulangKey::PageUp),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Pause => Ok(SimulangKey::Pause),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Play => Ok(SimulangKey::Play),
      #[cfg(unix)]
      Key::Power => Ok(SimulangKey::Power),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      #[allow(deprecated)]
      Key::Print => Ok(SimulangKey::Print),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::PrintScr => Ok(SimulangKey::PrintScr),
      #[cfg(target_os = "windows")]
      Key::Processkey => Ok(SimulangKey::Processkey),
      #[cfg(target_os = "windows")]
      Key::RButton => Ok(SimulangKey::RButton),
      Key::RCommand => Ok(SimulangKey::RCommand),
      Key::RControl => Ok(SimulangKey::RControl),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Redo => Ok(SimulangKey::Redo),
      Key::Return => Ok(SimulangKey::Return),
      Key::RightArrow => Ok(SimulangKey::RightArrow),
      Key::RMenu => Ok(SimulangKey::RMenu),
      Key::ROption => Ok(SimulangKey::ROption),
      Key::RShift => Ok(SimulangKey::RShift),
      Key::RWin => Ok(SimulangKey::RWin),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Scroll => Ok(SimulangKey::Scroll),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::ScrollLock => Ok(SimulangKey::ScrollLock),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Select => Ok(SimulangKey::Select),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::ScriptSwitch => Ok(SimulangKey::ScriptSwitch),
      Key::Separator => Ok(SimulangKey::Separator),
      Key::Shift => Ok(SimulangKey::Shift),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::ShiftLock => Ok(SimulangKey::ShiftLock),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
      Key::Sleep => Ok(SimulangKey::Sleep),
      Key::Space => Ok(SimulangKey::Space),
      Key::Subtract => Ok(SimulangKey::Subtract),
      #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
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
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::AltGraph => Ok(SimulangKey::AltGraph),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Hyper => Ok(SimulangKey::Hyper),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Copy => Ok(SimulangKey::Copy),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Cut => Ok(SimulangKey::Cut),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Paste => Ok(SimulangKey::Paste),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::ZoomIn => Ok(SimulangKey::ZoomIn),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::ZoomOut => Ok(SimulangKey::ZoomOut),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LogOff => Ok(SimulangKey::LogOff),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Hibernate => Ok(SimulangKey::Hibernate),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::WakeUp => Ok(SimulangKey::WakeUp),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::AllCandidates => Ok(SimulangKey::AllCandidates),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Compose => Ok(SimulangKey::Compose),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::PreviousCandidate => Ok(SimulangKey::PreviousCandidate),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::SingleCandidate => Ok(SimulangKey::SingleCandidate),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::GroupFirst => Ok(SimulangKey::GroupFirst),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::GroupLast => Ok(SimulangKey::GroupLast),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::GroupNext => Ok(SimulangKey::GroupNext),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::GroupPrevious => Ok(SimulangKey::GroupPrevious),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::HiraganaKatakana => Ok(SimulangKey::HiraganaKatakana),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::ZenkakuHankaku => Ok(SimulangKey::ZenkakuHankaku),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::MediaPlay => Ok(SimulangKey::MediaPlay),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::MediaPause => Ok(SimulangKey::MediaPause),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::MediaRecord => Ok(SimulangKey::MediaRecord),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Close => Ok(SimulangKey::Close),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::New => Ok(SimulangKey::New),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Open => Ok(SimulangKey::Open),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::Save => Ok(SimulangKey::Save),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::SpellCheck => Ok(SimulangKey::SpellCheck),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::MailForward => Ok(SimulangKey::MailForward),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::MailReply => Ok(SimulangKey::MailReply),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::MailSend => Ok(SimulangKey::MailSend),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchCalculator => Ok(SimulangKey::LaunchCalculator),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchCalendar => Ok(SimulangKey::LaunchCalendar),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchMusicPlayer => Ok(SimulangKey::LaunchMusicPlayer),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchMyComputer => Ok(SimulangKey::LaunchMyComputer),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchPhone => Ok(SimulangKey::LaunchPhone),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchScreenSaver => Ok(SimulangKey::LaunchScreenSaver),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchSpreadsheet => Ok(SimulangKey::LaunchSpreadsheet),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchWebBrowser => Ok(SimulangKey::LaunchWebBrowser),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchWebCam => Ok(SimulangKey::LaunchWebCam),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchWordProcessor => Ok(SimulangKey::LaunchWordProcessor),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp3 => Ok(SimulangKey::LaunchApp3),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp4 => Ok(SimulangKey::LaunchApp4),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp5 => Ok(SimulangKey::LaunchApp5),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp6 => Ok(SimulangKey::LaunchApp6),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp7 => Ok(SimulangKey::LaunchApp7),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp8 => Ok(SimulangKey::LaunchApp8),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp9 => Ok(SimulangKey::LaunchApp9),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp10 => Ok(SimulangKey::LaunchApp10),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp11 => Ok(SimulangKey::LaunchApp11),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp12 => Ok(SimulangKey::LaunchApp12),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp13 => Ok(SimulangKey::LaunchApp13),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp14 => Ok(SimulangKey::LaunchApp14),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp15 => Ok(SimulangKey::LaunchApp15),
      #[cfg(all(unix, not(target_os = "macos")))]
      Key::LaunchApp16 => Ok(SimulangKey::LaunchApp16),
      other => Err(Error::from_reason(format!(
        "Key {other:?} is not supported on this platform.",
      ))),
    }
  }
}
