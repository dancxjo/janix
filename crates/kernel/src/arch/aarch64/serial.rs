//! aarch64 PL011 UART driver

use core::ptr;
use core::sync::atomic::{AtomicU64, Ordering};

static BASE: AtomicU64 = AtomicU64::new(0x0900_0000);

pub struct Serial;

impl Serial {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(&self, base: u64) {
        BASE.store(base, Ordering::Relaxed);

        unsafe {
            // Bring PL011 up at 115200 8N1
            ptr::write_volatile((base + 0x30) as *mut u32, 0); // UARTCR: disable while configuring
            ptr::write_volatile((base + 0x44) as *mut u32, 0x7ff); // UARTICR: clear pending interrupts
            ptr::write_volatile((base + 0x24) as *mut u32, 13); // UARTIBRD: integer baud divisor
            ptr::write_volatile((base + 0x28) as *mut u32, 1); // UARTFBRD: fractional baud divisor
            ptr::write_volatile((base + 0x2c) as *mut u32, (3 << 5) | (1 << 4)); // UARTLCR_H: 8N1, FIFO enable
            ptr::write_volatile((base + 0x30) as *mut u32, (1 << 9) | (1 << 8) | 1);
            // UARTCR: enable TX/RX/UART
        }
    }

    fn base(&self) -> u64 {
        BASE.load(Ordering::Relaxed)
    }

    fn is_transmit_ready(&self) -> bool {
        unsafe {
            let fr = ptr::read_volatile((self.base() + 0x18) as *const u32);
            (fr & (1 << 5)) == 0
        }
    }

    pub fn putc(&self, c: u8) {
        while !self.is_transmit_ready() {
            core::hint::spin_loop();
        }
        unsafe {
            ptr::write_volatile((self.base() + 0x00) as *mut u32, c as u32);
        }
    }

    pub fn write(&self, bytes: &[u8]) {
        for &b in bytes {
            self.putc(b);
        }
    }
}
