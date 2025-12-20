//! Helper types for graph operations. Not syscall-visible.
//!
//! These types use `Vec` and are intended for internal kernel use or higher-level abstractions,
//! not for the raw syscall boundary.

use alloc::vec::Vec;
use crate::{ThingId, syscall_defs::SymbolId, wire::graph::WirePropValue};

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
        pred: crate::Predicate,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum GraphEvent {
    ThingCreated(ThingId),
    ThingUpdated(ThingId),
    LinkAdded {
        src: ThingId,
        dst: ThingId,
        pred: crate::Predicate,
    },
    BatchUpdateComplete,
}
