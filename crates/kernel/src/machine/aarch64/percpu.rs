use crate::sched::percpu::PerCpu;

#[repr(C, align(16))]
pub struct ArchPerCpu {
    /// Core scheduler state
    pub core: PerCpu,
    
    /// Scratch space for exception entry
    pub scratch_x0: u64,
    
    /// Kernel Stack Top for current task (for EL0 -> EL1 transition)
    pub kernel_stack_top: u64,
}

impl ArchPerCpu {
    pub const fn new(cpu_id: u32, thing: abi::ids::ThingId, rq_thing: abi::ids::ThingId) -> Self {
        Self {
            core: PerCpu {
                cpu_id,
                _padding: 0,
                thing,
                current_task: crate::sched::task::TaskId(0),
                idle_task: crate::sched::task::TaskId(0),
                run_queue_thing: rq_thing,
                in_switch: false,
                _padding2: [0; 3],
                preempt_disabled: 0,
                irq_depth: 0,
                exception_stack_ptr: 0,
            },
            scratch_x0: 0,
            kernel_stack_top: 0,
        }
    }
}

pub unsafe fn init_percpu(percpu: &'static mut ArchPerCpu) {
    let addr = percpu as *mut _ as u64;
    core::arch::asm!("msr tpidr_el1, {}", in(reg) addr);
}

pub fn get_local() -> &'static mut ArchPerCpu {
    let addr: u64;
    unsafe {
        core::arch::asm!("mrs {}, tpidr_el1", out(reg) addr);
        &mut *(addr as *mut ArchPerCpu)
    }
}
