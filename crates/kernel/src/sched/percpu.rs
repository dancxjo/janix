use crate::sched::task::TaskId;
use abi::ids::ThingId;

pub struct PerCpu {
    pub cpu_id: u32,
    pub thing: ThingId,
    pub current_task: Option<TaskId>,
    // Run queue could be here or global. Plan said per-CPU run queue via Scheduler?
    // "scheduler.main --[contains]--> run_queue.N"
    pub run_queue_thing: ThingId, 
}

impl PerCpu {
    pub fn new(cpu_id: u32, thing: ThingId, run_queue_thing: ThingId) -> Self {
        Self {
            cpu_id,
            thing,
            current_task: None, // Or idle?
            run_queue_thing,
        }
    }
}
