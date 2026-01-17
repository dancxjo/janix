
pub mod irq_ring;

use core::sync::atomic::{AtomicUsize, Ordering};

static TIME_FN: AtomicUsize = AtomicUsize::new(0);

pub fn register_time_source(f: fn() -> u64) {
    TIME_FN.store(f as usize, Ordering::Relaxed);
}

pub fn now() -> u64 {
    let ptr = TIME_FN.load(Ordering::Relaxed);
    if ptr == 0 { return 0; }
    let f: fn() -> u64 = unsafe { core::mem::transmute(ptr) };
    f()
}
