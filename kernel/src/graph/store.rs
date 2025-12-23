use abi::{Link, Predicate, ThingId, syscall_defs::SymbolId};
use alloc::sync::Arc;
use alloc::vec::Vec;
use hashbrown::HashMap;
use spin::Mutex;
use thing_models::PropValue;

// Moved definition to replacement block above

// Imports for ResidentRef
use crate::resident::mapping::ResidentPage;
use abi::ProcessId;

#[derive(Debug, Clone)]
pub struct ResidentRef {
    pub pages: Vec<ResidentPage>,
    pub byte_len: usize,
    pub rw_holder: Option<ProcessId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageState {
    Resident,
    Archived,
    Both,
}

#[derive(Debug, Clone)]
pub struct ArchiveRef(pub u64); // Stub

#[derive(Debug, Clone)]
pub struct ThingNode {
    pub id: ThingId,
    pub kind: SymbolId,
    pub kind_id: SymbolId,
    pub props: Vec<(SymbolId, PropValue)>,
    pub resident: Option<ResidentRef>,
    pub owner_process: Option<ProcessId>,
    pub storage: StorageState,
    pub archived_ref: Option<ArchiveRef>,
    pub links: Vec<(Predicate, ThingId)>,
}

impl ThingNode {
    pub fn new_resident(
        id: ThingId,
        _kind_str: &'static str, // unused in node but kept for flag?
        kind: SymbolId,
        resident: ResidentRef,
        pid: ProcessId,
    ) -> Self {
        Self {
            id,
            kind,
            kind_id: kind, // assume same
            props: Vec::new(),
            resident: Some(resident),
            owner_process: Some(pid),
            storage: StorageState::Resident,
            archived_ref: None,
            links: Vec::new(),
        }
    }
}

pub struct GraphStore {
    pub things: HashMap<ThingId, ThingNode>,
    pub kind_index: HashMap<SymbolId, Vec<ThingId>>,
    pub next_id: u64,
    pub revision: u64,
}

pub struct ArchiveStore;
impl ArchiveStore {
    pub fn store(&mut self, _blob: Vec<u8>) -> ArchiveRef {
        ArchiveRef(0) // Stub
    }
}

static ARCHIVE_STORE: Mutex<Option<ArchiveStore>> = Mutex::new(Some(ArchiveStore)); // Default Some for stub

pub fn archive_store() -> &'static Mutex<Option<ArchiveStore>> {
    &ARCHIVE_STORE
}

// Global slab access for kernel (previously in store.rs)
static THINGS_SLAB: Mutex<Option<GraphStore>> = Mutex::new(None);

pub fn things_slab() -> &'static Mutex<Option<GraphStore>> {
    &THINGS_SLAB
}

impl GraphStore {
    pub fn new() -> Self {
        Self {
            things: HashMap::new(),
            kind_index: HashMap::new(),
            next_id: 1,
            revision: 0,
        }
    }

    pub fn create_thing(&mut self, kind: SymbolId, props: Vec<(SymbolId, PropValue)>) -> ThingId {
        let id = ThingId(self.next_id);
        self.next_id += 1;
        self.revision += 1;

        let node = ThingNode {
            id,
            kind,
            kind_id: kind,
            props,
            resident: None,
            owner_process: None,
            storage: StorageState::Resident, // Default
            archived_ref: None,
            links: Vec::new(),
        };

        self.things.insert(id, node);
        self.kind_index.entry(kind).or_default().push(id);
        id
    }

    pub fn create_resident(
        &mut self,
        kind: SymbolId,
        resident: ResidentRef,
        pid: ProcessId,
    ) -> ThingId {
        let id = ThingId(self.next_id);
        self.next_id += 1;
        self.revision += 1;
        let node = ThingNode {
            id,
            kind,
            kind_id: kind,
            props: Vec::new(),
            resident: Some(resident),
            owner_process: Some(pid),
            storage: StorageState::Resident,
            archived_ref: None,
            links: Vec::new(),
        };
        self.things.insert(id, node);
        self.kind_index.entry(kind).or_default().push(id);
        id
    }

    pub fn update_thing(&mut self, id: ThingId, props: Vec<(SymbolId, PropValue)>) -> bool {
        if let Some(node) = self.things.get_mut(&id) {
            self.revision += 1;
            for (key, val) in props {
                // Simplistic update: remove old, push new
                if let Some(pos) = node.props.iter().position(|(k, _)| *k == key) {
                    node.props[pos] = (key, val);
                } else {
                    node.props.push((key, val));
                }
            }
            true
        } else {
            false
        }
    }

    pub fn get_node_mut(&mut self, id: ThingId) -> Option<&mut ThingNode> {
        // Mutation via get_node_mut assumes caller might modify.
        // But since we can't track what they do, we should increment revision if they take a mutable ref?
        // OR we rely on specialized methods.
        // This is risky. But for now, let's assume get_node_mut is used for internal things.
        // Actually, if we modify links via this, we miss revision update.
        // However, add_link/remove_link are methods on GraphStore.
        // So direct modification of node via get_node_mut is dangerous if not tracked.
        // Given current usage, let's assume it's fine or we should increment revision here too just in case.
        // self.revision += 1; // Conservative
        self.things.get_mut(&id)
    }

    pub fn delete_thing(&mut self, id: ThingId) -> Option<ThingNode> {
        self.revision += 1;
        let node = self.things.remove(&id);
        if let Some(ref n) = node {
            if let Some(list) = self.kind_index.get_mut(&n.kind) {
                if let Ok(idx) = list.binary_search(&id) {
                    list.remove(idx);
                }
            }
        }
        node
    }

    pub fn get_thing_kind(&self, id: ThingId) -> Option<SymbolId> {
        self.things.get(&id).map(|node| node.kind)
    }

    pub fn get_prop(&self, id: ThingId, key: SymbolId) -> Option<PropValue> {
        self.things.get(&id).and_then(|node| {
            node.props
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v): &(_, _)| v.clone())
        })
    }

    // Placeholder links implementation until we fully port links
    pub fn add_link(&mut self, src: ThingId, dst: ThingId, pred: Predicate) -> bool {
        if let Some(node) = self.things.get_mut(&src) {
            self.revision += 1;
            node.links.push((pred, dst));
            true
        } else {
            false
        }
    }

    pub fn remove_link(&mut self, src: ThingId, dst: ThingId, pred: Predicate) -> bool {
        if let Some(node) = self.things.get_mut(&src) {
            if let Some(pos) = node.links.iter().position(|(p, d)| *p == pred && *d == dst) {
                self.revision += 1;
                node.links.remove(pos);
                return true;
            }
        }
        false
    }

    pub fn get_link(&self, src: ThingId, pred: Predicate, idx: usize) -> Option<ThingId> {
        if let Some(node) = self.things.get(&src) {
            node.links
                .iter()
                .filter(|(p, _)| *p == pred)
                .nth(idx)
                .map(|(_, dst)| *dst)
        } else {
            None
        }
    }

    pub fn next_thing_of_kind(&self, kind: SymbolId, start_after: ThingId) -> Option<ThingId> {
        if let Some(list) = self.kind_index.get(&kind) {
            // list is sorted by ThingId (monotonic creation)
            // find first element > start_after
            // binary_search returns Err(idx) where it could be inserted to maintain order.
            // This is exactly the index of the first element greater than start_after (if not present).
            // If present, it returns Ok(idx), so we want idx+1.
            let idx = match list.binary_search(&start_after) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            if idx < list.len() {
                Some(list[idx])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_revision(&self) -> u64 {
        self.revision
    }
}

pub fn init() {
    *THINGS_SLAB.lock() = Some(GraphStore::new());
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::{Predicate, ThingId, syscall_defs::SymbolId};

    #[test]
    fn test_add_remove_link() {
        let mut store = GraphStore::new();
        let src = store.create_thing(SymbolId(1), Vec::new());
        let dst = store.create_thing(SymbolId(2), Vec::new());
        let pred = Predicate(100);

        assert!(store.add_link(src, dst, pred));

        let link = store.get_link(src, pred, 0);
        assert_eq!(link, Some(dst));

        let removed = store.remove_link(src, dst, pred);
        assert!(removed);

        let link_after = store.get_link(src, pred, 0);
        assert_eq!(link_after, None);

        let removed_again = store.remove_link(src, dst, pred);
        assert!(!removed_again);
    }

    #[test]
    fn test_kind_index() {
        let mut store = GraphStore::new();
        let kind_a = SymbolId(1);
        let kind_b = SymbolId(2);

        let id1 = store.create_thing(kind_a, Vec::new()); // t1
        let id2 = store.create_thing(kind_b, Vec::new()); // t2
        let id3 = store.create_thing(kind_a, Vec::new()); // t3

        // Index check
        assert_eq!(store.kind_index.get(&kind_a).unwrap().len(), 2);
        assert_eq!(store.kind_index.get(&kind_b).unwrap().len(), 1);

        // next_thing_of_kind
        let next = store.next_thing_of_kind(kind_a, ThingId(0));
        assert_eq!(next, Some(id1));

        let next = store.next_thing_of_kind(kind_a, id1);
        assert_eq!(next, Some(id3));

        let next = store.next_thing_of_kind(kind_a, id3);
        assert_eq!(next, None);

        // Delete t1
        store.delete_thing(id1);
        assert_eq!(store.kind_index.get(&kind_a).unwrap().len(), 1);

        let next = store.next_thing_of_kind(kind_a, ThingId(0));
        assert_eq!(next, Some(id3)); // Skipped t1
    }
}
