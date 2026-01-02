pub mod serial;

pub mod abi;
pub mod trap;
pub mod timer;
pub mod mmu;
pub mod context;
pub use mmu::AddressSpace;

pub type TrapFrame = trap::TrapContext; // Added

use core::arch::global_asm;
global_asm!(include_str!("vectors.S"));

use crate::machine::{Machine, MmioFlags, MmioMapping, MmioRange, Context};


use core::sync::atomic::{AtomicU64, Ordering};


static ARCH_MACHINE_IMPL: Riscv64Machine = Riscv64Machine::new();
pub static ARCH_MACHINE: &'static dyn Machine = &ARCH_MACHINE_IMPL;

struct Riscv64Machine {
    pub(crate) hhdm_offset: AtomicU64,
    pub(crate) kernel_phys_base: AtomicU64,
    pub(crate) kernel_virt_base: AtomicU64,
}

impl Riscv64Machine {
    pub const fn new() -> Self {
        Self {
            hhdm_offset: AtomicU64::new(0),
            kernel_phys_base: AtomicU64::new(0),
            kernel_virt_base: AtomicU64::new(0),
        }
    }
}

/// Per-CPU kernel stack top for trap entry.
/// This is used instead of direct sscratch write while in kernel mode
/// to avoid corrupting recursive traps.
#[no_mangle]
pub static KERNEL_STACK_TOP: AtomicU64 = AtomicU64::new(0);

impl Machine for Riscv64Machine {
    fn init(&self, info: crate::machine::PreBootInfo) {
        self.hhdm_offset.store(info.hhdm_offset, Ordering::Relaxed);
        self.kernel_phys_base.store(info.kernel_phys_base, Ordering::Relaxed);
        self.kernel_virt_base.store(info.kernel_virt_base, Ordering::Relaxed);

        // Capture bootloader's page tables FIRST before any address space operations
        mmu::init();
        
        // Initialize sscratch to 0 for kernel-mode trap handling
        // (kernel mode = sscratch is 0, user mode = sscratch is kernel stack)
        unsafe {
            core::arch::asm!("csrw sscratch, zero");
        }
        
        // Install trap vector
        extern "C" {
             static riscv64_trap_vector: u8; // Symbol
        }
        unsafe {
             let vector_addr = core::ptr::addr_of!(riscv64_trap_vector) as u64;
             // Mode = Direct (0)
             core::arch::asm!("csrw stvec, {}", in(reg) vector_addr);
             
             // Init Timer
             timer::init();
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

    fn set_kernel_stack(&self, top: u64) {
        // We store it in a static for trap entry to fetch.
        // This is safer than writing to sscratch while in kernel mode.
        KERNEL_STACK_TOP.store(top, Ordering::Release);
        
        // However, if we are currently in user mode (we aren't), sret would use sscratch.
        // The scheduler calls this while in S-mode.
        // The return-to-user path in vectors.S will load from KERNEL_STACK_TOP if needed.
        // Actually, let's keep it simple: return_from_trap will set sscratch.
    }

    fn virt_to_phys(&self, virt: u64) -> u64 {
        virt.wrapping_sub(self.hhdm_offset.load(Ordering::Relaxed))
    }
}
