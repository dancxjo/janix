#[cfg(not(test))]
use crate::syscall::{debug_write, exit};
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let _ = debug_write("STEM PANIC\n", 11);
    exit(101)
}
