#[cfg(not(test))]
use crate::syscall::{debug_write, exit};
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let _ = debug_write("PANIC encountered via stem::panic!\n", 1);
    // TODO: formatting support in v0 is minimal.
    // If we had a way to format, we'd print file/line here.
    exit(101)
}
