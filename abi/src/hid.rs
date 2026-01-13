//! Bristle HID Wire Protocol
//!
//! Types shared between HID drivers, Bristle broker, and consumer apps.
//! All types are `repr(C)` for wire compatibility.
//!
//! # Examples
//! ```
//! use abi::hid::{
//!     BristleEventHeader, EventType, Key, KeyEventPayload, Mods,
//!     BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
//! };
//!
//! let header = BristleEventHeader {
//!     magic: BRISTLE_EVENT_MAGIC,
//!     version: BRISTLE_EVENT_VERSION,
//!     event_type: EventType::KeyDown as u16,
//!     timestamp_ns: 42,
//!     payload_len: KeyEventPayload::SIZE as u32,
//! };
//! let payload = KeyEventPayload {
//!     key: Key::A as u16,
//!     mods: Mods::SHIFT,
//!     flags: 0,
//! };
//!
//! let mut bytes = Vec::new();
//! bytes.extend_from_slice(&header.to_bytes());
//! bytes.extend_from_slice(&payload.to_bytes());
//!
//! let header_back = BristleEventHeader::from_bytes(
//!     bytes[..BristleEventHeader::SIZE].try_into().unwrap()
//! ).unwrap();
//! let payload_back = KeyEventPayload::from_bytes(
//!     bytes[BristleEventHeader::SIZE..].try_into().unwrap()
//! );
//!
//! let event_type = header_back.event_type;
//! assert_eq!(event_type, EventType::KeyDown as u16);
//! assert_eq!(payload_back.key(), Key::A);
//! assert!(payload_back.mods().has_shift());
//! ```

/// Magic number for Bristle event headers: 'HIDE'
pub const BRISTLE_EVENT_MAGIC: u32 = 0x48494445;

/// Protocol version
pub const BRISTLE_EVENT_VERSION: u16 = 0;

// ============================================================================
// Event Types
// ============================================================================

/// Event type discriminant
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventType {
    KeyDown = 1,
    KeyUp = 2,
    PointerMove = 3,
    PointerButtonDown = 4,
    PointerButtonUp = 5,
    Scroll = 6,
    DeviceAdded = 7,
    DeviceRemoved = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HidParseError {
    TooShort,
    BadMagic,
    BadVersion,
    BadEventType,
    BadKind,
    LengthMismatch,
}

impl EventType {
    pub fn from_raw(value: u16) -> Result<Self, HidParseError> {
        match value {
            1 => Ok(EventType::KeyDown),
            2 => Ok(EventType::KeyUp),
            3 => Ok(EventType::PointerMove),
            4 => Ok(EventType::PointerButtonDown),
            5 => Ok(EventType::PointerButtonUp),
            6 => Ok(EventType::Scroll),
            7 => Ok(EventType::DeviceAdded),
            8 => Ok(EventType::DeviceRemoved),
            _ => Err(HidParseError::BadEventType),
        }
    }
}

// ============================================================================
// Key Representation (HID Usage Page 0x07)
// ============================================================================

/// Normalized key codes based on HID Usage Table (Keyboard/Keypad Page 0x07)
/// Apps receive this, never raw scancodes.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    // Letters (0x04-0x1D)
    A = 0x04, B = 0x05, C = 0x06, D = 0x07, E = 0x08, F = 0x09,
    G = 0x0A, H = 0x0B, I = 0x0C, J = 0x0D, K = 0x0E, L = 0x0F,
    M = 0x10, N = 0x11, O = 0x12, P = 0x13, Q = 0x14, R = 0x15,
    S = 0x16, T = 0x17, U = 0x18, V = 0x19, W = 0x1A, X = 0x1B,
    Y = 0x1C, Z = 0x1D,

    // Numbers (0x1E-0x27)
    Num1 = 0x1E, Num2 = 0x1F, Num3 = 0x20, Num4 = 0x21, Num5 = 0x22,
    Num6 = 0x23, Num7 = 0x24, Num8 = 0x25, Num9 = 0x26, Num0 = 0x27,

    // Special keys
    Enter = 0x28,
    Escape = 0x29,
    Backspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Equal = 0x2E,
    LeftBracket = 0x2F,
    RightBracket = 0x30,
    Backslash = 0x31,
    Semicolon = 0x33,
    Quote = 0x34,
    Grave = 0x35,
    Comma = 0x36,
    Period = 0x37,
    Slash = 0x38,
    CapsLock = 0x39,

    // Function keys (0x3A-0x45)
    F1 = 0x3A, F2 = 0x3B, F3 = 0x3C, F4 = 0x3D, F5 = 0x3E, F6 = 0x3F,
    F7 = 0x40, F8 = 0x41, F9 = 0x42, F10 = 0x43, F11 = 0x44, F12 = 0x45,

    // Navigation
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    Right = 0x4F,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,

    // Modifiers
    LeftCtrl = 0xE0,
    LeftShift = 0xE1,
    LeftAlt = 0xE2,
    LeftMeta = 0xE3,
    RightCtrl = 0xE4,
    RightShift = 0xE5,
    RightAlt = 0xE6,
    RightMeta = 0xE7,

    // Escape hatch for unknown keys
    Unknown = 0xFFFF,
}

impl Key {
    /// Get human-readable name for the key
    pub fn name(self) -> &'static str {
        match self {
            Key::A => "A", Key::B => "B", Key::C => "C", Key::D => "D",
            Key::E => "E", Key::F => "F", Key::G => "G", Key::H => "H",
            Key::I => "I", Key::J => "J", Key::K => "K", Key::L => "L",
            Key::M => "M", Key::N => "N", Key::O => "O", Key::P => "P",
            Key::Q => "Q", Key::R => "R", Key::S => "S", Key::T => "T",
            Key::U => "U", Key::V => "V", Key::W => "W", Key::X => "X",
            Key::Y => "Y", Key::Z => "Z",
            Key::Num1 => "1", Key::Num2 => "2", Key::Num3 => "3",
            Key::Num4 => "4", Key::Num5 => "5", Key::Num6 => "6",
            Key::Num7 => "7", Key::Num8 => "8", Key::Num9 => "9", Key::Num0 => "0",
            Key::Enter => "Enter", Key::Escape => "Esc", Key::Backspace => "Bksp",
            Key::Tab => "Tab", Key::Space => "Space", Key::Minus => "-",
            Key::Equal => "=", Key::LeftBracket => "[", Key::RightBracket => "]",
            Key::Backslash => "\\", Key::Semicolon => ";", Key::Quote => "'",
            Key::Grave => "`", Key::Comma => ",", Key::Period => ".",
            Key::Slash => "/", Key::CapsLock => "Caps",
            Key::F1 => "F1", Key::F2 => "F2", Key::F3 => "F3", Key::F4 => "F4",
            Key::F5 => "F5", Key::F6 => "F6", Key::F7 => "F7", Key::F8 => "F8",
            Key::F9 => "F9", Key::F10 => "F10", Key::F11 => "F11", Key::F12 => "F12",
            Key::PrintScreen => "PrtSc", Key::ScrollLock => "ScrLk", Key::Pause => "Pause",
            Key::Insert => "Ins", Key::Home => "Home", Key::PageUp => "PgUp",
            Key::Delete => "Del", Key::End => "End", Key::PageDown => "PgDn",
            Key::Right => "Right", Key::Left => "Left", Key::Down => "Down", Key::Up => "Up",
            Key::LeftCtrl => "LCtrl", Key::LeftShift => "LShift", Key::LeftAlt => "LAlt",
            Key::LeftMeta => "LMeta", Key::RightCtrl => "RCtrl", Key::RightShift => "RShift",
            Key::RightAlt => "RAlt", Key::RightMeta => "RMeta",
            Key::Unknown => "?",
        }
    }

    /// Convert from raw u16 value
    pub fn from_raw(value: u16) -> Self {
        // Safety: we validate known values, unknown becomes Unknown
        match value {
            0x04 => Key::A, 0x05 => Key::B, 0x06 => Key::C, 0x07 => Key::D,
            0x08 => Key::E, 0x09 => Key::F, 0x0A => Key::G, 0x0B => Key::H,
            0x0C => Key::I, 0x0D => Key::J, 0x0E => Key::K, 0x0F => Key::L,
            0x10 => Key::M, 0x11 => Key::N, 0x12 => Key::O, 0x13 => Key::P,
            0x14 => Key::Q, 0x15 => Key::R, 0x16 => Key::S, 0x17 => Key::T,
            0x18 => Key::U, 0x19 => Key::V, 0x1A => Key::W, 0x1B => Key::X,
            0x1C => Key::Y, 0x1D => Key::Z,
            0x1E => Key::Num1, 0x1F => Key::Num2, 0x20 => Key::Num3,
            0x21 => Key::Num4, 0x22 => Key::Num5, 0x23 => Key::Num6,
            0x24 => Key::Num7, 0x25 => Key::Num8, 0x26 => Key::Num9, 0x27 => Key::Num0,
            0x28 => Key::Enter, 0x29 => Key::Escape, 0x2A => Key::Backspace,
            0x2B => Key::Tab, 0x2C => Key::Space, 0x2D => Key::Minus,
            0x2E => Key::Equal, 0x2F => Key::LeftBracket, 0x30 => Key::RightBracket,
            0x31 => Key::Backslash, 0x33 => Key::Semicolon, 0x34 => Key::Quote,
            0x35 => Key::Grave, 0x36 => Key::Comma, 0x37 => Key::Period,
            0x38 => Key::Slash, 0x39 => Key::CapsLock,
            0x3A => Key::F1, 0x3B => Key::F2, 0x3C => Key::F3, 0x3D => Key::F4,
            0x3E => Key::F5, 0x3F => Key::F6, 0x40 => Key::F7, 0x41 => Key::F8,
            0x42 => Key::F9, 0x43 => Key::F10, 0x44 => Key::F11, 0x45 => Key::F12,
            0x46 => Key::PrintScreen, 0x47 => Key::ScrollLock, 0x48 => Key::Pause,
            0x49 => Key::Insert, 0x4A => Key::Home, 0x4B => Key::PageUp,
            0x4C => Key::Delete, 0x4D => Key::End, 0x4E => Key::PageDown,
            0x4F => Key::Right, 0x50 => Key::Left, 0x51 => Key::Down, 0x52 => Key::Up,
            0xE0 => Key::LeftCtrl, 0xE1 => Key::LeftShift, 0xE2 => Key::LeftAlt,
            0xE3 => Key::LeftMeta, 0xE4 => Key::RightCtrl, 0xE5 => Key::RightShift,
            0xE6 => Key::RightAlt, 0xE7 => Key::RightMeta,
            _ => Key::Unknown,
        }
    }
}

// ============================================================================
// Modifiers and Locks
// ============================================================================

/// Modifier key bitmask (currently held)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mods(pub u8);

impl Mods {
    pub const SHIFT: u8 = 1 << 0;
    pub const CTRL: u8 = 1 << 1;
    pub const ALT: u8 = 1 << 2;
    pub const META: u8 = 1 << 3;
    pub const ALTGR: u8 = 1 << 4;

    pub fn has_shift(self) -> bool { self.0 & Self::SHIFT != 0 }
    pub fn has_ctrl(self) -> bool { self.0 & Self::CTRL != 0 }
    pub fn has_alt(self) -> bool { self.0 & Self::ALT != 0 }
    pub fn has_meta(self) -> bool { self.0 & Self::META != 0 }
}

/// Lock state bitmask (toggle state)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Locks(pub u8);

impl Locks {
    pub const CAPS: u8 = 1 << 0;
    pub const NUM: u8 = 1 << 1;
    pub const SCROLL: u8 = 1 << 2;
}

// ============================================================================
// Wire Format: Bristle Event (Bristle → Apps)
// ============================================================================

/// Bristle event header (20 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct BristleEventHeader {
    pub magic: u32,         // BRISTLE_EVENT_MAGIC
    pub version: u16,       // BRISTLE_EVENT_VERSION
    pub event_type: u16,    // EventType discriminant
    pub timestamp_ns: u64,  // Monotonic timestamp
    pub payload_len: u32,   // Bytes following header
}

impl BristleEventHeader {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..4].copy_from_slice(&self.magic.to_le_bytes());
        buf[4..6].copy_from_slice(&self.version.to_le_bytes());
        buf[6..8].copy_from_slice(&self.event_type.to_le_bytes());
        buf[8..16].copy_from_slice(&self.timestamp_ns.to_le_bytes());
        buf[16..20].copy_from_slice(&self.payload_len.to_le_bytes());
        buf
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Result<Self, HidParseError> {
        let header = Self {
            magic: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            version: u16::from_le_bytes(bytes[4..6].try_into().unwrap()),
            event_type: u16::from_le_bytes(bytes[6..8].try_into().unwrap()),
            timestamp_ns: u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
            payload_len: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
        };

        if header.magic != BRISTLE_EVENT_MAGIC {
            return Err(HidParseError::BadMagic);
        }
        if header.version != BRISTLE_EVENT_VERSION {
            return Err(HidParseError::BadVersion);
        }
        EventType::from_raw(header.event_type)?;
        Ok(header)
    }
}

/// KeyDown/KeyUp payload (4 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct KeyEventPayload {
    pub key: u16,    // Key enum value
    pub mods: u8,    // Mods bitmask
    pub flags: u8,   // bit0 = repeat
}

impl KeyEventPayload {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn key(&self) -> Key { Key::from_raw(self.key) }
    pub fn mods(&self) -> Mods { Mods(self.mods) }
    pub fn is_repeat(&self) -> bool { self.flags & 1 != 0 }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..2].copy_from_slice(&self.key.to_le_bytes());
        buf[2] = self.mods;
        buf[3] = self.flags;
        buf
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            key: u16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            mods: bytes[2],
            flags: bytes[3],
        }
    }
}

// ============================================================================
// Wire Format: Raw Input Envelope (Drivers → Bristle)
// ============================================================================

/// Input device kind
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputDeviceKind {
    Keyboard = 1,
    Mouse = 2,
    Consumer = 3,
    Gamepad = 4,
}

impl InputDeviceKind {
    pub fn from_raw(value: u8) -> Result<Self, HidParseError> {
        match value {
            1 => Ok(InputDeviceKind::Keyboard),
            2 => Ok(InputDeviceKind::Mouse),
            3 => Ok(InputDeviceKind::Consumer),
            4 => Ok(InputDeviceKind::Gamepad),
            _ => Err(HidParseError::BadKind),
        }
    }
}

/// Raw input envelope from drivers (16 bytes header + payload)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct RawInputEnvelope {
    pub device_id: u64,     // ThingId of device
    pub timestamp_ns: u64,  // Driver-side timestamp
    pub kind: u8,           // InputDeviceKind
    pub payload_len: u8,    // Length of payload
    pub _pad: [u8; 6],      // Alignment padding
    // payload bytes follow
}

impl RawInputEnvelope {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..8].copy_from_slice(&self.device_id.to_le_bytes());
        buf[8..16].copy_from_slice(&self.timestamp_ns.to_le_bytes());
        buf[16] = self.kind;
        buf[17] = self.payload_len;
        buf[18..24].copy_from_slice(&self._pad);
        buf
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), HidParseError> {
        if bytes.len() < Self::SIZE {
            return Err(HidParseError::TooShort);
        }
        let envelope = Self {
            device_id: u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            timestamp_ns: u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
            kind: bytes[16],
            payload_len: bytes[17],
            _pad: bytes[18..24].try_into().unwrap(),
        };

        InputDeviceKind::from_raw(envelope.kind)?;
        let total_len = Self::SIZE + envelope.payload_len as usize;
        if bytes.len() != total_len {
            return Err(HidParseError::LengthMismatch);
        }
        Ok((envelope, &bytes[Self::SIZE..]))
    }
}

/// PS/2 keyboard payload (2 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ps2KeyPayload {
    pub scancode: u8,  // Raw scancode (bit 7 = break)
    pub flags: u8,     // bit0 = extended (E0 prefix)
}

impl Ps2KeyPayload {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.scancode, self.flags]
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self { scancode: bytes[0], flags: bytes[1] }
    }
}

// ============================================================================
// Pointer Events
// ============================================================================

/// Pointer move payload (4 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PointerMovePayload {
    pub dx: i16,    // Relative X movement
    pub dy: i16,    // Relative Y movement
}

impl PointerMovePayload {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..2].copy_from_slice(&self.dx.to_le_bytes());
        buf[2..4].copy_from_slice(&self.dy.to_le_bytes());
        buf
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            dx: i16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            dy: i16::from_le_bytes(bytes[2..4].try_into().unwrap()),
        }
    }
}

/// Pointer button payload (2 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PointerButtonPayload {
    pub button: u8,     // 0=left, 1=right, 2=middle
    pub _pad: u8,
}

impl PointerButtonPayload {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.button, self._pad]
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self { button: bytes[0], _pad: bytes[1] }
    }
}

/// Scroll payload (4 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ScrollPayload {
    pub dx: i16,    // Horizontal scroll
    pub dy: i16,    // Vertical scroll
}

impl ScrollPayload {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut buf = [0u8; Self::SIZE];
        buf[0..2].copy_from_slice(&self.dx.to_le_bytes());
        buf[2..4].copy_from_slice(&self.dy.to_le_bytes());
        buf
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self {
            dx: i16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            dy: i16::from_le_bytes(bytes[2..4].try_into().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    // Key::from_raw tests
    #[test]
    fn key_from_raw_letters() {
        assert_eq!(Key::from_raw(0x04), Key::A);
        assert_eq!(Key::from_raw(0x1D), Key::Z);
        assert_eq!(Key::from_raw(0x10), Key::M);
    }

    #[test]
    fn key_from_raw_numbers() {
        assert_eq!(Key::from_raw(0x1E), Key::Num1);
        assert_eq!(Key::from_raw(0x27), Key::Num0);
    }

    #[test]
    fn key_from_raw_special() {
        assert_eq!(Key::from_raw(0x28), Key::Enter);
        assert_eq!(Key::from_raw(0x29), Key::Escape);
        assert_eq!(Key::from_raw(0x2A), Key::Backspace);
        assert_eq!(Key::from_raw(0x2C), Key::Space);
    }

    #[test]
    fn key_from_raw_function_keys() {
        assert_eq!(Key::from_raw(0x3A), Key::F1);
        assert_eq!(Key::from_raw(0x45), Key::F12);
    }

    #[test]
    fn key_from_raw_modifiers() {
        assert_eq!(Key::from_raw(0xE0), Key::LeftCtrl);
        assert_eq!(Key::from_raw(0xE1), Key::LeftShift);
        assert_eq!(Key::from_raw(0xE4), Key::RightCtrl);
        assert_eq!(Key::from_raw(0xE7), Key::RightMeta);
    }

    #[test]
    fn key_from_raw_unknown_fallback() {
        assert_eq!(Key::from_raw(0xFF), Key::Unknown);
        assert_eq!(Key::from_raw(0x00), Key::Unknown);
        assert_eq!(Key::from_raw(0xFFFE), Key::Unknown);
    }

    // Key::name tests
    #[test]
    fn key_name_returns_correct_string() {
        assert_eq!(Key::A.name(), "A");
        assert_eq!(Key::Enter.name(), "Enter");
        assert_eq!(Key::Space.name(), "Space");
        assert_eq!(Key::F1.name(), "F1");
        assert_eq!(Key::LeftCtrl.name(), "LCtrl");
        assert_eq!(Key::Unknown.name(), "?");
    }

    // Mods tests
    #[test]
    fn mods_has_shift() {
        let mods = Mods(Mods::SHIFT);
        assert!(mods.has_shift());
        assert!(!mods.has_ctrl());
        assert!(!mods.has_alt());
        assert!(!mods.has_meta());
    }

    #[test]
    fn mods_combined() {
        let mods = Mods(Mods::SHIFT | Mods::CTRL | Mods::ALT);
        assert!(mods.has_shift());
        assert!(mods.has_ctrl());
        assert!(mods.has_alt());
        assert!(!mods.has_meta());
    }

    #[test]
    fn mods_empty() {
        let mods = Mods(0);
        assert!(!mods.has_shift());
        assert!(!mods.has_ctrl());
        assert!(!mods.has_alt());
        assert!(!mods.has_meta());
    }

    #[test]
    fn mods_bitflag_algebra() {
        let mods = Mods(Mods::SHIFT | Mods::CTRL | Mods::ALT);
        assert_eq!(Mods(mods.0 | mods.0), mods);
        assert_eq!(Mods(mods.0 & 0), Mods(0));

        let toggled = Mods(mods.0 ^ Mods::SHIFT);
        assert!(!toggled.has_shift());
        assert!(toggled.has_ctrl());
        assert!(toggled.has_alt());
    }

    #[test]
    fn mods_roundtrip_byte() {
        let mods = Mods(Mods::SHIFT | Mods::META);
        let round = Mods(mods.0);
        assert_eq!(mods, round);
    }

    // KeyEventPayload tests
    #[test]
    fn key_event_payload_is_repeat() {
        let payload = KeyEventPayload { key: 0x04, mods: 0, flags: 0 };
        assert!(!payload.is_repeat());

        let payload_repeat = KeyEventPayload { key: 0x04, mods: 0, flags: 1 };
        assert!(payload_repeat.is_repeat());
    }

    #[test]
    fn key_event_payload_key() {
        let payload = KeyEventPayload { key: 0x04, mods: 0, flags: 0 };
        assert_eq!(payload.key(), Key::A);
    }

    #[test]
    fn key_event_payload_mods() {
        let payload = KeyEventPayload { key: 0x04, mods: Mods::SHIFT | Mods::CTRL, flags: 0 };
        let mods = payload.mods();
        assert!(mods.has_shift());
        assert!(mods.has_ctrl());
    }

    // Struct size checks for wire compatibility
    #[test]
    fn bristle_event_header_size() {
        assert_eq!(core::mem::size_of::<BristleEventHeader>(), 20);
    }

    #[test]
    fn key_event_payload_size() {
        assert_eq!(core::mem::size_of::<KeyEventPayload>(), 4);
    }

    #[test]
    fn raw_input_envelope_size() {
        assert_eq!(core::mem::size_of::<RawInputEnvelope>(), 24);
    }

    #[test]
    fn ps2_key_payload_size() {
        assert_eq!(core::mem::size_of::<Ps2KeyPayload>(), 2);
    }

    #[test]
    fn pointer_move_payload_size() {
        assert_eq!(core::mem::size_of::<PointerMovePayload>(), 4);
    }

    #[test]
    fn pointer_button_payload_size() {
        assert_eq!(core::mem::size_of::<PointerButtonPayload>(), 2);
    }

    #[test]
    fn scroll_payload_size() {
        assert_eq!(core::mem::size_of::<ScrollPayload>(), 4);
    }

    // Alignment checks for ABI stability
    #[test]
    fn bristle_event_header_alignment() {
        assert_eq!(align_of::<BristleEventHeader>(), 1);
    }

    #[test]
    fn key_event_payload_alignment() {
        assert_eq!(align_of::<KeyEventPayload>(), 1);
    }

    #[test]
    fn raw_input_envelope_alignment() {
        assert_eq!(align_of::<RawInputEnvelope>(), 1);
    }

    #[test]
    fn pointer_payload_alignment() {
        assert_eq!(align_of::<PointerMovePayload>(), 1);
        assert_eq!(align_of::<ScrollPayload>(), 1);
    }

    // Magic and version constants
    #[test]
    fn bristle_event_magic_is_hide() {
        // 'HIDE' in ASCII = 0x48494445
        assert_eq!(BRISTLE_EVENT_MAGIC, 0x48494445);
    }

    #[test]
    fn bristle_event_version_is_zero() {
        assert_eq!(BRISTLE_EVENT_VERSION, 0);
    }

    #[test]
    fn bristle_event_header_golden_bytes() {
        let header = BristleEventHeader {
            magic: BRISTLE_EVENT_MAGIC,
            version: BRISTLE_EVENT_VERSION,
            event_type: EventType::KeyDown as u16,
            timestamp_ns: 0x1122_3344_5566_7788,
            payload_len: 4,
        };
        let bytes = header.to_bytes();
        let expected = [
            0x45, 0x44, 0x49, 0x48, // magic "HIDE" LE
            0x00, 0x00,             // version
            0x01, 0x00,             // event_type
            0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, // timestamp
            0x04, 0x00, 0x00, 0x00, // payload_len
        ];
        assert_eq!(bytes, expected);
        let parsed = BristleEventHeader::from_bytes(&bytes).unwrap();
        let event_type = parsed.event_type;
        let timestamp_ns = parsed.timestamp_ns;
        let payload_len = parsed.payload_len;
        assert_eq!(event_type, EventType::KeyDown as u16);
        assert_eq!(timestamp_ns, 0x1122_3344_5566_7788);
        assert_eq!(payload_len, 4);
    }

    #[test]
    fn bristle_event_header_rejects_bad_magic_and_version() {
        let mut bytes = BristleEventHeader {
            magic: BRISTLE_EVENT_MAGIC,
            version: BRISTLE_EVENT_VERSION,
            event_type: EventType::KeyUp as u16,
            timestamp_ns: 0,
            payload_len: 0,
        }
        .to_bytes();

        bytes[0] ^= 0xFF;
        assert!(matches!(
            BristleEventHeader::from_bytes(&bytes),
            Err(HidParseError::BadMagic)
        ));

        let mut bytes = BristleEventHeader {
            magic: BRISTLE_EVENT_MAGIC,
            version: BRISTLE_EVENT_VERSION,
            event_type: EventType::KeyUp as u16,
            timestamp_ns: 0,
            payload_len: 0,
        }
        .to_bytes();
        bytes[4] = 1;
        assert!(matches!(
            BristleEventHeader::from_bytes(&bytes),
            Err(HidParseError::BadVersion)
        ));
    }

    #[test]
    fn bristle_event_header_versioning_policy() {
        let bytes = BristleEventHeader {
            magic: BRISTLE_EVENT_MAGIC,
            version: 1,
            event_type: EventType::KeyDown as u16,
            timestamp_ns: 0,
            payload_len: 0,
        }
        .to_bytes();
        assert!(matches!(
            BristleEventHeader::from_bytes(&bytes),
            Err(HidParseError::BadVersion)
        ));
    }

    #[test]
    fn payload_golden_bytes() {
        let key_payload = KeyEventPayload { key: 0x04, mods: Mods::SHIFT | Mods::CTRL, flags: 1 };
        assert_eq!(key_payload.to_bytes(), [0x04, 0x00, 0x03, 0x01]);
        let parsed = KeyEventPayload::from_bytes(&key_payload.to_bytes());
        assert_eq!(parsed.key(), Key::A);
        assert!(parsed.mods().has_shift());
        assert!(parsed.mods().has_ctrl());
        assert!(parsed.is_repeat());

        let move_payload = PointerMovePayload { dx: -2, dy: 300 };
        assert_eq!(move_payload.to_bytes(), [0xfe, 0xff, 0x2c, 0x01]);
        let parsed = PointerMovePayload::from_bytes(&move_payload.to_bytes());
        let dx = parsed.dx;
        let dy = parsed.dy;
        assert_eq!(dx, -2);
        assert_eq!(dy, 300);

        let scroll_payload = ScrollPayload { dx: 120, dy: -120 };
        assert_eq!(scroll_payload.to_bytes(), [0x78, 0x00, 0x88, 0xff]);
        let parsed = ScrollPayload::from_bytes(&scroll_payload.to_bytes());
        let dx = parsed.dx;
        let dy = parsed.dy;
        assert_eq!(dx, 120);
        assert_eq!(dy, -120);
    }

    #[test]
    fn raw_input_envelope_invariants() {
        let envelope = RawInputEnvelope {
            device_id: 0x1111_2222_3333_4444,
            timestamp_ns: 0x0102_0304_0506_0708,
            kind: InputDeviceKind::Keyboard as u8,
            payload_len: 2,
            _pad: [0u8; 6],
        };
        const TOTAL_LEN: usize = RawInputEnvelope::SIZE + 2;
        let mut bytes = [0u8; TOTAL_LEN];
        bytes[..RawInputEnvelope::SIZE].copy_from_slice(&envelope.to_bytes());
        bytes[RawInputEnvelope::SIZE..].copy_from_slice(&[0xaa, 0xbb]);
        let (parsed, payload) = RawInputEnvelope::from_bytes(&bytes).unwrap();
        let device_id = parsed.device_id;
        let payload_len = parsed.payload_len;
        let expected_device_id = envelope.device_id;
        assert_eq!(device_id, expected_device_id);
        assert_eq!(payload_len, 2);
        assert_eq!(payload, &[0xaa, 0xbb]);

        let bad = &bytes[..TOTAL_LEN - 1];
        assert!(matches!(
            RawInputEnvelope::from_bytes(&bad),
            Err(HidParseError::LengthMismatch)
        ));

        let mut bad_kind = bytes;
        bad_kind[16] = 0xff;
        assert!(matches!(
            RawInputEnvelope::from_bytes(&bad_kind),
            Err(HidParseError::BadKind)
        ));
    }

    #[test]
    fn raw_input_envelope_length_bounds() {
        let mut bytes = [0u8; RawInputEnvelope::SIZE];
        bytes[16] = InputDeviceKind::Keyboard as u8;
        bytes[17] = 0;
        assert!(RawInputEnvelope::from_bytes(&bytes).is_ok());

        bytes[17] = 1;
        assert!(matches!(
            RawInputEnvelope::from_bytes(&bytes),
            Err(HidParseError::LengthMismatch)
        ));

        let mut max_payload = [0u8; RawInputEnvelope::SIZE + 255];
        max_payload[16] = InputDeviceKind::Keyboard as u8;
        max_payload[17] = 255;
        assert!(RawInputEnvelope::from_bytes(&max_payload).is_ok());

        let mut too_long = [0u8; RawInputEnvelope::SIZE + 256];
        too_long[..RawInputEnvelope::SIZE + 255].copy_from_slice(&max_payload);
        assert!(matches!(
            RawInputEnvelope::from_bytes(&too_long),
            Err(HidParseError::LengthMismatch)
        ));
    }

    #[test]
    fn key_mapping_is_unique_and_named() {
        let known: &[(u16, Key)] = &[
            (0x04, Key::A), (0x05, Key::B), (0x06, Key::C), (0x07, Key::D),
            (0x08, Key::E), (0x09, Key::F), (0x0A, Key::G), (0x0B, Key::H),
            (0x0C, Key::I), (0x0D, Key::J), (0x0E, Key::K), (0x0F, Key::L),
            (0x10, Key::M), (0x11, Key::N), (0x12, Key::O), (0x13, Key::P),
            (0x14, Key::Q), (0x15, Key::R), (0x16, Key::S), (0x17, Key::T),
            (0x18, Key::U), (0x19, Key::V), (0x1A, Key::W), (0x1B, Key::X),
            (0x1C, Key::Y), (0x1D, Key::Z),
            (0x1E, Key::Num1), (0x1F, Key::Num2), (0x20, Key::Num3), (0x21, Key::Num4),
            (0x22, Key::Num5), (0x23, Key::Num6), (0x24, Key::Num7), (0x25, Key::Num8),
            (0x26, Key::Num9), (0x27, Key::Num0),
            (0x28, Key::Enter), (0x29, Key::Escape), (0x2A, Key::Backspace), (0x2B, Key::Tab),
            (0x2C, Key::Space), (0x2D, Key::Minus), (0x2E, Key::Equal), (0x2F, Key::LeftBracket),
            (0x30, Key::RightBracket), (0x31, Key::Backslash), (0x33, Key::Semicolon),
            (0x34, Key::Quote), (0x35, Key::Grave), (0x36, Key::Comma),
            (0x37, Key::Period), (0x38, Key::Slash), (0x39, Key::CapsLock),
            (0x3A, Key::F1), (0x3B, Key::F2), (0x3C, Key::F3), (0x3D, Key::F4),
            (0x3E, Key::F5), (0x3F, Key::F6), (0x40, Key::F7), (0x41, Key::F8),
            (0x42, Key::F9), (0x43, Key::F10), (0x44, Key::F11), (0x45, Key::F12),
            (0x46, Key::PrintScreen), (0x47, Key::ScrollLock), (0x48, Key::Pause),
            (0x49, Key::Insert), (0x4A, Key::Home), (0x4B, Key::PageUp),
            (0x4C, Key::Delete), (0x4D, Key::End), (0x4E, Key::PageDown),
            (0x4F, Key::Right), (0x50, Key::Left), (0x51, Key::Down), (0x52, Key::Up),
            (0xE0, Key::LeftCtrl), (0xE1, Key::LeftShift), (0xE2, Key::LeftAlt),
            (0xE3, Key::LeftMeta), (0xE4, Key::RightCtrl), (0xE5, Key::RightShift),
            (0xE6, Key::RightAlt), (0xE7, Key::RightMeta),
        ];

        for i in 0..known.len() {
            let (raw_i, key_i) = known[i];
            assert_eq!(Key::from_raw(raw_i), key_i);
            assert!(!key_i.name().is_empty());
            assert_ne!(key_i, Key::Unknown);
            for j in (i + 1)..known.len() {
                let (raw_j, key_j) = known[j];
                assert_ne!(raw_i, raw_j);
                assert_ne!(key_i, key_j);
            }
        }
    }

    #[test]
    fn fuzz_lite_parsing_does_not_panic() {
        let mut seed: u64 = 0x1234_5678_9abc_def0;
        for _ in 0..1024 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let mut header_bytes = [0u8; BristleEventHeader::SIZE];
            for (idx, slot) in header_bytes.iter_mut().enumerate() {
                *slot = ((seed >> ((idx % 8) * 8)) & 0xff) as u8;
            }
            let _ = BristleEventHeader::from_bytes(&header_bytes);

            let len = (seed as usize) % 64;
            let mut envelope_bytes = [0u8; 64];
            for (idx, slot) in envelope_bytes.iter_mut().enumerate().take(len) {
                *slot = ((seed >> ((idx % 8) * 8)) & 0xff) as u8;
            }
            let parsed = RawInputEnvelope::from_bytes(&envelope_bytes[..len]);
            if let Ok((env, payload)) = parsed {
                assert_eq!(len, RawInputEnvelope::SIZE + env.payload_len as usize);
                assert_eq!(payload.len(), env.payload_len as usize);
                assert!(InputDeviceKind::from_raw(env.kind).is_ok());
            }
        }
    }

    #[test]
    fn abi_layout_hash_guard() {
        macro_rules! offset_of {
            ($ty:ty, $field:ident) => {{
                let uninit = core::mem::MaybeUninit::<$ty>::uninit();
                let base = uninit.as_ptr();
                unsafe { (core::ptr::addr_of!((*base).$field) as usize) - (base as usize) }
            }};
        }

        fn fnv64(mut hash: u64, value: u64) -> u64 {
            const FNV_PRIME: u64 = 1099511628211;
            hash ^= value;
            hash = hash.wrapping_mul(FNV_PRIME);
            hash
        }

        let mut hash = 0xcbf29ce484222325u64;

        hash = fnv64(hash, size_of::<BristleEventHeader>() as u64);
        hash = fnv64(hash, align_of::<BristleEventHeader>() as u64);
        hash = fnv64(hash, offset_of!(BristleEventHeader, magic) as u64);
        hash = fnv64(hash, offset_of!(BristleEventHeader, version) as u64);
        hash = fnv64(hash, offset_of!(BristleEventHeader, event_type) as u64);
        hash = fnv64(hash, offset_of!(BristleEventHeader, timestamp_ns) as u64);
        hash = fnv64(hash, offset_of!(BristleEventHeader, payload_len) as u64);

        hash = fnv64(hash, size_of::<KeyEventPayload>() as u64);
        hash = fnv64(hash, align_of::<KeyEventPayload>() as u64);
        hash = fnv64(hash, offset_of!(KeyEventPayload, key) as u64);
        hash = fnv64(hash, offset_of!(KeyEventPayload, mods) as u64);
        hash = fnv64(hash, offset_of!(KeyEventPayload, flags) as u64);

        hash = fnv64(hash, size_of::<RawInputEnvelope>() as u64);
        hash = fnv64(hash, align_of::<RawInputEnvelope>() as u64);
        hash = fnv64(hash, offset_of!(RawInputEnvelope, device_id) as u64);
        hash = fnv64(hash, offset_of!(RawInputEnvelope, timestamp_ns) as u64);
        hash = fnv64(hash, offset_of!(RawInputEnvelope, kind) as u64);
        hash = fnv64(hash, offset_of!(RawInputEnvelope, payload_len) as u64);

        hash = fnv64(hash, size_of::<PointerMovePayload>() as u64);
        hash = fnv64(hash, align_of::<PointerMovePayload>() as u64);
        hash = fnv64(hash, offset_of!(PointerMovePayload, dx) as u64);
        hash = fnv64(hash, offset_of!(PointerMovePayload, dy) as u64);

        hash = fnv64(hash, size_of::<ScrollPayload>() as u64);
        hash = fnv64(hash, align_of::<ScrollPayload>() as u64);
        hash = fnv64(hash, offset_of!(ScrollPayload, dx) as u64);
        hash = fnv64(hash, offset_of!(ScrollPayload, dy) as u64);

        assert_eq!(hash, 0x5791d02bd6fe2b5e);
    }
}
