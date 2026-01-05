#![no_std]
#![no_main]

extern crate alloc; // Might be needed for hidden alloc/panic handlers if not pulled in transitively properly, but usually thing_std handles it.
// Actually thing_std usually exports what's needed.

use thing_std::*; // For init? No, bloom::run does init.
// But _start needs to call main. thing_std provides _start.

#[no_mangle]
pub extern "C" fn main() {
    bloom::run();
}
