//! x86_64 architecture implementation

use super::Arch;
use core::arch::asm;

pub mod serial;

pub struct X86Arch;

impl Arch for X86Arch {
    fn irq_disable(&self) -> u64 {
        let flags: u64;
        unsafe {
            asm!("pushfq; pop {}; cli", out(reg) flags, options(nomem, preserves_flags));
        }
        flags
    }

    fn irq_restore(&self, token: u64) {
        unsafe {
            if token & 0x200 != 0 {
                asm!("sti", options(nomem, preserves_flags));
            }
        }
    }

    fn halt(&self) -> ! {
        loop {
            unsafe {
                asm!("cli; hlt");
            }
        }
    }

    fn idle(&self) {
        unsafe {
            asm!("hlt");
        }
    }

    fn cpu_id(&self) -> u32 {
        0 // Single CPU for now
    }
}
