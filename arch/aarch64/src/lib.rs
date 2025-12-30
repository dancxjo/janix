#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
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
pub use bridge::{Bridge, TICK_HOOK, PAGE_FAULT_HOOK, set_tick_hook, set_page_fault_hook};

/// Boot log macro for early boot messages.
#[macro_export]
macro_rules! bootlog {
    ($($arg:tt)*) => {{
        use kernel::bridge::HardwareBridge;
        $crate::bridge::Bridge.log(alloc::format!($($arg)*).as_str());
        $crate::bridge::Bridge.log("\n");
    }}
}

pub struct Arch;

impl Arch {
    pub fn boot() -> ! {
        entry::arch_entry()
    }
}
