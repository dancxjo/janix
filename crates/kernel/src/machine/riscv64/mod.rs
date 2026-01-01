pub mod serial;
pub mod serial;
pub mod abi;
pub mod trap;
pub mod timer;

pub type TrapFrame = trap::TrapContext; // Added

use core::arch::global_asm;
global_asm!(include_str!("vectors.S"));

use crate::machine::{Machine, MmioFlags, MmioMapping, MmioRange, Context};

pub static ARCH_MACHINE: &'static dyn Machine = &Riscv64Machine;

struct Riscv64Machine;

impl Machine for Riscv64Machine {
    fn init(&self, _info: crate::machine::PreBootInfo) {
        // Install trap vector
        extern "C" {
             static riscv64_trap_vector: u8; // Symbol
        }
        unsafe {
             let vector_addr = core::ptr::addr_of!(riscv64_trap_vector) as u64;
             // Mode = Direct (0)
             core::arch::asm!("csrw stvec, {}", in(reg) vector_addr);
        }
    }

    fn console_write(&self, bytes: &[u8]) -> usize {
        serial::Serial::new().write(bytes);
        bytes.len()
    }

    fn mmio_map(&self, _range: MmioRange, _flags: MmioFlags) -> Option<MmioMapping> {
        // Placeholder identity map? Or fail?
        // Code expects it to work for UART at least. 
        // But for now, returning None is safer than faking it wrong.
        None
    }

    fn irq_disable(&self) -> u64 {
        let sstatus: u64;
        unsafe {
             core::arch::asm!("csrr {}, sstatus", out(reg) sstatus);
             core::arch::asm!("csrci sstatus, 0x2"); // Clear SIE (bit 1)
        }
        sstatus
    }

    fn irq_restore(&self, token: u64) {
        if token & 0x2 != 0 {
             unsafe { core::arch::asm!("csrsi sstatus, 0x2"); }
        } else {
             unsafe { core::arch::asm!("csrci sstatus, 0x2"); }
        }
    }

    fn halt(&self) -> ! {
        loop {
            unsafe { core::arch::asm!("wfi"); }
        }
    }

    fn idle(&self) {
        unsafe { core::arch::asm!("wfi"); }
    }

    fn switch_to(&self, _old_ctx: &mut Context, _new_ctx: &Context) {
        // Placeholder
    }

    fn task_entry_stub(&self) -> u64 {
        0
    }
}
