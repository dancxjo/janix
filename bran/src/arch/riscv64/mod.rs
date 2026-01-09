//! riscv64 architecture-specific implementation.

use core::arch::asm;
use kernel::BootRuntime;

/// The BootRuntime implementation for riscv64.
pub struct Runtime {
    serial: SerialPort,
}

impl Runtime {
    pub const fn new() -> Self {
        Self {
            serial: SerialPort,
        }
    }
}

impl BootRuntime for Runtime {
    fn putchar(&self, c: u8) {
        self.serial.putchar(c);
    }

    fn halt(&self) -> ! {
        hcf()
    }
}

/// Serial port implementation for riscv64 using QEMU virt UART.
pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self {
        Self
    }
}

impl SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // QEMU virt machine UART base address
            let base = 0x10000000 as *mut u8;
            base.write_volatile(c);
        }
    }
}

/// Halt and catch fire - enters an infinite wait-for-interrupt loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}
