extern crate alloc;

pub const UI_SCENE_MAGIC: u32 = 0x5343_4e45; // "SCNE"
pub const UI_SCENE_VERSION: u16 = 1;
pub const UI_SCENE_HEADER_BYTES: usize = 24;
pub const UI_SCENE_NODE_BYTES: usize = 128;
pub const UI_SCENE_NODE_PAYLOAD_BYTES: usize = 44;
pub const UI_SCENE_PARENT_NONE: u32 = 0xFFFF_FFFF;

pub const UI_SCENE_HEADER_MAGIC_OFFSET: usize = 0;
pub const UI_SCENE_HEADER_VERSION_OFFSET: usize = 4;
pub const UI_SCENE_HEADER_NODE_COUNT_OFFSET: usize = 8;
pub const UI_SCENE_HEADER_NODE_BYTES_OFFSET: usize = 12;
pub const UI_SCENE_HEADER_STRING_BYTES_OFFSET: usize = 16;

pub const UI_SCENE_NODE_ID_OFFSET: usize = 0;
pub const UI_SCENE_NODE_PARENT_OFFSET: usize = 4;
pub const UI_SCENE_NODE_KIND_OFFSET: usize = 8;
pub const UI_SCENE_NODE_FLAGS_OFFSET: usize = 10;
pub const UI_SCENE_NODE_WIDTH_KIND_OFFSET: usize = 12;
pub const UI_SCENE_NODE_HEIGHT_KIND_OFFSET: usize = 13;
pub const UI_SCENE_NODE_FLEX_BASIS_KIND_OFFSET: usize = 14;
pub const UI_SCENE_NODE_WIDTH_VALUE_OFFSET: usize = 16;
pub const UI_SCENE_NODE_HEIGHT_VALUE_OFFSET: usize = 20;
pub const UI_SCENE_NODE_FLEX_BASIS_VALUE_OFFSET: usize = 24;
pub const UI_SCENE_NODE_MIN_WIDTH_OFFSET: usize = 28;
pub const UI_SCENE_NODE_MIN_HEIGHT_OFFSET: usize = 32;
pub const UI_SCENE_NODE_MAX_WIDTH_OFFSET: usize = 36;
pub const UI_SCENE_NODE_MAX_HEIGHT_OFFSET: usize = 40;
pub const UI_SCENE_NODE_MARGIN_LEFT_OFFSET: usize = 44;
pub const UI_SCENE_NODE_MARGIN_TOP_OFFSET: usize = 48;
pub const UI_SCENE_NODE_MARGIN_RIGHT_OFFSET: usize = 52;
pub const UI_SCENE_NODE_MARGIN_BOTTOM_OFFSET: usize = 56;
pub const UI_SCENE_NODE_PADDING_LEFT_OFFSET: usize = 60;
pub const UI_SCENE_NODE_PADDING_TOP_OFFSET: usize = 64;
pub const UI_SCENE_NODE_PADDING_RIGHT_OFFSET: usize = 68;
pub const UI_SCENE_NODE_PADDING_BOTTOM_OFFSET: usize = 72;
pub const UI_SCENE_NODE_FLEX_GROW_OFFSET: usize = 76;
pub const UI_SCENE_NODE_FLEX_SHRINK_OFFSET: usize = 80;
pub const UI_SCENE_NODE_PAYLOAD_OFFSET: usize = 84;

pub const UI_SCENE_WINDOW_WID_OFFSET: usize = 0;
pub const UI_SCENE_WINDOW_TITLE_OFFSET_OFFSET: usize = 8;
pub const UI_SCENE_WINDOW_TITLE_LEN_OFFSET: usize = 12;
pub const UI_SCENE_WINDOW_MIN_W_OFFSET: usize = 16;
pub const UI_SCENE_WINDOW_MIN_H_OFFSET: usize = 20;
pub const UI_SCENE_WINDOW_MAX_W_OFFSET: usize = 24;
pub const UI_SCENE_WINDOW_MAX_H_OFFSET: usize = 28;
pub const UI_SCENE_WINDOW_INIT_W_OFFSET: usize = 32;
pub const UI_SCENE_WINDOW_INIT_H_OFFSET: usize = 36;

pub const UI_SCENE_FLEX_DIR_OFFSET: usize = 0;
pub const UI_SCENE_FLEX_ALIGN_OFFSET: usize = 1;
pub const UI_SCENE_FLEX_JUSTIFY_OFFSET: usize = 2;
pub const UI_SCENE_FLEX_GAP_OFFSET: usize = 4;

pub const UI_SCENE_TEXT_TEXT_OFFSET_OFFSET: usize = 0;
pub const UI_SCENE_TEXT_TEXT_LEN_OFFSET: usize = 4;
pub const UI_SCENE_TEXT_FONT_OFFSET_OFFSET: usize = 8;
pub const UI_SCENE_TEXT_FONT_LEN_OFFSET: usize = 12;
pub const UI_SCENE_TEXT_SIZE_OFFSET: usize = 16;
pub const UI_SCENE_TEXT_COLOR_OFFSET: usize = 20;

pub const UI_SCENE_RECT_COLOR_OFFSET: usize = 0;
pub const UI_SCENE_RECT_RADIUS_OFFSET: usize = 4;

pub const UI_SCENE_IMAGE_KEY_OFFSET_OFFSET: usize = 0;
pub const UI_SCENE_IMAGE_KEY_LEN_OFFSET: usize = 4;
pub const UI_SCENE_IMAGE_FIT_OFFSET: usize = 8;

pub const UI_SCENE_CHECKBOX_CHECKED_OFFSET: usize = 0;
pub const UI_SCENE_CHECKBOX_LABEL_OFFSET_OFFSET: usize = 4;
pub const UI_SCENE_CHECKBOX_LABEL_LEN_OFFSET: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum NodeKind {
    Window = 1,
    Flex = 2,
    Text = 3,
    Rect = 4,
    Image = 5,
    Checkbox = 6,
    Unknown(u16),
}

impl NodeKind {
    pub fn from_raw(raw: u16) -> Self {
        match raw {
            1 => NodeKind::Window,
            2 => NodeKind::Flex,
            3 => NodeKind::Text,
            4 => NodeKind::Rect,
            5 => NodeKind::Image,
            6 => NodeKind::Checkbox,
            _ => NodeKind::Unknown(raw),
        }
    }

    pub fn as_raw(self) -> u16 {
        match self {
            NodeKind::Window => 1,
            NodeKind::Flex => 2,
            NodeKind::Text => 3,
            NodeKind::Rect => 4,
            NodeKind::Image => 5,
            NodeKind::Checkbox => 6,
            NodeKind::Unknown(raw) => raw,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SizeKind {
    Auto = 0,
    Px = 1,
    Pct = 2,
}

impl SizeKind {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            1 => SizeKind::Px,
            2 => SizeKind::Pct,
            _ => SizeKind::Auto,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlexDirection {
    Row = 0,
    Column = 1,
}

impl FlexDirection {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            1 => FlexDirection::Column,
            _ => FlexDirection::Row,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AlignItems {
    Start = 0,
    Center = 1,
    End = 2,
    Stretch = 3,
}

impl AlignItems {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            1 => AlignItems::Center,
            2 => AlignItems::End,
            3 => AlignItems::Stretch,
            _ => AlignItems::Start,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JustifyContent {
    Start = 0,
    Center = 1,
    End = 2,
}

impl JustifyContent {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            1 => JustifyContent::Center,
            2 => JustifyContent::End,
            _ => JustifyContent::Start,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ImageFit {
    Fill = 0,
    Contain = 1,
    Cover = 2,
    None = 3,
}

impl ImageFit {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            1 => ImageFit::Contain,
            2 => ImageFit::Cover,
            3 => ImageFit::None,
            _ => ImageFit::Fill,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SizeSpec {
    pub kind: SizeKind,
    pub value: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StringRef {
    pub offset: u32,
    pub len: u32,
}

impl StringRef {
    pub const fn empty() -> Self {
        Self { offset: 0, len: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeInsets {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowMeta {
    pub wid: u64,
    pub title: StringRef,
    pub min_w: i32,
    pub min_h: i32,
    pub max_w: i32,
    pub max_h: i32,
    pub init_w: i32,
    pub init_h: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexMeta {
    pub direction: FlexDirection,
    pub align: AlignItems,
    pub justify: JustifyContent,
    pub gap: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextMeta {
    pub text: StringRef,
    pub font: StringRef,
    pub size: i32,
    pub color: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectMeta {
    pub color: u32,
    pub radius: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageMeta {
    pub key: StringRef,
    pub fit: ImageFit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckboxMeta {
    pub checked: bool,
    pub label: StringRef,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneHeader {
    pub magic: u32,
    pub version: u16,
    pub node_count: u32,
    pub node_bytes: u32,
    pub string_bytes: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
    BadMagic(u32),
    BadVersion(u16),
    Truncated,
    InvalidNodeBytes(u32),
}

pub struct UiScene<'a> {
    header: SceneHeader,
    nodes: &'a [u8],
    strings: &'a [u8],
}

impl<'a> UiScene<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        if bytes.len() < UI_SCENE_HEADER_BYTES {
            return Err(DecodeError::Truncated);
        }
        let magic = read_u32(bytes, UI_SCENE_HEADER_MAGIC_OFFSET)?;
        if magic != UI_SCENE_MAGIC {
            return Err(DecodeError::BadMagic(magic));
        }
        let version = read_u16(bytes, UI_SCENE_HEADER_VERSION_OFFSET)?;
        if version != UI_SCENE_VERSION {
            return Err(DecodeError::BadVersion(version));
        }
        let node_count = read_u32(bytes, UI_SCENE_HEADER_NODE_COUNT_OFFSET)?;
        let node_bytes = read_u32(bytes, UI_SCENE_HEADER_NODE_BYTES_OFFSET)?;
        if node_bytes != UI_SCENE_NODE_BYTES as u32 {
            return Err(DecodeError::InvalidNodeBytes(node_bytes));
        }
        let string_bytes = read_u32(bytes, UI_SCENE_HEADER_STRING_BYTES_OFFSET)?;
        let nodes_len = node_count
            .checked_mul(node_bytes)
            .ok_or(DecodeError::Truncated)? as usize;
        let nodes_start = UI_SCENE_HEADER_BYTES;
        let strings_start = nodes_start + nodes_len;
        let strings_end = strings_start + string_bytes as usize;
        if bytes.len() < strings_end {
            return Err(DecodeError::Truncated);
        }
        let nodes = &bytes[nodes_start..nodes_start + nodes_len];
        let strings = &bytes[strings_start..strings_end];
        Ok(Self {
            header: SceneHeader {
                magic,
                version,
                node_count,
                node_bytes,
                string_bytes,
            },
            nodes,
            strings,
        })
    }

    pub fn header(&self) -> SceneHeader {
        self.header
    }

    pub fn node_count(&self) -> usize {
        self.header.node_count as usize
    }

    pub fn node(&self, index: usize) -> Option<NodeView<'a>> {
        if index >= self.node_count() {
            return None;
        }
        let start = index * UI_SCENE_NODE_BYTES;
        let end = start + UI_SCENE_NODE_BYTES;
        Some(NodeView {
            bytes: &self.nodes[start..end],
            strings: self.strings,
        })
    }

    pub fn nodes(&'a self) -> NodeIter<'a> {
        NodeIter {
            scene: self,
            index: 0,
        }
    }

    pub fn string(&self, r: StringRef) -> Option<&'a str> {
        if r.len == 0 {
            return Some("");
        }
        let start = r.offset as usize;
        let end = start + r.len as usize;
        if end > self.strings.len() {
            return None;
        }
        core::str::from_utf8(&self.strings[start..end]).ok()
    }

    pub fn strings_bytes(&self) -> &'a [u8] {
        self.strings
    }
}

pub struct NodeIter<'a> {
    scene: &'a UiScene<'a>,
    index: usize,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = NodeView<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.scene.node(self.index)?;
        self.index += 1;
        Some(node)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NodeView<'a> {
    bytes: &'a [u8],
    strings: &'a [u8],
}

impl<'a> NodeView<'a> {
    pub fn id(&self) -> u32 {
        read_u32(self.bytes, UI_SCENE_NODE_ID_OFFSET).unwrap_or(0)
    }

    pub fn parent(&self) -> Option<u32> {
        let raw = read_u32(self.bytes, UI_SCENE_NODE_PARENT_OFFSET).unwrap_or(UI_SCENE_PARENT_NONE);
        if raw == UI_SCENE_PARENT_NONE {
            None
        } else {
            Some(raw)
        }
    }

    pub fn kind(&self) -> NodeKind {
        let raw = read_u16(self.bytes, UI_SCENE_NODE_KIND_OFFSET).unwrap_or(0);
        NodeKind::from_raw(raw)
    }

    pub fn flags(&self) -> u16 {
        read_u16(self.bytes, UI_SCENE_NODE_FLAGS_OFFSET).unwrap_or(0)
    }

    pub fn width(&self) -> SizeSpec {
        SizeSpec {
            kind: SizeKind::from_raw(self.bytes[UI_SCENE_NODE_WIDTH_KIND_OFFSET]),
            value: read_i32(self.bytes, UI_SCENE_NODE_WIDTH_VALUE_OFFSET).unwrap_or(0),
        }
    }

    pub fn height(&self) -> SizeSpec {
        SizeSpec {
            kind: SizeKind::from_raw(self.bytes[UI_SCENE_NODE_HEIGHT_KIND_OFFSET]),
            value: read_i32(self.bytes, UI_SCENE_NODE_HEIGHT_VALUE_OFFSET).unwrap_or(0),
        }
    }

    pub fn flex_basis(&self) -> SizeSpec {
        SizeSpec {
            kind: SizeKind::from_raw(self.bytes[UI_SCENE_NODE_FLEX_BASIS_KIND_OFFSET]),
            value: read_i32(self.bytes, UI_SCENE_NODE_FLEX_BASIS_VALUE_OFFSET).unwrap_or(0),
        }
    }

    pub fn min_width(&self) -> i32 {
        read_i32(self.bytes, UI_SCENE_NODE_MIN_WIDTH_OFFSET).unwrap_or(0)
    }

    pub fn min_height(&self) -> i32 {
        read_i32(self.bytes, UI_SCENE_NODE_MIN_HEIGHT_OFFSET).unwrap_or(0)
    }

    pub fn max_width(&self) -> i32 {
        read_i32(self.bytes, UI_SCENE_NODE_MAX_WIDTH_OFFSET).unwrap_or(0)
    }

    pub fn max_height(&self) -> i32 {
        read_i32(self.bytes, UI_SCENE_NODE_MAX_HEIGHT_OFFSET).unwrap_or(0)
    }

    pub fn margin(&self) -> EdgeInsets {
        EdgeInsets {
            left: read_i32(self.bytes, UI_SCENE_NODE_MARGIN_LEFT_OFFSET).unwrap_or(0),
            top: read_i32(self.bytes, UI_SCENE_NODE_MARGIN_TOP_OFFSET).unwrap_or(0),
            right: read_i32(self.bytes, UI_SCENE_NODE_MARGIN_RIGHT_OFFSET).unwrap_or(0),
            bottom: read_i32(self.bytes, UI_SCENE_NODE_MARGIN_BOTTOM_OFFSET).unwrap_or(0),
        }
    }

    pub fn padding(&self) -> EdgeInsets {
        EdgeInsets {
            left: read_i32(self.bytes, UI_SCENE_NODE_PADDING_LEFT_OFFSET).unwrap_or(0),
            top: read_i32(self.bytes, UI_SCENE_NODE_PADDING_TOP_OFFSET).unwrap_or(0),
            right: read_i32(self.bytes, UI_SCENE_NODE_PADDING_RIGHT_OFFSET).unwrap_or(0),
            bottom: read_i32(self.bytes, UI_SCENE_NODE_PADDING_BOTTOM_OFFSET).unwrap_or(0),
        }
    }

    pub fn flex_grow(&self) -> f32 {
        read_f32(self.bytes, UI_SCENE_NODE_FLEX_GROW_OFFSET).unwrap_or(0.0)
    }

    pub fn flex_shrink(&self) -> f32 {
        read_f32(self.bytes, UI_SCENE_NODE_FLEX_SHRINK_OFFSET).unwrap_or(0.0)
    }

    pub fn window_meta(&self) -> Option<WindowMeta> {
        if self.kind() != NodeKind::Window {
            return None;
        }
        let payload = self.payload();
        Some(WindowMeta {
            wid: read_u64(payload, UI_SCENE_WINDOW_WID_OFFSET).unwrap_or(0),
            title: StringRef {
                offset: read_u32(payload, UI_SCENE_WINDOW_TITLE_OFFSET_OFFSET).unwrap_or(0),
                len: read_u32(payload, UI_SCENE_WINDOW_TITLE_LEN_OFFSET).unwrap_or(0),
            },
            min_w: read_i32(payload, UI_SCENE_WINDOW_MIN_W_OFFSET).unwrap_or(0),
            min_h: read_i32(payload, UI_SCENE_WINDOW_MIN_H_OFFSET).unwrap_or(0),
            max_w: read_i32(payload, UI_SCENE_WINDOW_MAX_W_OFFSET).unwrap_or(0),
            max_h: read_i32(payload, UI_SCENE_WINDOW_MAX_H_OFFSET).unwrap_or(0),
            init_w: read_i32(payload, UI_SCENE_WINDOW_INIT_W_OFFSET).unwrap_or(0),
            init_h: read_i32(payload, UI_SCENE_WINDOW_INIT_H_OFFSET).unwrap_or(0),
        })
    }

    pub fn flex_meta(&self) -> Option<FlexMeta> {
        if self.kind() != NodeKind::Flex {
            return None;
        }
        let payload = self.payload();
        Some(FlexMeta {
            direction: FlexDirection::from_raw(payload[UI_SCENE_FLEX_DIR_OFFSET]),
            align: AlignItems::from_raw(payload[UI_SCENE_FLEX_ALIGN_OFFSET]),
            justify: JustifyContent::from_raw(payload[UI_SCENE_FLEX_JUSTIFY_OFFSET]),
            gap: read_i32(payload, UI_SCENE_FLEX_GAP_OFFSET).unwrap_or(0),
        })
    }

    pub fn text_meta(&self) -> Option<TextMeta> {
        if self.kind() != NodeKind::Text {
            return None;
        }
        let payload = self.payload();
        Some(TextMeta {
            text: StringRef {
                offset: read_u32(payload, UI_SCENE_TEXT_TEXT_OFFSET_OFFSET).unwrap_or(0),
                len: read_u32(payload, UI_SCENE_TEXT_TEXT_LEN_OFFSET).unwrap_or(0),
            },
            font: StringRef {
                offset: read_u32(payload, UI_SCENE_TEXT_FONT_OFFSET_OFFSET).unwrap_or(0),
                len: read_u32(payload, UI_SCENE_TEXT_FONT_LEN_OFFSET).unwrap_or(0),
            },
            size: read_i32(payload, UI_SCENE_TEXT_SIZE_OFFSET).unwrap_or(0),
            color: read_u32(payload, UI_SCENE_TEXT_COLOR_OFFSET).unwrap_or(0),
        })
    }

    pub fn rect_meta(&self) -> Option<RectMeta> {
        if self.kind() != NodeKind::Rect {
            return None;
        }
        let payload = self.payload();
        Some(RectMeta {
            color: read_u32(payload, UI_SCENE_RECT_COLOR_OFFSET).unwrap_or(0),
            radius: read_i32(payload, UI_SCENE_RECT_RADIUS_OFFSET).unwrap_or(0),
        })
    }

    pub fn image_meta(&self) -> Option<ImageMeta> {
        if self.kind() != NodeKind::Image {
            return None;
        }
        let payload = self.payload();
        Some(ImageMeta {
            key: StringRef {
                offset: read_u32(payload, UI_SCENE_IMAGE_KEY_OFFSET_OFFSET).unwrap_or(0),
                len: read_u32(payload, UI_SCENE_IMAGE_KEY_LEN_OFFSET).unwrap_or(0),
            },
            fit: ImageFit::from_raw(payload[UI_SCENE_IMAGE_FIT_OFFSET]),
        })
    }

    pub fn checkbox_meta(&self) -> Option<CheckboxMeta> {
        if self.kind() != NodeKind::Checkbox {
            return None;
        }
        let payload = self.payload();
        Some(CheckboxMeta {
            checked: payload[UI_SCENE_CHECKBOX_CHECKED_OFFSET] != 0,
            label: StringRef {
                offset: read_u32(payload, UI_SCENE_CHECKBOX_LABEL_OFFSET_OFFSET).unwrap_or(0),
                len: read_u32(payload, UI_SCENE_CHECKBOX_LABEL_LEN_OFFSET).unwrap_or(0),
            },
        })
    }

    pub fn resolve_string(&self, r: StringRef) -> Option<&'a str> {
        if r.len == 0 {
            return Some("");
        }
        let start = r.offset as usize;
        let end = start + r.len as usize;
        if end > self.strings.len() {
            return None;
        }
        core::str::from_utf8(&self.strings[start..end]).ok()
    }

    fn payload(&self) -> &'a [u8] {
        &self.bytes[UI_SCENE_NODE_PAYLOAD_OFFSET..UI_SCENE_NODE_PAYLOAD_OFFSET + UI_SCENE_NODE_PAYLOAD_BYTES]
    }
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, DecodeError> {
    let end = offset + 2;
    if end > bytes.len() {
        return Err(DecodeError::Truncated);
    }
    Ok(u16::from_le_bytes(bytes[offset..end].try_into().unwrap()))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, DecodeError> {
    let end = offset + 4;
    if end > bytes.len() {
        return Err(DecodeError::Truncated);
    }
    Ok(u32::from_le_bytes(bytes[offset..end].try_into().unwrap()))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, DecodeError> {
    let end = offset + 8;
    if end > bytes.len() {
        return Err(DecodeError::Truncated);
    }
    Ok(u64::from_le_bytes(bytes[offset..end].try_into().unwrap()))
}

fn read_i32(bytes: &[u8], offset: usize) -> Result<i32, DecodeError> {
    let end = offset + 4;
    if end > bytes.len() {
        return Err(DecodeError::Truncated);
    }
    Ok(i32::from_le_bytes(bytes[offset..end].try_into().unwrap()))
}

fn read_f32(bytes: &[u8], offset: usize) -> Result<f32, DecodeError> {
    let end = offset + 4;
    if end > bytes.len() {
        return Err(DecodeError::Truncated);
    }
    Ok(f32::from_le_bytes(bytes[offset..end].try_into().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_rejects_bad_magic() {
        let mut buf = vec![0u8; UI_SCENE_HEADER_BYTES];
        buf[0..4].copy_from_slice(&0x1234_5678u32.to_le_bytes());
        buf[4..6].copy_from_slice(&UI_SCENE_VERSION.to_le_bytes());
        let err = UiScene::decode(&buf).unwrap_err();
        assert!(matches!(err, DecodeError::BadMagic(0x1234_5678)));
    }

    #[test]
    fn decode_rejects_bad_version() {
        let mut buf = vec![0u8; UI_SCENE_HEADER_BYTES];
        buf[0..4].copy_from_slice(&UI_SCENE_MAGIC.to_le_bytes());
        buf[4..6].copy_from_slice(&999u16.to_le_bytes());
        let err = UiScene::decode(&buf).unwrap_err();
        assert!(matches!(err, DecodeError::BadVersion(999)));
    }
}
