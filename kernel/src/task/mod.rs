use crate::arch::imp::task::ArchContext;
use crate::simd::SimdState;

pub mod scheduler;
pub use scheduler::Scheduler;

pub type TaskId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Running,
    Blocked,
    Dead,
}

pub struct Task {
    pub id: TaskId,
    pub state: TaskState,

    pub kstack_base: *mut u8,
    pub kstack_size: usize,
    pub kstack_top: u64,

    pub ctx: ArchContext,

    pub simd: SimdState,
}

// Global scheduler instance
static mut SCHEDULER: Option<Scheduler> = None;

pub fn init() {
    unsafe {
        let ptr = core::ptr::addr_of_mut!(SCHEDULER);
        if (*ptr).is_none() {
            // Create the boot task (Task 0)
            let mut sched = Scheduler::new();
            sched.init_boot_task();
            *ptr = Some(sched);
        }
    }
}

pub fn spawn(entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
    let rt = crate::runtime();
    let irq_state = rt.irq_disable();
    let res = unsafe {
        let ptr = core::ptr::addr_of_mut!(SCHEDULER);
        if let Some(sched) = (*ptr).as_mut() {
            sched.spawn(entry, arg)
        } else {
            // Panic with interrupts disabled is fine, panic handler handles it
            panic!("Scheduler not initialized");
        }
    };
    rt.irq_restore(irq_state);
    res
}

pub fn yield_now() {
    let rt = crate::runtime();
    let irq_state = rt.irq_disable();
    unsafe {
        let ptr = core::ptr::addr_of_mut!(SCHEDULER);
        if let Some(sched) = (*ptr).as_mut() {
            sched.yield_now();
        }
    }
    rt.irq_restore(irq_state);
}

// For diagnostics
pub fn dump_stats() {
    unsafe {
        let ptr = core::ptr::addr_of_mut!(SCHEDULER);
        if let Some(sched) = (*ptr).as_ref() {
            crate::kinfo!("Sched: tasks={} current={:?}", sched.task_count(), sched.current_id());
        }
    }
}
