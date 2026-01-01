#![no_std]
#![no_main]

use core::panic::PanicInfo;
extern crate alloc;

use thing_std::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("SPROUT: I am alive! (and exiting)");
    
    // Exit immediately to trigger kernel panic
    thing_std::sys_exit(1);
}
