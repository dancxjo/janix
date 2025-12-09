/// Trait for hardware timer abstraction
pub trait HardwareTimer: Sync + Send {
    /// Initialize the hardware timer for this CPU / system.
    /// Called once at boot per arch.
    fn init(&self);

    /// Returns a monotonically increasing time in nanoseconds since boot.
    /// (Monotonic, not wall-clock.)
    fn now_ns(&self) -> u64;

    /// Schedule a timer interrupt at or after `deadline_ns`.
    /// For now this can be a no-op or “next tick” on architectures where this is hard.
    fn set_deadline_ns(&self, deadline_ns: u64);
}

static mut TIMER: Option<&'static dyn HardwareTimer> = None;

/// Register the global timer instance.
/// This should be called by the architecture initialization code.
pub fn register_timer(timer: &'static dyn HardwareTimer) {
    unsafe {
        TIMER = Some(timer);
    }
}

/// Get the global timer instance.
/// Panics if no timer has been registered.
pub fn timer() -> &'static dyn HardwareTimer {
    unsafe {
        match TIMER {
            Some(t) => t,
            None => panic!("No hardware timer registered"),
        }
    }
}

/// Returns a monotonically increasing time in nanoseconds since boot.
pub fn monotonic_now_ns() -> u64 {
    timer().now_ns()
}

/// Convert ns to a simple `Duration` type (your own, not std).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Duration {
    pub secs: u64,
    pub nanos: u32,
}

pub fn ns_to_duration(ns: u64) -> Duration {
    let secs = ns / 1_000_000_000;
    let rem = ns % 1_000_000_000;
    Duration {
        secs,
        nanos: rem as u32,
    }
}
