use stem::petals::{Color, Rect, Size, Styled, Text, FontKey};
use stem::petals::builder::Node;

pub struct Button<'a> {
    label: &'a str,
    focused: bool,
    pressed: bool,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            focused: false,
            pressed: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn build(self) -> Node {
        let bg_color = if self.pressed {
            Color::from_argb_u32(0xFFAAAAAA) // Darker when pressed
        } else if self.focused {
             Color::from_argb_u32(0xFFDDDDDD) // Lighter when focused
        } else {
             Color::from_argb_u32(0xFFCCCCCC) // Default grey
        };

        let border_color = if self.focused {
            Color::from_argb_u32(0xFF007AFF) // Blue focus ring
        } else {
            Color::from_argb_u32(0xFF888888) // Default border
        };

        let text = Text::new(self.label)
            .color(Color::from_argb_u32(0xFF000000))
            .font(FontKey::new("sans-serif").size(14))
            .margin(6); // Inner padding

        let mut bg_rect: Node = Rect::new()
            .color(bg_color)
            .width(Size::Pct(100))
            .height(Size::Pct(100))
            .radius(3)
            .into();

        bg_rect.children.push(text.into());

        let mut border_rect: Node = Rect::new()
            .color(border_color)
            .width(Size::Auto)
            .height(Size::Auto)
            .radius(4)
            .into();

        border_rect.style.padding = stem::petals::builder::EdgeInsets::all(2);
        border_rect.children.push(bg_rect);

        border_rect
    }
}
