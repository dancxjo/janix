//! ThingOS Kernel Core (v0.3)
//!
//! This crate provides the arch-neutral core runtime.

#![no_std]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod boot;
pub mod bytespace;
pub mod display;
pub mod log;
pub mod machine;
pub mod memory;
pub mod place;
pub mod platform;
pub mod sched;
pub mod serial;
pub mod syscall;
pub mod trap;

// Stub/New modules
pub mod proc;

pub use boot::{boot, pre_boot};
pub use machine::PreBootInfo;
