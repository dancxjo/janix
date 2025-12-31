#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
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
pub mod user_space;

// Re-export key items for convenience
pub use bridge::{set_page_fault_hook, set_tick_hook, Bridge, PAGE_FAULT_HOOK, TICK_HOOK};

/// Boot log macro for early boot messages.
#[macro_export]
macro_rules! bootlog {
    ($($arg:tt)*) => {{
        use kernel::bridge::CpuBridge;
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
