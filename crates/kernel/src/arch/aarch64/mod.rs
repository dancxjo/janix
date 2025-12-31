//! aarch64 architecture implementation

use super::Arch;
use core::arch::asm;

pub mod serial;

pub struct Aarch64Arch;

impl Arch for Aarch64Arch {
    fn irq_disable(&self) -> u64 {
        let flags: u64;
        unsafe {
            asm!("mrs {}, daif; msr daifset, #0xf", out(reg) flags, options(nomem, preserves_flags));
        }
        flags
    }

    fn irq_restore(&self, token: u64) {
        unsafe {
            asm!("msr daif, {}", in(reg) token, options(nomem, preserves_flags));
        }
    }

    fn halt(&self) -> ! {
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }

    fn idle(&self) {
        unsafe {
            asm!("wfi");
        }
    }

    fn cpu_id(&self) -> u32 {
        0
    }
}
