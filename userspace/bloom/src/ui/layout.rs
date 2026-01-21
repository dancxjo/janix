use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use stem::thing::ThingId;
use crate::damage::Rect;
use crate::ui::snapshot::{UiSnapshot, UiNodeSnapshot};
use abi::schema::keys;
use crate::asset::AssetBank;
use crate::font_graph::{self, FontStyle};

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

impl LayoutTree {
    pub fn find_rect(&self, id: ThingId) -> Option<Rect> {
        let root = self.root.as_ref()?;
        Self::find_rect_in(root, id)
    }

    fn find_rect_in(node: &LayoutNode, id: ThingId) -> Option<Rect> {
        if node.id == id {
            return Some(node.rect);
        }
        for child in &node.children {
            if let Some(rect) = Self::find_rect_in(child, id) {
                return Some(rect);
            }
        }
        None
    }
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

    pub fn solve(&mut self, snapshot: &UiSnapshot, screen_w: i32, screen_h: i32, assets: &AssetBank, symbols: &impl SymbolResolver) -> LayoutTree {
        crate::trace_span!("ui.layout.solve");
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

        {
            crate::trace_span!("ui.layout.tree_flow");
            Self::layout_children(snapshot, root_node, &mut root_layout, assets, symbols, &mut self.measure_cache);
        }

        LayoutTree { root: Some(root_layout) }
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
                let fill_parent = Self::get_prop(child_node, keys::UI_FILL_PARENT, symbols) != 0;
                if fill_parent {
                    w = layout.rect.w;
                    h = layout.rect.h;
                }

                // Check if centering requested
                let center_x = Self::get_prop(child_node, keys::UI_CENTER_X, symbols) != 0;
                let center_y = Self::get_prop(child_node, keys::UI_CENTER_Y, symbols) != 0;

                if (center_x || center_y) && (w == 0 || h == 0) {
                    // Try to measure if it's text
                    if let Some(text) = Self::get_str_prop(child_node, keys::UI_TEXT, symbols) {
                        let font_name = Self::get_str_prop(child_node, keys::UI_FONT_STACK, symbols)
                            .or_else(|| Self::get_str_prop(child_node, keys::UI_FONT, symbols))
                            .unwrap_or_else(|| "Noto Sans".into());
                        let size = Self::get_prop(child_node, keys::UI_FONT_SIZE, symbols) as f32;
                        let font_size = if size == 0.0 { 16.0 } else { size };

                        let (cache_text, is_time) = if Self::is_time_text(&text) {
                            (String::from("##:##:##"), true)
                        } else {
                            (text.clone(), false)
                        };
                        let cache_key = (cache_text, font_name.clone(), font_size as u32);
                        let dims = if let Some(d) = cache.get(&cache_key) {
                            crate::trace_counter!("ui.layout.cache_hits", 1);
                            Some(*d)
                        } else {
                            crate::trace_counter!("ui.layout.cache_misses", 1);
                            let d = if is_time {
                                Self::measure_time_text(&font_name, font_size, assets)
                            } else {
                                Self::measure_text(&text, &font_name, font_size, assets)
                            };
                            if let Some(res) = d {
                                cache.insert(cache_key, res);
                            }
                            d
                        };

                        if let Some(dims) = dims {
                            if w == 0 {
                                w = dims.0 as i32;
                            }
                            if h == 0 {
                                h = dims.1 as i32;
                            }
                        }
                    }
                }

                let mut x = Self::get_prop(child_node, keys::UI_X, symbols) as i32;
                let mut y = Self::get_prop(child_node, keys::UI_Y, symbols) as i32;

                if fill_parent {
                    x = 0;
                    y = 0;
                }

                // Edge-relative positioning (insets from parent edges)
                // Debug: log inset values for debugging
                let inset_right = Self::get_prop(child_node, keys::UI_INSET_RIGHT, symbols) as i32;
                let inset_bottom = Self::get_prop(child_node, keys::UI_INSET_BOTTOM, symbols) as i32;
                if inset_right > 0 {
                    x = layout.rect.w - inset_right - w;
                }
                if inset_bottom > 0 {
                    y = layout.rect.h - inset_bottom - h;
                }

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

                Self::layout_children(snapshot, child_node, &mut child_layout, assets, symbols, cache);
                layout.children.push(child_layout);
            }
        }

        // Sort children by Z-index
        layout.children.sort_by_key(|n| n.z_index);
    }

    fn measure_text(text: &str, font_name: &str, size: f32, assets: &AssetBank) -> Option<(f32, f32)> {
        crate::trace_span!("ui.layout.measure_text");
        let now_ms = crate::log_ratelimit::now_ms();
        let _log_enabled = crate::log_ratelimit::log_every(1000, now_ms);
        
        let mut graph_result = None;

        font_graph::with_graph(|graph| {
            let style = FontStyle::default();
            let stack = graph.resolve_stack(Some(font_name));
            if stack.is_empty() {
                return;
            }

            let primary_face_id = stack
                .iter()
                .find_map(|family| graph.select_face_for_family(*family, style));
            let primary_face_id = match primary_face_id {
                Some(id) => id,
                None => return,
            };
            let primary_font = match graph.font_for_face(primary_face_id) {
                Some(font) => font,
                None => return,
            };
            let line_metrics = primary_font.font.horizontal_line_metrics(size);
            let line_height = line_metrics.map(|m| m.new_line_size).unwrap_or(size * 1.2);

            let mut width: f32 = 0.0;
            let mut max_width: f32 = 0.0;
            let mut lines_accum: f32 = 1.0;

            for ch in text.chars() {
                if ch == '\n' {
                    max_width = max_width.max(width);
                    width = 0.0;
                    lines_accum += 1.0;
                    continue;
                }
                let resolved = graph
                    .resolve_face_for_glyph(&stack, style, ch as u32)
                    .or_else(|| graph.resolved_face_by_id(primary_face_id));
                if let Some(face) = resolved {
                    if let Some(font) = graph.font_for_face(face.face_id) {
                        let metrics = font.font.metrics(ch, size);
                        width += metrics.advance_width;
                    }
                }
            }
            max_width = max_width.max(width);
            graph_result = Some((max_width, lines_accum * line_height));
        });

        if graph_result.is_some() {
            return graph_result;
        }

        // FALLBACK: Use assets directly
        let fonts = assets.get_fonts();
        if fonts.is_empty() {
            return None;
        }

        let font = fonts.iter().find(|f| f.name.contains(font_name))
            .or_else(|| fonts.iter().find(|f| f.name.contains("NotoSans-Regular")))
            .unwrap_or(&fonts[0]);

        let line_metrics = font.font.horizontal_line_metrics(size);
        let line_height = line_metrics.map(|m| m.new_line_size).unwrap_or(size * 1.2);

        let mut width: f32 = 0.0;
        let mut max_width: f32 = 0.0;
        let mut lines_accum: f32 = 1.0;

        for ch in text.chars() {
            if ch == '\n' {
                max_width = max_width.max(width);
                width = 0.0;
                lines_accum += 1.0;
                continue;
            }
            let metrics = font.font.metrics(ch, size);
            width += metrics.advance_width;
        }
        max_width = max_width.max(width);

        if max_width == 0.0 {
            None
        } else {
            Some((max_width, lines_accum * line_height))
        }
    }

    fn measure_time_text(font_name: &str, size: f32, assets: &AssetBank) -> Option<(f32, f32)> {
        let mut result = None;
        font_graph::with_graph(|graph| {
            let style = FontStyle::default();
            let stack = graph.resolve_stack(Some(font_name));
            if stack.is_empty() {
                return;
            }
            let primary_face_id = stack
                .iter()
                .find_map(|family| graph.select_face_for_family(*family, style));
            let primary_face_id = match primary_face_id {
                Some(id) => id,
                None => return,
            };
            let (line_height, colon_width) = {
                let font = match graph.font_for_face(primary_face_id) {
                    Some(font) => font,
                    None => return,
                };
                let line_height = font
                    .font
                    .horizontal_line_metrics(size)
                    .map(|m| m.new_line_size)
                    .unwrap_or(size * 1.2);
                let colon = font.font.metrics(':', size).advance_width;
                (line_height, colon)
            };

            let mut max_digit = 0.0;
            for ch in '0'..='9' {
                let resolved = graph
                    .resolve_face_for_glyph(&stack, style, ch as u32)
                    .or_else(|| graph.resolved_face_by_id(primary_face_id));
                if let Some(face) = resolved {
                    if let Some(font) = graph.font_for_face(face.face_id) {
                        let metrics = font.font.metrics(ch, size);
                        if metrics.advance_width > max_digit {
                            max_digit = metrics.advance_width;
                        }
                    }
                }
            }
            let width = (max_digit * 6.0) + (colon_width * 2.0);
            result = Some((width, line_height));
        });

        if result.is_some() {
            return result;
        }

        // FALLBACK
        let fonts = assets.get_fonts();
        if fonts.is_empty() {
            return None;
        }

        let font = fonts.iter().find(|f| f.name.contains(font_name))
            .or_else(|| fonts.iter().find(|f| f.name.contains("NotoSans-Regular")))
            .unwrap_or(&fonts[0]);

        let line_height = font.font.horizontal_line_metrics(size)
            .map(|m| m.new_line_size)
            .unwrap_or(size * 1.2);
        let colon_width = font.font.metrics(':', size).advance_width;

        let mut max_digit = 0.0;
        for ch in '0'..='9' {
            let metrics = font.font.metrics(ch, size);
            if metrics.advance_width > max_digit {
                max_digit = metrics.advance_width;
            }
        }
        let width = (max_digit * 6.0) + (colon_width * 2.0);
        Some((width, line_height))
    }

    fn is_time_text(text: &str) -> bool {
        let bytes = text.as_bytes();
        if bytes.len() != 8 {
            return false;
        }
        if bytes[2] != b':' || bytes[5] != b':' {
            return false;
        }
        for (idx, b) in bytes.iter().enumerate() {
            if idx == 2 || idx == 5 {
                continue;
            }
            if !b.is_ascii_digit() && *b != b'-' {
                return false;
            }
        }
        true
    }

    fn get_prop(node: &UiNodeSnapshot, key: &str, symbols: &impl SymbolResolver) -> u64 {
        if let Some(id) = symbols.resolve(key) {
            return *node.props.get(&id).unwrap_or(&0);
        }
        0
    }

    fn get_str_prop(node: &UiNodeSnapshot, key: &str, symbols: &impl SymbolResolver) -> Option<String> {
        if let Some(id) = symbols.resolve(key) {
            return node.strings.get(&id).cloned();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::snapshot::{UiNodeSnapshot, UiNodeKind};
    use alloc::collections::BTreeMap;
    use alloc::vec;
    use abi::schema::keys;
    use alloc::string::ToString;

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
        snapshot.nodes.insert(root_id, UiNodeSnapshot {
            id: root_id,
            kind: UiNodeKind::Root,
            props: root_props,
            strings: BTreeMap::new(),
            children: vec![child_id],
        });

        // Child node: 100x50, centered
        let mut child_props = BTreeMap::new();
        child_props.insert(3, 100); // UI_WIDTH
        child_props.insert(4, 50);  // UI_HEIGHT
        child_props.insert(5, 1);   // UI_CENTER_X
        child_props.insert(6, 1);   // UI_CENTER_Y

        snapshot.nodes.insert(child_id, UiNodeSnapshot {
            id: child_id,
            kind: UiNodeKind::Window, // or whatever
            props: child_props,
            strings: BTreeMap::new(),
            children: vec![],
        });

        let assets = AssetBank::new();
        let resolver = MockSymbolResolver::new();

        // Screen 1280x720.
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
