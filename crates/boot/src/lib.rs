#![no_std]
#![feature(used_with_arg)]

extern crate alloc;

pub mod bootinfo;
pub mod limine;

pub use limine::{collect, get_hhdm, get_rsdp, get_cmdline, get_memory_map};
pub use bootinfo::*;
