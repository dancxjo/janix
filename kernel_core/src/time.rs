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

/// Real-time clock: wall-clock time, ideally UTC since Unix epoch.
pub trait RealTimeClock: Sync + Send {
    /// Initialize the RTC hardware / integration.
    fn init(&self);

    /// Returns (seconds, nanoseconds) since Unix epoch (1970-01-01T00:00:00Z),
    /// or a best-effort approximation if true UTC time is unavailable.
    fn now_utc(&self) -> (u64, u32);
}

static mut RTC: Option<&'static dyn RealTimeClock> = None;

/// Access the global RTC instance.
pub fn rtc() -> Option<&'static dyn RealTimeClock> {
    unsafe { RTC }
}

/// Register the concrete RTC instance (called from arch init).
pub fn register_rtc(rtc_impl: &'static dyn RealTimeClock) {
    unsafe {
        RTC = Some(rtc_impl);
    }
}

/// Helper: gets current UTC time in a single u64 ns value.
pub fn system_time_ns() -> Option<u64> {
    rtc().map(|r| {
        let (secs, nanos) = r.now_utc();
        secs.saturating_mul(1_000_000_000) + nanos as u64
    })
}
