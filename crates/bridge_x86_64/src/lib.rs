#![no_std]

use hw::HardwareBridge;
use core::arch::asm;

pub struct Bridge;

impl HardwareBridge for Bridge {
    fn log(&self, msg: &str) {
        for b in msg.bytes() {
            unsafe {
                asm!("out dx, al", in("dx") 0x3F8u16, in("al") b);
            }
        }
    }

    fn ticks(&self) -> u64 {
        0 // TODO
    }

    fn idle(&self) {
        unsafe { asm!("hlt"); }
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
        unsafe { asm!("cli"); }
    }

    fn irq_enable(&self) {
        unsafe { asm!("sti"); }
    }
}
