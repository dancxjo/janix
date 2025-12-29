use abi::wire::input::{RawKeyEvent, Key, RawKeyKind};

pub fn normalize(raw: &RawKeyEvent) -> Option<Key> {
    if raw.kind != RawKeyKind::ScancodeSet1 {
        // Not supported yet
        return None;
    }

    let is_e0 = (raw.flags & 1) != 0;
    let code = raw.code;

    // PS/2 Set 1
    // Note: The driver passes 'code' as the lower 7 bits of the scancode usually?
    // Wait, driver: "code = b & 0x7F".
    // So if I release A (0x9E), driver sends code=0x1E, state=Up.
    // So 'code' is the make code.
    
    if is_e0 {
        match code {
            0x1C => Some(Key::Enter), // Keypad Enter usually, often mapped to Enter. 
                                     // Or separate KeypadEnter? Key enum has Enter.
                                     // Actually usually KeypadEnter is distinguished.
                                     // I'll map to Enter for now.
            0x1D => Some(Key::RCtrl),
            0x35 => Some(Key::Slash), // Keypad Slash
            0x37 => Some(Key::PrintScreen), // PrtSc E0 37 or E0 2A E0 37
            0x38 => Some(Key::RAlt),
            0x47 => Some(Key::Home),
            0x48 => Some(Key::Up),
            0x49 => Some(Key::PageUp),
            0x4B => Some(Key::Left),
            0x4D => Some(Key::Right),
            0x4F => Some(Key::End),
            0x50 => Some(Key::Down),
            0x51 => Some(Key::PageDown),
            0x52 => Some(Key::Insert),
            0x53 => Some(Key::Delete),
            0x5B => Some(Key::LWin),
            0x5C => Some(Key::RWin),
            0x5D => Some(Key::Menu),
            _ => None,
        }
    } else {
        match code {
            0x01 => Some(Key::Esc),
            0x02 => Some(Key::Key1),
            0x03 => Some(Key::Key2),
            0x04 => Some(Key::Key3),
            0x05 => Some(Key::Key4),
            0x06 => Some(Key::Key5),
            0x07 => Some(Key::Key6),
            0x08 => Some(Key::Key7),
            0x09 => Some(Key::Key8),
            0x0A => Some(Key::Key9),
            0x0B => Some(Key::Key0),
            0x0C => Some(Key::Minus),
            0x0D => Some(Key::Equal),
            0x0E => Some(Key::Backspace),
            0x0F => Some(Key::Tab),
            0x10 => Some(Key::Q),
            0x11 => Some(Key::W),
            0x12 => Some(Key::E),
            0x13 => Some(Key::R),
            0x14 => Some(Key::T),
            0x15 => Some(Key::Y),
            0x16 => Some(Key::U),
            0x17 => Some(Key::I),
            0x18 => Some(Key::O),
            0x19 => Some(Key::P),
            0x1A => Some(Key::LBracket),
            0x1B => Some(Key::RBracket),
            0x1C => Some(Key::Enter),
            0x1D => Some(Key::LCtrl),
            0x1E => Some(Key::A),
            0x1F => Some(Key::S),
            0x20 => Some(Key::D),
            0x21 => Some(Key::F),
            0x22 => Some(Key::G),
            0x23 => Some(Key::H),
            0x24 => Some(Key::J),
            0x25 => Some(Key::K),
            0x26 => Some(Key::L),
            0x27 => Some(Key::Semicolon),
            0x28 => Some(Key::Quote),
            0x29 => Some(Key::Backtick),
            0x2A => Some(Key::LShift),
            0x2B => Some(Key::Backslash),
            0x2C => Some(Key::Z),
            0x2D => Some(Key::X),
            0x2E => Some(Key::C),
            0x2F => Some(Key::V),
            0x30 => Some(Key::B),
            0x31 => Some(Key::N),
            0x32 => Some(Key::M),
            0x33 => Some(Key::Comma),
            0x34 => Some(Key::Dot),
            0x35 => Some(Key::Slash),
            0x36 => Some(Key::RShift),
            0x37 => Some(Key::KeypadStar),
            0x38 => Some(Key::LAlt),
            0x39 => Some(Key::Space),
            0x3A => Some(Key::CapsLock),
            0x3B => Some(Key::F1),
            0x3C => Some(Key::F2),
            0x3D => Some(Key::F3),
            0x3E => Some(Key::F4),
            0x3F => Some(Key::F5),
            0x40 => Some(Key::F6),
            0x41 => Some(Key::F7),
            0x42 => Some(Key::F8),
            0x43 => Some(Key::F9),
            0x44 => Some(Key::F10),
            0x45 => Some(Key::NumLock),
            0x46 => Some(Key::ScrollLock),
            0x47 => Some(Key::Keypad7),
            0x48 => Some(Key::Keypad8),
            0x49 => Some(Key::Keypad9),
            0x4A => Some(Key::KeypadMinus),
            0x4B => Some(Key::Keypad4),
            0x4C => Some(Key::Keypad5),
            0x4D => Some(Key::Keypad6),
            0x4E => Some(Key::KeypadPlus),
            0x4F => Some(Key::Keypad1),
            0x50 => Some(Key::Keypad2),
            0x51 => Some(Key::Keypad3),
            0x52 => Some(Key::Keypad0),
            0x53 => Some(Key::KeypadDot),
            0x57 => Some(Key::F11),
            0x58 => Some(Key::F12),
            _ => None,
        }
    }
}
