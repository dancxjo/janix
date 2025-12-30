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

pub struct Arch;

impl Arch {
    pub fn boot() -> ! {
        entry::arch_entry()
    }
}
