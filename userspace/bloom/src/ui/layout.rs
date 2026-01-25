use alloc::vec::Vec;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use stem::thing::ThingId;
use crate::damage::Rect;
use crate::ui::constants::TITLE_BAR_HEIGHT;
use crate::ui::snapshot::{UiSnapshot, UiNodeSnapshot, UiNodeKind};
use abi::schema::keys;
use crate::asset::AssetBank;
use crate::font_graph::{self, FontStyle};

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: ThingId,
    pub kind: UiNodeKind,
    pub rect: Rect,
    pub z_index: i32,
    pub children: Vec<LayoutNode>,
}

#[derive(Clone)]
pub struct LayoutTree {
    pub root: Option<LayoutNode>,
}

impl LayoutTree {
    pub fn find_rect(&self, id: ThingId) -> Option<Rect> {
        let root = self.root.as_ref()?;
        Self::find_rect_in(root, id)
    }

    pub fn find_node(&self, id: ThingId) -> Option<&LayoutNode> {
        let root = self.root.as_ref()?;
        Self::find_node_in(root, id)
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

    fn find_node_in(node: &LayoutNode, id: ThingId) -> Option<&LayoutNode> {
        if node.id == id {
            return Some(node);
        }
        for child in &node.children {
            if let Some(found) = Self::find_node_in(child, id) {
                return Some(found);
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
            kind: root_node.kind,
            rect: Rect::new(0, 0, screen_w, screen_h),
            z_index: 0,
            children: Vec::new(),
        };

        {
            crate::trace_span!("ui.layout.tree_flow");
            Self::layout_children(
                snapshot,
                root_node,
                &mut root_layout,
                assets, 
                symbols,
                &mut self.measure_cache,
                None, // prev_layout
                &BTreeSet::new(), // dirty
                &BTreeSet::new(), // subtree
            );
        }

        LayoutTree { root: Some(root_layout) }
    }

    pub fn solve_partial(
        &mut self,
        snapshot: &UiSnapshot,
        screen_w: i32,
        screen_h: i32,
        assets: &AssetBank,
        symbols: &impl SymbolResolver,
        prev_layout: Option<&LayoutTree>,
        dirty_windows: &BTreeSet<ThingId>,
        layout_dirty: &BTreeSet<ThingId>,
        subtree_dirty: &BTreeSet<ThingId>,
    ) -> LayoutTree {
        crate::trace_span!("ui.layout.solve");
        let root_id = match snapshot.root_id {
            Some(id) => id,
            None => return LayoutTree { root: None },
        };

        let root_node = match snapshot.nodes.get(&root_id) {
            Some(n) => n,
            None => return LayoutTree { root: None },
        };

        if dirty_windows.is_empty() {
            if let Some(prev) = prev_layout {
                if let Some(prev_root) = prev.root.as_ref() {
                    if prev_root.rect.w == screen_w && prev_root.rect.h == screen_h {
                        crate::trace_event!("ui.layout.path", "reuse_full");
                        return prev.clone();
                    }
                }
            }
        }

        let mut root_layout = LayoutNode {
            id: root_id,
            kind: root_node.kind,
            rect: Rect::new(0, 0, screen_w, screen_h),
            z_index: 0,
            children: Vec::new(),
        };

        let prev_root = prev_layout.and_then(|t| t.root.as_ref());

        {
            crate::trace_span!("ui.layout.tree_flow");
            // For the root's direct children (windows/panels), we can rely on dirty_windows set
            // OR use the generic recursion if consistent. 
            // The existing optimization for dirty_windows is valid but let's unify it 
            // or keep it but use the new recursion for the "dirty" windows.
            // Actually, let's just delegate to layout_children which handles all recursion.
            Self::layout_children(
                snapshot, 
                root_node, 
                &mut root_layout, 
                assets, 
                symbols, 
                &mut self.measure_cache,
                prev_root,
                layout_dirty,
                subtree_dirty
            );
        }

        LayoutTree { root: Some(root_layout) }
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_children(
        snapshot: &UiSnapshot, 
        node: &UiNodeSnapshot, 
        layout: &mut LayoutNode, 
        assets: &AssetBank, 
        symbols: &impl SymbolResolver,
        cache: &mut BTreeMap<(String, String, u32), (f32, f32)>,
        prev_node: Option<&LayoutNode>,
        layout_dirty: &BTreeSet<ThingId>,
        subtree_dirty: &BTreeSet<ThingId>,
    ) {
        let parent_is_window = node.kind == UiNodeKind::Window;
        let parent_shaded = parent_is_window && Self::get_prop(node, keys::UI_WINDOW_SHADED, symbols) != 0;
        let parent_scroll = if node.kind == UiNodeKind::Viewport {
            (
                Self::get_prop(node, keys::UI_SCROLL_X, symbols) as i32,
                Self::get_prop(node, keys::UI_SCROLL_Y, symbols) as i32,
            )
        } else {
            (0, 0)
        };

        if parent_shaded {
            return;
        }

        // Map previous children by ID for O(1) matching
        let prev_children_map: BTreeMap<ThingId, &LayoutNode> = prev_node
            .map(|pn| pn.children.iter().map(|c| (c.id, c)).collect())
            .unwrap_or_default();

        for child_id in &node.children {
            if let Some(child_node) = snapshot.nodes.get(child_id) {
                let prev_child = prev_children_map.get(child_id).copied();

                let child_layout = Self::layout_child(
                    snapshot,
                    child_node,
                    *child_id,
                    layout,
                    assets,
                    symbols,
                    cache,
                    parent_scroll,
                    prev_child,
                    layout_dirty,
                    subtree_dirty,
                );
                layout.children.push(child_layout);
            }
        }

        // Sort children by Z-index
        layout.children.sort_by_key(|n| n.z_index);
    }

    #[allow(clippy::too_many_arguments)]
    fn layout_child(
        snapshot: &UiSnapshot,
        child_node: &UiNodeSnapshot,
        child_id: ThingId,
        layout: &LayoutNode,
        assets: &AssetBank,
        symbols: &impl SymbolResolver,
        cache: &mut BTreeMap<(String, String, u32), (f32, f32)>,
        parent_scroll: (i32, i32),
        prev_child: Option<&LayoutNode>,
        layout_dirty: &BTreeSet<ThingId>,
        subtree_dirty: &BTreeSet<ThingId>,
    ) -> LayoutNode {
        // Reuse Check:
        // 1. Child is not dirty itself
        // 2. Child subtree is not dirty
        // 3. We have a previous layout for this child
        // 4. Parent constraint invariant: The "content box" for this child is essentially same size
        //    (For simplicity here, we check if parent size match. More granular check possible but this covers 90% cases)
        // Note: We need to be careful. If the parent size changed, the child's relative positioning might change.
        // We calculate new position/size. If they match the old position/size (relative to old parent), we can reuse subtree.
        // Wait, we can't know new pos/size without running the layout logic below.
        // So we run the calc below (cheap math), and THEN decide whether to recurse or reuse.
        
        let parent_is_window = layout.kind == UiNodeKind::Window;
        let parent_is_viewport = layout.kind == UiNodeKind::Viewport;
        let title_bar_h = TITLE_BAR_HEIGHT;

        let is_window = child_node.kind == UiNodeKind::Window;
        let child_shaded =
            is_window && Self::get_prop(child_node, keys::UI_WINDOW_SHADED, symbols) != 0;

        let mut w = Self::get_prop(child_node, keys::UI_WIDTH, symbols) as i32;
        let mut h = Self::get_prop(child_node, keys::UI_HEIGHT, symbols) as i32;
        let fill_parent = Self::get_prop(child_node, keys::UI_FILL_PARENT, symbols) != 0;
        if fill_parent {
            w = layout.rect.w;
            h = layout.rect.h;
        }

        if is_window {
            if child_shaded {
                h = title_bar_h;
            } else if h > 0 {
                h = h.saturating_add(title_bar_h);
            } else {
                h = title_bar_h;
            }
        }

        let center_x = Self::get_prop(child_node, keys::UI_CENTER_X, symbols) != 0;
        let center_y = Self::get_prop(child_node, keys::UI_CENTER_Y, symbols) != 0;

        // Measure text block when dimensions not explicitly set
        if w == 0 || h == 0 {
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

        if parent_is_window {
            y = y.saturating_add(title_bar_h);
            if fill_parent {
                h = (layout.rect.h - title_bar_h).max(0);
            }
        }

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
        if parent_is_viewport {
            x = x.saturating_sub(parent_scroll.0);
            y = y.saturating_sub(parent_scroll.1);
        }

        let z = Self::get_prop(child_node, keys::UI_Z_INDEX, symbols) as i32;
        let absolute_x = layout.rect.x + x;
        let absolute_y = layout.rect.y + y;
        let absolute_rect = Rect::new(absolute_x, absolute_y, w, h);

        let mut child_layout = LayoutNode {
            id: child_id,
            kind: child_node.kind,
            rect: absolute_rect,
            z_index: z,
            children: Vec::new(),
        };

        // REUSE LOGIC
        // If clean and size matches, reuse subtree!
        let is_clean = !layout_dirty.contains(&child_id) && !subtree_dirty.contains(&child_id);
        
        if is_clean {
            if let Some(prev) = prev_child {
                // Determine if we can reuse the previous subtree.
                // We must ensure the *inputs* to the subtree layout are invariant.
                // The inputs are: child props (invariant since !dirty) and child size (w, h).
                // If w and h calculated above match prev.rect.w and prev.rect.h, 
                // then the internal layout of the child should be identical.
                if prev.rect.w == w && prev.rect.h == h {
                     // Reuse!
                     crate::trace_counter!("ui.layout.subtree_reuse", 1);
                     child_layout.children = prev.children.clone();
                     
                     // If position changed, we must translate all descendants
                     if prev.rect.x != absolute_x || prev.rect.y != absolute_y {
                         let dx = absolute_x - prev.rect.x;
                         let dy = absolute_y - prev.rect.y;
                         Self::translate_subtree(&mut child_layout.children, dx, dy);
                     }
                     
                     return child_layout;
                }
            }
        }

        Self::layout_children(
            snapshot, 
            child_node, 
            &mut child_layout, 
            assets, 
            symbols, 
            cache,
            prev_child,
            layout_dirty,
            subtree_dirty
        );

        child_layout
    }

    fn translate_subtree(nodes: &mut [LayoutNode], dx: i32, dy: i32) {
        for node in nodes.iter_mut() {
            node.rect.x += dx;
            node.rect.y += dy;
            if !node.children.is_empty() {
                Self::translate_subtree(&mut node.children, dx, dy);
            }
        }
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
            svg_content: None,
            window_icon_content: None,
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
            svg_content: None,
            window_icon_content: None,
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
        assert_eq!(child.rect.y, 255);
        assert_eq!(child.rect.w, 100);
        assert_eq!(child.rect.h, 90);
    }
}
