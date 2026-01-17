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

#[derive(Clone, Copy)]
pub struct KindIds {
    pub root: u32,
    pub window: u32,
    pub panel: u32,
    pub text: u32,
    pub image: u32,
    pub overlay: u32,
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

        let mut snapshot = Self::new();
        snapshot.root_id = Some(root_id);
        snapshot.traverse(root_id, &kind_ids);
        snapshot
    }

    fn traverse(&mut self, id: ThingId, kind_ids: &KindIds) {
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
        let mut buf = [0u8; 1024];
        if let Ok(len) = dump_edges(id, &mut buf) {
             let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
             for line in s.lines() {
                 if line.is_empty() { continue; }

                 
                 // Robust Cypher Parsing
                 // Expected: (src_base:srcID:src_kind)-[:REL]->(dst_base:dstID:dst_kind)
                 if line.contains("-[:CHILD_OF]->") {
                     let (src_part, dst_part) = match (line.find('('), line.rfind('(')) {
                         (Some(start), Some(end)) if start != end => {
                             let src_end = line.find(')').unwrap_or(0);
                             let src = &line[start + 1 .. src_end];
                             let dst_end = line.rfind(')').unwrap_or(line.len());
                             let dst = &line[end + 1 .. dst_end];
                             (src, dst)
                         }
                         _ => continue,
                     };

                     let extract_id = |part: &str| -> Option<u64> {
                         let mut pieces = part.split(':');
                         pieces.next(); // skip basename
                         pieces.next().and_then(|id_str| u64::from_str_radix(id_str, 16).ok())
                     };

                     if let (Some(src_id), Some(dst_id)) = (extract_id(src_part), extract_id(dst_part)) {
                         // Convention: link(child, CHILD_OF, parent) => (child)-[:CHILD_OF]->(parent)
                         // If the current node (id) is the parent (dst), then src is the child.
                         if dst_id == id.0 && src_id != id.0 {
                             children.push(ThingId(src_id));
                             stem::info!("SNAPSHOT: Node[{:x}] child -> {:x}", id.0, src_id);
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

        stem::info!("SNAPSHOT: Node[{:x}] kind={:?} children={} props={}", id.0, kind, children.len(), props.len());

        self.nodes.insert(id, UiNodeSnapshot {
            id,
            kind,
            props,
            children: children.clone(),
        });

        for child_id in children {
            self.traverse(child_id, kind_ids);
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
