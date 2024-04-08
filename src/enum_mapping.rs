use enigo::{Axis, Button, Coordinate, Direction, Key};

macro_rules! create_enum_mapping {
  ($js_enum:ident, $rust_enum:ident, [$($variant:ident),* $(,)?]) => {
      #[napi(js_name = $rust_enum)]
      pub enum $js_enum {
          $($variant),*
      }

      impl From<$js_enum> for $rust_enum {
          fn from(item: $js_enum) -> Self {
              match item {
                  $($js_enum::$variant => <$rust_enum>::$variant),*
              }
          }
      }
  };
}

create_enum_mapping!(JsButton, Button, [Left, Middle, Right, Back, Forward, ScrollUp, ScrollDown, ScrollLeft, ScrollRight]);
create_enum_mapping!(JsDirection, Direction, [Press, Release, Click]);
create_enum_mapping!(JsCoordinate, Coordinate, [Abs, Rel]);
create_enum_mapping!(JsAxis, Axis, [Horizontal, Vertical]);
create_enum_mapping!(
  JsKey, Key, 
  [
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
    Alt,
    Apps,
    Attn,
    Backspace,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Cancel,
    CapsLock,
    Clear,
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
    Delete,
    Divide,
    DownArrow,
    End,
    Ereof,
    Escape,
    Execute,
    Exsel,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Final,
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
    Home,
    Ico00,
    IcoClear,
    IcoHelp,
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
    LButton,
    LControl,
    LeftArrow,
    LMenu,
    LShift,
    LWin,
    MButton,
    MediaNextTrack,
    MediaPlayPause,
    MediaPrevTrack,
    MediaStop,
    Meta,
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
    Option,
    PA1,
    Packet,
    PageDown,
    PageUp,
    Pause,
    Play,
    Print,
    Processkey,
    RButton,
    RControl,
    Return,
    RightArrow,
    RMenu,
    RShift,
    RWin,
    Scroll,
    Select,
    Separator,
    Shift,
    Sleep,
    Snapshot,
    Space,
    Subtract,
    Tab,
    UpArrow,
    VolumeDown,
    VolumeMute,
    VolumeUp,
    XButton1,
    XButton2,
    Zoom,
  ]
);
