use core::panic::PanicInfo;
use crate::syscall::{debug_write, exit};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let _ = debug_write("PANIC encountered via stem::panic!\n");
    // TODO: formatting support in v0 is minimal.
    // If we had a way to format, we'd print file/line here.
    exit(101)
}
