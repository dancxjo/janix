//! loongarch64 16550 UART driver

use core::ptr;

const UART_BASE: usize = 0x1fe001e0;

pub struct Serial;

impl Serial {
    pub const fn new() -> Self {
        Self
    }

    fn is_transmit_ready(&self) -> bool {
        unsafe {
            let lsr = ptr::read_volatile((UART_BASE + 0x05) as *const u8);
            (lsr & 0x20) != 0
        }
    }

    pub fn putc(&self, c: u8) {
        while !self.is_transmit_ready() {
            core::hint::spin_loop();
        }
        unsafe {
            ptr::write_volatile((UART_BASE + 0x00) as *mut u8, c);
        }
    }

    pub fn write(&self, bytes: &[u8]) {
        for &b in bytes {
            self.putc(b);
        }
    }
}
