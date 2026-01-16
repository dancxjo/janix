use alloc::vec::Vec;
use alloc::string::String;
use stem::thing::ThingId;
use crate::geometry::Color;
use crate::damage::Rect;
use crate::ui::layout::{LayoutTree, LayoutNode};
use crate::ui::snapshot::{UiSnapshot, UiNodeSnapshot, UiNodeKind};
use abi::schema::keys;

#[derive(Debug, Clone)]
pub enum PaintObject {
    Rect {
        rect: Rect,
        color: Color,
        radius: u32,
    },
    Text {
        rect: Rect,
        text: String,
        font: String,
        size: f32,
        color: Color,
    },
    Image {
        rect: Rect,
        // image_ref: ...
    },
}

pub struct PaintScene {
    pub objects: Vec<PaintObject>,
}

pub struct PaintBuilder;

impl PaintBuilder {
    pub fn build(snapshot: &UiSnapshot, layout: &LayoutTree) -> PaintScene {
        let mut objects = Vec::new();
        if let Some(root) = &layout.root {
            Self::build_recursive(snapshot, root, &mut objects);
        }
        PaintScene { objects }
    }

    fn build_recursive(snapshot: &UiSnapshot, layout_node: &LayoutNode, objects: &mut Vec<PaintObject>) {
        if let Some(node_snapshot) = snapshot.nodes.get(&layout_node.id) {
            // Create paint object based on kind and properties
            if let Some(obj) = Self::create_paint_object(node_snapshot, layout_node) {
                objects.push(obj);
            }
        }

        for child in &layout_node.children {
            Self::build_recursive(snapshot, child, objects);
        }
    }

    fn create_paint_object(node: &UiNodeSnapshot, layout: &LayoutNode) -> Option<PaintObject> {
        // v0: Check kind and pluck styles
        // We temporarily use kind sym comparison or string mapping if we had it.
        // For now, let's just use the props to infer what to draw.
        
        if node.props.contains_key(&Self::intern(keys::UI_TEXT)) {
            let text = Self::get_str_prop(node, keys::UI_TEXT).unwrap_or_default();
            let font = Self::get_str_prop(node, keys::UI_FONT).unwrap_or_else(|| "NotoSans-Regular.ttf".into());
            let size = Self::get_prop(node, keys::UI_FONT_SIZE) as f32;
            let color = Color::from_u32(Self::get_prop(node, keys::UI_COLOR) as u32);
            
            return Some(PaintObject::Text {
                rect: layout.rect.clone(),
                text,
                font,
                size: if size == 0.0 { 16.0 } else { size },
                color,
            });
        }

        // Default to a colored rect if it has dimensions or a color
        let color_val = Self::get_prop(node, keys::UI_COLOR);
        if color_val != 0 || layout.rect.w > 0 {
            return Some(PaintObject::Rect {
                rect: layout.rect.clone(),
                color: Color::from_u32(color_val as u32),
                radius: Self::get_prop(node, keys::UI_RADIUS) as u32,
            });
        }

        None
    }

    fn intern(key: &str) -> u32 {
        stem::thing::sys::intern(key).unwrap_or(0)
    }

    fn get_prop(node: &UiNodeSnapshot, key: &str) -> u64 {
        let id = Self::intern(key);
        *node.props.get(&id).unwrap_or(&0)
    }

    fn get_str_prop(node: &UiNodeSnapshot, key: &str) -> Option<String> {
        // In thing-os, string props might be stored as Bytespace IDs or interned symbols.
        // For v0, let's assume UI_TEXT is a Bytespace ID we can read.
        let val = Self::get_prop(node, key);
        if val == 0 { return None; }
        
        let bs_id = ThingId(val);
        let mut buf = [0u8; 1024];
        if let Ok(len) = stem::thing::sys::bytespace_read(bs_id, 0, &mut buf) {
            return Some(String::from(core::str::from_utf8(&buf[..len]).unwrap_or_default()));
        }
        None
    }
}
