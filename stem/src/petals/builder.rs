extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::thing::ThingId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Auto,
    Px(i32),
    Pct(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeInsets {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl EdgeInsets {
    pub const fn all(value: i32) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(pub u32);

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(abi::pixel::Color::from_rgb(r, g, b).0)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(abi::pixel::Color::from_rgba(r, g, b, a).0)
    }

    pub const fn from_argb_u32(v: u32) -> Self {
        Self(v)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontKey {
    pub(crate) name: String,
    pub(crate) size: i32,
}

impl FontKey {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
        }
    }

    pub fn size(mut self, size: i32) -> Self {
        self.size = size;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JustifyContent {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFit {
    Fill,
    Contain,
    Cover,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    pub width: Size,
    pub height: Size,
    pub min_width: Option<i32>,
    pub min_height: Option<i32>,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
    pub margin: EdgeInsets,
    pub padding: EdgeInsets,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Size,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: Size::Auto,
            height: Size::Auto,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            margin: EdgeInsets::all(0),
            padding: EdgeInsets::all(0),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            flex_basis: Size::Auto,
        }
    }
}

pub trait Styled: Sized {
    fn style_mut(&mut self) -> &mut Style;

    fn width(mut self, size: Size) -> Self {
        self.style_mut().width = size;
        self
    }

    fn height(mut self, size: Size) -> Self {
        self.style_mut().height = size;
        self
    }

    fn min_size(mut self, w: i32, h: i32) -> Self {
        self.style_mut().min_width = Some(w);
        self.style_mut().min_height = Some(h);
        self
    }

    fn max_size(mut self, w: i32, h: i32) -> Self {
        self.style_mut().max_width = Some(w);
        self.style_mut().max_height = Some(h);
        self
    }

    fn margin(mut self, value: i32) -> Self {
        self.style_mut().margin = EdgeInsets::all(value);
        self
    }

    fn padding(mut self, value: i32) -> Self {
        self.style_mut().padding = EdgeInsets::all(value);
        self
    }

    fn flex_grow(mut self, value: f32) -> Self {
        self.style_mut().flex_grow = value;
        self
    }

    fn flex_shrink(mut self, value: f32) -> Self {
        self.style_mut().flex_shrink = value;
        self
    }

    fn flex_basis(mut self, size: Size) -> Self {
        self.style_mut().flex_basis = size;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Scene {
    pub(crate) root: Option<Node>,
}

impl Scene {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn window(mut self, window: Window) -> Self {
        self.root = Some(window.node);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Window {
    pub(crate) node: Node,
}

impl Window {
    pub fn new(wid: ThingId) -> Self {
        Self {
            node: Node::new(NodeData::Window(WindowData {
                wid,
                title: None,
                min_size: None,
                max_size: None,
                init_size: None,
            })),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        if let NodeData::Window(ref mut data) = self.node.data {
            data.title = Some(title.to_string());
        }
        self
    }

    pub fn min_size(mut self, w: i32, h: i32) -> Self {
        if let NodeData::Window(ref mut data) = self.node.data {
            data.min_size = Some((w, h));
        }
        self
    }

    pub fn max_size(mut self, w: i32, h: i32) -> Self {
        if let NodeData::Window(ref mut data) = self.node.data {
            data.max_size = Some((w, h));
        }
        self
    }

    pub fn initial_size(mut self, w: i32, h: i32) -> Self {
        if let NodeData::Window(ref mut data) = self.node.data {
            data.init_size = Some((w, h));
        }
        self
    }

    pub fn root(mut self, child: impl Into<Node>) -> Self {
        self.node.children.clear();
        self.node.children.push(child.into());
        self
    }
}

impl Styled for Window {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.node.style
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Flex {
    pub(crate) node: Node,
}

impl Flex {
    pub fn row() -> Self {
        Self::new(FlexDirection::Row)
    }

    pub fn column() -> Self {
        Self::new(FlexDirection::Column)
    }

    fn new(direction: FlexDirection) -> Self {
        Self {
            node: Node::new(NodeData::Flex(FlexData {
                direction,
                align: AlignItems::Stretch,
                justify: JustifyContent::Start,
                gap: 0,
            })),
        }
    }

    pub fn gap(mut self, gap: i32) -> Self {
        if let NodeData::Flex(ref mut data) = self.node.data {
            data.gap = gap;
        }
        self
    }

    pub fn align_items(mut self, align: AlignItems) -> Self {
        if let NodeData::Flex(ref mut data) = self.node.data {
            data.align = align;
        }
        self
    }

    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        if let NodeData::Flex(ref mut data) = self.node.data {
            data.justify = justify;
        }
        self
    }

    pub fn push(mut self, child: impl Into<Node>) -> Self {
        self.node.children.push(child.into());
        self
    }
}

impl Styled for Flex {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.node.style
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub(crate) node: Node,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            node: Node::new(NodeData::Text(TextData {
                text: text.to_string(),
                font: None,
                size: 0,
                color: Color::from_argb_u32(0xFFFFFFFF),
            })),
        }
    }

    pub fn font(mut self, font: FontKey) -> Self {
        if let NodeData::Text(ref mut data) = self.node.data {
            data.font = Some(font.name);
            data.size = font.size;
        }
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        if let NodeData::Text(ref mut data) = self.node.data {
            data.color = color;
        }
        self
    }
}

impl Styled for Text {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.node.style
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rect {
    pub(crate) node: Node,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            node: Node::new(NodeData::Rect(RectData {
                color: Color::from_argb_u32(0x00000000),
                radius: None,
            })),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        if let NodeData::Rect(ref mut data) = self.node.data {
            data.color = color;
        }
        self
    }

    pub fn radius(mut self, radius: i32) -> Self {
        if let NodeData::Rect(ref mut data) = self.node.data {
            data.radius = Some(radius);
        }
        self
    }
}

impl Styled for Rect {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.node.style
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub(crate) node: Node,
}

impl Image {
    pub fn new(key: &str) -> Self {
        Self {
            node: Node::new(NodeData::Image(ImageData {
                key: key.to_string(),
                fit: ImageFit::Fill,
            })),
        }
    }

    pub fn fit(mut self, fit: ImageFit) -> Self {
        if let NodeData::Image(ref mut data) = self.node.data {
            data.fit = fit;
        }
        self
    }
}

impl Styled for Image {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.node.style
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    pub(crate) node: Node,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            node: Node::new(NodeData::Canvas),
        }
    }

    pub fn push(mut self, child: impl Into<Node>) -> Self {
        self.node.children.push(child.into());
        self
    }

    pub fn push_at(mut self, child: impl Into<Node>, x: i32, y: i32) -> Self {
        let mut node = child.into();
        node.style.margin.left = x;
        node.style.margin.top = y;
        self.node.children.push(node);
        self
    }
}

impl Styled for Canvas {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.node.style
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub(crate) node: Node,
}

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            node: Node::new(NodeData::Line(LineData {
                x1,
                y1,
                x2,
                y2,
                width: 1,
                color: Color::from_argb_u32(0xFF000000),
            })),
        }
    }

    pub fn width(mut self, width: i32) -> Self {
        if let NodeData::Line(ref mut data) = self.node.data {
            data.width = width;
        }
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        if let NodeData::Line(ref mut data) = self.node.data {
            data.color = color;
        }
        self
    }
}

impl Styled for Line {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.node.style
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Checkbox {
    pub(crate) node: Node,
}

impl Checkbox {
    pub fn new(checked: bool) -> Self {
        Self {
            node: Node::new(NodeData::Checkbox(CheckboxData {
                checked,
                label: None,
            })),
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        if let NodeData::Checkbox(ref mut data) = self.node.data {
            data.label = Some(label.to_string());
        }
        self
    }
}

impl Styled for Checkbox {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.node.style
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Node {
    pub(crate) data: NodeData,
    pub(crate) style: Style,
    pub(crate) children: Vec<Node>,
}

impl Node {
    fn new(data: NodeData) -> Self {
        Self {
            data,
            style: Style::default(),
            children: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum NodeData {
    Window(WindowData),
    Flex(FlexData),
    Text(TextData),
    Rect(RectData),
    Image(ImageData),
    Canvas,
    Line(LineData),
    Checkbox(CheckboxData),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WindowData {
    pub(crate) wid: ThingId,
    pub(crate) title: Option<String>,
    pub(crate) min_size: Option<(i32, i32)>,
    pub(crate) max_size: Option<(i32, i32)>,
    pub(crate) init_size: Option<(i32, i32)>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FlexData {
    pub(crate) direction: FlexDirection,
    pub(crate) align: AlignItems,
    pub(crate) justify: JustifyContent,
    pub(crate) gap: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextData {
    pub(crate) text: String,
    pub(crate) font: Option<String>,
    pub(crate) size: i32,
    pub(crate) color: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RectData {
    pub(crate) color: Color,
    pub(crate) radius: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ImageData {
    pub(crate) key: String,
    pub(crate) fit: ImageFit,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LineData {
    pub(crate) x1: i32,
    pub(crate) y1: i32,
    pub(crate) x2: i32,
    pub(crate) y2: i32,
    pub(crate) width: i32,
    pub(crate) color: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CheckboxData {
    pub(crate) checked: bool,
    pub(crate) label: Option<String>,
}

impl From<Window> for Node {
    fn from(value: Window) -> Self {
        value.node
    }
}

impl From<Flex> for Node {
    fn from(value: Flex) -> Self {
        value.node
    }
}

impl From<Text> for Node {
    fn from(value: Text) -> Self {
        value.node
    }
}

impl From<Rect> for Node {
    fn from(value: Rect) -> Self {
        value.node
    }
}

impl From<Image> for Node {
    fn from(value: Image) -> Self {
        value.node
    }
}

impl From<Canvas> for Node {
    fn from(value: Canvas) -> Self {
        value.node
    }
}

impl From<Line> for Node {
    fn from(value: Line) -> Self {
        value.node
    }
}

impl From<Checkbox> for Node {
    fn from(value: Checkbox) -> Self {
        value.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thing::ThingId;

    #[test]
    fn flex_builder_pushes_children() {
        let flex = Flex::row().push(Text::new("a")).push(Text::new("b"));
        assert_eq!(flex.node.children.len(), 2);
    }

    #[test]
    fn window_root_replaces_children() {
        let wid = ThingId::default();
        let window = Window::new(wid)
            .root(Text::new("a"))
            .root(Text::new("b"));
        assert_eq!(window.node.children.len(), 1);
    }
}
