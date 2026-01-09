//! riscv64 architecture-specific implementation.

use core::arch::asm;
use kernel::BootRuntime;

/// Serial port implementation for riscv64 using QEMU virt UART.
pub struct SerialPort;

impl BootRuntime for SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // QEMU virt machine UART base address
            let base = 0x10000000 as *mut u8;
            base.write_volatile(c);
        }
    }

    fn halt(&self) -> ! {
        hcf()
    }
}

/// Halt and catch fire - enters an infinite wait-for-interrupt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}
