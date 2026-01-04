//! Per-CPU data structure
//!
//! Accessed via GS segment base.

use crate::sched::percpu::PerCpu;

#[repr(C)]
pub struct x86PerCpu {
    /// Core scheduler state
    pub core: PerCpu,

    /// Self-reference for validation/access
    pub this: *const x86PerCpu,

    /// LAPIC ID
    pub lapic_id: u32,

    /// User->Kernel transition stack (SYSCALL)
    pub syscall_rsp: u64,

    /// Scratch space for syscall entry
    pub scratch_rax: u64,
}

impl x86PerCpu {
    pub const fn new(cpu_id: u32, lapic_id: u32) -> Self {
        Self {
            core: PerCpu {
                cpu_id,
                _padding: 0,
                thing: abi::ids::ThingId(0),
                current_task: crate::sched::task::TaskId(0),
                idle_task: crate::sched::task::TaskId(0),
                run_queue_thing: abi::ids::ThingId(0),
                in_switch: false,
                _padding2: [0; 3],
                preempt_disabled: 0,
                irq_depth: 0,
                exception_stack_ptr: 0,
            },
            this: core::ptr::null(),
            lapic_id,
            syscall_rsp: 0,
            scratch_rax: 0,
        }
    }
}

/// Initialize GS base for the current CPU
///
/// Safety: Argument must be a valid static PerCpu lifetime.
pub unsafe fn init_gs_base(percpu: &'static mut x86PerCpu) {
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
