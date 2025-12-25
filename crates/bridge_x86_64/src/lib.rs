#![no_std]

#[cfg(target_arch = "x86_64")]
use core::arch::asm;
use hw::HardwareBridge;

pub struct Bridge;

#[cfg(target_arch = "x86_64")]
impl HardwareBridge for Bridge {
    fn log(&self, msg: &str) {
        unsafe {
            for b in msg.bytes() {
                asm!("out dx, al", in("dx") 0x3F8u16, in("al") b);
            }
        }
    }

    fn ticks(&self) -> u64 {
        let eax: u32;
        let edx: u32;
        unsafe {
            asm!("rdtsc", out("eax") eax, out("edx") edx, options(nomem, nostack));
        }
        ((edx as u64) << 32) | (eax as u64)
    }

    fn idle(&self) {
        unsafe {
            asm!("hlt");
        }
    }

    fn shutdown(&self) -> ! {
        unsafe {
            // QEMU ISA debug exit
            asm!("out dx, ax", in("dx") 0x604u16, in("ax") 0x2000u16);
            loop {
                asm!("hlt");
            }
        }
    }

    fn irq_disable(&self) {
        unsafe {
            asm!("cli");
        }
    }

    fn irq_enable(&self) {
        unsafe {
            asm!("sti");
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
impl HardwareBridge for Bridge {
    fn log(&self, _msg: &str) {}
    fn ticks(&self) -> u64 {
        0
    }
    fn idle(&self) {}
    fn shutdown(&self) -> ! {
        loop {}
    }
    fn irq_disable(&self) {}
    fn irq_enable(&self) {}
}
