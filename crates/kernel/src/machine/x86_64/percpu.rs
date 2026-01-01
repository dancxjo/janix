//! Per-CPU data structure
//!
//! Accessed via GS segment base.

use crate::proc::Thread;

#[repr(C)]
pub struct PerCpu {
    /// Self-reference for validation/access
    pub this: *const PerCpu,
    
    /// Current logical CPU ID
    pub cpu_id: u32,
    
    /// LAPIC ID
    pub lapic_id: u32,
    
    /// Current running thread
    pub current_thread: *mut Thread,
    
    /// User->Kernel transition stack (SYSCALL)
    pub syscall_rsp: u64,
    
    /// Interrupt entry stack (RSP0) - currently unused if TSS handles it
    pub kernel_rsp: u64,

    /// Scratch space for syscall entry
    pub scratch_rax: u64,
}

impl PerCpu {
    pub const fn new(cpu_id: u32, lapic_id: u32) -> Self {
        Self {
            this: core::ptr::null(),
            cpu_id,
            lapic_id,
            current_thread: core::ptr::null_mut(),
            syscall_rsp: 0,
            kernel_rsp: 0,
            scratch_rax: 0,
        }
    }
}

/// Initialize GS base for the current CPU
///
/// Safety: Argument must be a valid static PerCpu lifetime.
pub unsafe fn init_gs_base(percpu: &'static mut PerCpu) {
    percpu.this = percpu as *const _;
    
    let addr = percpu as *const _ as u64;
    let lo = addr as u32;
    let hi = (addr >> 32) as u32;
    
    core::arch::asm!(
        "wrmsr",
        in("ecx") 0xC0000101u32, // IA32_GS_BASE
        in("eax") lo,
        in("edx") hi,
    );
}
