#![no_std]
#![no_main]

use stem::println;
use stem::syscall::{yield_now, sleep_ms, spawn_thread};

#[link_section = ".text.entry"]
#[no_mangle]
extern "C" fn _start() -> ! {
    println!("THREADS: starting");

    // Spawn Thread B
    // Stack grows down, so we pass the end of the array.
    let stack_top = {
        (core::ptr::addr_of_mut!(THREAD_B_STACK) as usize) + 16384
    };

    match spawn_thread(thread_b, stack_top) {
        Ok(tid) => println!("Spawned thread B with ID {}", tid),
        Err(e) => println!("Failed to spawn thread B: {:?}", e),
    }

    // Become Thread A (this _start function)
    let mut i: usize = 0;
    loop {
        println!("A: tick {}", i);
        i = i.wrapping_add(1);
        yield_now();
        sleep_ms(1000);
    }
}

#[no_mangle]
extern "C" fn thread_b() -> ! {
    let mut i: usize = 0;
    loop {
        println!("B: tick {}", i);
        i = i.wrapping_add(1);
        yield_now();
        sleep_ms(1000);
    }
}

// Stack for Thread B
// We need a decent size stack. 16KB is plenty.
#[repr(align(16))]
#[allow(dead_code)]
struct Stack([u8; 16384]);
static mut THREAD_B_STACK: Stack = Stack([0; 16384]);

