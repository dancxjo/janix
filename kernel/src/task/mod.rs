pub mod scheduler;
pub mod loader;

pub use scheduler::Scheduler;

use crate::simd::SimdState;
use crate::BootRuntime;
use crate::BootTasking;

pub type TaskId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Running,
    Blocked,
    Dead,
}

pub struct Task<R: BootRuntime> {
    pub id: TaskId,
    pub state: TaskState,

    pub kstack_base: *mut u8,
    pub kstack_size: usize,
    pub kstack_top: u64,

    pub ctx: <R::Tasking as BootTasking>::Context,
    pub aspace: <R::Tasking as BootTasking>::AddressSpace,

    pub simd: SimdState,
}

pub fn init<R: BootRuntime>() {
    scheduler::init::<R>();
}

pub fn spawn<R: BootRuntime>(entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
    scheduler::spawn::<R>(entry, arg)
}

pub fn yield_now<R: BootRuntime>() {
    scheduler::yield_now::<R>();
}

pub fn preempt_disable<R: BootRuntime>() {
    let lock = scheduler::SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        sched.preempt_disable();
    }
}

pub fn preempt_enable<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();
    
    let switch_params = {
        let lock = scheduler::SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.preempt_enable()
        } else {
            None
        }
    };
    
    if let Some((old_ctx, new_ctx)) = switch_params {
        unsafe {
            rt.tasking().switch(&mut *old_ctx, &*new_ctx);
        }
    }
    
    rt.irq_restore(irq);
}

pub fn resched_if_needed<R: BootRuntime>() {
    // Similar to preempt_enable but without decrementing (conceptually checks "is resched needed?")
    // But actually, we just want to explicit check.
    // For now, let's just use yield_now if needed?
    // Actually, `schedule_point(PreemptTick)` logic inside `preempt_enable` handles the check.
    // So this might just be a no-op or a direct check.
    // Let's implement it as a check for `need_resched` and call `yield_now` (or equivalent) if true.
    // BUT we need to be careful about recursion.
    // For v0, let's leave it as a TODO or a simple "maybe yield".
    // "resched_if_needed at safe point" -> usually checks flags.
    
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();
    
    let switch_params = {
        let lock = scheduler::SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            // We use PreemptTick reason to check need_resched flag inside schedule_point?
            // Wait, schedule_point(PreemptTick) checks if disabled.
            // If we are here, we are at a safe point, so we assume preemption is enabled (or we ignore depth?).
            // Actually, if we are at a safe point (e.g. syscall return), we should yield if need_resched is set.
            // But we can't access `need_resched` without the lock.
            // Let's rely on `sched.schedule_point(PreemptTick)` to return None if disabled, 
            // OR we need a new reason `ReschedIfNeeded`?
            // For now, let's abuse `CooperativeYield` if we see the flag? No.
            // Let's allow `PreemptTick` to serve this purpose.
            
            // Wait, implementation of `schedule_point` for `PreemptTick`:
            // if disable_depth > 0 { need_resched = true; return None; }
            // else { prepare_yield() }
            
            // So calling it here works perfectly.
            sched.schedule_point(scheduler::ScheduleReason::PreemptTick)
        } else {
            None
        }
    };

    if let Some((old_ctx, new_ctx)) = switch_params {
        unsafe {
            rt.tasking().switch(&mut *old_ctx, &*new_ctx);
        }
    }
    
    rt.irq_restore(irq);
}

pub fn dump_stats<R: BootRuntime>() {
    scheduler::dump_stats::<R>();
}

pub fn run_scheduler<R: BootRuntime>() -> ! {
    loop {
        yield_now::<R>();
        core::hint::spin_loop();
    }
}
