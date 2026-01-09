//! loongarch64 architecture-specific implementation.

use core::arch::asm;
use kernel::BootRuntime;

/// Serial port implementation for loongarch64 using NS16550A-compatible UART.
pub struct SerialPort;

impl BootRuntime for SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // LoongArch QEMU virt machine UART base (NS16550A compatible)
            let base = 0x1fe001e0 as *mut u8;
            base.write_volatile(c);
        }
    }

    fn halt(&self) -> ! {
        hcf()
    }
}

/// Halt and catch fire - enters an infinite idle loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("idle 0") };
    }
}
