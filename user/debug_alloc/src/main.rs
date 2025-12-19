#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_os::prelude::*;

#[thing_os::main]
fn main() {
    println!("debug_alloc: start");

    // Basic heap exercises
    let mut v = Vec::<u64>::new();
    for i in 0..1024 {
        v.push(i);
    }
    println!("debug_alloc: pushed 1024");

    // Force a reallocation
    for i in 1024..16384 {
        v.push(i);
    }
    println!("debug_alloc: pushed 16k");

    println!("debug_alloc: done");
}
