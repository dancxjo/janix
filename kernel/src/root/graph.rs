use super::resources::ResourceHandle;
use abi::symbols::SymbolId;
use alloc::collections::BTreeMap;

use core::sync::atomic::AtomicU64;
use alloc::collections::VecDeque;

pub type ThingId = u64;

pub struct Node {
    pub kind: SymbolId,
    pub props: BTreeMap<SymbolId, u64>,
    pub resource: Option<ResourceHandle>,
    pub watches: alloc::vec::Vec<(u64, ThingId)>,
    // Edges: list of (RelKind, Target)
    pub edges: alloc::vec::Vec<(SymbolId, ThingId)>,
}

pub struct Commit {
    pub seq: u64,
    pub data: alloc::vec::Vec<u8>,
}

pub struct GlobalWatch {
    pub id: u64,
    pub spec_ptr: u64, // We store the pointer to user query for now
    pub stream_handle: ResourceHandle, 
    pub kind_filter: SymbolId, // "kind == Bytespace"
    pub missing_fact: SymbolId, // "missing fact(detector=...)"
    
    // Batching support
    pub next_seq: u64,
    pub pending: VecDeque<Commit>,
    pub pending_bytes: usize,
    pub overflowed: bool,
}

// Limits
pub const MAX_PENDING_COMMITS: usize = 256;
pub const MAX_PENDING_BYTES: usize = 8 * 1024 * 1024; // 8 MiB

pub struct Graph {
    pub nodes: BTreeMap<ThingId, Node>,
    pub next_id: ThingId,
    pub root_seq: AtomicU64,
    pub kind_index: BTreeMap<SymbolId, alloc::vec::Vec<ThingId>>,
    pub global_watches: BTreeMap<u64, GlobalWatch>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            next_id: 1,
            root_seq: AtomicU64::new(0),
            kind_index: BTreeMap::new(),
            global_watches: BTreeMap::new(),
        }
    }

    pub fn alloc(&mut self, kind: SymbolId) -> ThingId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(
            id,
            Node {
                kind,
                props: BTreeMap::new(),
                resource: None,
                watches: alloc::vec::Vec::new(),
                edges: alloc::vec::Vec::new(),
            },
        );

        self.kind_index.entry(kind).or_default().push(id);

        id
    }

    pub fn get_kind(&self, id: ThingId) -> Option<SymbolId> {
        self.nodes.get(&id).map(|n| n.kind)
    }

    pub fn get_resource(&self, id: ThingId) -> Option<ResourceHandle> {
        self.nodes.get(&id).and_then(|n| n.resource.clone())
    }

    pub fn get_node_mut(&mut self, id: ThingId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn link(&mut self, src: ThingId, rel: SymbolId, dst: ThingId) {
        if let Some(node) = self.nodes.get_mut(&src) {
            node.edges.push((rel, dst));
        }
    }
}
