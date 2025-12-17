#![no_std]
#![no_main]

use thing_os::prelude::*;

#[thing_os::main]
fn main() {
    println!("Hello, World! I am a freestanding userland app!");
    sleep_ms(1000);
    println!("I slept for 1 second.");
}
