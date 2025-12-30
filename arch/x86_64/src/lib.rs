#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![allow(unused)]

extern crate alloc;

pub mod boot;
pub mod bridge;
pub mod bringup;
pub mod early_log;
pub mod entry;
pub mod heap;
pub mod loader;
pub mod memory_intrinsics;
pub mod paging;
pub mod simd;

// Re-export key items for convenience
pub use bridge::{
    set_page_fault_hook, set_tick_hook, Bridge, HHDM_OFFSET, PAGE_FAULT_HOOK, TICK_HOOK,
};
pub use early_log::print_hex;
pub use entry::FRAMEBUFFER_INFO;

pub struct Arch;

impl Arch {
    pub fn boot() -> ! {
        entry::arch_entry()
    }
}
