use alloc::vec::Vec;
use crate::{PropKey, PropValue, ThingId, Link};

pub type Prop = (PropKey, PropValue);

#[derive(Debug, Clone)]
pub enum GraphOp {
    CreateThing {
        kind: &'static str,
        props: Vec<Prop>,
    },
    UpdateProps {
        id: ThingId,
        props: Vec<Prop>,
    },
}

#[derive(Debug, Clone)]
pub enum GraphEvent {
    ThingCreated {
        id: ThingId,
        kind: &'static str,
        kind_id: ThingId,
    },
    ThingDeleted {
        id: ThingId,
        kind: &'static str,
        kind_id: ThingId,
    },
    PropUpdated {
        id: ThingId,
        kind: &'static str,
        kind_id: ThingId,
        key: &'static str,
        old: Option<PropValue>,
        new: PropValue,
    },
    LinkAdded(Link),
    LinkRemoved(Link),
}

pub trait GraphSink {
    fn submit(&mut self, op: GraphOp) -> Result<(), &'static str>;
}

// Helper struct for query results (simplified version of ThingGetSyscallResult logic)
#[derive(Debug, Clone)]
pub struct ThingProps {
    // We can use a callback or return a simple vector for kernel-internal use
    pub props: Vec<(PropKey, PropValue)>,
}

pub trait GraphDriver: GraphSink {
    fn subscribe(&mut self, kind: &'static str, handler: fn(&GraphEvent));
    fn get_thing(&self, id: ThingId) -> Option<ThingProps>;
}
