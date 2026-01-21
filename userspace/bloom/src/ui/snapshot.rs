use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use stem::thing::sys::get_kind;
use stem::thing::ThingId;
use abi::ids::HandleId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiNodeKind {
    Unknown,
    Root,
    Window,
    Panel,
    Text,
    Image,
    Overlay,
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
        } else if id == kinds.image {
            Self::Image
        } else if id == kinds.overlay {
            Self::Overlay
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
    pub image: u32,
    pub overlay: u32,
}

impl KindIds {
    /// Create empty (uninitialized) KindIds
    pub fn empty() -> Self {
        Self {
            root: 0,
            window: 0,
            panel: 0,
            text: 0,
            image: 0,
            overlay: 0,
        }
    }

    /// Intern all kind symbols from Root (expensive - do once at init)
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
        };
        crate::trace_counter!("ui.init.syscalls.intern_kinds", 6);
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
}

impl UiKeys {
    /// Create empty (uninitialized) UiKeys
    pub fn empty() -> Self {
        Self {
            x: 0, y: 0, w: 0, h: 0, color: 0, text: 0, font: 0,
            font_size: 0, font_stack: 0, font_debug: 0, radius: 0,
            title: 0, hidden: 0, z_index: 0, center_x: 0, center_y: 0,
            fill_parent: 0, bg_color: 0, fg_color: 0, has_child: 0,
            inset_right: 0, inset_bottom: 0,
        }
    }

    /// Intern all key symbols from Root (expensive - do once at init)
    pub fn intern() -> Self {
        crate::trace_span!("ui.init.intern_keys");
        use abi::schema::{keys, rels};
        let keys = Self {
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
        };
        crate::trace_counter!("ui.init.syscalls.intern_keys", 22);
        keys
    }
    
    /// Get the array of numeric property keys (for bulk fetch)
    pub fn numeric_keys(&self) -> [u32; 17] {
        [
            self.x, self.y, self.w, self.h, self.color, self.radius,
            self.hidden, self.z_index, self.center_x, self.center_y,
            self.fill_parent, self.bg_color, self.fg_color, 
            self.inset_right, self.inset_bottom,
            self.font_size, self.font_debug,
        ]
    }
    
    /// Get the array of string property keys (for bulk fetch)
    pub fn string_keys(&self) -> [u32; 4] {
        [self.text, self.font, self.font_stack, self.title]
    }
}

#[derive(Debug, Clone)]
pub struct UiNodeSnapshot {
    pub id: ThingId,
    pub kind: UiNodeKind,
    pub props: BTreeMap<u32, u64>,
    pub strings: BTreeMap<u32, String>,
    pub children: Vec<ThingId>,
}

#[derive(Debug, Clone)]
pub struct UiSnapshot {
    pub root_id: Option<ThingId>,
    pub nodes: BTreeMap<ThingId, UiNodeSnapshot>,
}

impl UiSnapshot {
    pub fn new() -> Self {
        Self {
            root_id: None,
            nodes: BTreeMap::new(),
        }
    }

    /// Capture UI snapshot using pre-cached keys and kinds (no interning per frame)
    pub fn capture(root_id: ThingId, keys: &UiKeys, kinds: &KindIds) -> Self {
        let mut snapshot = Self::new();
        snapshot.root_id = Some(root_id);
        {
            crate::trace_span!("ui.snap.traverse_all");
            snapshot.traverse(root_id, kinds, keys);
        }
        crate::trace_counter!("ui.snap.nodes_total", snapshot.nodes.len());
        snapshot
    }

    fn traverse(&mut self, id: ThingId, kind_ids: &KindIds, keys: &UiKeys) {
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
                if k == UiNodeKind::Text {
                    crate::trace_counter!("ui.snap.text_nodes", 1);
                }
                k
            },
            None => return,
        };

        // Query children via edges
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

        let mut props = BTreeMap::new();
        let mut strings = BTreeMap::new();

        // Numeric properties - use cached key IDs (no interning!)
        let numeric_keys = keys.numeric_keys();
        {
            crate::trace_span!("ui.snap.prop_get");
            let valid_keys: Vec<u32> = numeric_keys.iter().copied().filter(|&k| k != 0).collect();
            crate::trace_counter!("snap.syscalls.prop_get", valid_keys.len());
            crate::trace_counter!("ui.snap.prop_get.calls", valid_keys.len());
            for &p in &valid_keys {
                // Using u32 key ID directly - no interning needed!
                if let Ok(val) = stem::thing::sys::prop_get(id, p) {
                    props.insert(p, val);
                }
            }
        }

        // String properties
        let string_keys = keys.string_keys();
        {
            let valid_keys: Vec<u32> = string_keys.iter().copied().filter(|&k| k != 0).collect();
            crate::trace_counter!("ui.snap.read_string.calls", valid_keys.len());
            for &p in &valid_keys {
                crate::trace_counter!("snap.syscalls.prop_get", 1);
                if let Ok(val) = stem::thing::sys::prop_get(id, p) {
                    if val != 0 {
                        let s = {
                            crate::trace_span!("ui.snap.read_string");
                            crate::trace_counter!("snap.syscalls.read_string", 2); // info + read
                            self.read_string(ThingId::from_u64(val))
                        };
                        if let Some(s) = s {
                            strings.insert(p, s);
                        }
                    }
                }
            }
        }

        self.nodes.insert(
            id,
            UiNodeSnapshot {
                id,
                kind,
                props,
                strings,
                children: children.clone(),
            },
        );

        // Recurse
        for child in children {
            self.traverse(child, kind_ids, keys);
        }
    }

    fn read_string(&self, bs_id: ThingId) -> Option<String> {
        use stem::thing::sys::{bytespace_info, bytespace_read};
        let size = bytespace_info(bs_id).ok()?;
        if size == 0 {
            return Some(String::new());
        }
        let mut buf = alloc::vec![0u8; size];
        let len = bytespace_read(bs_id, 0, &mut buf).ok()?;
        Some(String::from(core::str::from_utf8(&buf[..len]).unwrap_or("")))
    }

    pub fn diff(&self, prev: &Self) -> Vec<ThingId> {
        let mut changed = Vec::new();

        // Nodes in self but not in prev, or different content
        for (id, node) in &self.nodes {
            if let Some(prev_node) = prev.nodes.get(id) {
                if !self.nodes_equal(node, prev_node) {
                    changed.push(*id);
                }
            } else {
                changed.push(*id);
            }
        }

        // Nodes in prev but not in self (deleted)
        for id in prev.nodes.keys() {
            if !self.nodes.contains_key(id) {
                changed.push(*id);
            }
        }

        changed
    }

    fn nodes_equal(&self, a: &UiNodeSnapshot, b: &UiNodeSnapshot) -> bool {
        if a.kind != b.kind {
            return false;
        }
        if a.props != b.props {
            return false;
        }
        if a.strings != b.strings {
            return false;
        }
        if a.children != b.children {
            return false;
        }
        true
    }
}
