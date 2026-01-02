//! PS/2 Set 1 Scancode Parser and US Keymap

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Unknown,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Escape, Backspace, Tab, Enter, Space,
    Insert, Delete, Home, End, PageUp, PageDown,
    Left, Up, Right, Down,
    LeftShift, RightShift, LeftCtrl, RightCtrl, LeftAlt, RightAlt,
    CapsLock, NumLock, ScrollLock,
    Minus, Equal, LeftBracket, RightBracket, Backslash, Semicolon, Quote, Comma, Period, Slash,
    Grave,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub pressed: bool,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

pub struct Parser {
    state: ParseState,
}

enum ParseState {
    Normal,
    E0,
}

impl Parser {
    pub fn new() -> Self {
        Self { state: ParseState::Normal }
    }

    /// Parse a byte. Returns Some(KeyEvent) if a complete event is formed.
    /// Note: This is a simplified PS/2 Set 1 parser.
    pub fn parse(&mut self, byte: u8, mods: &Modifiers) -> Option<KeyEvent> {
        match self.state {
            ParseState::Normal => {
                if byte == 0xE0 {
                    self.state = ParseState::E0;
                    None
                } else {
                    let pressed = byte < 0x80;
                    let code_byte = byte & 0x7F;
                    let code = map_scancode_set1(code_byte, false);
                    Some(KeyEvent {
                        code,
                        pressed,
                        modifiers: *mods,
                    })
                }
            }
            ParseState::E0 => {
                self.state = ParseState::Normal;
                let pressed = byte < 0x80;
                let code_byte = byte & 0x7F;
                let code = map_scancode_set1(code_byte, true);
                Some(KeyEvent {
                    code,
                    pressed,
                    modifiers: *mods,
                })
            }
        }
    }
}

fn map_scancode_set1(code: u8, extended: bool) -> KeyCode {
    // Basic US mapping
    if !extended {
        match code {
            0x01 => KeyCode::Escape,
            0x02 => KeyCode::Num1,
            0x03 => KeyCode::Num2,
            0x04 => KeyCode::Num3,
            0x05 => KeyCode::Num4,
            0x06 => KeyCode::Num5,
            0x07 => KeyCode::Num6,
            0x08 => KeyCode::Num7,
            0x09 => KeyCode::Num8,
            0x0A => KeyCode::Num9,
            0x0B => KeyCode::Num0,
            0x0C => KeyCode::Minus,
            0x0D => KeyCode::Equal,
            0x0E => KeyCode::Backspace,
            0x0F => KeyCode::Tab,
            0x10 => KeyCode::Q,
            0x11 => KeyCode::W,
            0x12 => KeyCode::E,
            0x13 => KeyCode::R,
            0x14 => KeyCode::T,
            0x15 => KeyCode::Y,
            0x16 => KeyCode::U,
            0x17 => KeyCode::I,
            0x18 => KeyCode::O,
            0x19 => KeyCode::P,
            0x1A => KeyCode::LeftBracket,
            0x1B => KeyCode::RightBracket,
            0x1C => KeyCode::Enter,
            0x1D => KeyCode::LeftCtrl,
            0x1E => KeyCode::A,
            0x1F => KeyCode::S,
            0x20 => KeyCode::D,
            0x21 => KeyCode::F,
            0x22 => KeyCode::G,
            0x23 => KeyCode::H,
            0x24 => KeyCode::J,
            0x25 => KeyCode::K,
            0x26 => KeyCode::L,
            0x27 => KeyCode::Semicolon,
            0x28 => KeyCode::Quote,
            0x29 => KeyCode::Grave,
            0x2A => KeyCode::LeftShift,
            0x2B => KeyCode::Backslash,
            0x2C => KeyCode::Z,
            0x2D => KeyCode::X,
            0x2E => KeyCode::C,
            0x2F => KeyCode::V,
            0x30 => KeyCode::B,
            0x31 => KeyCode::N,
            0x32 => KeyCode::M,
            0x33 => KeyCode::Comma,
            0x34 => KeyCode::Period,
            0x35 => KeyCode::Slash,
            0x36 => KeyCode::RightShift,
            0x38 => KeyCode::LeftAlt,
            0x39 => KeyCode::Space,
            0x3A => KeyCode::CapsLock,
            0x3B => KeyCode::F1,
            0x3C => KeyCode::F2,
            0x3D => KeyCode::F3,
            0x3E => KeyCode::F4,
            0x3F => KeyCode::F5,
            0x40 => KeyCode::F6,
            0x41 => KeyCode::F7,
            0x42 => KeyCode::F8,
            0x43 => KeyCode::F9,
            0x44 => KeyCode::F10,
            0x57 => KeyCode::F11,
            0x58 => KeyCode::F12,
            _ => KeyCode::Unknown,
        }
    } else {
        match code {
            0x1C => KeyCode::Enter, // Keypad Enter
            0x1D => KeyCode::RightCtrl,
            0x38 => KeyCode::RightAlt,
            0x47 => KeyCode::Home,
            0x48 => KeyCode::Up,
            0x49 => KeyCode::PageUp,
            0x4B => KeyCode::Left,
            0x4D => KeyCode::Right,
            0x4F => KeyCode::End,
            0x50 => KeyCode::Down,
            0x51 => KeyCode::PageDown,
            0x52 => KeyCode::Insert,
            0x53 => KeyCode::Delete,
            _ => KeyCode::Unknown,
        }
    }
}

pub fn to_char(key: KeyCode, shift: bool) -> Option<char> {
    // Basic US mapping
    match key {
        KeyCode::A => Some(if shift { 'A' } else { 'a' }),
        KeyCode::B => Some(if shift { 'B' } else { 'b' }),
        KeyCode::C => Some(if shift { 'C' } else { 'c' }),
        KeyCode::D => Some(if shift { 'D' } else { 'd' }),
        KeyCode::E => Some(if shift { 'E' } else { 'e' }),
        KeyCode::F => Some(if shift { 'F' } else { 'f' }),
        KeyCode::G => Some(if shift { 'G' } else { 'g' }),
        KeyCode::H => Some(if shift { 'H' } else { 'h' }),
        KeyCode::I => Some(if shift { 'I' } else { 'i' }),
        KeyCode::J => Some(if shift { 'J' } else { 'j' }),
        KeyCode::K => Some(if shift { 'K' } else { 'k' }),
        KeyCode::L => Some(if shift { 'L' } else { 'l' }),
        KeyCode::M => Some(if shift { 'M' } else { 'm' }),
        KeyCode::N => Some(if shift { 'N' } else { 'n' }),
        KeyCode::O => Some(if shift { 'O' } else { 'o' }),
        KeyCode::P => Some(if shift { 'P' } else { 'p' }),
        KeyCode::Q => Some(if shift { 'Q' } else { 'q' }),
        KeyCode::R => Some(if shift { 'R' } else { 'r' }),
        KeyCode::S => Some(if shift { 'S' } else { 's' }),
        KeyCode::T => Some(if shift { 'T' } else { 't' }),
        KeyCode::U => Some(if shift { 'U' } else { 'u' }),
        KeyCode::V => Some(if shift { 'V' } else { 'v' }),
        KeyCode::W => Some(if shift { 'W' } else { 'w' }),
        KeyCode::X => Some(if shift { 'X' } else { 'x' }),
        KeyCode::Y => Some(if shift { 'Y' } else { 'y' }),
        KeyCode::Z => Some(if shift { 'Z' } else { 'z' }),
        
        KeyCode::Num1 => Some(if shift { '!' } else { '1' }),
        KeyCode::Num2 => Some(if shift { '@' } else { '2' }),
        KeyCode::Num3 => Some(if shift { '#' } else { '3' }),
        KeyCode::Num4 => Some(if shift { '$' } else { '4' }),
        KeyCode::Num5 => Some(if shift { '%' } else { '5' }),
        KeyCode::Num6 => Some(if shift { '^' } else { '6' }),
        KeyCode::Num7 => Some(if shift { '&' } else { '7' }),
        KeyCode::Num8 => Some(if shift { '*' } else { '8' }),
        KeyCode::Num9 => Some(if shift { '(' } else { '9' }),
        KeyCode::Num0 => Some(if shift { ')' } else { '0' }),
        
        KeyCode::Space => Some(' '),
        KeyCode::Enter => Some('\n'),
        KeyCode::Tab => Some('\t'),
        KeyCode::Minus => Some(if shift { '_' } else { '-' }),
        KeyCode::Equal => Some(if shift { '+' } else { '=' }),
        KeyCode::LeftBracket => Some(if shift { '{' } else { '[' }),
        KeyCode::RightBracket => Some(if shift { '}' } else { ']' }),
        KeyCode::Backslash => Some(if shift { '|' } else { '\\' }),
        KeyCode::Semicolon => Some(if shift { ':' } else { ';' }),
        KeyCode::Quote => Some(if shift { '"' } else { '\'' }),
        KeyCode::Comma => Some(if shift { '<' } else { ',' }),
        KeyCode::Period => Some(if shift { '>' } else { '.' }),
        KeyCode::Slash => Some(if shift { '?' } else { '/' }),
        KeyCode::Grave => Some(if shift { '~' } else { '`' }),
        
        _ => None,
    }
}
