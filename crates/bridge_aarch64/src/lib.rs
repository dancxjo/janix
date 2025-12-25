#![no_std]

#[cfg(target_arch = "aarch64")]
use core::arch::asm;
use hw::HardwareBridge;

pub struct Bridge;

#[cfg(target_arch = "aarch64")]
impl HardwareBridge for Bridge {
    fn log(&self, msg: &str) {
        // PL011 UART at 0x09000000 (QEMU virt default)
        const UART0: *mut u8 = 0x09000000 as *mut u8;
        // FR Register offset 0x18. TXFF is bit 5?
        // PL011: FR (Flag Register) at +0x18.
        // TXFF (Transmit FIFO Full) is Bit 5.
        // We wait while TXFF is 1.
        const UARTFR: *mut u32 = 0x09000018 as *mut u32; // 0x09000000 + 0x18
        
        for b in msg.bytes() {
            unsafe {
                // Wait while TXFF (bit 5) is set
                while (core::ptr::read_volatile(UARTFR) & (1 << 5)) != 0 {
                    core::hint::spin_loop();
                }
                core::ptr::write_volatile(UART0, b);
            }
        }
    }

    fn ticks(&self) -> u64 {
        0 // TODO
    }

    fn idle(&self) {
        unsafe {
            asm!("wfi");
        }
    }

    fn shutdown(&self) -> ! {
        // PSCI SYSTEM_OFF or QEMU semihosting logic could go here.
        // For now, loop forever.
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }

    fn irq_disable(&self) {
        unsafe {
            asm!("msr daifset, #2");
        }
    }

    fn irq_enable(&self) {
        unsafe {
            asm!("msr daifclr, #2");
        }
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl HardwareBridge for Bridge {
    fn log(&self, _msg: &str) {}
    fn ticks(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn shutdown(&self) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
}
