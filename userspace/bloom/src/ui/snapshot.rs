use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use stem::thing::sys::get_kind;
use stem::thing::ThingId;
use abi::ids::HandleId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiNodeKind {
    Unknown, Root, Window, Panel, Text, Image, Overlay, Inline,
}

impl UiNodeKind {
    pub fn from_symbol(id: u32, kinds: &KindIds) -> Self {
        if id == 0 { return Self::Unknown; }
        if id == kinds.root { Self::Root }
        else if id == kinds.window { Self::Window }
        else if id == kinds.panel { Self::Panel }
        else if id == kinds.text { Self::Text }
        else if id == kinds.image { Self::Image }
        else if id == kinds.overlay { Self::Overlay }
        else if id == kinds.inline { Self::Inline }
        else { Self::Unknown }
    }
}

#[derive(Clone)]
pub struct KindIds {
    pub root: u32, pub window: u32, pub panel: u32,
    pub text: u32, pub image: u32, pub overlay: u32,
    pub inline: u32,
}

impl KindIds {
    pub fn empty() -> Self {
        Self { root: 0, window: 0, panel: 0, text: 0, image: 0, overlay: 0, inline: 0 }
    }
    pub fn intern() -> Self {
        use abi::schema::kinds;
        crate::trace_span!("ui.init.intern_kinds");
        let kids = Self {
            root: stem::thing::sys::intern(kinds::UI_ROOT).unwrap_or(0),
            window: stem::thing::sys::intern(kinds::UI_WINDOW).unwrap_or(0),
            panel: stem::thing::sys::intern(kinds::UI_PANEL).unwrap_or(0),
            text: stem::thing::sys::intern(kinds::UI_TEXT).unwrap_or(0),
            image: stem::thing::sys::intern(kinds::UI_IMAGE).unwrap_or(0),
            overlay: stem::thing::sys::intern(kinds::UI_OVERLAY).unwrap_or(0),
            inline: stem::thing::sys::intern(kinds::UI_INLINE).unwrap_or(0),
        };
        crate::trace_counter!("ui.init.syscalls.intern_kinds", 7);
        kids
    }
}

#[derive(Clone)]
pub struct UiKeys {
    pub x: u32, pub y: u32, pub w: u32, pub h: u32,
    pub color: u32, pub text: u32, pub font: u32,
    pub font_size: u32, pub font_stack: u32, pub font_debug: u32,
    pub radius: u32, pub title: u32, pub hidden: u32,
    pub z_index: u32, pub center_x: u32, pub center_y: u32,
    pub fill_parent: u32, pub bg_color: u32, pub fg_color: u32,
    pub has_child: u32, pub inset_right: u32, pub inset_bottom: u32,
    pub inline_mode: u32, pub svg_bytes: u32,
    pub window_icon: u32,
}

impl UiKeys {
    pub fn empty() -> Self {
        Self {
            x: 0, y: 0, w: 0, h: 0, color: 0, text: 0, font: 0,
            font_size: 0, font_stack: 0, font_debug: 0, radius: 0,
            title: 0, hidden: 0, z_index: 0, center_x: 0, center_y: 0,
            fill_parent: 0, bg_color: 0, fg_color: 0, has_child: 0,
            inset_right: 0, inset_bottom: 0, inline_mode: 0, svg_bytes: 0,
            window_icon: 0,
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
        };
        crate::trace_counter!("ui.init.syscalls.intern_keys", 25);
        k
    }
    pub fn numeric_keys(&self) -> [u32; 20] {
        [self.x, self.y, self.w, self.h, self.color, self.radius,
         self.hidden, self.z_index, self.center_x, self.center_y,
         self.fill_parent, self.bg_color, self.fg_color, 
         self.inset_right, self.inset_bottom, self.font_size, self.font_debug,
         self.inline_mode, self.svg_bytes, self.window_icon]
    }
    pub fn string_keys(&self) -> [u32; 4] {
        [self.text, self.font, self.font_stack, self.title]
    }
    pub fn all_keys(&self) -> [u32; 24] {
        [self.x, self.y, self.w, self.h, self.color, self.radius,
         self.hidden, self.z_index, self.center_x, self.center_y,
         self.fill_parent, self.bg_color, self.fg_color, 
         self.inset_right, self.inset_bottom, self.font_size, self.font_debug,
         self.inline_mode, self.svg_bytes, self.window_icon,
         self.text, self.font, self.font_stack, self.title]
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
    pub window_icon_content: Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
}

/// Cached asset entry: bytespace_id -> decoded content
#[derive(Clone, Default)]
pub struct AssetCache {
    strings: BTreeMap<u64, String>,
    svgs: BTreeMap<u64, alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
}

impl AssetCache {
    pub fn new() -> Self { Self { strings: BTreeMap::new(), svgs: BTreeMap::new() } }
    
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

    pub fn get_or_parse_svg(&mut self, bs_id: ThingId) -> Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>> {
        let id = bs_id.to_u64_lossy();
        if let Some(cmds) = self.svgs.get(&id) {
            return Some(cmds.clone());
        }
        
        // Parse
        let xml = Self::read_string_raw(bs_id)?;
        let mut parser = crate::svg::SvgParser::new();
        // SVG icons are usually small, parsing is fast enough to do on-demand if cached.
        let cmds = parser.parse(&xml);
        let arc_cmds = alloc::sync::Arc::new(cmds);
        
        self.svgs.insert(id, arc_cmds.clone());
        Some(arc_cmds)
    }
    
    fn read_string_raw(bs_id: ThingId) -> Option<String> {
        use stem::thing::sys::{bytespace_info, bytespace_read};
        crate::trace_counter!("snap.syscalls.read_string", 2);
        let size = bytespace_info(bs_id).ok()?;
        if size == 0 { return Some(String::new()); }
        let mut buf = alloc::vec![0u8; size];
        let len = bytespace_read(bs_id, 0, &mut buf).ok()?;
        Some(String::from(core::str::from_utf8(&buf[..len]).unwrap_or("")))
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
    
    pub fn len(&self) -> usize { self.strings.len() + self.svgs.len() }
}

#[derive(Clone)]
pub struct UiSnapshot {
    pub root_id: Option<ThingId>,
    pub nodes: BTreeMap<ThingId, UiNodeSnapshot>,
}

impl UiSnapshot {
    pub fn new() -> Self {
        Self { root_id: None, nodes: BTreeMap::new() }
    }

    /// Full capture from scratch
    pub fn capture(root_id: ThingId, keys: &UiKeys, kinds: &KindIds) -> Self {
        let mut cache = AssetCache::new();
        Self::capture_with_cache(root_id, keys, kinds, &mut cache)
    }
    
    /// Capture with asset cache (Phase C optimization)
    pub fn capture_with_cache(root_id: ThingId, keys: &UiKeys, kinds: &KindIds, cache: &mut AssetCache) -> Self {
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
    
    /// Incremental update: only refresh specific dirty nodes (Phase F)
    pub fn update_nodes(&mut self, dirty_ids: &[ThingId], keys: &UiKeys, kinds: &KindIds, cache: &mut AssetCache) {
        crate::trace_span!("ui.snap.incremental");
        crate::trace_counter!("ui.snap.incremental_nodes", dirty_ids.len());
        
        for &id in dirty_ids {
            // Remove old node data
            self.nodes.remove(&id);
            // Re-traverse just this node (not its children unless they're also dirty)
            self.traverse_single(id, kinds, keys, cache);
        }
    }

    fn traverse(&mut self, id: ThingId, kind_ids: &KindIds, keys: &UiKeys, cache: &mut AssetCache) {
        if self.nodes.contains_key(&id) { return; }
        
        let kind_sym = {
            crate::trace_span!("ui.snap.get_kind");
            crate::trace_counter!("snap.syscalls.get_kind", 1);
            get_kind(id).ok()
        };
        let kind = match kind_sym {
            Some(sym) => {
                let k = UiNodeKind::from_symbol(sym.0 as u32, kind_ids);
                if k == UiNodeKind::Text { crate::trace_counter!("ui.snap.text_nodes", 1); }
                k
            },
            None => return,
        };

        let mut children = Vec::new();
        let mut edges_buf = [abi::types::Edge::default(); 64];
        {
            crate::trace_span!("ui.snap.get_edges");
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

        let (props, strings, svg_content, window_icon_content) = self.fetch_properties(id, keys, cache);
        self.nodes.insert(id, UiNodeSnapshot { id, kind, props, strings, children: children.clone(), svg_content, window_icon_content });
        for child in children { self.traverse(child, kind_ids, keys, cache); }
    }
    
    /// Traverse a single node without recursing (for incremental updates)
    fn traverse_single(&mut self, id: ThingId, kind_ids: &KindIds, keys: &UiKeys, cache: &mut AssetCache) {
        let kind_sym = {
            crate::trace_counter!("snap.syscalls.get_kind", 1);
            get_kind(id).ok()
        };
        let kind = match kind_sym {
            Some(sym) => UiNodeKind::from_symbol(sym.0 as u32, kind_ids),
            None => return,
        };

        let mut children = Vec::new();
        let mut edges_buf = [abi::types::Edge::default(); 64];
        {
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

        let (props, strings, svg_content, window_icon_content) = self.fetch_properties(id, keys, cache);
        self.nodes.insert(id, UiNodeSnapshot { id, kind, props, strings, children, svg_content, window_icon_content });
    }
    
    fn fetch_properties(
        &self, 
        id: ThingId, 
        keys: &UiKeys, 
        cache: &mut AssetCache
    ) -> (BTreeMap<u32, u64>, BTreeMap<u32, String>, Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>, Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>) {
        let mut props = BTreeMap::new();
        let mut strings = BTreeMap::new();
        let mut svg_content = None;
        let mut window_icon_content = None;

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
                                 svg_content = cache.get_or_parse_svg(ThingId::from_u64(val));
                             }
                        } else if key == keys.window_icon {
                             if val != 0 {
                                 props.insert(key, val);
                                 window_icon_content = cache.get_or_parse_svg(ThingId::from_u64(val));
                             }
                        } else {
                            props.insert(key, val);
                        }
                    }
                }
            }
            Err(_) => {
                crate::trace_counter!("ui.snap.bulk_fallback", 1);
                self.fetch_fallback(id, keys, &mut props, &mut strings, &mut svg_content, &mut window_icon_content, cache);
            }
        }
        (props, strings, svg_content, window_icon_content)
    }
    
    fn fetch_fallback(
        &self, 
        id: ThingId, 
        keys: &UiKeys, 
        props: &mut BTreeMap<u32, u64>, 
        strings: &mut BTreeMap<u32, String>, 
        svg_content: &mut Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
        window_icon_content: &mut Option<alloc::sync::Arc<Vec<crate::drawlist::DrawCmd>>>,
        cache: &mut AssetCache
    ) {
        for &p in &keys.numeric_keys() {
            if p == 0 { continue; }
            crate::trace_counter!("snap.syscalls.prop_get", 1);
            if let Ok(val) = stem::thing::sys::prop_get(id, p) { 
                props.insert(p, val); 
                if p == keys.svg_bytes && val != 0 {
                    *svg_content = cache.get_or_parse_svg(ThingId::from_u64(val));
                }
                if p == keys.window_icon && val != 0 {
                    *window_icon_content = cache.get_or_parse_svg(ThingId::from_u64(val));
                }
            }
        }
        for &p in &keys.string_keys() {
            if p == 0 { continue; }
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
                if !self.nodes_equal(node, prev_node) { changed.push(*id); }
            } else { changed.push(*id); }
        }
        for id in prev.nodes.keys() {
            if !self.nodes.contains_key(id) { changed.push(*id); }
        }
        changed
    }

    fn nodes_equal(&self, a: &UiNodeSnapshot, b: &UiNodeSnapshot) -> bool {
        a.kind == b.kind && a.props == b.props && a.strings == b.strings && a.children == b.children
    }
}
