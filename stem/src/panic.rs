#[cfg(all(not(test), any(target_os = "none", target_os = "thingos")))]
use crate::syscall::{debug_write, exit};
#[cfg(all(not(test), any(target_os = "none", target_os = "thingos")))]
use core::panic::PanicInfo;

#[cfg(all(not(test), any(target_os = "none", target_os = "thingos")))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let _ = debug_write("STEM PANIC\n", 11);
    exit(101)
}
