use abi::wire::input::{Key, ModState, KeyState, KeyEvent, TextEvent, TextEventKind};
use alloc::string::{String, ToString};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DeadKey {
    Acute,      // '
    Grave,      // `
    Circumflex, // ^
    Tilde,      // ~
    Diaeresis,  // "
}

pub struct InputEngine {
    pending_dead: Option<DeadKey>,
}

impl InputEngine {
    pub fn new() -> Self {
        Self { pending_dead: None }
    }

    pub fn process(&mut self, event: &KeyEvent) -> Option<TextEvent> {
        if event.state != KeyState::Down {
            return None;
        }

        let mods = event.mods;
        // Suppress text if Ctrl or Alt (Left) is held.
        // AltGr (Right Alt) is used for layout, so we don't suppress.
        // Need to check specific flag implementation in Main.
        if mods.ctrl || mods.alt {
            return None;
        }

        let base_char = map_us_layout(event.key, mods.shift)?;

        // Check if this char triggers a dead key in US-Intl
        if let Some(dk) = get_dead_key(base_char) {
            // New dead key input.
            // If there was a pending dead key, flush it as a character first.
            let mut text = String::new();
            if let Some(old_dk) = self.pending_dead {
                text.push(dead_key_visual(old_dk));
            }
            
            self.pending_dead = Some(dk);
            
            if !text.is_empty() {
                return Some(TextEvent {
                    text,
                    kind: TextEventKind::Commit,
                    device: event.device,
                    time_ns: event.time_ns,
                });
            } else {
                return None;
            }
        }

        // It is a normal character.
        let ch = base_char;

        if let Some(dk) = self.pending_dead {
            self.pending_dead = None;
            // Try compose
            if let Some(composed) = compose(dk, ch) {
                 return Some(TextEvent {
                    text: composed.to_string(),
                    kind: TextEventKind::Commit,
                    device: event.device,
                    time_ns: event.time_ns,
                });
            } else {
                // Fallback: emit dead glyph + char
                let mut text = String::new();
                text.push(dead_key_visual(dk));
                text.push(ch);
                return Some(TextEvent {
                    text,
                    kind: TextEventKind::Commit,
                    device: event.device,
                    time_ns: event.time_ns,
                });
            }
        } else {
            return Some(TextEvent {
                text: ch.to_string(),
                kind: TextEventKind::Commit,
                device: event.device,
                time_ns: event.time_ns,
            });
        }
    }
}

fn map_us_layout(key: Key, shift: bool) -> Option<char> {
    match key {
        Key::Space => Some(' '),
        Key::Enter => Some('\n'), 
        // Example mappings
        Key::A => Some(if shift { 'A' } else { 'a' }),
        Key::B => Some(if shift { 'B' } else { 'b' }),
        Key::C => Some(if shift { 'C' } else { 'c' }),
        Key::D => Some(if shift { 'D' } else { 'd' }),
        Key::E => Some(if shift { 'E' } else { 'e' }),
        Key::F => Some(if shift { 'F' } else { 'f' }),
        Key::G => Some(if shift { 'G' } else { 'g' }),
        Key::H => Some(if shift { 'H' } else { 'h' }),
        Key::I => Some(if shift { 'I' } else { 'i' }),
        Key::J => Some(if shift { 'J' } else { 'j' }),
        Key::K => Some(if shift { 'K' } else { 'k' }),
        Key::L => Some(if shift { 'L' } else { 'l' }),
        Key::M => Some(if shift { 'M' } else { 'm' }),
        Key::N => Some(if shift { 'N' } else { 'n' }),
        Key::O => Some(if shift { 'O' } else { 'o' }),
        Key::P => Some(if shift { 'P' } else { 'p' }),
        Key::Q => Some(if shift { 'Q' } else { 'q' }),
        Key::R => Some(if shift { 'R' } else { 'r' }),
        Key::S => Some(if shift { 'S' } else { 's' }),
        Key::T => Some(if shift { 'T' } else { 't' }),
        Key::U => Some(if shift { 'U' } else { 'u' }),
        Key::V => Some(if shift { 'V' } else { 'v' }),
        Key::W => Some(if shift { 'W' } else { 'w' }),
        Key::X => Some(if shift { 'X' } else { 'x' }),
        Key::Y => Some(if shift { 'Y' } else { 'y' }),
        Key::Z => Some(if shift { 'Z' } else { 'z' }),
        Key::Key1 => Some(if shift { '!' } else { '1' }),
        Key::Key2 => Some(if shift { '@' } else { '2' }),
        Key::Key3 => Some(if shift { '#' } else { '3' }),
        Key::Key4 => Some(if shift { '$' } else { '4' }),
        Key::Key5 => Some(if shift { '%' } else { '5' }),
        Key::Key6 => Some(if shift { '^' } else { '6' }),
        Key::Key7 => Some(if shift { '&' } else { '7' }),
        Key::Key8 => Some(if shift { '*' } else { '8' }),
        Key::Key9 => Some(if shift { '(' } else { '9' }),
        Key::Key0 => Some(if shift { ')' } else { '0' }),
        Key::Minus => Some(if shift { '_' } else { '-' }),
        Key::Equal => Some(if shift { '+' } else { '=' }),
        Key::LBracket => Some(if shift { '{' } else { '[' }),
        Key::RBracket => Some(if shift { '}' } else { ']' }),
        Key::Backslash => Some(if shift { '|' } else { '\\' }),
        Key::Semicolon => Some(if shift { ':' } else { ';' }),
        Key::Quote => Some(if shift { '"' } else { '\'' }),
        Key::Backtick => Some(if shift { '~' } else { '`' }),
        Key::Comma => Some(if shift { '<' } else { ',' }),
        Key::Dot => Some(if shift { '>' } else { '.' }),
        Key::Slash => Some(if shift { '?' } else { '/' }),
        
        Key::Keypad0 => Some('0'),
        Key::Keypad1 => Some('1'),
        Key::Keypad2 => Some('2'),
        Key::Keypad3 => Some('3'),
        Key::Keypad4 => Some('4'),
        Key::Keypad5 => Some('5'),
        Key::Keypad6 => Some('6'),
        Key::Keypad7 => Some('7'),
        Key::Keypad8 => Some('8'),
        Key::Keypad9 => Some('9'),
        Key::KeypadDot => Some('.'),
        Key::KeypadPlus => Some('+'),
        Key::KeypadMinus => Some('-'),
        Key::KeypadStar => Some('*'),
        
        _ => None,
    }
}

fn get_dead_key(ch: char) -> Option<DeadKey> {
    match ch {
        '\'' => Some(DeadKey::Acute),
        '`' => Some(DeadKey::Grave),
        '^' => Some(DeadKey::Circumflex),
        '~' => Some(DeadKey::Tilde),
        '"' => Some(DeadKey::Diaeresis),
        _ => None
    }
}

fn dead_key_visual(dk: DeadKey) -> char {
    match dk {
        DeadKey::Acute => '\'',
        DeadKey::Grave => '`',
        DeadKey::Circumflex => '^',
        DeadKey::Tilde => '~',
        DeadKey::Diaeresis => '"',
    }
}

fn compose(dk: DeadKey, ch: char) -> Option<char> {
    match (dk, ch) {
        (DeadKey::Acute, 'a') => Some('á'),
        (DeadKey::Acute, 'e') => Some('é'),
        (DeadKey::Acute, 'i') => Some('í'),
        (DeadKey::Acute, 'o') => Some('ó'),
        (DeadKey::Acute, 'u') => Some('ú'),
        (DeadKey::Acute, 'y') => Some('ý'),
        (DeadKey::Acute, 'A') => Some('Á'),
        (DeadKey::Acute, 'E') => Some('É'),
        (DeadKey::Acute, 'I') => Some('Í'),
        (DeadKey::Acute, 'O') => Some('Ó'),
        (DeadKey::Acute, 'U') => Some('Ú'),
        (DeadKey::Acute, 'Y') => Some('Ý'),
        (DeadKey::Acute, ' ') => Some('\''),

        (DeadKey::Grave, 'a') => Some('à'),
        (DeadKey::Grave, 'e') => Some('è'),
        (DeadKey::Grave, 'i') => Some('ì'),
        (DeadKey::Grave, 'o') => Some('ò'),
        (DeadKey::Grave, 'u') => Some('ù'),
        (DeadKey::Grave, 'A') => Some('À'),
        (DeadKey::Grave, 'E') => Some('È'),
        (DeadKey::Grave, 'I') => Some('Ì'),
        (DeadKey::Grave, 'O') => Some('Ò'),
        (DeadKey::Grave, 'U') => Some('Ù'),
        (DeadKey::Grave, ' ') => Some('`'),

        (DeadKey::Circumflex, 'a') => Some('â'),
        (DeadKey::Circumflex, 'e') => Some('ê'),
        (DeadKey::Circumflex, 'i') => Some('î'),
        (DeadKey::Circumflex, 'o') => Some('ô'),
        (DeadKey::Circumflex, 'u') => Some('û'),
        (DeadKey::Circumflex, 'A') => Some('Â'),
        (DeadKey::Circumflex, 'E') => Some('Ê'),
        (DeadKey::Circumflex, 'I') => Some('Î'),
        (DeadKey::Circumflex, 'O') => Some('Ô'),
        (DeadKey::Circumflex, 'U') => Some('Û'),
        (DeadKey::Circumflex, ' ') => Some('^'),

        (DeadKey::Diaeresis, 'a') => Some('ä'),
        (DeadKey::Diaeresis, 'e') => Some('ë'),
        (DeadKey::Diaeresis, 'i') => Some('ï'),
        (DeadKey::Diaeresis, 'o') => Some('ö'),
        (DeadKey::Diaeresis, 'u') => Some('ü'),
        (DeadKey::Diaeresis, 'A') => Some('Ä'),
        (DeadKey::Diaeresis, 'E') => Some('Ë'),
        (DeadKey::Diaeresis, 'I') => Some('Ï'),
        (DeadKey::Diaeresis, 'O') => Some('Ö'),
        (DeadKey::Diaeresis, 'U') => Some('Ü'),
        (DeadKey::Diaeresis, ' ') => Some('"'),

        (DeadKey::Tilde, 'a') => Some('ã'),
        (DeadKey::Tilde, 'o') => Some('õ'),
        (DeadKey::Tilde, 'n') => Some('ñ'),
        (DeadKey::Tilde, 'A') => Some('Ã'),
        (DeadKey::Tilde, 'O') => Some('Õ'),
        (DeadKey::Tilde, 'N') => Some('Ñ'),
        (DeadKey::Tilde, ' ') => Some('~'),

        _ => None,
    }
}
