use crate::asset::AssetBank;
use crate::damage::Rect;
use crate::ui::snapshot::{UiNodeSnapshot, UiSnapshot};
use abi::schema::keys;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use stem::thing::ThingId;

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: ThingId,
    pub rect: Rect,
    pub z_index: i32,
    pub children: Vec<LayoutNode>,
}

pub struct LayoutTree {
    pub root: Option<LayoutNode>,
}

pub trait SymbolResolver {
    fn resolve(&self, key: &str) -> Option<u32>;
}

pub struct LayoutSolver {
    pub measure_cache: BTreeMap<(String, String, u32), (f32, f32)>,
}

impl LayoutSolver {
    pub fn new() -> Self {
        Self {
            measure_cache: BTreeMap::new(),
        }
    }

    pub fn solve(
        &mut self,
        snapshot: &UiSnapshot,
        screen_w: i32,
        screen_h: i32,
        assets: &AssetBank,
        symbols: &impl SymbolResolver,
    ) -> LayoutTree {
        let root_id = match snapshot.root_id {
            Some(id) => id,
            None => return LayoutTree { root: None },
        };

        let root_node = match snapshot.nodes.get(&root_id) {
            Some(n) => n,
            None => return LayoutTree { root: None },
        };

        // For v0, we assume the root covers the whole screen or handles its own layout.
        let mut root_layout = LayoutNode {
            id: root_id,
            rect: Rect::new(0, 0, screen_w, screen_h),
            z_index: 0,
            children: Vec::new(),
        };

        Self::layout_children(
            snapshot,
            root_node,
            &mut root_layout,
            assets,
            symbols,
            &mut self.measure_cache,
        );

        LayoutTree {
            root: Some(root_layout),
        }
    }

    fn layout_children(
        snapshot: &UiSnapshot,
        node: &UiNodeSnapshot,
        layout: &mut LayoutNode,
        assets: &AssetBank,
        symbols: &impl SymbolResolver,
        cache: &mut BTreeMap<(String, String, u32), (f32, f32)>,
    ) {
        for child_id in &node.children {
            if let Some(child_node) = snapshot.nodes.get(child_id) {
                // Determine layout strategy for this node.

                let mut w = Self::get_prop(child_node, keys::UI_WIDTH, symbols) as i32;
                let mut h = Self::get_prop(child_node, keys::UI_HEIGHT, symbols) as i32;

                // Check if centering requested
                let center_x = Self::get_prop(child_node, keys::UI_CENTER_X, symbols) != 0;
                let center_y = Self::get_prop(child_node, keys::UI_CENTER_Y, symbols) != 0;

                if center_x || center_y {
                    // Try to measure if it's text
                    if let Some(text) = Self::get_str_prop(child_node, keys::UI_TEXT, symbols) {
                        let font_name = Self::get_str_prop(child_node, keys::UI_FONT, symbols)
                            .unwrap_or_else(|| "NotoSans-Regular.ttf".into());
                        let size = Self::get_prop(child_node, keys::UI_FONT_SIZE, symbols) as f32;
                        let font_size = if size == 0.0 { 16.0 } else { size };

                        let cache_key = (text.clone(), font_name.clone(), font_size as u32);
                        let dims = if let Some(d) = cache.get(&cache_key) {
                            Some(*d)
                        } else {
                            let d = Self::measure_text(&text, &font_name, font_size, assets);
                            if let Some(res) = d {
                                cache.insert(cache_key, res);
                            }
                            d
                        };

                        if let Some(dims) = dims {
                            w = dims.0 as i32;
                            h = dims.1 as i32;
                        }
                    }
                }

                let mut x = Self::get_prop(child_node, keys::UI_X, symbols) as i32;
                let mut y = Self::get_prop(child_node, keys::UI_Y, symbols) as i32;

                if center_x {
                    x = (layout.rect.w - w) / 2;
                }
                if center_y {
                    y = (layout.rect.h - h) / 2;
                }

                let z = Self::get_prop(child_node, keys::UI_Z_INDEX, symbols) as i32;

                let mut child_layout = LayoutNode {
                    id: *child_id,
                    rect: Rect::new(layout.rect.x + x, layout.rect.y + y, w, h),
                    z_index: z,
                    children: Vec::new(),
                };

                Self::layout_children(
                    snapshot,
                    child_node,
                    &mut child_layout,
                    assets,
                    symbols,
                    cache,
                );
                layout.children.push(child_layout);
            }
        }

        // Sort children by Z-index
        layout.children.sort_by_key(|n| n.z_index);
    }

    fn measure_text(
        text: &str,
        font_name: &str,
        size: f32,
        assets: &AssetBank,
    ) -> Option<(f32, f32)> {
        // Find font
        let fonts = assets.get_fonts(); // Get all ready fonts
        let font = fonts
            .iter()
            .find(|f| f.name.contains(font_name))
            .or_else(|| fonts.first()); // Fallback

        if let Some(f) = font {
            // Basic measurement: width sum + max height
            // fontdue has metrics
            let metrics = f.font.horizontal_line_metrics(size);
            if let Some(m) = metrics {
                let mut width = 0.0;
                for ch in text.chars() {
                    let metrics = f.font.metrics(ch, size);
                    width += metrics.advance_width;
                }
                return Some((width, m.new_line_size));
            }
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
    use crate::ui::snapshot::{UiNodeKind, UiNodeSnapshot};
    use abi::schema::keys;
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
            map.insert(keys::UI_CENTER_X.to_string(), 5);
            map.insert(keys::UI_CENTER_Y.to_string(), 6);
            Self { map }
        }
    }

    impl SymbolResolver for MockSymbolResolver {
        fn resolve(&self, key: &str) -> Option<u32> {
            self.map.get(key).cloned()
        }
    }

    #[test]
    fn test_layout_centering() {
        let mut snapshot = UiSnapshot::new();

        fn make_id(n: u8) -> ThingId {
            let mut b = [0u8; 16];
            b[0] = n;
            ThingId(b)
        }

        let root_id = make_id(1);
        let child_id = make_id(2);
        snapshot.root_id = Some(root_id);

        // Root node
        let root_props = BTreeMap::new();
        snapshot.nodes.insert(
            root_id,
            UiNodeSnapshot {
                id: root_id,
                kind: UiNodeKind::Root,
                props: root_props,
                strings: BTreeMap::new(),
                children: vec![child_id],
            },
        );

        // Child node: 100x50, centered
        let mut child_props = BTreeMap::new();
        child_props.insert(3, 100); // UI_WIDTH
        child_props.insert(4, 50); // UI_HEIGHT
        child_props.insert(5, 1); // UI_CENTER_X
        child_props.insert(6, 1); // UI_CENTER_Y

        snapshot.nodes.insert(
            child_id,
            UiNodeSnapshot {
                id: child_id,
                kind: UiNodeKind::Window, // or whatever
                props: child_props,
                strings: BTreeMap::new(),
                children: vec![],
            },
        );

        let assets = AssetBank::new();
        let resolver = MockSymbolResolver::new();

        // Screen 800x600.
        // Child 100x50 centered should be at x=350, y=275
        let mut solver = LayoutSolver::new();
        let tree = solver.solve(&snapshot, 800, 600, &assets, &resolver);

        assert!(tree.root.is_some());
        let root = tree.root.unwrap();
        assert_eq!(root.children.len(), 1);

        let child = &root.children[0];
        assert_eq!(child.rect.x, 350);
        assert_eq!(child.rect.y, 275);
        assert_eq!(child.rect.w, 100);
        assert_eq!(child.rect.h, 50);
    }
}
