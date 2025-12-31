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

pub mod arch;
pub mod boot;
pub mod graph;
pub mod log;
pub mod machine;
pub mod sched;
pub mod serial;
pub mod symbols;
pub mod syscall;

// Stub modules
pub mod bytespace;
pub mod caps;
pub mod proc;

// Re-export the main entry point
pub use boot::boot;
