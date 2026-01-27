extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use abi::ids::HandleId;
use abi::ui_scene::{
    AlignItems, FlexDirection, ImageFit, JustifyContent, NodeKind, SizeKind, StringRef,
    UI_SCENE_HEADER_BYTES, UI_SCENE_HEADER_MAGIC_OFFSET, UI_SCENE_HEADER_NODE_BYTES_OFFSET,
    UI_SCENE_HEADER_NODE_COUNT_OFFSET, UI_SCENE_HEADER_STRING_BYTES_OFFSET,
    UI_SCENE_HEADER_VERSION_OFFSET, UI_SCENE_MAGIC, UI_SCENE_NODE_BYTES,
    UI_SCENE_NODE_FLEX_BASIS_KIND_OFFSET, UI_SCENE_NODE_FLEX_BASIS_VALUE_OFFSET,
    UI_SCENE_NODE_FLEX_GROW_OFFSET, UI_SCENE_NODE_FLEX_SHRINK_OFFSET,
    UI_SCENE_NODE_HEIGHT_KIND_OFFSET, UI_SCENE_NODE_HEIGHT_VALUE_OFFSET, UI_SCENE_NODE_ID_OFFSET,
    UI_SCENE_NODE_KIND_OFFSET, UI_SCENE_NODE_MARGIN_BOTTOM_OFFSET, UI_SCENE_NODE_MARGIN_LEFT_OFFSET,
    UI_SCENE_NODE_MARGIN_RIGHT_OFFSET, UI_SCENE_NODE_MARGIN_TOP_OFFSET,
    UI_SCENE_NODE_MAX_HEIGHT_OFFSET, UI_SCENE_NODE_MAX_WIDTH_OFFSET,
    UI_SCENE_NODE_MIN_HEIGHT_OFFSET, UI_SCENE_NODE_MIN_WIDTH_OFFSET,
    UI_SCENE_NODE_PADDING_BOTTOM_OFFSET, UI_SCENE_NODE_PADDING_LEFT_OFFSET,
    UI_SCENE_NODE_PADDING_RIGHT_OFFSET, UI_SCENE_NODE_PADDING_TOP_OFFSET,
    UI_SCENE_NODE_PARENT_OFFSET, UI_SCENE_NODE_PAYLOAD_OFFSET, UI_SCENE_NODE_WIDTH_KIND_OFFSET,
    UI_SCENE_NODE_WIDTH_VALUE_OFFSET, UI_SCENE_PARENT_NONE, UI_SCENE_VERSION,
    UI_SCENE_WINDOW_INIT_H_OFFSET, UI_SCENE_WINDOW_INIT_W_OFFSET, UI_SCENE_WINDOW_MAX_H_OFFSET,
    UI_SCENE_WINDOW_MAX_W_OFFSET, UI_SCENE_WINDOW_MIN_H_OFFSET, UI_SCENE_WINDOW_MIN_W_OFFSET,
    UI_SCENE_WINDOW_TITLE_LEN_OFFSET, UI_SCENE_WINDOW_TITLE_OFFSET_OFFSET, UI_SCENE_WINDOW_WID_OFFSET,
    UI_SCENE_FLEX_ALIGN_OFFSET, UI_SCENE_FLEX_DIR_OFFSET, UI_SCENE_FLEX_GAP_OFFSET,
    UI_SCENE_FLEX_JUSTIFY_OFFSET, UI_SCENE_TEXT_COLOR_OFFSET, UI_SCENE_TEXT_FONT_LEN_OFFSET,
    UI_SCENE_TEXT_FONT_OFFSET_OFFSET, UI_SCENE_TEXT_SIZE_OFFSET, UI_SCENE_TEXT_TEXT_LEN_OFFSET,
    UI_SCENE_TEXT_TEXT_OFFSET_OFFSET, UI_SCENE_RECT_COLOR_OFFSET, UI_SCENE_RECT_RADIUS_OFFSET,
    UI_SCENE_IMAGE_FIT_OFFSET, UI_SCENE_IMAGE_KEY_LEN_OFFSET, UI_SCENE_IMAGE_KEY_OFFSET_OFFSET,
    UI_SCENE_CHECKBOX_CHECKED_OFFSET, UI_SCENE_CHECKBOX_LABEL_LEN_OFFSET,
    UI_SCENE_CHECKBOX_LABEL_OFFSET_OFFSET,
};

use crate::errors::{Error, Result};
use crate::petals::builder::{
    AlignItems as BuilderAlign, CheckboxData, FlexData, FlexDirection as BuilderDirection,
    ImageData, ImageFit as BuilderFit, JustifyContent as BuilderJustify, Node, NodeData, Scene,
    Size, Style, TextData, WindowData,
};

pub fn pack_scene(scene: &Scene) -> Result<Vec<u8>> {
    let root = scene
        .root
        .as_ref()
        .ok_or(Error::Errno(abi::errors::Errno::EINVAL))?;
    let node_count = count_nodes(root);
    let mut string_table = StringTable::new();
    collect_strings(root, &mut string_table);
    string_table.finalize();

    let mut nodes_bytes = Vec::with_capacity(node_count * UI_SCENE_NODE_BYTES);
    let mut next_id = 0u32;
    pack_node(
        root,
        UI_SCENE_PARENT_NONE,
        &mut next_id,
        &string_table,
        &mut nodes_bytes,
    );

    let mut out = Vec::with_capacity(
        UI_SCENE_HEADER_BYTES + nodes_bytes.len() + string_table.bytes.len(),
    );
    out.resize(UI_SCENE_HEADER_BYTES, 0);
    write_u32(&mut out, UI_SCENE_HEADER_MAGIC_OFFSET, UI_SCENE_MAGIC);
    write_u16(&mut out, UI_SCENE_HEADER_VERSION_OFFSET, UI_SCENE_VERSION);
    write_u32(
        &mut out,
        UI_SCENE_HEADER_NODE_COUNT_OFFSET,
        node_count as u32,
    );
    write_u32(
        &mut out,
        UI_SCENE_HEADER_NODE_BYTES_OFFSET,
        UI_SCENE_NODE_BYTES as u32,
    );
    write_u32(
        &mut out,
        UI_SCENE_HEADER_STRING_BYTES_OFFSET,
        string_table.bytes.len() as u32,
    );
    out.extend_from_slice(&nodes_bytes);
    out.extend_from_slice(&string_table.bytes);
    Ok(out)
}

fn count_nodes(node: &Node) -> usize {
    let mut count = 1;
    for child in &node.children {
        count += count_nodes(child);
    }
    count
}

fn collect_strings(node: &Node, table: &mut StringTable) {
    match &node.data {
        NodeData::Window(WindowData { title, .. }) => {
            if let Some(title) = title {
                table.add(title);
            }
        }
        NodeData::Text(TextData { text, font, .. }) => {
            table.add(text);
            if let Some(font) = font {
                table.add(font);
            }
        }
        NodeData::Image(ImageData { key, .. }) => {
            table.add(key);
        }
        NodeData::Checkbox(CheckboxData { label, .. }) => {
            if let Some(label) = label {
                table.add(label);
            }
        }
        _ => {}
    }
    for child in &node.children {
        collect_strings(child, table);
    }
}

fn pack_node(
    node: &Node,
    parent_id: u32,
    next_id: &mut u32,
    table: &StringTable,
    out: &mut Vec<u8>,
) {
    let id = *next_id;
    *next_id = next_id.wrapping_add(1);
    let mut buf = [0u8; UI_SCENE_NODE_BYTES];
    write_u32_slice(&mut buf, UI_SCENE_NODE_ID_OFFSET, id);
    write_u32_slice(&mut buf, UI_SCENE_NODE_PARENT_OFFSET, parent_id);

    let (kind, payload_writer) = match &node.data {
        NodeData::Window(data) => (NodeKind::Window, PayloadWriter::Window(data)),
        NodeData::Flex(data) => (NodeKind::Flex, PayloadWriter::Flex(data)),
        NodeData::Text(data) => (NodeKind::Text, PayloadWriter::Text(data)),
        NodeData::Rect(data) => (NodeKind::Rect, PayloadWriter::Rect(data)),
        NodeData::Image(data) => (NodeKind::Image, PayloadWriter::Image(data)),
        NodeData::Checkbox(data) => (NodeKind::Checkbox, PayloadWriter::Checkbox(data)),
    };
    write_u16_slice(&mut buf, UI_SCENE_NODE_KIND_OFFSET, kind.as_raw());
    write_style(&mut buf, &node.style);
    write_payload(&mut buf, payload_writer, table);
    out.extend_from_slice(&buf);

    for child in &node.children {
        pack_node(child, id, next_id, table, out);
    }
}

fn write_style(buf: &mut [u8; UI_SCENE_NODE_BYTES], style: &Style) {
    write_size(
        buf,
        UI_SCENE_NODE_WIDTH_KIND_OFFSET,
        UI_SCENE_NODE_WIDTH_VALUE_OFFSET,
        style.width,
    );
    write_size(
        buf,
        UI_SCENE_NODE_HEIGHT_KIND_OFFSET,
        UI_SCENE_NODE_HEIGHT_VALUE_OFFSET,
        style.height,
    );
    write_size(
        buf,
        UI_SCENE_NODE_FLEX_BASIS_KIND_OFFSET,
        UI_SCENE_NODE_FLEX_BASIS_VALUE_OFFSET,
        style.flex_basis,
    );
    write_i32_slice(
        buf,
        UI_SCENE_NODE_MIN_WIDTH_OFFSET,
        style.min_width.unwrap_or(-1),
    );
    write_i32_slice(
        buf,
        UI_SCENE_NODE_MIN_HEIGHT_OFFSET,
        style.min_height.unwrap_or(-1),
    );
    write_i32_slice(
        buf,
        UI_SCENE_NODE_MAX_WIDTH_OFFSET,
        style.max_width.unwrap_or(-1),
    );
    write_i32_slice(
        buf,
        UI_SCENE_NODE_MAX_HEIGHT_OFFSET,
        style.max_height.unwrap_or(-1),
    );
    write_i32_slice(buf, UI_SCENE_NODE_MARGIN_LEFT_OFFSET, style.margin.left);
    write_i32_slice(buf, UI_SCENE_NODE_MARGIN_TOP_OFFSET, style.margin.top);
    write_i32_slice(buf, UI_SCENE_NODE_MARGIN_RIGHT_OFFSET, style.margin.right);
    write_i32_slice(buf, UI_SCENE_NODE_MARGIN_BOTTOM_OFFSET, style.margin.bottom);
    write_i32_slice(buf, UI_SCENE_NODE_PADDING_LEFT_OFFSET, style.padding.left);
    write_i32_slice(buf, UI_SCENE_NODE_PADDING_TOP_OFFSET, style.padding.top);
    write_i32_slice(buf, UI_SCENE_NODE_PADDING_RIGHT_OFFSET, style.padding.right);
    write_i32_slice(buf, UI_SCENE_NODE_PADDING_BOTTOM_OFFSET, style.padding.bottom);
    write_f32_slice(buf, UI_SCENE_NODE_FLEX_GROW_OFFSET, style.flex_grow);
    write_f32_slice(buf, UI_SCENE_NODE_FLEX_SHRINK_OFFSET, style.flex_shrink);
}

fn write_size(buf: &mut [u8; UI_SCENE_NODE_BYTES], kind_offset: usize, value_offset: usize, size: Size) {
    match size {
        Size::Auto => {
            buf[kind_offset] = SizeKind::Auto as u8;
            write_i32_slice(buf, value_offset, 0);
        }
        Size::Px(v) => {
            buf[kind_offset] = SizeKind::Px as u8;
            write_i32_slice(buf, value_offset, v);
        }
        Size::Pct(v) => {
            buf[kind_offset] = SizeKind::Pct as u8;
            write_i32_slice(buf, value_offset, v as i32);
        }
    }
}

enum PayloadWriter<'a> {
    Window(&'a WindowData),
    Flex(&'a FlexData),
    Text(&'a TextData),
    Rect(&'a crate::petals::builder::RectData),
    Image(&'a ImageData),
    Checkbox(&'a CheckboxData),
}

fn write_payload(buf: &mut [u8; UI_SCENE_NODE_BYTES], payload: PayloadWriter<'_>, table: &StringTable) {
    let payload_buf = &mut buf[UI_SCENE_NODE_PAYLOAD_OFFSET..UI_SCENE_NODE_PAYLOAD_OFFSET + abi::ui_scene::UI_SCENE_NODE_PAYLOAD_BYTES];
    match payload {
        PayloadWriter::Window(data) => {
            write_u64_slice(payload_buf, UI_SCENE_WINDOW_WID_OFFSET, data.wid.to_u64_lossy());
            let title = table.ref_for(data.title.as_deref());
            write_u32_slice(payload_buf, UI_SCENE_WINDOW_TITLE_OFFSET_OFFSET, title.offset);
            write_u32_slice(payload_buf, UI_SCENE_WINDOW_TITLE_LEN_OFFSET, title.len);
            let (min_w, min_h) = data.min_size.unwrap_or((-1, -1));
            let (max_w, max_h) = data.max_size.unwrap_or((-1, -1));
            let (init_w, init_h) = data.init_size.unwrap_or((-1, -1));
            write_i32_slice(payload_buf, UI_SCENE_WINDOW_MIN_W_OFFSET, min_w);
            write_i32_slice(payload_buf, UI_SCENE_WINDOW_MIN_H_OFFSET, min_h);
            write_i32_slice(payload_buf, UI_SCENE_WINDOW_MAX_W_OFFSET, max_w);
            write_i32_slice(payload_buf, UI_SCENE_WINDOW_MAX_H_OFFSET, max_h);
            write_i32_slice(payload_buf, UI_SCENE_WINDOW_INIT_W_OFFSET, init_w);
            write_i32_slice(payload_buf, UI_SCENE_WINDOW_INIT_H_OFFSET, init_h);
        }
        PayloadWriter::Flex(data) => {
            payload_buf[UI_SCENE_FLEX_DIR_OFFSET] = match data.direction {
                BuilderDirection::Row => FlexDirection::Row as u8,
                BuilderDirection::Column => FlexDirection::Column as u8,
            };
            payload_buf[UI_SCENE_FLEX_ALIGN_OFFSET] = match data.align {
                BuilderAlign::Start => AlignItems::Start as u8,
                BuilderAlign::Center => AlignItems::Center as u8,
                BuilderAlign::End => AlignItems::End as u8,
                BuilderAlign::Stretch => AlignItems::Stretch as u8,
            };
            payload_buf[UI_SCENE_FLEX_JUSTIFY_OFFSET] = match data.justify {
                BuilderJustify::Start => JustifyContent::Start as u8,
                BuilderJustify::Center => JustifyContent::Center as u8,
                BuilderJustify::End => JustifyContent::End as u8,
            };
            write_i32_slice(payload_buf, UI_SCENE_FLEX_GAP_OFFSET, data.gap);
        }
        PayloadWriter::Text(data) => {
            let text = table.ref_for(Some(&data.text));
            let font = table.ref_for(data.font.as_deref());
            write_u32_slice(payload_buf, UI_SCENE_TEXT_TEXT_OFFSET_OFFSET, text.offset);
            write_u32_slice(payload_buf, UI_SCENE_TEXT_TEXT_LEN_OFFSET, text.len);
            write_u32_slice(payload_buf, UI_SCENE_TEXT_FONT_OFFSET_OFFSET, font.offset);
            write_u32_slice(payload_buf, UI_SCENE_TEXT_FONT_LEN_OFFSET, font.len);
            write_i32_slice(payload_buf, UI_SCENE_TEXT_SIZE_OFFSET, data.size);
            write_u32_slice(payload_buf, UI_SCENE_TEXT_COLOR_OFFSET, data.color.0);
        }
        PayloadWriter::Rect(data) => {
            write_u32_slice(payload_buf, UI_SCENE_RECT_COLOR_OFFSET, data.color.0);
            write_i32_slice(payload_buf, UI_SCENE_RECT_RADIUS_OFFSET, data.radius.unwrap_or(-1));
        }
        PayloadWriter::Image(data) => {
            let key = table.ref_for(Some(&data.key));
            write_u32_slice(payload_buf, UI_SCENE_IMAGE_KEY_OFFSET_OFFSET, key.offset);
            write_u32_slice(payload_buf, UI_SCENE_IMAGE_KEY_LEN_OFFSET, key.len);
            payload_buf[UI_SCENE_IMAGE_FIT_OFFSET] = match data.fit {
                BuilderFit::Fill => ImageFit::Fill as u8,
                BuilderFit::Contain => ImageFit::Contain as u8,
                BuilderFit::Cover => ImageFit::Cover as u8,
                BuilderFit::None => ImageFit::None as u8,
            };
        }
        PayloadWriter::Checkbox(data) => {
            payload_buf[UI_SCENE_CHECKBOX_CHECKED_OFFSET] = if data.checked { 1 } else { 0 };
            let label = table.ref_for(data.label.as_deref());
            write_u32_slice(payload_buf, UI_SCENE_CHECKBOX_LABEL_OFFSET_OFFSET, label.offset);
            write_u32_slice(payload_buf, UI_SCENE_CHECKBOX_LABEL_LEN_OFFSET, label.len);
        }
    }
}

struct StringTable {
    refs: BTreeMap<String, StringRef>,
    bytes: Vec<u8>,
}

impl StringTable {
    fn new() -> Self {
        Self {
            refs: BTreeMap::new(),
            bytes: Vec::new(),
        }
    }

    fn add(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        self.refs
            .entry(s.to_string())
            .or_insert(StringRef { offset: 0, len: s.len() as u32 });
    }

    fn finalize(&mut self) {
        for (key, entry) in self.refs.iter_mut() {
            entry.offset = self.bytes.len() as u32;
            self.bytes.extend_from_slice(key.as_bytes());
        }
    }

    fn ref_for(&self, s: Option<&str>) -> StringRef {
        let s = match s {
            Some(s) if !s.is_empty() => s,
            _ => return StringRef::empty(),
        };
        self.refs.get(s).copied().unwrap_or(StringRef::empty())
    }
}

fn write_u16(buf: &mut [u8], offset: usize, value: u16) {
    buf[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_u16_slice(buf: &mut [u8], offset: usize, value: u16) {
    buf[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32_slice(buf: &mut [u8], offset: usize, value: u32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_u64_slice(buf: &mut [u8], offset: usize, value: u64) {
    buf[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn write_i32_slice(buf: &mut [u8], offset: usize, value: i32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_f32_slice(buf: &mut [u8], offset: usize, value: f32) {
    buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::petals::builder::{Checkbox, Color, Flex, FontKey, Scene, Text, Window};
    use crate::thing::ThingId;

    #[test]
    fn deterministic_packing() {
        let wid = ThingId::default();
        let scene = Scene::new().window(
            Window::new(wid).title("Clock").root(
                Flex::column()
                    .gap(8)
                    .padding(12)
                    .push(
                        Text::new("04:04:31")
                            .font(FontKey::new("NotoSans").size(24))
                            .color(Color::rgb(240, 240, 240)),
                    )
                    .push(Checkbox::new(true).label("UTC")),
            ),
        );
        let bytes_a = pack_scene(&scene).expect("pack scene");
        let bytes_b = pack_scene(&scene).expect("pack scene");
        assert_eq!(bytes_a, bytes_b);
    }

    #[test]
    fn pack_unpack_roundtrip() {
        let wid = ThingId::default();
        let scene = Scene::new().window(
            Window::new(wid)
                .title("Hello")
                .root(Text::new("Hi").font(FontKey::new("NotoSans").size(16))),
        );
        let bytes = pack_scene(&scene).expect("pack scene");
        let decoded = abi::ui_scene::UiScene::decode(&bytes).expect("decode scene");
        assert_eq!(decoded.node_count(), 2);
        let window = decoded.node(0).unwrap();
        assert_eq!(window.kind(), NodeKind::Window);
        let meta = window.window_meta().unwrap();
        assert_eq!(decoded.string(meta.title).unwrap(), "Hello");
    }
}
