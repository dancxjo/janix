//! aarch64 architecture-specific implementation.

use core::arch::asm;
use kernel::BootRuntime;

/// Serial port implementation for aarch64 using PL011 UART.
pub struct SerialPort;

impl BootRuntime for SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // PL011 base address is 0x09000000 for qemu-virt
            let base = 0x09000000 as *mut u8;
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
