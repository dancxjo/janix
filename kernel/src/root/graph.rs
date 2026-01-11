use abi::kinds::ThingKind;
use alloc::collections::BTreeMap;
use super::resources::ResourceHandle;

pub type ThingId = u64;

pub struct Node {
    pub kind: ThingKind,
    pub props: BTreeMap<u64, u64>,
    pub resource: Option<ResourceHandle>,
    // Barebones watch list: Vec<(u64 mask, stream_id)>
    // If we have many watches, this needs to be better.
    pub watches: alloc::vec::Vec<(u64, ThingId)>,
    // Edges format: (direction, rel, other_node_id)
    // For v0.1: just outgoing edges in a list.
    // Actually, prompt says: "node graph edge representation (adjacency list? map rel->vec ids?)"
    // We will use Vec<(RelKey, ThingId)> for outgoing edges.
    pub edges: alloc::vec::Vec<(abi::kinds::RelKey, ThingId)>, 
}

pub struct Graph {
    pub nodes: BTreeMap<ThingId, Node>,
    pub next_id: ThingId,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: BTreeMap::new(), next_id: 1 }
    }
    
    pub fn alloc(&mut self, kind: ThingKind) -> ThingId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, Node {
             kind, 
             props: BTreeMap::new(), 
             resource: None,
             watches: alloc::vec::Vec::new(),
             edges: alloc::vec::Vec::new(),
        });
        id
    }
    
    pub fn get_kind(&self, id: ThingId) -> Option<ThingKind> {
        self.nodes.get(&id).map(|n| n.kind)
    }
    
    pub fn get_resource(&self, id: ThingId) -> Option<ResourceHandle> {
         self.nodes.get(&id).and_then(|n| n.resource.clone())
    }
    
    pub fn get_node_mut(&mut self, id: ThingId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn link(&mut self, src: ThingId, rel: abi::kinds::RelKey, dst: ThingId) {
        if let Some(node) = self.nodes.get_mut(&src) {
            node.edges.push((rel, dst));
        }
    }
}
