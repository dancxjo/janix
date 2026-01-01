#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("LOG SMOKE: Hello from userspace!");
    log_info("LOG SMOKE: This should appear in place.logs.");
    sys_exit(0);
}
