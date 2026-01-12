use super::resources::ResourceHandle;
use abi::symbols::SymbolId;
use alloc::collections::BTreeMap;

pub type ThingId = u64;

pub struct Node {
    pub kind: SymbolId,
    pub props: BTreeMap<SymbolId, u64>,
    pub resource: Option<ResourceHandle>,
    pub watches: alloc::vec::Vec<(u64, ThingId)>,
    // Edges: list of (RelKind, Target)
    pub edges: alloc::vec::Vec<(SymbolId, ThingId)>,
}

pub struct Graph {
    pub nodes: BTreeMap<ThingId, Node>,
    pub next_id: ThingId,
    pub kind_index: BTreeMap<SymbolId, alloc::vec::Vec<ThingId>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            next_id: 1,
            kind_index: BTreeMap::new(),
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
