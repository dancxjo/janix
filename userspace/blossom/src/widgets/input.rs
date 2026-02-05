use alloc::string::{String, ToString};
use stem::petals::{Color, Rect, Size, Styled, Text, FontKey};
use stem::petals::builder::Node;
use abi::hid::Key;

#[derive(Default, Clone)]
pub struct TextInputState {
    pub text: String,
    pub cursor: usize,
    pub focused: bool,
}

impl TextInputState {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            focused: false,
        }
    }

    pub fn handle_key(&mut self, key: Key, shift: bool) {
        if !self.focused {
            return;
        }

        match key {
            Key::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            Key::Right => {
                if self.cursor < self.text.len() {
                    self.cursor += 1;
                }
            }
            Key::Backspace => {
                if self.cursor > 0 {
                    self.text.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
            }
            Key::Delete => {
                if self.cursor < self.text.len() {
                    self.text.remove(self.cursor);
                }
            }
            Key::Home => self.cursor = 0,
            Key::End => self.cursor = self.text.len(),
            _ => {
                if let Some(c) = key_to_char(key, shift) {
                    self.text.insert(self.cursor, c);
                    self.cursor += 1;
                }
            }
        }
    }
}

fn key_to_char(key: Key, shift: bool) -> Option<char> {
    let c = match key {
        Key::A => 'a', Key::B => 'b', Key::C => 'c', Key::D => 'd', Key::E => 'e',
        Key::F => 'f', Key::G => 'g', Key::H => 'h', Key::I => 'i', Key::J => 'j',
        Key::K => 'k', Key::L => 'l', Key::M => 'm', Key::N => 'n', Key::O => 'o',
        Key::P => 'p', Key::Q => 'q', Key::R => 'r', Key::S => 's', Key::T => 't',
        Key::U => 'u', Key::V => 'v', Key::W => 'w', Key::X => 'x', Key::Y => 'y',
        Key::Z => 'z',
        Key::Num1 => '1', Key::Num2 => '2', Key::Num3 => '3', Key::Num4 => '4',
        Key::Num5 => '5', Key::Num6 => '6', Key::Num7 => '7', Key::Num8 => '8',
        Key::Num9 => '9', Key::Num0 => '0',
        Key::Space => ' ',
        Key::Minus => '-', Key::Equal => '=',
        Key::LeftBracket => '[', Key::RightBracket => ']',
        Key::Backslash => '\\', Key::Semicolon => ';',
        Key::Quote => '\'', Key::Grave => '`',
        Key::Comma => ',', Key::Period => '.', Key::Slash => '/',
        _ => return None,
    };

    if shift {
        Some(match c {
            'a'..='z' => c.to_ascii_uppercase(),
            '1' => '!', '2' => '@', '3' => '#', '4' => '$', '5' => '%',
            '6' => '^', '7' => '&', '8' => '*', '9' => '(', '0' => ')',
            '-' => '_', '=' => '+',
            '[' => '{', ']' => '}', '\\' => '|',
            ';' => ':', '\'' => '"', '`' => '~',
            ',' => '<', '.' => '>', '/' => '?',
            _ => c,
        })
    } else {
        Some(c)
    }
}

pub struct TextInput<'a> {
    state: &'a TextInputState,
    placeholder: Option<&'a str>,
}

impl<'a> TextInput<'a> {
    pub fn new(state: &'a TextInputState) -> Self {
        Self {
            state,
            placeholder: None,
        }
    }

    pub fn placeholder(mut self, text: &'a str) -> Self {
        self.placeholder = Some(text);
        self
    }

    pub fn build(self) -> Node {
        // Colors
        let border_color = if self.state.focused {
            Color::from_argb_u32(0xFF007AFF) // Blue focus
        } else {
            Color::from_argb_u32(0xFF888888) // Grey border
        };

        let bg_color = Color::from_argb_u32(0xFFFFFFFF); // White bg

        let text_color = if self.state.text.is_empty() {
             Color::from_argb_u32(0xFF999999) // Placeholder grey
        } else {
             Color::from_argb_u32(0xFF000000) // Black
        };

        let mut display_text = if self.state.text.is_empty() {
             self.placeholder.unwrap_or("").to_string()
        } else {
             self.state.text.clone()
        };

        if self.state.focused {
             // Simple cursor visualization
             if self.state.cursor <= display_text.len() {
                // If text is empty and we are focused, we still want to show cursor
                if display_text.is_empty() {
                    display_text.push('|');
                } else {
                    display_text.insert(self.state.cursor, '|');
                }
             }
        }

        let text = Text::new(&display_text)
            .color(text_color)
            .font(FontKey::new("monospace").size(14))
            .margin(4);

        let mut bg_rect: Node = Rect::new()
            .color(bg_color)
            .width(Size::Pct(100))
            .height(Size::Pct(100))
            .radius(3)
            .into();

        bg_rect.children.push(text.into());

        let mut border_rect: Node = Rect::new()
            .color(border_color)
            .width(Size::Px(200)) // Default width? Or pass through?
            .height(Size::Px(26))
            .radius(4)
            .into();

        border_rect.style.padding = stem::petals::builder::EdgeInsets::all(2);
        border_rect.children.push(bg_rect);

        border_rect
    }
}
