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
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

extern crate alloc;


pub mod boot;
pub mod log;
pub mod machine;
pub mod platform;
pub mod place;
pub mod sched;
pub mod serial;
pub mod syscall;
pub mod trap;
pub mod memory;

// Stub modules

pub mod caps;
pub mod proc;

// Re-export the main entry points
pub use boot::{boot, pre_boot};
pub use machine::PreBootInfo;
