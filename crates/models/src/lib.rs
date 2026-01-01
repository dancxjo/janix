#![no_std]

use abi::ids::{SymbolId, ThingId};

/// A Place is a container for Things.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaceBody {
    pub name: SymbolId,
}

/// A Relationship is a directed link between two Things with a semantic predicate.
/// Note: Relationship is itself a Thing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RelationshipBody {
    pub from: ThingId,
    pub to: ThingId,
    pub predicate: SymbolId,
}
