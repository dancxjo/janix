//! loongarch64 architecture implementation

use super::Arch;
use core::arch::asm;

pub mod serial;

pub struct Loongarch64Arch;

impl Arch for Loongarch64Arch {
    fn irq_disable(&self) -> u64 {
        let flags: u64;
        unsafe {
            asm!("csrrd {}, 0x0; ori $zero, $zero, 0", out(reg) flags, options(nomem, preserves_flags));
        }
        flags
    }

    fn irq_restore(&self, token: u64) {
        let _ = token;
        // TODO: Implement
    }

    fn halt(&self) -> ! {
        loop {
            unsafe {
                asm!("idle 0");
            }
        }
    }

    fn idle(&self) {
        unsafe {
            asm!("idle 0");
        }
    }

    fn cpu_id(&self) -> u32 {
        0
    }
}
