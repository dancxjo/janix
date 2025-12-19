#![no_std]
#![no_main]

extern crate alloc;

#[thing_os::main]
fn main() {
    usb::driver_main();
}
