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

pub struct KindIds {
    pub root: u32,
    pub window: u32,
    pub panel: u32,
    pub text: u32,
    pub image: u32,
    pub overlay: u32,
}

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
    pub fn intern() -> Self {
        use abi::schema::{keys, rels};
        Self {
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
        }
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

    pub fn capture(root_id: ThingId) -> Self {
        use abi::schema::kinds;
        let kind_ids = KindIds {
            root: stem::thing::sys::intern(kinds::UI_ROOT).unwrap_or(0),
            window: stem::thing::sys::intern(kinds::UI_WINDOW).unwrap_or(0),
            panel: stem::thing::sys::intern(kinds::UI_PANEL).unwrap_or(0),
            text: stem::thing::sys::intern(kinds::UI_TEXT).unwrap_or(0),
            image: stem::thing::sys::intern(kinds::UI_IMAGE).unwrap_or(0),
            overlay: stem::thing::sys::intern(kinds::UI_OVERLAY).unwrap_or(0),
        };

        // Diagnostic: log intern results once in a while
        let now_ms = crate::log_ratelimit::now_ms();
        if crate::log_ratelimit::log_every(2000, now_ms) {
            crate::log!("[bloom][ui] KindIds: root={} window={} panel={} text={} image={}",
                kind_ids.root, kind_ids.window, kind_ids.panel, kind_ids.text, kind_ids.image);
            if kind_ids.root == 0 || kind_ids.window == 0 {
                crate::log!("[bloom][ui] WARNING: UI_ROOT or UI_WINDOW failed to intern!");
            }
        }

        let keys = UiKeys::intern();

        let mut snapshot = Self::new();
        snapshot.root_id = Some(root_id);
        snapshot.traverse(root_id, &kind_ids, &keys);
        snapshot
    }

    fn traverse(&mut self, id: ThingId, kind_ids: &KindIds, keys: &UiKeys) {
        if self.nodes.contains_key(&id) {
            return;
        }

        let kind_sym = get_kind(id).ok();
        let kind = match kind_sym {
            Some(sym) => UiNodeKind::from_symbol(sym.0 as u32, kind_ids),
            None => return,
        };

        // Query children via edges
        let mut children = Vec::new();
        let mut edges_buf = [abi::types::Edge::default(); 64];
        if let Ok(count) = stem::thing::sys::get_edges(id, &mut edges_buf) {
            for edge in &edges_buf[..count] {
                // Check if (id)-[:HAS_CHILD]->(child)
                let rel_u64 = edge.predicate.to_u64_lossy();
                let target_u64 = edge.to.to_u64_lossy();
                
                if rel_u64 == keys.has_child as u64 && target_u64 != id.to_u64_lossy() {
                    children.push(edge.to);
                }
            }
        }

        let mut props = BTreeMap::new();
        let mut strings = BTreeMap::new();

        // Standard properties
        let prop_list = [
            keys.x, keys.y, keys.w, keys.h, keys.color, keys.radius,
            keys.hidden, keys.z_index, keys.center_x, keys.center_y,
            keys.fill_parent, keys.bg_color, keys.fg_color, keys.inset_right, keys.inset_bottom,
            keys.font_size, keys.font_debug,
        ];

        for &p in &prop_list {
            if p == 0 { continue; }
            if let Ok(val) = stem::thing::sys::prop_get(id, p) {
                props.insert(p, val);
            }
        }

        // String properties
        let str_list = [keys.text, keys.font, keys.font_stack, keys.title];
        for &p in &str_list {
            if p == 0 { continue; }
            if let Ok(val) = stem::thing::sys::prop_get(id, p) {
                // val is Bytespace ID
                if val != 0 {
                    if let Some(s) = self.read_string(ThingId::from_u64(val)) {
                        strings.insert(p, s);
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

        // Nodes in prev but not in self (deleted) - we also need to damage their old areas.
        // Actually the caller uses this to mark damage.
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
