pub mod serial;
pub mod serial;
pub mod abi;
pub mod trap;
pub mod timer;

pub type TrapFrame = trap::TrapContext; // Added

use core::arch::global_asm;
global_asm!(include_str!("vectors.S"));

use crate::machine::{Machine, MmioFlags, MmioMapping, MmioRange, Context};

pub static ARCH_MACHINE: &'static dyn Machine = &LoongArchMachine;

struct LoongArchMachine;

impl Machine for LoongArchMachine {
    fn init(&self, _info: crate::machine::PreBootInfo) {
        // Install trap vector
        extern "C" {
             static loongarch64_trap_vector: u8; // Symbol
        }
        unsafe {
             let vector_addr = core::ptr::addr_of!(loongarch64_trap_vector) as u64;
             // Set EBASE (CSR 0x4)
             core::arch::asm!("csrwr {}, 0x4", in(reg) vector_addr);
        }
    }

    fn console_write(&self, bytes: &[u8]) -> usize {
        serial::Serial::new().write(bytes);
        bytes.len()
    }

    fn mmio_map(&self, _range: MmioRange, _flags: MmioFlags) -> Option<MmioMapping> {
        None
    }

    fn irq_disable(&self) -> u64 {
        let crmd: u64;
        unsafe {
             // Read CRMD (Current Request Mode Definition) - CSR 0x0
             core::arch::asm!("csrrd {}, 0x0", out(reg) crmd);
             // Clear IE (Interrupt Enable) - bit 2
             let new_crmd = crmd & !0x4;
             core::arch::asm!("csrwr {}, 0x0", in(reg) new_crmd);
        }
        crmd
    }

    fn irq_restore(&self, token: u64) {
        let ie = token & 0x4;
        unsafe {
            let mut current: u64;
            core::arch::asm!("csrrd {}, 0x0", out(reg) current);
            if ie != 0 {
                current |= 0x4;
            } else {
                current &= !0x4;
            }
            core::arch::asm!("csrwr {}, 0x0", in(reg) current);
        }
    }

    fn halt(&self) -> ! {
        loop {
            unsafe { core::arch::asm!("idle 0"); }
        }
    }

    fn idle(&self) {
        unsafe { core::arch::asm!("idle 0"); }
    }

    fn switch_to(&self, _old_ctx: &mut Context, _new_ctx: &Context) {
        // Placeholder
    }

    fn task_entry_stub(&self) -> u64 {
        0
    }
}
