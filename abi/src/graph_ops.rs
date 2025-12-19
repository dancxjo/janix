use alloc::vec::Vec;
use crate::{ThingId, wire::graph::WirePropValue, syscall_defs::SymbolId};

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
