#![no_std]
#![no_main]

use stem::syscall::{debug_write, sleep_ms, exit};

#[no_mangle]
pub fn main() -> i32 {
    let _ = debug_write(b"SPROUT: hello\n");

    loop {
        let _ = sleep_ms(1000);
        let _ = debug_write(b"SPROUT: still here...\n");
    }
    
    // Unreachable, but correct signature
    // 0
}
