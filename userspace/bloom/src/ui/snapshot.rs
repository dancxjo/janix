use abi::ids::HandleId;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::{vec, vec::Vec};
use stem::thing::sys::get_kind;
use stem::thing::ThingId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiNodeKind {
    Unknown,
    Root,
    Window,
    Panel,
    Text,
    TextRun,
    Image,
    Overlay,
    Inline,
    Viewport,
    Tile,
    Chrome,
}

impl UiNodeKind {
    pub fn from_symbol(id: u32, kinds: &KindIds) -> Self {
        if id == 0 {
            return Self::Unknown;
        }
        if id == kinds.root {
            Self::Root
        } else if id == kinds.window {
            Self::Window
        } else if id == kinds.panel {
            Self::Panel
        } else if id == kinds.text {
            Self::Text
        } else if id == kinds.text_run {
            Self::TextRun
        } else if id == kinds.image {
            Self::Image
        } else if id == kinds.overlay {
            Self::Overlay
        } else if id == kinds.inline {
            Self::Inline
        } else if id == kinds.viewport {
            Self::Viewport
        } else if id == kinds.tile {
            Self::Tile
        } else if id == kinds.chrome {
            Self::Chrome
        } else {
            Self::Unknown
        }
    }
}

#[derive(Clone)]
pub struct KindIds {
    pub root: u32,
    pub window: u32,
    pub panel: u32,
    pub text: u32,
    pub text_run: u32,
    pub image: u32,
    pub overlay: u32,
    pub inline: u32,
    pub viewport: u32,
    pub tile: u32,
    pub chrome: u32,
}

impl KindIds {
    pub fn empty() -> Self {
        Self {
            root: 0,
            window: 0,
            panel: 0,
            text: 0,
            text_run: 0,
            image: 0,
            overlay: 0,
            inline: 0,
            viewport: 0,
            tile: 0,
            chrome: 0,
        }
    }
    pub fn intern() -> Self {
        use abi::schema::kinds;
        crate::trace_span!("ui.init.intern_kinds");
        let kids = Self {
            root: stem::thing::sys::intern(kinds::UI_ROOT).unwrap_or(0),
            window: stem::thing::sys::intern(kinds::UI_WINDOW).unwrap_or(0),
            panel: stem::thing::sys::intern(kinds::UI_PANEL).unwrap_or(0),
            text: stem::thing::sys::intern(kinds::UI_TEXT).unwrap_or(0),
            text_run: stem::thing::sys::intern(kinds::UI_TEXT_RUN).unwrap_or(0),
            image: stem::thing::sys::intern(kinds::UI_IMAGE).unwrap_or(0),
            overlay: stem::thing::sys::intern(kinds::UI_OVERLAY).unwrap_or(0),
            inline: stem::thing::sys::intern(kinds::UI_INLINE).unwrap_or(0),
            viewport: stem::thing::sys::intern(kinds::UI_VIEWPORT).unwrap_or(0),
            tile: stem::thing::sys::intern(kinds::UI_TILE).unwrap_or(0),
            chrome: stem::thing::sys::intern(kinds::UI_CHROME).unwrap_or(0),
        };
        crate::trace_counter!("ui.init.syscalls.intern_kinds", 11);
        kids
    }
}

#[derive(Clone)]
pub struct UiKeys {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub color: u32,
    pub text: u32,
    pub font: u32,
    pub font_size: u32,
    pub font_stack: u32,
    pub font_debug: u32,
    pub radius: u32,
    pub title: u32,
    pub hidden: u32,
    pub z_index: u32,
    pub center_x: u32,
    pub center_y: u32,
    pub fill_parent: u32,
    pub bg_color: u32,
    pub fg_color: u32,
    pub has_child: u32,
    pub inset_right: u32,
    pub inset_bottom: u32,
    pub inline_mode: u32,
    pub svg_bytes: u32,
    pub window_icon: u32,
    pub window_shaded: u32,
    pub scroll_x: u32,
    pub scroll_y: u32,
    pub clip: u32,
    pub tile_asset: u32,
    pub tile_state: u32,
    pub drawlist_bytespace: u32,
    pub drawlist_gen: u32,
}

impl UiKeys {
    pub fn empty() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            color: 0,
            text: 0,
            font: 0,
            font_size: 0,
            font_stack: 0,
            font_debug: 0,
            radius: 0,
            title: 0,
            hidden: 0,
            z_index: 0,
            center_x: 0,
            center_y: 0,
            fill_parent: 0,
            bg_color: 0,
            fg_color: 0,
            has_child: 0,
            inset_right: 0,
            inset_bottom: 0,
            inline_mode: 0,
            svg_bytes: 0,
            window_icon: 0,
            window_shaded: 0,
            scroll_x: 0,
            scroll_y: 0,
            clip: 0,
            tile_asset: 0,
            tile_state: 0,
            drawlist_bytespace: 0,
            drawlist_gen: 0,
        }
    }
    pub fn intern() -> Self {
        crate::trace_span!("ui.init.intern_keys");
        use abi::schema::{keys, rels};
        let k = Self {
            x: stem::thing::sys::intern(keys::UI_X).unwrap_or(0),
            y: stem::thing::sys::intern(keys::UI_Y).unwrap_or(0),
            w: stem::thing::sys::intern(keys::UI_WIDTH).unwrap_or(0),
            h: stem::thing::sys::intern(keys::UI_HEIGHT).unwrap_or(0),
            color: stem::thing::sys::intern(keys::UI_COLOR).unwrap_or(0),
            text: stem::thing::sys::intern(keys::UI_TEXT).unwrap_or(0),
            font: stem::thing::sys::intern(keys::UI_FONT).unwrap_or(0),
            font_size: stem::thing::sys::intern(keys::UI_FONT_SIZE).unwrap_or(0),
            font_stack: stem::thing::sys::intern(keys::UI_FONT_STACK).unwrap_or(0),
            font_debug: stem::thing::sys::intern(keys::UI_FONT_DEBUG).unwrap_or(0),
            radius: stem::thing::sys::intern(keys::UI_RADIUS).unwrap_or(0),
            title: stem::thing::sys::intern(keys::UI_TITLE).unwrap_or(0),
            hidden: stem::thing::sys::intern(keys::UI_HIDDEN).unwrap_or(0),
            z_index: stem::thing::sys::intern(keys::UI_Z_INDEX).unwrap_or(0),
            center_x: stem::thing::sys::intern(keys::UI_CENTER_X).unwrap_or(0),
            center_y: stem::thing::sys::intern(keys::UI_CENTER_Y).unwrap_or(0),
            fill_parent: stem::thing::sys::intern(keys::UI_FILL_PARENT).unwrap_or(0),
            bg_color: stem::thing::sys::intern(keys::UI_BG_COLOR).unwrap_or(0),
            fg_color: stem::thing::sys::intern(keys::UI_FG_COLOR).unwrap_or(0),
            has_child: stem::thing::sys::intern(rels::HAS_CHILD).unwrap_or(0),
            inset_right: stem::thing::sys::intern(keys::UI_INSET_RIGHT).unwrap_or(0),
            inset_bottom: stem::thing::sys::intern(keys::UI_INSET_BOTTOM).unwrap_or(0),
            inline_mode: stem::thing::sys::intern(keys::UI_INLINE_MODE).unwrap_or(0),
            svg_bytes: stem::thing::sys::intern(keys::UI_SVG_BYTES).unwrap_or(0),
            window_icon: stem::thing::sys::intern(keys::UI_WINDOW_ICON).unwrap_or(0),
            window_shaded: stem::thing::sys::intern(keys::UI_WINDOW_SHADED).unwrap_or(0),
            scroll_x: stem::thing::sys::intern(keys::UI_SCROLL_X).unwrap_or(0),
            scroll_y: stem::thing::sys::intern(keys::UI_SCROLL_Y).unwrap_or(0),
            clip: stem::thing::sys::intern(keys::UI_CLIP).unwrap_or(0),
            tile_asset: stem::thing::sys::intern(keys::UI_TILE_ASSET).unwrap_or(0),
            tile_state: stem::thing::sys::intern(keys::UI_TILE_STATE).unwrap_or(0),
            drawlist_bytespace: stem::thing::sys::intern(keys::UI_DRAWLIST_BYTESPACE).unwrap_or(0),
            drawlist_gen: stem::thing::sys::intern(keys::UI_DRAWLIST_GEN).unwrap_or(0),
        };
        crate::trace_counter!("ui.init.syscalls.intern_keys", 33);
        k
    }
    pub fn numeric_keys(&self) -> [u32; 28] {
        [
            self.x,
            self.y,
            self.w,
            self.h,
            self.color,
            self.radius,
            self.hidden,
            self.z_index,
            self.center_x,
            self.center_y,
            self.fill_parent,
            self.bg_color,
            self.fg_color,
            self.inset_right,
            self.inset_bottom,
            self.font_size,
            self.font_debug,
            self.inline_mode,
            self.svg_bytes,
            self.window_icon,
            self.window_shaded,
            self.scroll_x,
            self.scroll_y,
            self.clip,
            self.tile_asset,
            self.tile_state,
            self.drawlist_bytespace,
            self.drawlist_gen,
        ]
    }
    pub fn string_keys(&self) -> [u32; 4] {
        [self.text, self.font, self.font_stack, self.title]
    }
    pub fn all_keys(&self) -> [u32; 32] {
        [
            self.x,
            self.y,
            self.w,
            self.h,
            self.color,
            self.radius,
            self.hidden,
            self.z_index,
            self.center_x,
            self.center_y,
            self.fill_parent,
            self.bg_color,
            self.fg_color,
            self.inset_right,
            self.inset_bottom,
            self.font_size,
            self.font_debug,
            self.inline_mode,
            self.svg_bytes,
            self.window_icon,
            self.window_shaded,
            self.scroll_x,
            self.scroll_y,
            self.clip,
            self.tile_asset,
            self.tile_state,
            self.text,
            self.font,
            self.font_stack,
            self.title,
            self.drawlist_bytespace,
            self.drawlist_gen,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct UiNodeSnapshot {
    pub id: ThingId,
    pub kind: UiNodeKind,
    pub props: BTreeMap<u32, u64>,
    pub strings: BTreeMap<u32, String>,
    pub children: Vec<ThingId>,
    pub svg_content: Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
    pub svg_source: Option<ThingId>,
    pub svg_hash: Option<u64>,
    pub window_icon_content: Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
    pub window_icon_source: Option<ThingId>,
    pub window_icon_hash: Option<u64>,
    pub drawlist_content: Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
    pub drawlist_source: Option<ThingId>,
    pub drawlist_hash: Option<u64>,
}

/// Cached asset entry: bytespace_id -> decoded content
#[derive(Clone, Default)]
pub struct AssetCache {
    strings: BTreeMap<u64, String>,
    svgs: BTreeMap<u64, SvgAsset>,
    drawlists: BTreeMap<u64, DrawListAsset>,
}

#[derive(Clone)]
pub struct DrawListAsset {
    pub cmds: alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>,
    pub hash: u64,
}

#[derive(Clone)]
pub struct SvgAsset {
    pub cmds: alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>,
    pub xml_hash: u64,
}

impl AssetCache {
    pub fn new() -> Self {
        Self {
            strings: BTreeMap::new(),
            svgs: BTreeMap::new(),
            drawlists: BTreeMap::new(),
        }
    }

    /// Get cached string or read from bytespace and cache it
    pub fn get_or_read_string(&mut self, bs_id: ThingId) -> Option<String> {
        let id = bs_id.to_u64_lossy();
        if let Some(s) = self.strings.get(&id) {
            crate::trace_counter!("ui.snap.string_cache_hit", 1);
            return Some(s.clone());
        }
        crate::trace_counter!("ui.snap.string_cache_miss", 1);
        let s = Self::read_string_raw(bs_id)?;
        self.strings.insert(id, s.clone());
        Some(s)
    }

    pub fn get_or_parse_svg(&mut self, bs_id: ThingId) -> Option<SvgAsset> {
        let id = bs_id.to_u64_lossy();
        if let Some(cmds) = self.svgs.get(&id) {
            return Some(cmds.clone());
        }

        // Parse
        let xml = Self::read_string_raw(bs_id)?;
        let xml_hash = hash_bytes(xml.as_bytes());
        let mut parser = crate::svg::SvgParser::new();
        // SVG icons are usually small, parsing is fast enough to do on-demand if cached.
        let cmds = parser.parse(&xml);
        let arc_cmds = alloc::sync::Arc::new(cmds);
        let asset = SvgAsset {
            cmds: arc_cmds,
            xml_hash,
        };

        self.svgs.insert(id, asset.clone());
        Some(asset)
    }

    pub fn get_or_parse_drawlist(&mut self, bs_id: ThingId, gen: u64) -> Option<DrawListAsset> {
        let id = bs_id.to_u64_lossy();
        if let Some(asset) = self.drawlists.get(&id) {
            if asset.hash == gen {
                return Some(asset.clone());
            }
        }

        // Read and decode
        let data = Self::read_bytes_raw(bs_id)?;
        let cmds = crate::drawlist::decode_native_drawlist(&data);
        let asset = DrawListAsset {
            cmds: alloc::sync::Arc::new(cmds),
            hash: gen,
        };

        self.drawlists.insert(id, asset.clone());
        Some(asset)
    }

    fn read_bytes_raw(bs_id: ThingId) -> Option<Vec<u8>> {
        use stem::thing::sys::{bytespace_info, bytespace_read};
        let size = bytespace_info(bs_id).ok()?;
        let mut buf = vec![0u8; size];
        bytespace_read(bs_id, 0, &mut buf).ok()?;
        Some(buf)
    }

    fn read_string_raw(bs_id: ThingId) -> Option<String> {
        use stem::thing::sys::{bytespace_info, bytespace_read};
        crate::trace_counter!("snap.syscalls.read_string", 2);
        let size = bytespace_info(bs_id).ok()?;
        if size == 0 {
            return Some(String::new());
        }
        let mut buf = alloc::vec![0u8; size];
        let len = bytespace_read(bs_id, 0, &mut buf).ok()?;
        Some(String::from(
            core::str::from_utf8(&buf[..len]).unwrap_or(""),
        ))
    }

    /// Invalidate a specific bytespace entry (when we know it changed)
    pub fn invalidate(&mut self, bs_id: u64) {
        self.strings.remove(&bs_id);
        self.svgs.remove(&bs_id);
    }

    /// Clear entire cache (e.g., on watch notification that strings changed)
    pub fn clear(&mut self) {
        self.strings.clear();
        self.svgs.clear();
    }

    pub fn len(&self) -> usize {
        self.strings.len() + self.svgs.len()
    }
}

fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[derive(Clone)]
pub struct UiSnapshot {
    pub root_id: Option<ThingId>,
    pub nodes: BTreeMap<ThingId, UiNodeSnapshot>,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeChange {
    pub id: ThingId,
    pub layout_dirty: bool,
    pub measure_dirty: bool,
    pub paint_dirty: bool,
}

impl NodeChange {
    fn new(id: ThingId) -> Self {
        Self {
            id,
            layout_dirty: false,
            measure_dirty: false,
            paint_dirty: false,
        }
    }

    fn merge(&mut self, other: NodeChange) {
        self.layout_dirty |= other.layout_dirty;
        self.measure_dirty |= other.measure_dirty;
        self.paint_dirty |= other.paint_dirty;
    }
}

impl UiSnapshot {
    pub fn new() -> Self {
        Self {
            root_id: None,
            nodes: BTreeMap::new(),
        }
    }

    /// Full capture from scratch
    pub fn capture(root_id: ThingId, keys: &UiKeys, kinds: &KindIds) -> Self {
        let mut cache = AssetCache::new();
        Self::capture_with_cache(root_id, keys, kinds, &mut cache)
    }

    /// Capture with asset cache (Phase C optimization)
    pub fn capture_with_cache(
        root_id: ThingId,
        keys: &UiKeys,
        kinds: &KindIds,
        cache: &mut AssetCache,
    ) -> Self {
        let mut snapshot = Self::new();
        snapshot.root_id = Some(root_id);
        {
            crate::trace_span!("ui.snap.traverse_all");
            snapshot.traverse(root_id, kinds, keys, cache);
        }
        crate::trace_counter!("ui.snap.nodes_total", snapshot.nodes.len());
        crate::trace_counter!("ui.snap.cache_size", cache.len());
        snapshot
    }

    /// Garbage collect unreachable nodes from the snapshot.
    /// This is critical for preventing memory leaks in the incremental update engine
    /// as nodes are removed from the graph but remain in `self.nodes`.
    pub fn prune(&mut self) {
        let root = match self.root_id {
            Some(id) => id,
            None => {
                self.nodes.clear();
                return;
            }
        };

        crate::trace_span!("ui.snap.prune");
        // Mark
        let mut reachable = BTreeSet::new();
        let mut stack = Vec::new();
        stack.push(root);

        while let Some(current) = stack.pop() {
            if reachable.contains(&current) {
                continue;
            }
            reachable.insert(current);

            if let Some(node) = self.nodes.get(&current) {
                for child in &node.children {
                    stack.push(*child);
                }
            }
        }

        // Sweep
        let before_count = self.nodes.len();
        self.nodes.retain(|id, _| reachable.contains(id));
        let removed = before_count.saturating_sub(self.nodes.len());

        if removed > 0 {
            crate::log!(
                "[bloom][snap] Pruned {} unreachable nodes ({} -> {})",
                removed,
                before_count,
                self.nodes.len()
            );
        }
        crate::trace_counter!("ui.snap.pruned_nodes", removed);
    }

    /// Incremental update: refresh only dirty nodes (props/edges).
    pub fn update_dirty(
        &mut self,
        dirty: &super::DirtySet,
        keys: &UiKeys,
        kinds: &KindIds,
        cache: &mut AssetCache,
    ) -> Vec<NodeChange> {
        crate::trace_span!("snap.update_dirty");
        if dirty.is_empty() {
            return Vec::new();
        }

        let mut changes: BTreeMap<ThingId, NodeChange> = BTreeMap::new();

        for &id in dirty.props() {
            if let Some(change) = self.refresh_node_props(id, keys, kinds, cache) {
                changes
                    .entry(id)
                    .and_modify(|existing| existing.merge(change))
                    .or_insert(change);
            }
        }

        for &id in dirty.edges() {
            self.refresh_node_edges(id, keys, kinds, cache, &mut changes);
        }

        changes.into_values().collect()
    }

    fn traverse(&mut self, id: ThingId, kind_ids: &KindIds, keys: &UiKeys, cache: &mut AssetCache) {
        if self.nodes.contains_key(&id) {
            return;
        }

        let kind_sym = {
            crate::trace_span!("ui.snap.get_kind");
            crate::trace_counter!("snap.syscalls.get_kind", 1);
            get_kind(id).ok()
        };
        let kind = match kind_sym {
            Some(sym) => {
                let k = UiNodeKind::from_symbol(sym.0 as u32, kind_ids);
                if matches!(k, UiNodeKind::Text | UiNodeKind::TextRun) {
                    crate::trace_counter!("ui.snap.text_nodes", 1);
                }
                k
            }
            None => return,
        };

        let children = self.fetch_children(id, keys);
        let (
            props,
            strings,
            svg_content,
            svg_source,
            svg_hash,
            window_icon_content,
            window_icon_source,
            window_icon_hash,
            drawlist_content,
            drawlist_source,
            drawlist_hash,
        ) = self.fetch_properties(id, keys, cache);
        self.nodes.insert(
            id,
            UiNodeSnapshot {
                id,
                kind,
                props,
                strings,
                children: children.clone(),
                svg_content,
                svg_source,
                svg_hash,
                window_icon_content,
                window_icon_source,
                window_icon_hash,
                drawlist_content,
                drawlist_source,
                drawlist_hash,
            },
        );
        for child in children {
            self.traverse(child, kind_ids, keys, cache);
        }
    }

    /// Traverse a single node without recursing (for incremental updates)
    fn traverse_single(
        &mut self,
        id: ThingId,
        kind_ids: &KindIds,
        keys: &UiKeys,
        cache: &mut AssetCache,
    ) {
        let kind_sym = {
            crate::trace_counter!("snap.syscalls.get_kind", 1);
            get_kind(id).ok()
        };
        let kind = match kind_sym {
            Some(sym) => UiNodeKind::from_symbol(sym.0 as u32, kind_ids),
            None => return,
        };

        let children = self.fetch_children(id, keys);
        let (
            props,
            strings,
            svg_content,
            svg_source,
            svg_hash,
            window_icon_content,
            window_icon_source,
            window_icon_hash,
            drawlist_content,
            drawlist_source,
            drawlist_hash,
        ) = self.fetch_properties(id, keys, cache);
        self.nodes.insert(
            id,
            UiNodeSnapshot {
                id,
                kind,
                props,
                strings,
                children,
                svg_content,
                svg_source,
                svg_hash,
                window_icon_content,
                window_icon_source,
                window_icon_hash,
                drawlist_content,
                drawlist_source,
                drawlist_hash,
            },
        );
    }

    fn refresh_node_props(
        &mut self,
        id: ThingId,
        keys: &UiKeys,
        kinds: &KindIds,
        cache: &mut AssetCache,
    ) -> Option<NodeChange> {
        let (
            props,
            strings,
            svg_content,
            svg_source,
            svg_hash,
            window_icon_content,
            window_icon_source,
            window_icon_hash,
            drawlist_content,
            drawlist_source,
            drawlist_hash,
        ) = self.fetch_properties(id, keys, cache);

        if let Some(node) = self.nodes.get_mut(&id) {
            let mut change = NodeChange::new(id);
            let mut any_changed = false;

            if node.props != props {
                any_changed = true;
                if Self::layout_keys_changed(&node.props, &props, keys) {
                    change.layout_dirty = true;
                }
                if Self::measure_keys_changed(&node.props, &props, &node.strings, &strings, keys) {
                    change.measure_dirty = true;
                }
                node.props = props;
            }
            if node.strings != strings {
                if !any_changed {
                    if Self::measure_keys_changed(
                        &node.props,
                        &node.props,
                        &node.strings,
                        &strings,
                        keys,
                    ) {
                        change.measure_dirty = true;
                    }
                    any_changed = true;
                }
                node.strings = strings;
            }
            if node.svg_hash != svg_hash {
                any_changed = true;
                node.svg_content = svg_content;
                node.svg_source = svg_source;
                node.svg_hash = svg_hash;
            }
            if node.window_icon_hash != window_icon_hash {
                any_changed = true;
                node.window_icon_content = window_icon_content;
                node.window_icon_source = window_icon_source;
                node.window_icon_hash = window_icon_hash;
            }
            if node.drawlist_hash != drawlist_hash {
                any_changed = true;
                node.drawlist_content = drawlist_content;
                node.drawlist_source = drawlist_source;
                node.drawlist_hash = drawlist_hash;
            }

            if any_changed {
                change.paint_dirty = true;
                return Some(change);
            }
            None
        } else {
            self.traverse_single(id, kinds, keys, cache);
            let mut change = NodeChange::new(id);
            change.layout_dirty = true;
            change.measure_dirty = true;
            change.paint_dirty = true;
            Some(change)
        }
    }

    fn refresh_node_edges(
        &mut self,
        id: ThingId,
        keys: &UiKeys,
        kinds: &KindIds,
        cache: &mut AssetCache,
        changes: &mut BTreeMap<ThingId, NodeChange>,
    ) {
        if !self.nodes.contains_key(&id) {
            self.traverse_single(id, kinds, keys, cache);
        }

        let children = self.fetch_children(id, keys);
        if let Some(node) = self.nodes.get_mut(&id) {
            if node.children != children {
                let mut change = NodeChange::new(id);
                change.layout_dirty = true;
                change.paint_dirty = true;
                changes
                    .entry(id)
                    .and_modify(|existing| existing.merge(change))
                    .or_insert(change);
            }
            node.children = children.clone();
        }

        for child in children {
            if !self.nodes.contains_key(&child) {
                self.traverse(child, kinds, keys, cache);
                let mut change = NodeChange::new(child);
                change.layout_dirty = true;
                change.measure_dirty = true;
                change.paint_dirty = true;
                changes.insert(child, change);
            }
        }
    }

    fn fetch_children(&self, id: ThingId, keys: &UiKeys) -> Vec<ThingId> {
        let mut children = Vec::new();
        let mut edges_buf = [abi::types::Edge::default(); 64];
        {
            crate::trace_span!("snap.refresh_node_edges");
            crate::trace_counter!("snap.syscalls.get_edges", 1);
            if let Ok(count) = stem::thing::sys::get_edges(id, &mut edges_buf) {
                for edge in &edges_buf[..count] {
                    let rel_u64 = edge.predicate.to_u64_lossy();
                    let target_u64 = edge.to.to_u64_lossy();
                    if rel_u64 == keys.has_child as u64 && target_u64 != id.to_u64_lossy() {
                        children.push(edge.to);
                    }
                }
            }
        }
        children
    }

    fn fetch_properties(
        &self,
        id: ThingId,
        keys: &UiKeys,
        cache: &mut AssetCache,
    ) -> (
        BTreeMap<u32, u64>,
        BTreeMap<u32, String>,
        Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
        Option<ThingId>,
        Option<u64>,
        Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
        Option<ThingId>,
        Option<u64>,
        Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
        Option<ThingId>,
        Option<u64>,
    ) {
        crate::trace_span!("snap.refresh_node_props");
        let mut props = BTreeMap::new();
        let mut strings = BTreeMap::new();
        let mut svg_content = None;
        let mut svg_source = None;
        let mut svg_hash = None;
        let mut window_icon_content = None;
        let mut window_icon_source = None;
        let mut window_icon_hash = None;
        let mut drawlist_content = None;
        let mut drawlist_source = None;
        let mut drawlist_hash = None;

        let all_keys = keys.all_keys();
        let valid_keys: Vec<u32> = all_keys.iter().copied().filter(|&k| k != 0).collect();

        let bulk_result = {
            crate::trace_span!("ui.snap.prop_get");
            crate::trace_counter!("snap.syscalls.prop_get", 1);
            crate::trace_counter!("ui.snap.prop_get.calls", 1);
            stem::thing::sys::props_get_many(id, &valid_keys)
        };

        match bulk_result {
            Ok(response) => {
                let string_keys = keys.string_keys();
                for (i, &key) in valid_keys.iter().enumerate() {
                    if response.present_mask & (1 << i) != 0 {
                        let val = response.values[i];
                        if string_keys.contains(&key) {
                            if val != 0 {
                                crate::trace_span!("ui.snap.read_string");
                                if let Some(s) = cache.get_or_read_string(ThingId::from_u64(val)) {
                                    strings.insert(key, s);
                                }
                            }
                        } else if key == keys.svg_bytes {
                            if val != 0 {
                                props.insert(key, val); // Keep raw prop too
                                let bs_id = ThingId::from_u64(val);
                                if let Some(asset) = cache.get_or_parse_svg(bs_id) {
                                    svg_content = Some(asset.cmds);
                                    svg_source = Some(bs_id);
                                    svg_hash = Some(asset.xml_hash);
                                }
                            }
                        } else if key == keys.window_icon {
                            if val != 0 {
                                props.insert(key, val);
                                let bs_id = ThingId::from_u64(val);
                                if let Some(asset) = cache.get_or_parse_svg(bs_id) {
                                    window_icon_content = Some(asset.cmds);
                                    window_icon_source = Some(bs_id);
                                    window_icon_hash = Some(asset.xml_hash);
                                }
                            }
                        } else if key == keys.tile_asset {
                            if val != 0 {
                                props.insert(key, val);
                                let bs_id = ThingId::from_u64(val);
                                if let Some(asset) = cache.get_or_parse_svg(bs_id) {
                                    svg_content = Some(asset.cmds);
                                    svg_source = Some(bs_id);
                                    svg_hash = Some(asset.xml_hash);
                                }
                            }
                        } else if key == keys.drawlist_bytespace {
                            if val != 0 {
                                props.insert(key, val);
                                let bs_id = ThingId::from_u64(val);
                                let gen = bulk_result
                                    .as_ref()
                                    .ok()
                                    .and_then(|r| {
                                        let idx = valid_keys
                                            .iter()
                                            .position(|&k| k == keys.drawlist_gen)?;
                                        if r.present_mask & (1 << idx) != 0 {
                                            Some(r.values[idx])
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or(0);
                                if let Some(asset) = cache.get_or_parse_drawlist(bs_id, gen) {
                                    drawlist_content = Some(asset.cmds);
                                    drawlist_source = Some(bs_id);
                                    drawlist_hash = Some(asset.hash);
                                }
                            }
                        } else {
                            props.insert(key, val);
                        }
                    }
                }
            }
            Err(_) => {
                crate::trace_counter!("ui.snap.bulk_fallback", 1);
                self.fetch_fallback(
                    id,
                    keys,
                    &mut props,
                    &mut strings,
                    &mut svg_content,
                    &mut svg_source,
                    &mut svg_hash,
                    &mut window_icon_content,
                    &mut window_icon_source,
                    &mut window_icon_hash,
                    &mut drawlist_content,
                    &mut drawlist_source,
                    &mut drawlist_hash,
                    cache,
                );
            }
        }
        (
            props,
            strings,
            svg_content,
            svg_source,
            svg_hash,
            window_icon_content,
            window_icon_source,
            window_icon_hash,
            drawlist_content,
            drawlist_source,
            drawlist_hash,
        )
    }

    fn fetch_fallback(
        &self,
        id: ThingId,
        keys: &UiKeys,
        props: &mut BTreeMap<u32, u64>,
        strings: &mut BTreeMap<u32, String>,
        svg_content: &mut Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
        svg_source: &mut Option<ThingId>,
        svg_hash: &mut Option<u64>,
        window_icon_content: &mut Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
        window_icon_source: &mut Option<ThingId>,
        window_icon_hash: &mut Option<u64>,
        drawlist_content: &mut Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
        drawlist_source: &mut Option<ThingId>,
        drawlist_hash: &mut Option<u64>,
        cache: &mut AssetCache,
    ) {
        for &p in &keys.numeric_keys() {
            if p == 0 {
                continue;
            }
            crate::trace_counter!("snap.syscalls.prop_get", 1);
            if let Ok(val) = stem::thing::sys::prop_get(id, p) {
                props.insert(p, val);
                if p == keys.svg_bytes && val != 0 {
                    let bs_id = ThingId::from_u64(val);
                    if let Some(asset) = cache.get_or_parse_svg(bs_id) {
                        *svg_content = Some(asset.cmds);
                        *svg_source = Some(bs_id);
                        *svg_hash = Some(asset.xml_hash);
                    }
                }
                if p == keys.window_icon && val != 0 {
                    let bs_id = ThingId::from_u64(val);
                    if let Some(asset) = cache.get_or_parse_svg(bs_id) {
                        *window_icon_content = Some(asset.cmds);
                        *window_icon_source = Some(bs_id);
                        *window_icon_hash = Some(asset.xml_hash);
                    }
                }
                if p == keys.tile_asset && val != 0 {
                    let bs_id = ThingId::from_u64(val);
                    if let Some(asset) = cache.get_or_parse_svg(bs_id) {
                        *svg_content = Some(asset.cmds);
                        *svg_source = Some(bs_id);
                        *svg_hash = Some(asset.xml_hash);
                    }
                }
                if p == keys.drawlist_bytespace && val != 0 {
                    let bs_id = ThingId::from_u64(val);
                    let gen = stem::thing::sys::prop_get(id, keys.drawlist_gen).unwrap_or(0);
                    if let Some(asset) = cache.get_or_parse_drawlist(bs_id, gen) {
                        *drawlist_content = Some(asset.cmds);
                        *drawlist_source = Some(bs_id);
                        *drawlist_hash = Some(asset.hash);
                    }
                }
            }
        }
        for &p in &keys.string_keys() {
            if p == 0 {
                continue;
            }
            crate::trace_counter!("snap.syscalls.prop_get", 1);
            if let Ok(val) = stem::thing::sys::prop_get(id, p) {
                if val != 0 {
                    if let Some(s) = cache.get_or_read_string(ThingId::from_u64(val)) {
                        strings.insert(p, s);
                    }
                }
            }
        }
    }

    pub fn diff(&self, prev: &Self) -> Vec<ThingId> {
        let mut changed = Vec::new();
        for (id, node) in &self.nodes {
            if let Some(prev_node) = prev.nodes.get(id) {
                if !self.nodes_equal(node, prev_node) {
                    changed.push(*id);
                }
            } else {
                changed.push(*id);
            }
        }
        for id in prev.nodes.keys() {
            if !self.nodes.contains_key(id) {
                changed.push(*id);
            }
        }
        changed
    }

    /// Compute a hash of all windows for cache invalidation.
    /// Includes window IDs, bounds, and title hashes.
    pub fn windows_hash(&self) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        let mut window_count = 0u32;
        for (id, node) in &self.nodes {
            if node.kind == UiNodeKind::Window {
                window_count += 1;
                // FNV-1a hash mix
                hash ^= id.to_u64_lossy();
                hash = hash.wrapping_mul(0x100000001b3);
                // Include key properties to detect changes
                for (k, v) in &node.props {
                    hash ^= (*k as u64) ^ v;
                    hash = hash.wrapping_mul(0x100000001b3);
                }
            }
        }
        // Include window count in hash
        hash ^= window_count as u64;
        hash
    }

    /// Count of window nodes in the snapshot
    pub fn window_count(&self) -> usize {
        self.nodes
            .values()
            .filter(|n| n.kind == UiNodeKind::Window)
            .count()
    }

    fn nodes_equal(&self, a: &UiNodeSnapshot, b: &UiNodeSnapshot) -> bool {
        a.kind == b.kind
            && a.props == b.props
            && a.strings == b.strings
            && a.children == b.children
            && a.svg_hash == b.svg_hash
            && a.window_icon_hash == b.window_icon_hash
            && a.drawlist_hash == b.drawlist_hash
    }

    fn layout_keys_changed(
        old: &BTreeMap<u32, u64>,
        new: &BTreeMap<u32, u64>,
        keys: &UiKeys,
    ) -> bool {
        let layout_keys = [
            keys.x,
            keys.y,
            keys.w,
            keys.h,
            keys.center_x,
            keys.center_y,
            keys.fill_parent,
            keys.inset_right,
            keys.inset_bottom,
            keys.z_index,
            keys.window_shaded,
        ];
        layout_keys
            .iter()
            .copied()
            .filter(|k| *k != 0)
            .any(|k| old.get(&k).unwrap_or(&0) != new.get(&k).unwrap_or(&0))
    }

    fn measure_keys_changed(
        old_props: &BTreeMap<u32, u64>,
        new_props: &BTreeMap<u32, u64>,
        old_strings: &BTreeMap<u32, String>,
        new_strings: &BTreeMap<u32, String>,
        keys: &UiKeys,
    ) -> bool {
        let mut changed = false;
        let text_key = keys.text;
        let font_key = keys.font;
        let font_stack_key = keys.font_stack;
        let size_key = keys.font_size;

        if text_key != 0 && old_strings.get(&text_key) != new_strings.get(&text_key) {
            changed = true;
        }
        if font_key != 0 && old_strings.get(&font_key) != new_strings.get(&font_key) {
            changed = true;
        }
        if font_stack_key != 0
            && old_strings.get(&font_stack_key) != new_strings.get(&font_stack_key)
        {
            changed = true;
        }
        if size_key != 0
            && old_props.get(&size_key).unwrap_or(&0) != new_props.get(&size_key).unwrap_or(&0)
        {
            changed = true;
        }
        changed
    }
}
