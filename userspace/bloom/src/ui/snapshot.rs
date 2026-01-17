use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeMap;
use stem::thing::ThingId;
use stem::thing::sys::{prop_get, get_kind};
use abi::schema::{keys, rels};
use abi::symbols::SymbolId;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub enum UiNodeKind {
    Root,
    Window,
    Panel,
    Text,
    Image,
    Overlay,
    Unknown(u32),
}

#[derive(Clone, Copy)]
pub struct KindIds {
    pub root: u32,
    pub window: u32,
    pub panel: u32,
    pub text: u32,
    pub image: u32,
    pub overlay: u32,
}

#[derive(Clone, Copy)]
pub struct UiKeys {
    pub x: SymbolId,
    pub y: SymbolId,
    pub w: SymbolId,
    pub h: SymbolId,
    pub color: SymbolId,
    pub text: SymbolId,
    pub font: SymbolId,
    pub font_size: SymbolId,
    pub radius: SymbolId,
    pub title: SymbolId,
    pub hidden: SymbolId,
    pub z_index: SymbolId,
    pub center_x: SymbolId,
    pub center_y: SymbolId,
    pub fill_parent: SymbolId,
    pub bg_color: SymbolId,
    pub fg_color: SymbolId,
    pub has_child: SymbolId,
}

impl UiKeys {
    pub fn intern() -> Self {
        Self {
            x: stem::thing::sys::intern(keys::UI_X).unwrap_or(0),
            y: stem::thing::sys::intern(keys::UI_Y).unwrap_or(0),
            w: stem::thing::sys::intern(keys::UI_WIDTH).unwrap_or(0),
            h: stem::thing::sys::intern(keys::UI_HEIGHT).unwrap_or(0),
            color: stem::thing::sys::intern(keys::UI_COLOR).unwrap_or(0),
            text: stem::thing::sys::intern(keys::UI_TEXT).unwrap_or(0),
            font: stem::thing::sys::intern(keys::UI_FONT).unwrap_or(0),
            font_size: stem::thing::sys::intern(keys::UI_FONT_SIZE).unwrap_or(0),
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
        }
    }
}

impl UiNodeKind {
    pub fn from_symbol(id: u32, kinds: &KindIds) -> Self {
        if id == kinds.root { Self::Root }
        else if id == kinds.window { Self::Window }
        else if id == kinds.panel { Self::Panel }
        else if id == kinds.text { Self::Text }
        else if id == kinds.image { Self::Image }
        else if id == kinds.overlay { Self::Overlay }
        else { Self::Unknown(id) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiNodeSnapshot {
    pub id: ThingId,
    pub kind: UiNodeKind,
    pub props: BTreeMap<SymbolId, u64>,
    pub strings: BTreeMap<SymbolId, String>,
    pub children: Vec<ThingId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiSnapshot {
    pub nodes: BTreeMap<ThingId, UiNodeSnapshot>,
    pub root_id: Option<ThingId>,
}

impl UiSnapshot {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            root_id: None,
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
        let mut edges_buf = [abi::types::GraphEdge::default(); 64];
        if let Ok(count) = stem::thing::sys::get_edges(id, &mut edges_buf) {
            for edge in &edges_buf[..count] {
                // Check if (id)-[:HAS_CHILD]->(child)
                // edge.rel corresponds to the relationship ID
                if edge.rel == keys.has_child as u64 && edge.target != id.0 {
                    children.push(ThingId(edge.target));
                }
            }
        }

        let mut props = BTreeMap::new();
        let mut strings = BTreeMap::new();
        
        // Property fetching using pre-interned keys
        let common_keys = [
            (keys.x, keys::UI_X), (keys.y, keys::UI_Y), (keys.w, keys::UI_WIDTH), (keys.h, keys::UI_HEIGHT),
            (keys.z_index, keys::UI_Z_INDEX), (keys.hidden, keys::UI_HIDDEN),
        ];

        for (key_id, key_str) in common_keys {
            if let Ok(val) = prop_get(id, key_str) {
                props.insert(key_id, val);
            }
        }

        let kind_keys = match kind {
            UiNodeKind::Text => vec![
                (keys.text, keys::UI_TEXT), (keys.font, keys::UI_FONT), 
                (keys.font_size, keys::UI_FONT_SIZE), (keys.color, keys::UI_COLOR),
                (keys.center_x, keys::UI_CENTER_X), (keys.center_y, keys::UI_CENTER_Y),
            ],
            UiNodeKind::Panel | UiNodeKind::Window => vec![
                (keys.radius, keys::UI_RADIUS), (keys.title, keys::UI_TITLE), 
                (keys.bg_color, keys::UI_BG_COLOR), (keys.fg_color, keys::UI_FG_COLOR),
                (keys.fill_parent, keys::UI_FILL_PARENT),
            ],
            _ => vec![],
        };

        for (key_id, key_str) in kind_keys {
            if let Ok(val) = prop_get(id, key_str) {
                props.insert(key_id, val);
                
                // If this is a string property, snapshot its content
                if key_id == keys.text || key_id == keys.font || key_id == keys.title {
                    if val != 0 {
                        let mut s_buf = [0u8; 1024];
                        if let Ok(len) = stem::thing::sys::bytespace_read(ThingId(val), 0, &mut s_buf) {
                            let s = String::from(core::str::from_utf8(&s_buf[..len]).unwrap_or_default());
                            strings.insert(key_id, s);
                        }
                    }
                }
            }
        }

        self.nodes.insert(id, UiNodeSnapshot {
            id,
            kind,
            props,
            strings,
            children: children.clone(),
        });

        for child_id in children {
            self.traverse(child_id, kind_ids, keys);
        }
    }

    /// Returns a list of ThingIds that have changed between this snapshot and another.
    pub fn diff(&self, other: &UiSnapshot) -> Vec<ThingId> {
        let mut dirty = Vec::new();

        // Check for modified or new nodes
        for (id, node) in &self.nodes {
            match other.nodes.get(id) {
                Some(other_node) => {
                    if node != other_node {
                        dirty.push(*id);
                    }
                }
                None => {
                    dirty.push(*id);
                }
            }
        }

        // Check for deleted nodes
        for id in other.nodes.keys() {
            if !self.nodes.contains_key(id) {
                dirty.push(*id);
            }
        }

        dirty
    }
}
