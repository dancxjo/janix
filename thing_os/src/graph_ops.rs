//! Helper types for graph operations.
//!
//! These types use `Vec` and are intended for internal use or higher-level abstractions,
//! not for the raw syscall boundary.

use alloc::vec::Vec;
use abi::{ThingId, syscall_defs::SymbolId, wire::graph::WirePropValue, Predicate};

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

#[derive(Debug, Clone, Copy)]
pub enum GraphEvent {
    ThingCreated(ThingId),
    ThingUpdated(ThingId),
    LinkAdded {
        src: ThingId,
        dst: ThingId,
        pred: Predicate,
    },
    BatchUpdateComplete,
}

#[derive(Debug, Clone)]
pub struct ThingProps {
    pub kind: SymbolId,
    pub props: Vec<(SymbolId, WirePropValue)>,
}

pub trait GraphSink {
    fn submit(&mut self, op: GraphOp) -> Result<(), &'static str>;
}

pub trait GraphDriver: GraphSink {
    fn subscribe(&mut self, kind: SymbolId, cb: fn(&GraphEvent));
    fn get_thing(&self, id: ThingId) -> Option<ThingProps>;
}
