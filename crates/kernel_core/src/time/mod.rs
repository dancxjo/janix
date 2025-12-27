use core::sync::atomic::{AtomicU64, Ordering};
use crate::graph::GraphStore;
use thing_models::builtins::ids::THING_TIME_INSTANCE;
use thing_models::builtins::ids::THING_TIME_NOW_KIND;
// Use the model struct for the property body, assuming it exists in thing_models
// or we can use a local struct if it matches serialization.
// However, the seed uses `thing_models::core::time::TimeNow`.
// We'll trust that matching struct for updates.

// Monotonic time in nanoseconds since boot
static MONOTONIC_NS: AtomicU64 = AtomicU64::new(0);

// We might want valid system time offset later, for now system = monotonic
static SYSTEM_OFFSET_NS: AtomicU64 = AtomicU64::new(0);

pub fn monotonic_ns() -> u64 {
    MONOTONIC_NS.load(Ordering::Relaxed)
}

pub fn system_ns() -> u64 {
    monotonic_ns() + SYSTEM_OFFSET_NS.load(Ordering::Relaxed)
}

pub fn tick(graph: &mut GraphStore, delta_ns: u64) {
    let now = MONOTONIC_NS.fetch_add(delta_ns, Ordering::Relaxed) + delta_ns;
    
    // We do NOT update the graph here anymore.
    // rtc_x86 or a userspace timekeeper is responsible for updating the TimeNow thing.
    // This avoids conflicts and pushing graph updates from interrupt context.
}
