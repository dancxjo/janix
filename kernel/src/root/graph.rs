use abi::root::{JournalOp, OpKind};
use alloc::collections::{BTreeMap, BTreeSet};

pub struct NodeRecord {
    // Placeholder for future properties
}

pub struct Graph {
    // "Nodes: BTreeMap<u64, NodeRecord>"
    nodes: BTreeMap<u64, NodeRecord>,
    
    // "Edges: BTreeMap<u64, EdgeRecord>"
    // Switched to implicit edges (src, dst) as permitted by prompt.
    // Storing as adjacency list for efficient cascade delete.
    // Key: Node ID, Value: Set of connected Node IDs (outgoing? undirected? implies directed src->dst)
    // PutEdge(src, dst).
    // Let's track outgoing edges: src -> Set<dst>
    // And maybe incoming for fast cleaning?
    // "DelNode: cascade delete all incident edges"
    // To do this efficiently, we need to find edges where `node` is src OR dst.
    
    // Map (src, dst) -> () to check existence/metadata
    edges: BTreeMap<(u64, u64), ()>,
    
    // Adjacency for cleanups
    outgoing: BTreeMap<u64, BTreeSet<u64>>,
    incoming: BTreeMap<u64, BTreeSet<u64>>,
}

impl Graph {
    pub const fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
            outgoing: BTreeMap::new(),
            incoming: BTreeMap::new(),
        }
    }

    pub fn apply(&mut self, op: &JournalOp) {
        match op.kind {
            OpKind::PutNode => {
                let id = op.a;
                // "PutNode: create if missing"
                self.nodes.entry(id).or_insert(NodeRecord {});
            }
            OpKind::PutEdge => {
                let src = op.a;
                let dst = op.b;
                // "PutEdge: create edge"
                // Implicit requirement: Nodes must exist? 
                // "Roots v0... No schemas... Graph is purely derived"
                // Usually graphs enforce node existence constraints.
                // "Graph is purely derived. The journal is the authority."
                // If I put an edge for non-existent nodes, does it work?
                // Let's assume yes, or we auto-create nodes? 
                // "PutNode: create if missing" implies we have to explicitly PutNode.
                // But "No dangling edges. Ever."
                // So we probably should check if nodes exist.
                // If not, we ignore? Or auto-create? 
                // "No dangling edges. Ever." implies we should probably ignore if nodes missing.
                
                if self.nodes.contains_key(&src) && self.nodes.contains_key(&dst) {
                    self.edges.insert((src, dst), ());
                    self.outgoing.entry(src).or_default().insert(dst);
                    self.incoming.entry(dst).or_default().insert(src);
                }
            }
            OpKind::DelNode => {
                let id = op.a;
                // "DelNode: cascade delete all incident edges"
                if self.nodes.remove(&id).is_some() {
                    // Remove all edges where id is src
                    if let Some(dsts) = self.outgoing.remove(&id) {
                        for dst in dsts {
                            self.edges.remove(&(id, dst));
                            if let Some(srcs) = self.incoming.get_mut(&dst) {
                                srcs.remove(&id);
                            }
                        }
                    }
                    
                    // Remove all edges where id is dst
                    if let Some(srcs) = self.incoming.remove(&id) {
                        for src in srcs {
                            self.edges.remove(&(src, id));
                            if let Some(dsts) = self.outgoing.get_mut(&src) {
                                dsts.remove(&id);
                            }
                        }
                    }
                }
            }
            OpKind::DelEdge => {
                let src = op.a;
                let dst = op.b;
                // "DelEdge: remove edge if present"
                if self.edges.remove(&(src, dst)).is_some() {
                    if let Some(dsts) = self.outgoing.get_mut(&src) {
                        dsts.remove(&dst);
                    }
                    if let Some(srcs) = self.incoming.get_mut(&dst) {
                        srcs.remove(&src);
                    }
                }
            }
        }
    }
}
