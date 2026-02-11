//! hello_std: Proof-of-concept for vendored Rust standard library.
//!
//! This app demonstrates that std compiles and links against Thing-OS
//! using our vendored, patched Rust source tree.

#![feature(restricted_std)]
#![no_main]

extern crate std;
extern crate alloc;

use std::string::String;
use std::vec;
use stem::info;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("hello_std: running with REAL std linked!");

    // Prove std::string works
    let greeting = String::from("Hello from std::String on ThingOS!");
    info!("hello_std: {}", greeting);

    // Prove std::vec works
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let sum: i32 = nums.iter().sum();
    info!("hello_std: sum of {:?} = {}", nums, sum);

    // Prove std::format! works
    let formatted = std::format!("std::format! works: 2 + 2 = {}", 2 + 2);
    info!("hello_std: {}", formatted);

    // Prove core::any still works through std
    info!(
        "hello_std: type_name::<String>() = {}",
        core::any::type_name::<String>()
    );

    info!("hello_std: std is alive on ThingOS!");

    loop {
        stem::yield_now();
    }
}
