//! Canonical Graph Types Registry
//! 
//! This module defines the strict, packed, pointer-free structs used in the Thing-OS Graph ABI.

pub mod system; // Legacy/Syscall types (u64 based for now, slowly migrating)
pub use system::*;

// Re-export Wire IDs
pub use crate::wire::{ThingId, BlobId, KindId, SymbolId, PredicateId};
// Re-export Adapter
pub use crate::ids::HandleId;

// Graph Atoms
pub mod edge;
pub mod kind;
pub mod predicate;

// Logic/System
pub mod log_event;
pub mod thread;
pub mod process;
pub mod task;

// Assets
pub mod asset;
pub mod font;
pub mod window;

// Time
pub mod instant;

// Exports
pub use edge::Edge;
pub use kind::Kind;
pub use predicate::Predicate;
pub use log_event::LogEvent;
pub use thread::Thread;
pub use process::Process;
pub use task::Task;
pub use asset::Asset;
pub use font::Font;
pub use window::Window;
pub use instant::{Instant, Duration};
