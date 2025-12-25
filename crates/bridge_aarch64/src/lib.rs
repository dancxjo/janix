#![no_std]

use hw::HardwareBridge;
use core::arch::asm;

pub struct Bridge;

impl HardwareBridge for Bridge {
    fn log(&self, msg: &str) {
        // PL011 UART at 0x09000000 (QEMU virt default)
        const UART0: *mut u8 = 0x09000000 as *mut u8;
        for b in msg.bytes() {
            unsafe {
                // Wait for UART to be ready check excluded for simplicity in "thin crust"
                core::ptr::write_volatile(UART0, b);
            }
        }
    }

    fn ticks(&self) -> u64 {
        0 // TODO
    }

    fn idle(&self) {
        unsafe { asm!("wfi"); }
    }

    fn shutdown(&self) -> ! {
        // PSCI SYSTEM_OFF or QEMU semihosting logic could go here.
        // For now, loop forever.
        loop {
            unsafe { asm!("wfi"); }
        }
    }

    fn irq_disable(&self) {
        unsafe { asm!("msr daifset, #2"); }
    }

    fn irq_enable(&self) {
        unsafe { asm!("msr daifclr, #2"); }
    }
}
