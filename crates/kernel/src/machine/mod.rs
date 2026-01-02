//! Machine interface for hardware abstraction.
//!
pub use bitflags::bitflags;

#[cfg(target_arch = "x86_64")]
pub use x86_64::abi;
#[cfg(target_arch = "aarch64")]
pub use aarch64::abi;
#[cfg(target_arch = "riscv64")]
pub use riscv64::abi;
#[cfg(target_arch = "loongarch64")]
pub use loongarch64::abi;

pub mod context;
pub use context::{ArchContext, ArchTask, ArchTrap, CpuMode, TrapInfo, ResumeSpec};

// CurrentArch type alias - the scheduler uses this without knowing arch details
#[cfg(target_arch = "x86_64")]
pub type CurrentArch = x86_64::context::X86Arch;
#[cfg(target_arch = "aarch64")]
pub type CurrentArch = aarch64::context::AArch64Arch;
#[cfg(target_arch = "riscv64")]
pub type CurrentArch = riscv64::context::Riscv64Arch;
#[cfg(target_arch = "loongarch64")]
pub type CurrentArch = loongarch64::context::LoongArchArch;

// TaskContext type alias - the per-arch task context type
#[cfg(target_arch = "x86_64")]
pub type TaskContext = x86_64::context::TaskContext;
#[cfg(target_arch = "aarch64")]
pub type TaskContext = aarch64::context::TaskContext;
#[cfg(target_arch = "riscv64")]
pub type TaskContext = riscv64::context::TaskContext;
#[cfg(target_arch = "loongarch64")]
pub type TaskContext = loongarch64::context::TaskContext;

pub mod input;

#[cfg(target_arch = "x86_64")]
pub use x86_64::{TrapFrame, AddressSpace};
#[cfg(target_arch = "aarch64")]
pub use aarch64::{TrapFrame, AddressSpace};
#[cfg(target_arch = "riscv64")]
pub use riscv64::{TrapFrame, AddressSpace};
#[cfg(target_arch = "loongarch64")]
pub use loongarch64::{TrapFrame, AddressSpace};


#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "loongarch64")]
pub mod loongarch64;



#[cfg(target_arch = "x86_64")]
pub use x86_64::ARCH_MACHINE;
#[cfg(target_arch = "aarch64")]
pub use aarch64::ARCH_MACHINE;
#[cfg(target_arch = "riscv64")]
pub use riscv64::ARCH_MACHINE;
#[cfg(target_arch = "loongarch64")]
pub use loongarch64::ARCH_MACHINE;

/// Physical MMIO range.
pub struct MmioRange {
    pub phys: u64,
    pub len: usize,
}

/// Virtual mapping returned by the machine backend.
pub struct MmioMapping {
    pub virt: u64,
    pub len: usize,
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct MmioFlags: u32 {
        const DEVICE = 1 << 0;
        const UNCACHED = 1 << 1;
        const READ = 1 << 2;
        const WRITE = 1 << 3;
    }
}

/// Saved context for task switching (stack pointer)
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Context {
    pub sp: u64,
}

/// Information needed for machine initialization (handoff from Bran).
#[derive(Clone, Copy)]
pub struct PreBootInfo {
    /// HHDM offset for physical memory access
    pub hhdm_offset: u64,
    /// Kernel physical load address
    pub kernel_phys_base: u64,
    /// Kernel virtual base address  
    pub kernel_virt_base: u64,
}

/// Machine interface: Physics + Boot I/O.
/// 
/// This trait isolates the kernel from the hardware reality.
/// It provides:
/// 1. CPU primitives (execution physics) required by the scheduler.
/// 2. Early I/O (console, MMIO) required for boot and Platform bootstrapping.
pub trait Machine: Sync {
    /// Initialize the machine with handoff info.
    fn init(&self, _info: PreBootInfo) {}

    // --- I/O (Capabilities) ---

    /// Write bytes to the early console.
    fn console_write(&self, bytes: &[u8]) -> usize;

    /// Map a physical MMIO range and return a virtual mapping.
    /// Implementations must not assume an HHDM covers device ranges.
    fn mmio_map(&self, range: MmioRange, flags: MmioFlags) -> Option<MmioMapping>;
    
    // --- CPU (Physics) ---

    fn irq_disable(&self) -> u64;
    fn irq_restore(&self, token: u64);
    fn halt(&self) -> !;
    fn idle(&self);
    fn cpu_id(&self) -> u32 { 0 }
    
    /// Switch context from old to new
    fn switch_to(&self, old_ctx: &mut Context, new_ctx: &Context);

    fn irq_enable(&self) {}
    
    /// Entry point stub address for new tasks
    fn task_entry_stub(&self) -> u64;

    /// Translate kernel virtual address to physical address in canonical zones.
    /// This should handle:
    /// 1. HHDM (virt >= hhdm_offset) -> virt - hhdm_offset
    /// 2. Kernel Code/Data (virt >= kernel_virt_base) -> virt - virt_base + phys_base
    fn virt_to_phys(&self, virt: u64) -> u64;

    /// Update the kernel stack for the current CPU (for syscall/interrupt entry).
    fn set_kernel_stack(&self, _top: u64) {}
}

static mut MACHINE: Option<&'static dyn Machine> = None;

/// Install the architecture-provided machine implementation.
///
/// Safety: must be called exactly once during boot by the architecture code
/// before any machine() calls occur.
pub unsafe fn install(machine: &'static dyn Machine) {
    MACHINE = Some(machine);
}

/// Access the installed machine implementation.
pub fn machine() -> &'static dyn Machine {
    unsafe { MACHINE.expect("machine not installed") }
}

/// Trigger a benign fault (breakpoint) for smoke testing trap recording.
pub fn smoke_fault() {
    #[cfg(target_arch = "x86_64")]
    {
        use ::x86_64::instructions::interrupts;
        interrupts::int3();
    }

    #[cfg(target_arch = "aarch64")]
    unsafe { core::arch::asm!("brk #0") };

    #[cfg(target_arch = "riscv64")]
    unsafe { core::arch::asm!("ebreak") };

    #[cfg(target_arch = "loongarch64")]
    unsafe { core::arch::asm!("break 0") }; // or equivalent
}

pub fn idle() {
    machine().idle();
}

pub fn halt() -> ! {
    machine().halt()
}

pub fn irq_enable() {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!("sti", options(nomem, preserves_flags));
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!("msr daifclr, #2", options(nomem, preserves_flags));
        #[cfg(target_arch = "riscv64")]
        core::arch::asm!("csrsi sstatus, 2", options(nomem, preserves_flags));
        #[cfg(target_arch = "loongarch64")]
        {
             let mut val = 4u64;
             core::arch::asm!("csrxchg {}, {}, 0x0", inout(reg) val, in(reg) val);
             let _ = val;
        }
    }
}

pub fn irq_disable() -> u64 {
    machine().irq_disable()
}

pub fn irq_restore(token: u64) {
    machine().irq_restore(token)
}
