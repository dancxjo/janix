//! riscv64 architecture implementation

use super::Arch;
use core::arch::asm;

pub mod machine;
pub mod serial;

pub struct Riscv64Arch;

impl Arch for Riscv64Arch {
    fn irq_disable(&self) -> u64 {
        let flags: u64;
        unsafe {
            asm!("csrrci {}, sstatus, 2", out(reg) flags, options(nomem, preserves_flags));
        }
        flags
    }

    fn irq_restore(&self, token: u64) {
        unsafe {
            if token & 2 != 0 {
                asm!("csrsi sstatus, 2", options(nomem, preserves_flags));
            }
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
