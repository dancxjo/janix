#![no_std]
#![no_main]

use stem::kprintln;
use stem::syscall::{yield_now, sleep_ms, spawn_thread};

#[link_section = ".text.entry"]
#[no_mangle]
extern "C" fn _start() -> ! {
    kprintln!("THREADS: starting");

    // Spawn Thread B
    // Stack grows down, so we pass the end of the array.
    let stack_top = unsafe {
        (THREAD_B_STACK.0.as_ptr() as usize) + THREAD_B_STACK.0.len()
    };

    match spawn_thread(thread_b, stack_top) {
        Ok(tid) => kprintln!("Spawned thread B with ID {}", tid),
        Err(e) => kprintln!("Failed to spawn thread B: {:?}", e),
    }

    // Become Thread A (this _start function)
    for i in 0..10 {
        kprintln!("A: tick {}", i);
        let _ = yield_now();
        let _ = sleep_ms(200);
    }
    loop { yield_now().ok(); }
}

#[no_mangle]
extern "C" fn thread_b() -> ! {
    for i in 0..10 {
        kprintln!("B: tick {}", i);
        let _ = yield_now();
        let _ = sleep_ms(200);
    }
    loop { yield_now().ok(); }
}

// Stack for Thread B
// We need a decent size stack. 16KB is plenty.
#[repr(align(16))]
struct Stack([u8; 16384]);
static mut THREAD_B_STACK: Stack = Stack([0; 16384]);

