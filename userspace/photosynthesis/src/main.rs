#![feature(restricted_std)]
#![no_main]

extern crate stem;

use core::time::Duration;

#[stem::main]
fn main() -> ! {
    loop {
        stem::sleep(Duration::from_secs(1));
    }
}
