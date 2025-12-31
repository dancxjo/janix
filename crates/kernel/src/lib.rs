//! ThingOS Kernel Core (v0.3)
//!
//! This crate provides the arch-neutral core runtime:
//! - Machine/Architecture/ModuleProvider boundary traits
//! - Boot sequence orchestration
//! - Logging subsystem (graph-native)
//! - Symbol interning
//! - Graph store (Things + Links)
//! - Syscall dispatch
//! - Scheduler

#![no_std]

extern crate alloc;

pub mod machine;
pub mod boot;
pub mod log;
pub mod symbols;
pub mod graph;
pub mod syscall;
pub mod sched;
pub mod arch;
pub mod serial;


// Stub modules
pub mod proc;
pub mod bytespace;
pub mod caps;

// Re-export the main entry point
pub use boot::boot;
