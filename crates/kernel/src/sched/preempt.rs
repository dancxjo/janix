use core::sync::atomic::{AtomicUsize, Ordering};

static PREEMPT_DISABLE_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn disable() {
    PREEMPT_DISABLE_COUNT.fetch_add(1, Ordering::Relaxed);
    core::sync::atomic::compiler_fence(Ordering::SeqCst);
}

pub fn enable() {
    core::sync::atomic::compiler_fence(Ordering::SeqCst);
    PREEMPT_DISABLE_COUNT.fetch_sub(1, Ordering::Relaxed);
}

pub fn is_enabled() -> bool {
    PREEMPT_DISABLE_COUNT.load(Ordering::Relaxed) == 0
}
