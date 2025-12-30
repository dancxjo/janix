#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![allow(unused)]

extern crate alloc;

pub mod boot;
pub mod bridge;
pub mod bringup;
pub mod entry;
pub mod early_log;
pub mod heap;
pub mod loader;
pub mod paging;
pub mod simd;
pub mod memory_intrinsics;

// Re-export key items for convenience
pub use bridge::{Bridge, TICK_HOOK, PAGE_FAULT_HOOK, set_tick_hook, set_page_fault_hook, HHDM_OFFSET};
pub use entry::FRAMEBUFFER_INFO;
pub use early_log::print_hex;

pub struct Arch;

impl Arch {
    pub fn boot() -> ! {
        entry::arch_entry()
    }
}
