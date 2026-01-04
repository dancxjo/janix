#![no_std]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]

extern crate alloc;

pub mod boot;
pub mod bytespace;
pub mod interrupt;
pub mod log;
pub mod machine;
pub mod memory;
pub mod platform;
pub mod proc;
pub mod place;
pub mod sched;
pub mod serial;
pub mod syscall;
pub mod time;
pub mod trap;
pub mod watch;

pub use machine::PreBootInfo;

/// Global panic handler
struct SerialWriter;

impl core::fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::serial::write(s.as_bytes());
        Ok(())
    }
}

// Global panic handler removed from lib to avoid duplicate lang item in Bran
// It should be provided by the final binary (Bran for boot, apps for userland)

// alloc_error_handler in memory/allocator.rs
