#![no_std]
#![no_main]

#[cfg(target_os = "none")]
extern crate alloc;

// Panic handler is provided by thing_os::panic

#[thing_os::main]
fn main() {
    ps2_mouse_driver::driver_main();
}
