//! riscv64 16550 UART driver with SBI fallback

use core::arch::asm;
use core::ptr;
use core::sync::atomic::{AtomicU64, Ordering};

static BASE: AtomicU64 = AtomicU64::new(0);

pub struct Serial;

impl Serial {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(&self, base: u64) {
        BASE.store(base, Ordering::Relaxed);

        // Only try to init hardware if we have a valid base
        if base != 0 {
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
    }

    fn base(&self) -> u64 {
        BASE.load(Ordering::Relaxed)
    }

    /// Output a character via SBI legacy console_putchar
    /// This uses the SBI firmware which should always work
    fn sbi_putchar(c: u8) {
        unsafe {
            // SBI legacy console_putchar: EID = 0x01, FID = 0x00
            // Character goes in a0
            asm!(
                "li a7, 0x01",  // EID for console_putchar
                "li a6, 0x00",  // FID (not used for legacy but set anyway)
                "ecall",
                in("a0") c as usize,
                out("a7") _,
                out("a6") _,
                options(nostack)
            );
        }
    }

    pub fn putc(&self, c: u8) {
        // Use SBI console for reliable output on RISC-V
        Self::sbi_putchar(c);
    }

    pub fn write(&self, bytes: &[u8]) {
        for &b in bytes {
            self.putc(b);
        }
    }
}
