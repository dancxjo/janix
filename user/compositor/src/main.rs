#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
fn main() {
    let mut sys = runtime::UserlandSys::new();
    compositor::main(&mut sys);
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
