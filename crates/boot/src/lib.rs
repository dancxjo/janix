#![no_std]
#![feature(used_with_arg)]

extern crate alloc;

pub mod bootinfo;
pub mod limine;

pub use limine::collect;
pub use bootinfo::*;
