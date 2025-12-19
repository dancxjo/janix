use alloc::vec::Vec;
use hashbrown::HashMap;
use abi::{ThingId, PropValue, Predicate, Link, syscall_defs::SymbolId};
use spin::Mutex;
use alloc::sync::Arc;


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
   Both
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
        }
    }
}

pub struct GraphStore {
    pub things: HashMap<ThingId, ThingNode>,
    pub next_id: u64,
    // Add free_indices to support resident manager slot reuse logic (stub)
    // Actually resident manager uses slab.slots directly.
    // GraphStore is a wrapper around generic logic.
    // But resident/manager.rs calls store::things_slab() which returns a Locked Slab?
    // Wait. resident/manager.rs line 96: `store::things_slab()`.
    // usage in serialize.rs: store::archive_store()
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
    // expose slots for resident/manager logic compatibility if possible?
    // resident/manager.rs casts return of things_slab() to something with `slots` and `alloc()`.
    // My GraphStore struct has `things: HashMap`.
    // resident/manager.rs expects a SLAB (Vec/Array).
    // This is a MAJOR mismatch.
    // I replaced Slab with HashMap in ABI refactor.
    // But resident manager was not updated.
    // I must either update resident manager to use HashMap API OR restore Slab.
    // Given the time, updating resident manager to use `insert(id, node)` is better than rewriting Store to Slab.
    // But resident/manager touches `slots` directly.
    // I will mock `slots` or fix resident manager.
    // resident/manager.rs:
    // `slab.alloc()` -> `(idx, gen)`
    // `slab.slots[idx]` access.
    // This expects `Slab<ThingNode>`.
    
    // I SHOULD probably revert GraphStore to use Slab if I want minimal changes to resident manager.
    // But HashMap is cleaner for `ThingId`.
    // Let's modify resident manager to use `create_thing` API?
    // resident_alloc uses `ThingNode::new_resident`.
    // I should add `create_resident_thing` to `GraphStore`.
    
    // BUT resident manager accesses `slab.slots` directly.
    // I will rewrite resident/manager.rs to use `GraphStore` API.
    // This means `sys_resident_alloc` calls `store.create_resident(...)`.
    
    // First, let's fix ThingNode definition.
    
    pub fn new() -> Self {
        Self {
            things: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_thing(&mut self, kind: SymbolId, props: Vec<(SymbolId, PropValue)>) -> ThingId {
        let id = ThingId(self.next_id);
        self.next_id += 1;
        
        let node = ThingNode {
            id,
            kind,
            kind_id: kind,
            props,
            resident: None,
            owner_process: None,
            storage: StorageState::Resident, // Default
            archived_ref: None,
        };
        
        self.things.insert(id, node);
        id
    }
    
    pub fn create_resident(&mut self, kind: SymbolId, resident: ResidentRef, pid: ProcessId) -> ThingId {
        let id = ThingId(self.next_id);
        self.next_id += 1;
         let node = ThingNode {
            id,
            kind,
            kind_id: kind,
            props: Vec::new(),
            resident: Some(resident),
            owner_process: Some(pid),
            storage: StorageState::Resident,
            archived_ref: None,
        };
        self.things.insert(id, node);
        id
    }

    pub fn update_thing(&mut self, id: ThingId, props: Vec<(SymbolId, PropValue)>) -> bool {
        if let Some(node) = self.things.get_mut(&id) {
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
        self.things.get_mut(&id)
    }

    pub fn delete_thing(&mut self, id: ThingId) -> Option<ThingNode> {
        self.things.remove(&id)
    }

    pub fn get_thing_kind(&self, id: ThingId) -> Option<SymbolId> {
        self.things.get(&id).map(|node| node.kind)
    }

    pub fn get_prop(&self, id: ThingId, key: SymbolId) -> Option<PropValue> {
        self.things.get(&id).and_then(|node| {
            node.props.iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v.clone())
        })
    }
    
    // Placeholder links implementation until we fully port links
    pub fn add_link(&mut self, src: ThingId, dst: ThingId, pred: Predicate) -> bool {
        // TODO: Implement actual link storage
        true
    }
    
    pub fn get_link(&self, src: ThingId, pred: Predicate, idx: usize) -> Option<ThingId> {
         // TODO: Implement actual link storage
        None
    }
}

pub fn init() {
    *THINGS_SLAB.lock() = Some(GraphStore::new());
}
