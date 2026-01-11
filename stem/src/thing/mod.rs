pub mod id;
pub mod kind;
pub mod sys;
pub mod ref_;
pub mod keys;

pub use id::ThingId;
pub use kind::ThingKind;
pub use ref_::ThingRef;

pub trait Thing {
    const KIND: ThingKind;
}

pub use sys::try_typed;
pub mod debug;
pub use debug::DebugThing;
pub mod graph_debug;
pub use graph_debug::{DebugEdge, edge};
pub mod symbol;
pub mod query;
