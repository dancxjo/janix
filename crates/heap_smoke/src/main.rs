#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    log_info("HEAP SMOKE: PANIC!");
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("HEAP SMOKE: Starting...");
    
    // Allocate a vector
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);
        if i % 100 == 0 {
             log_info("HEAP SMOKE: Pushed 100 items...");
        }
    }
    
    log_info("HEAP SMOKE: Vector push success!");
    loop {
        sched_yield();
    }
}
