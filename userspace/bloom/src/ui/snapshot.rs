use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use stem::thing::ThingId;
use stem::thing::sys::{prop_get, dump_edges, get_kind};
use abi::schema::keys;
use abi::symbols::SymbolId;

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

impl UiNodeKind {
    pub fn from_symbol(id: SymbolId) -> Self {
        // Simple mapping based on known kinds if possible, or just store it.
        Self::Unknown(id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiNodeSnapshot {
    pub id: ThingId,
    pub kind: UiNodeKind,
    pub props: BTreeMap<SymbolId, u64>,
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
        let mut snapshot = Self::new();
        snapshot.root_id = Some(root_id);
        snapshot.traverse(root_id);
        snapshot
    }

    fn traverse(&mut self, id: ThingId) {
        if self.nodes.contains_key(&id) {
            return;
        }

        let kind_sym = get_kind(id).ok();
        let kind = match kind_sym {
            Some(sym) => UiNodeKind::from_symbol(sym.0 as u32),
            None => return,
        };

        // Query children via edges
        let mut children = Vec::new();
        let mut buf = [0u8; 1024];
        if let Ok(len) = dump_edges(id, &mut buf) {
             let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
             for line in s.lines() {
                 // Format: (src)-[:REL]->(dst_kindID:dst_kind)
                 // Example: (root1:ui.Root)-[:CHILD_OF]->(panel2:ui.Panel)
                 if line.contains("]-[:CHILD_OF]->(") {
                     if let Some(pos) = line.find("]->(") {
                         let target_part = &line[pos + 4 ..];
                         // target_part starts with "basenameID:kind)"
                         if let Some(colon_pos) = target_part.find(':') {
                             let id_part = &target_part[..colon_pos];
                             // ID is at the end of id_part in Hex.
                             // We need to strip the basename. 
                             // Basenames are lowercase. 
                             let hex_str = id_part.trim_start_matches(|c: char| c.is_lowercase());
                             if let Ok(target_id) = u64::from_str_radix(hex_str, 16) {
                                  children.push(ThingId(target_id));
                             }
                         }
                     }
                 }
             }
        }

        let mut props = BTreeMap::new();
        let known_keys = [
            keys::UI_X, keys::UI_Y, keys::UI_WIDTH, keys::UI_HEIGHT,
            keys::UI_COLOR, keys::UI_TEXT, keys::UI_FONT, keys::UI_FONT_SIZE,
            keys::UI_RADIUS, keys::UI_TITLE, keys::UI_HIDDEN, keys::UI_Z_INDEX,
            keys::UI_CENTER_X, keys::UI_CENTER_Y, keys::UI_FILL_PARENT,
            keys::UI_BG_COLOR, keys::UI_FG_COLOR,
        ];

        for key in known_keys {
            if let Ok(val) = prop_get(id, key) {
                if let Ok(key_id) = stem::thing::sys::intern(key) {
                    props.insert(key_id, val);
                }
            }
        }

        self.nodes.insert(id, UiNodeSnapshot {
            id,
            kind,
            props,
            children: children.clone(),
        });

        for child_id in children {
            self.traverse(child_id);
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
