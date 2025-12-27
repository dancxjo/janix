use crate::ThingId;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RawKeyKind {
    ScancodeSet1 = 0,
    UsbHidUsage = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyState {
    Down = 0,
    Up = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawKeyEvent {
    pub source: ThingId,
    pub time_ns: u64,
    pub kind: RawKeyKind,
    pub code: u32,
    pub state: KeyState,
    pub flags: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ModState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub gui: bool,
    pub caps_lock: bool,
    pub num_lock: bool,
    pub scroll_lock: bool,
    pub altgr: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Key {
    Reserved = 0,
    Esc,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Minus,
    Equal,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LBracket,
    RBracket,
    Enter,
    LCtrl,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Quote,
    Backtick,
    LShift,
    Backslash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    RShift,
    KeypadStar,
    LAlt,
    Space,
    CapsLock,
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
    NumLock,
    ScrollLock,
    Keypad7,
    Keypad8,
    Keypad9,
    KeypadMinus,
    Keypad4,
    Keypad5,
    Keypad6,
    KeypadPlus,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad0,
    KeypadDot,
    F11,
    F12,
    RAlt,
    RCtrl,
    Up,
    Down,
    Left,
    Right,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    LWin,
    RWin,
    Menu,
    PrintScreen,
    Pause,
    IntlBackslash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEvent {
    pub key: Key,
    pub state: KeyState,
    pub mods: ModState,
    pub repeat: bool,
    pub device: ThingId,
    pub time_ns: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextEventKind {
    Commit = 0,
    Preedit = 1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEvent {
    pub text: String,
    pub kind: TextEventKind,
    pub device: ThingId,
    pub time_ns: u64,
}
