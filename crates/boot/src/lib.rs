#![no_std]
#![feature(used_with_arg)]

extern crate alloc;

pub mod bootinfo;

#[cfg(feature = "limine")]
pub mod limine;

pub use bootinfo::*;
