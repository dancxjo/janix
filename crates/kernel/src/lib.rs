#![no_std]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]
#![feature(alloc_error_handler)]


extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod boot;
pub mod boot_grants;
pub mod bytespace;
pub mod interrupt;
pub mod log;
pub mod machine;
pub mod memory;
pub mod graph_api;
pub mod platform;
pub mod proc;
pub mod seeding;
pub mod sched;
pub mod serial;
pub mod syscall;
pub mod time;
pub mod trap;
pub mod watch;

pub use machine::PreBootInfo;




// Global panic handler removed from lib to avoid duplicate lang item in Bran
// It should be provided by the final binary (Bran for boot, apps for userland)

// alloc_error_handler in memory/allocator.rs
