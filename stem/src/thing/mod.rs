pub mod id;
pub mod keys;
pub mod kind;
pub mod ref_;
pub mod sys;

pub use id::HandleId;
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
pub use graph_debug::{edge, DebugEdge};
pub mod query;
pub mod symbol;
pub mod discovery;
