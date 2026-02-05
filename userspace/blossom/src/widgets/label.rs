use stem::petals::{Color, Text, Styled, FontKey};
use stem::petals::builder::Node;
use stem::thing::ThingId;

pub struct Label<'a> {
    text: &'a str,
    linked_node: Option<ThingId>,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            linked_node: None,
        }
    }

    pub fn linked_node(mut self, id: ThingId) -> Self {
        self.linked_node = Some(id);
        self
    }

    pub fn build(self) -> Node {
        let color = if self.linked_node.is_some() {
             Color::from_argb_u32(0xFF000088) // Dark blue hint for linked labels
        } else {
             Color::from_argb_u32(0xFF000000)
        };

        Text::new(self.text)
            .color(color)
            .font(FontKey::new("sans-serif").size(14))
            .margin(6)
            .into()
    }
}
