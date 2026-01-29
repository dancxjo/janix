use abi::schema::{keys as props, rels};
use abi::types::Edge;
use abi::wire::ThingId;
use stem::thing::sys;

/// Trait to abstract graph reading so we can test compiler without syscalls
pub trait SvgGraph {
    fn get_root(&self, doc: ThingId) -> Option<ThingId>;
    fn get_children(&self, elem: ThingId) -> alloc::vec::Vec<ThingId>;
    fn get_prop_str(&self, node: ThingId, key: &str) -> Option<alloc::string::String>;

    // For attributes, we need to find HAS_ATTR linked nodes and read ATTR_NAME, ATTR_VALUE
    fn get_attributes(
        &self,
        elem: ThingId,
    ) -> alloc::vec::Vec<(alloc::string::String, alloc::string::String)>;
}

pub struct SysSvgGraph;

impl SvgGraph for SysSvgGraph {
    fn get_root(&self, doc: ThingId) -> Option<ThingId> {
        let mut edges = [Edge::default(); 16];
        if let Ok(count) = sys::get_edges(doc, &mut edges) {
            for i in 0..count {
                let e = &edges[i]; // TODO: Check relation ID. Relation is stored as edge kind/value?
                                   // Edge struct: target, relation, etc.
                                   // Need to check abi::types::Edge definition.
                                   // Assuming we can match relation roughly or iterate.
                                   // For now, simpler: describe_edge?
                                   // Actually, without proper edge filtering helper, this is verbose.
                                   // Let's stub for now since tests use TestGraph.
                let _ = e;
            }
        }
        None // TODO
    }

    fn get_children(&self, _elem: ThingId) -> alloc::vec::Vec<ThingId> {
        alloc::vec::Vec::new() // TODO
    }

    fn get_prop_str(&self, node: ThingId, key: &str) -> Option<alloc::string::String> {
        if let Ok(_val) = sys::prop_get(node, key) {
            // TODO: Resolve symbol or Bytespace
            None
        } else {
            None
        }
    }

    fn get_attributes(
        &self,
        _elem: ThingId,
    ) -> alloc::vec::Vec<(alloc::string::String, alloc::string::String)> {
        alloc::vec::Vec::new() // TODO
    }
}
