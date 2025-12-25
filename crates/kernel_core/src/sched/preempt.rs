use super::scheduler::Scheduler;
use super::scheduler::SCHEDULER;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

pub static NEED_RESCHED: AtomicBool = AtomicBool::new(false);
pub static TICKS: AtomicU64 = AtomicU64::new(0);
pub static PREEMPT_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn preempt_disable() {
    PREEMPT_COUNT.fetch_add(1, Ordering::Relaxed);
    core::sync::atomic::compiler_fence(Ordering::SeqCst);
}

pub fn preempt_enable() {
    core::sync::atomic::compiler_fence(Ordering::SeqCst);
    PREEMPT_COUNT.fetch_sub(1, Ordering::Relaxed);
}

pub fn without_preemption<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    preempt_disable();
    let res = f();
    preempt_enable();
    res
}

pub fn with_scheduler<F, R>(f: F) -> R
where
    F: FnOnce(&mut Scheduler) -> R,
{
    without_preemption(|| {
        let mut sched = SCHEDULER.lock();
        f(&mut sched)
    })
}
