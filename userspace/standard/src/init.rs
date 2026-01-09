use crate::console::{ConsoleSink, set_console_sink};
use stem::sys_debug_putchar;

struct UserConsoleSink;

impl ConsoleSink for UserConsoleSink {
    fn putchar(&self, c: u8) {
        unsafe {
            sys_debug_putchar(c);
        }
    }
}

static CONSOLE: UserConsoleSink = UserConsoleSink;

#[no_mangle]
pub unsafe extern "C" fn __standard_init() {
    set_console_sink(&CONSOLE);
}
