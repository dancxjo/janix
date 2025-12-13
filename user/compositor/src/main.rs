#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
use thing_os::entry;

#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    entry(|sys| compositor::run(sys));
}

#[cfg(not(target_os = "none"))]
fn main() {
    panic!("compositor must run inside ThingOS");
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
