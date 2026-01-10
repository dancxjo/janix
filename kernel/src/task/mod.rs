// Modules
pub mod scheduler;

pub type TaskId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Running,
    Runnable,
    Blocked,
}

pub use scheduler::Scheduler;

use crate::simd::SimdState;
use crate::memory::paging::AddressSpace;
use crate::arch::TrapFrame;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ArchContext(pub u64);

pub struct Task {
    pub id: TaskId,
    pub state: TaskState,

    pub kstack_base: *mut u8,
    pub kstack_size: usize,
    pub kstack_top: u64,

    pub ctx: ArchContext,

    pub simd: SimdState,
    
    pub aspace: Option<AddressSpace>,
    pub tf: TrapFrame,
}

// Global scheduler instance
pub static mut SCHEDULER: Option<Scheduler> = None;

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

pub fn set_need_resched(val: bool) {
    unsafe {
        let ptr = core::ptr::addr_of_mut!(SCHEDULER);
        if let Some(sched) = (*ptr).as_ref() {
            sched.need_resched.store(val, core::sync::atomic::Ordering::Relaxed);
        }
    }
}

pub fn check_preemption() {
    let rt = crate::runtime();
    // We must be careful here. If we are returning to user, we are mostly safe to yield.
    // We enter a critical section to check/clear.
    // Actually yield_now handles the locking.
    
    let needed = unsafe {
        let ptr = core::ptr::addr_of_mut!(SCHEDULER);
        if let Some(sched) = (*ptr).as_ref() {
            sched.need_resched.swap(false, core::sync::atomic::Ordering::Relaxed)
        } else {
            false
        }
    };
    
    if needed {
        yield_now();
    }
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

pub fn run_scheduler() -> ! {
    loop {
        yield_now();
        // Simple busy wait or wfi hint could go here to save power,
        // but for now just busy loop + yield.
        // We can't use runtime().halt() because that kills the machine.
        core::hint::spin_loop(); 
    }
}
