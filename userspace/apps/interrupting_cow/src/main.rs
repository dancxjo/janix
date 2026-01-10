#![no_std]
#![no_main]

use standard::prelude::*;
// use stem::sys_yield; // We avoid yield to test preemption!

#[no_mangle]
pub fn main() -> i32 {
    println!("interrupting_cow: starting CPU bound loop...");
    
    let mut i: u64 = 0;
    loop {
        // Print every ~10M iterations to avoid flooding serial but show liveness
        if i % 10_000_000 == 0 {
            println!("interrupting_cow: moo {}", i);
        }
        i = i.wrapping_add(1);
        
        // Burn CPU
        core::hint::black_box(());
    }
}
