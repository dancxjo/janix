//! Graph trait and high-level queries
//!
//! Provides the primary API for interacting with system state through
//! the Graph/Thing/Relationship ontology.
use abi::ids::{GraphId, RelationshipId, SymbolId, ThingId};
use alloc::vec::Vec;
use ::graph::store;
use ::graph::symbols;

/// Core API for world interaction
pub trait Graph {
    /// Create a new Thing within this graph
    fn thing_create(
        &mut self,
        kind: SymbolId,
        schema: SymbolId,
        version: u32,
        payload: &[u8],
    ) -> ThingId;

    /// Create a relationship between two Things
    fn relationship_create(
        &mut self,
        from: ThingId,
        to: ThingId,
        predicate: SymbolId,
    ) -> RelationshipId;

    /// Find relationships originating from a Thing
    fn relationships_from(&self, from: ThingId, predicate: Option<SymbolId>)
        -> Vec<RelationshipId>;

    /// Find relationships pointing to a Thing
    fn relationships_to(&self, to: ThingId, predicate: Option<SymbolId>) -> Vec<RelationshipId>;
}

/// Query which Things are contained within a graph
///
/// This is a derived query over relationships_from(graph, predicate.contains)
pub fn contained_in(graph: GraphId) -> Vec<ThingId> {
    let pred_contains = symbols::intern(b"predicate.contains");
    let rels = store::relationships_from(graph);

    let mut things = Vec::new();
    for &rel_id in rels.iter() {
        if let Some(rel) = store::get_relationship(rel_id) {
            if rel.kind == pred_contains {
                things.push(rel.to);
            }
        }
    }
    things
}
