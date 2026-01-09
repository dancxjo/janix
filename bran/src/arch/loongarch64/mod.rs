//! loongarch64 architecture-specific implementation.

use core::arch::asm;
use kernel::BootRuntime;

/// The BootRuntime implementation for loongarch64.
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
    
    // Stubs
    fn phys_memory_map(&self) -> &'static [kernel::PhysRange] { &[] }
    fn modules(&self) -> &'static [kernel::BootModuleDesc] { &[] }
    fn page_size(&self) -> usize { 4096 }
    fn kernel_virt_base(&self) -> u64 { 0xffffffff80000000 }
    fn phys_to_virt_offset(&self) -> u64 { 0 }
    fn framebuffer(&self) -> Option<kernel::FramebufferInfo> { None }
    fn cpu_count(&self) -> usize { 1 }
    fn boot_cpu_id(&self) -> usize { 0 }
}

/// Serial port implementation for loongarch64 using NS16550A-compatible UART.
pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self {
        Self
    }
}

impl SerialPort {
    fn putchar(&self, c: u8) {
        unsafe {
            // LoongArch QEMU virt machine UART base (NS16550A compatible)
            let base = 0x1fe001e0 as *mut u8;
            base.write_volatile(c);
        }
    }
}

/// Halt and catch fire - enters an infinite idle loop.
pub fn hcf() -> ! {
    loop {
        unsafe { asm!("idle 0") };
    }
}
