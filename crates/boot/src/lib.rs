#![no_std]
#![feature(used_with_arg)]

extern crate alloc;

pub mod bootinfo;
pub mod limine;

pub use bootinfo::*;
pub use limine::{collect, get_cmdline, get_hhdm, get_memory_map, get_rsdp};
