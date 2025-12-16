#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

#[cfg(target_os = "none")]
use runtime::UserlandSys;

#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
fn main() {
    let mut sys = UserlandSys::new();
    ps2_mouse_driver::main(&mut sys);
}

#[cfg(not(target_os = "none"))]
fn main() {
    panic!("ps2_mouse_driver must run inside ThingOS");
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
