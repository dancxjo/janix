//! riscv64 16550 UART driver

use core::ptr;
use core::sync::atomic::{AtomicU64, Ordering};

static BASE: AtomicU64 = AtomicU64::new(0x1000_0000);

pub struct Serial;

impl Serial {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(&self, offset: u64) {
        let base = 0x1000_0000 + offset;
        BASE.store(base, Ordering::Relaxed);

        unsafe {
            // Bring 16550 up at 115200 8N1
            ptr::write_volatile((base + 1) as *mut u8, 0x00); // IER: disable interrupts
            ptr::write_volatile((base + 3) as *mut u8, 0x80); // LCR: enable DLAB
            ptr::write_volatile((base + 0) as *mut u8, 0x01); // DLL
            ptr::write_volatile((base + 1) as *mut u8, 0x00); // DLM
            ptr::write_volatile((base + 3) as *mut u8, 0x03); // LCR: 8N1
            ptr::write_volatile((base + 2) as *mut u8, 0x07); // FCR: enable FIFO, clear RX/TX
            ptr::write_volatile((base + 4) as *mut u8, 0x03); // MCR: assert DTR/RTS
        }
    }

    fn base(&self) -> u64 {
        BASE.load(Ordering::Relaxed)
    }

    fn is_transmit_ready(&self) -> bool {
        unsafe {
            let lsr = ptr::read_volatile((self.base() + 0x05) as *const u8);
            (lsr & 0x20) != 0
        }
    }

    pub fn putc(&self, c: u8) {
        while !self.is_transmit_ready() {
            core::hint::spin_loop();
        }
        unsafe {
            ptr::write_volatile((self.base() + 0x00) as *mut u8, c);
        }
    }

    pub fn write(&self, bytes: &[u8]) {
        for &b in bytes {
            self.putc(b);
        }
    }
}
