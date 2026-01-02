use crate::sched::task::TaskId;
use abi::ids::ThingId;

#[repr(C)]
pub struct PerCpu {
    pub cpu_id: u32,
    pub _padding: u32,
    pub thing: ThingId, // 16 bytes
    pub current_task: TaskId, // 0 if none
    pub idle_task: TaskId, // 0 if none
    pub run_queue_thing: ThingId, // 16 bytes
    
    /// Reentrancy guards
    pub in_switch: bool,
    pub _padding2: [u8; 3],  // Alignment for u32
    pub preempt_disabled: u32,
    pub irq_depth: u32,
    
    /// Architecture-specific exception stack (where applicable)
    pub exception_stack_ptr: u64,
}

impl PerCpu {
    pub fn new(cpu_id: u32, thing: ThingId, run_queue_thing: ThingId) -> Self {
        Self {
            cpu_id,
            _padding: 0,
            thing,
            current_task: TaskId(0),
            idle_task: TaskId(0),
            run_queue_thing,
            in_switch: false,
            _padding2: [0; 3],
            preempt_disabled: 0,
            irq_depth: 0,
            exception_stack_ptr: 0,
        }
    }
}
