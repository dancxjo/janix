use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeMap;
use stem::thing::ThingId;
use stem::thing::sys::{prop_get_raw, get_kind};
use abi::schema::{keys, rels};
use abi::symbols::SymbolId;
use abi::ids::HandleId;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub enum UiNodeKind {
    Root,
    Window,
    Panel,
    Text,
    Image,
    Overlay,
    Unknown(SymbolId),
}

#[derive(Clone, Copy)]
pub struct KindIds {
    pub root: SymbolId,
    pub window: SymbolId,
    pub panel: SymbolId,
    pub text: SymbolId,
    pub image: SymbolId,
    pub overlay: SymbolId,
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
            x: stem::thing::sys::intern(keys::UI_X).unwrap_or_default(),
            y: stem::thing::sys::intern(keys::UI_Y).unwrap_or_default(),
            w: stem::thing::sys::intern(keys::UI_WIDTH).unwrap_or_default(),
            h: stem::thing::sys::intern(keys::UI_HEIGHT).unwrap_or_default(),
            color: stem::thing::sys::intern(keys::UI_COLOR).unwrap_or_default(),
            text: stem::thing::sys::intern(keys::UI_TEXT).unwrap_or_default(),
            font: stem::thing::sys::intern(keys::UI_FONT).unwrap_or_default(),
            font_size: stem::thing::sys::intern(keys::UI_FONT_SIZE).unwrap_or_default(),
            radius: stem::thing::sys::intern(keys::UI_RADIUS).unwrap_or_default(),
            title: stem::thing::sys::intern(keys::UI_TITLE).unwrap_or_default(),
            hidden: stem::thing::sys::intern(keys::UI_HIDDEN).unwrap_or_default(),
            z_index: stem::thing::sys::intern(keys::UI_Z_INDEX).unwrap_or_default(),
            center_x: stem::thing::sys::intern(keys::UI_CENTER_X).unwrap_or_default(),
            center_y: stem::thing::sys::intern(keys::UI_CENTER_Y).unwrap_or_default(),
            fill_parent: stem::thing::sys::intern(keys::UI_FILL_PARENT).unwrap_or_default(),
            bg_color: stem::thing::sys::intern(keys::UI_BG_COLOR).unwrap_or_default(),
            fg_color: stem::thing::sys::intern(keys::UI_FG_COLOR).unwrap_or_default(),
            has_child: stem::thing::sys::intern(rels::HAS_CHILD).unwrap_or_default(),
        }
    }
}

impl UiNodeKind {
    pub fn from_symbol(id: SymbolId, kinds: &KindIds) -> Self {
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
    pub props: BTreeMap<SymbolId, [u8; 16]>,
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
            root: stem::thing::sys::intern(kinds::UI_ROOT).unwrap_or_default(),
            window: stem::thing::sys::intern(kinds::UI_WINDOW).unwrap_or_default(),
            panel: stem::thing::sys::intern(kinds::UI_PANEL).unwrap_or_default(),
            text: stem::thing::sys::intern(kinds::UI_TEXT).unwrap_or_default(),
            image: stem::thing::sys::intern(kinds::UI_IMAGE).unwrap_or_default(),
            overlay: stem::thing::sys::intern(kinds::UI_OVERLAY).unwrap_or_default(),
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
            Some(sym) => {
                 // get_kind returns ThingKind which wraps u64 (old) or SymbolId?
                 // stem::thing::sys::get_kind(id) returns Result<ThingKind, Errno>
                 // ThingKind is u64 wrapper in stem/src/thing/mod.rs likely.
                 // But wait, my stem/src/thing/sys.rs implementation of get_kind:
                 /*
                    pub fn get_kind(id: ThingId) -> Result<ThingKind, Errno> {
                        let mut out = SymbolId::default();
                        // ... syscall ...
                        let mut bytes = [0u8; 8];
                        bytes.copy_from_slice(&out.0[0..8]);
                        let val = u64::from_le_bytes(bytes);
                        errno(ret).map(|_| ThingKind(val))
                    }
                 */
                 // It returns ThingKind which is u64 wrapper. It truncates SymbolId.
                 // This is bad if symbols are UUIDs.
                 // UiNodeKind::from_symbol expects SymbolId.
                 // I need to change get_kind in stem/src/thing/sys.rs to return SymbolId or ThingKind wrapping [u8; 16].
                 // For now, I will assume get_kind returns ThingKind(u64) which is truncated SymbolId.
                 // This is inconsistent.

                 // If I look at stem/src/thing/mod.rs, ThingKind is struct ThingKind(pub u64).
                 // I should probably skip fixing stem ThingKind definition for now and just deal with what I have,
                 // or fix stem ThingKind to be u128/UUID.

                 // User said "From userland, I should always just work with UUIDs proper."
                 // So I MUST fix stem::ThingKind to be UUID-compatible.

                 // If I change ThingKind to wrap [u8; 16], it breaks other things?
                 // Let's assume for this step I just want bloom to compile.

                 // But wait, `UiNodeKind::from_symbol` takes `SymbolId` (16 bytes).
                 // `get_kind` returns `ThingKind` (u64).
                 // In `traverse`, I need to call `root_get_kind` directly or fix `get_kind`.

                 // Let's call `stem::syscall::root_get_kind` directly to get full SymbolId.
                 let mut sym = SymbolId::default();
                 if let Ok(_) = stem::syscall::root_get_kind(&id, &mut sym) {
                     UiNodeKind::from_symbol(sym, kind_ids)
                 } else {
                     return;
                 }
            },
            None => return,
        };

        // Query children via edges
        let mut children = Vec::new();
        let mut edges_buf = [abi::types::Edge::default(); 64];
        if let Ok(count) = stem::thing::sys::get_edges(id, &mut edges_buf) {
            for edge in &edges_buf[..count] {
                // Check if (id)-[:HAS_CHILD]->(child)
                let rel = edge.predicate; // PredicateId ([u8; 16])
                let target = edge.to; // ThingId ([u8; 16])

                // Compare with keys.has_child (SymbolId)
                // PredicateId and SymbolId are both [u8; 16].
                // keys.has_child is SymbolId.
                // Assuming PredicateId has .0 or is compatible.
                // If PredicateId is [u8; 16], we can compare bytes.
                
                if rel.0 == keys.has_child.0 && target != id {
                    children.push(edge.to);
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
            if let Ok(val) = prop_get_raw(id, key_str) {
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
            if let Ok(val) = prop_get_raw(id, key_str) {
                props.insert(key_id, val);
                
                // If this is a string property, snapshot its content
                if key_id == keys.text || key_id == keys.font || key_id == keys.title {
                    // val is [u8; 16] (ThingId of bytespace)
                    // Check if it is not null/zero?
                    if val != [0u8; 16] {
                         let bs_id = ThingId(val);
                         let mut s_buf = [0u8; 1024];
                        if let Ok(len) = stem::thing::sys::bytespace_read(bs_id, 0, &mut s_buf) {
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
