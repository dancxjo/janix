//! loongarch64 16550 UART driver

use core::ptr;

const UART_BASE: usize = 0x1fe001e0;

pub struct Serial;

impl Serial {
    pub const fn new() -> Self {
        Self
    }

    // Blind write for early diagnostics - prevents hangs if UART is unmapped or busy
    pub fn putc(&self, c: u8) {
        // No wait loop - blind fire
        unsafe {
            ptr::write_volatile((UART_BASE + 0x00) as *mut u8, c);
        }
    }

    pub fn write(&self, bytes: &[u8]) {
        for &b in bytes {
            self.putc(b);
            // Optional: crude delay if needed, but usually redundant with blind write
        }
    }
}
