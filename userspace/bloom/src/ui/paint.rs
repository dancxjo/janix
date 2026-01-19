use alloc::string::String;
use alloc::vec::Vec;

use crate::damage::Rect;
use crate::geometry::Color;
use crate::ui::layout::{LayoutNode, LayoutTree, SymbolResolver};
use crate::ui::snapshot::{UiNodeKind, UiNodeSnapshot, UiSnapshot};
use abi::schema::keys;
use abi::WireType::ThingId;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct PaintScene {
    pub objects: Vec<PaintObject>,
}

pub struct PaintBuilder;

impl PaintBuilder {
    pub fn build(
        snapshot: &UiSnapshot,
        layout: &LayoutTree,
        symbols: &impl SymbolResolver,
    ) -> PaintScene {
        let mut objects = Vec::new();
        if let Some(root) = &layout.root {
            Self::build_recursive(snapshot, root, &mut objects, symbols);
        }
        for obj in objects.iter() {
            match obj {
                PaintObject::Rect {
                    rect: _, color: _, ..
                } => {
                    // stem::info!("PAINT: obj=Rect rect={:?} color={:x}", rect, color.to_u32());
                }
                PaintObject::Text {
                    rect: _,
                    text: _,
                    font: _,
                    ..
                } => {
                    // stem::info!("PAINT: obj=Text rect={:?} text='{}' font={}", rect, text, font);
                }
                PaintObject::Image { rect: _ } => {
                    // stem::info!("PAINT: obj=Image rect={:?}", rect);
                }
            }
        }
        PaintScene { objects }
    }

    fn build_recursive(
        snapshot: &UiSnapshot,
        layout_node: &LayoutNode,
        objects: &mut Vec<PaintObject>,
        symbols: &impl SymbolResolver,
    ) {
        if let Some(node_snapshot) = snapshot.nodes.get(&layout_node.id) {
            // Create paint object based on kind and properties
            if let Some(obj) = Self::create_paint_object(node_snapshot, layout_node, symbols) {
                objects.push(obj);
            }
        }

        for child in &layout_node.children {
            Self::build_recursive(snapshot, child, objects, symbols);
        }
    }

    fn create_paint_object(
        node: &UiNodeSnapshot,
        layout: &LayoutNode,
        symbols: &impl SymbolResolver,
    ) -> Option<PaintObject> {
        // v0: Check kind and pluck styles

        // Check for UI_TEXT using symbol resolver
        let text_key_id = symbols.resolve(keys::UI_TEXT);
        let has_text = if let Some(id) = text_key_id {
            node.props.contains_key(&id)
        } else {
            false
        };

        if has_text {
            let text = Self::get_str_prop(node, keys::UI_TEXT, symbols).unwrap_or_default();
            let font = Self::get_str_prop(node, keys::UI_FONT, symbols)
                .unwrap_or_else(|| "NotoSans-Regular.ttf".into());
            let size = Self::get_prop(node, keys::UI_FONT_SIZE, symbols) as f32;

            let mut color_val = Self::get_prop(node, keys::UI_FG_COLOR, symbols);
            if color_val == 0 {
                color_val = Self::get_prop(node, keys::UI_COLOR, symbols);
            }
            let color = Color::from_u32(color_val as u32);

            return Some(PaintObject::Text {
                rect: layout.rect.clone(),
                text,
                font,
                size: if size == 0.0 { 16.0 } else { size },
                color,
            });
        }

        // Default to a colored rect if it has dimensions or a color
        let mut color_val = Self::get_prop(node, keys::UI_BG_COLOR, symbols);
        if color_val == 0 {
            color_val = Self::get_prop(node, keys::UI_COLOR, symbols);
        }

        if color_val != 0 || layout.rect.w > 0 {
            return Some(PaintObject::Rect {
                rect: layout.rect.clone(),
                color: Color::from_u32(color_val as u32),
                radius: Self::get_prop(node, keys::UI_RADIUS, symbols) as u32,
            });
        }

        None
    }

    fn get_prop(node: &UiNodeSnapshot, key: &str, symbols: &impl SymbolResolver) -> u64 {
        if let Some(id) = symbols.resolve(key) {
            return *node.props.get(&id).unwrap_or(&0);
        }
        0
    }

    fn get_str_prop(
        node: &UiNodeSnapshot,
        key: &str,
        symbols: &impl SymbolResolver,
    ) -> Option<String> {
        if let Some(id) = symbols.resolve(key) {
            return node.strings.get(&id).cloned();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;
    use alloc::string::ToString;
    use alloc::vec;

    struct MockSymbolResolver {
        map: BTreeMap<String, u32>,
    }

    impl MockSymbolResolver {
        fn new() -> Self {
            let mut map = BTreeMap::new();
            // Pre-seed common keys
            map.insert(keys::UI_X.to_string(), 1);
            map.insert(keys::UI_Y.to_string(), 2);
            map.insert(keys::UI_WIDTH.to_string(), 3);
            map.insert(keys::UI_HEIGHT.to_string(), 4);
            map.insert(keys::UI_COLOR.to_string(), 7);
            map.insert(keys::UI_BG_COLOR.to_string(), 8);
            Self { map }
        }
    }

    impl SymbolResolver for MockSymbolResolver {
        fn resolve(&self, key: &str) -> Option<u32> {
            self.map.get(key).cloned()
        }
    }

    #[test]
    fn test_paint_determinism() {
        // 1. Setup Snapshot
        let mut snapshot = UiSnapshot::new();
        fn make_id(n: u8) -> abi::ThingId {
            let mut b = [0u8; 16];
            b[0] = n;
            abi::ThingId(b)
        }
        let root_id = make_id(1);
        snapshot.root_id = Some(root_id);

        let mut props = BTreeMap::new();
        props.insert(8, 0xFF0000); // UI_BG_COLOR = Red

        snapshot.nodes.insert(
            root_id,
            UiNodeSnapshot {
                id: root_id,
                kind: UiNodeKind::Window,
                props,
                strings: BTreeMap::new(),
                children: vec![],
            },
        );

        // 2. Setup Layout
        let layout = LayoutTree {
            root: Some(LayoutNode {
                id: root_id,
                rect: Rect::new(0, 0, 100, 100),
                z_index: 0,
                children: vec![],
            }),
        };

        let resolver = MockSymbolResolver::new();

        // 3. Build twice
        let scene1 = PaintBuilder::build(&snapshot, &layout, &resolver);
        let scene2 = PaintBuilder::build(&snapshot, &layout, &resolver);

        // 4. Assert equality
        assert_eq!(scene1, scene2);
        assert_eq!(scene1.objects.len(), 1);
        match &scene1.objects[0] {
            PaintObject::Rect { color, .. } => {
                assert_eq!(color.to_u32(), 0xFF0000);
            }
            _ => panic!("Expected Rect"),
        }
    }
}
