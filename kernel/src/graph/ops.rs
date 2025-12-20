//! Helper types for graph operations.
//!
//! These types use `Vec` and are intended for internal use or higher-level abstractions,
//! not for the raw syscall boundary.

use alloc::vec::Vec;
use abi::{ThingId, syscall_defs::SymbolId, wire::graph::WirePropValue, Predicate};
pub use abi::GraphEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum GraphOp {
    CreateThing {
        kind: SymbolId,
        props: Vec<(SymbolId, WirePropValue)>,
    },
    UpdateThing {
        id: ThingId,
        props: Vec<(SymbolId, WirePropValue)>,
    },
    AddLink {
        src: ThingId,
        dst: ThingId,
        pred: Predicate,
    },
}

