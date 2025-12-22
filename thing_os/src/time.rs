use crate::sys::raw_syscall;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Simple duration type built on nanoseconds.
pub struct Duration {
    nanos: u64,
}

impl Duration {
    /// Create a `Duration` from seconds.
    pub fn from_secs(secs: u64) -> Self {
        Duration {
            nanos: secs * 1_000_000_000,
        }
    }

    /// Create a `Duration` from milliseconds.
    pub fn from_millis(ms: u64) -> Self {
        Duration {
            nanos: ms * 1_000_000,
        }
    }

    /// Create a `Duration` from nanoseconds.
    pub fn from_nanos(nanos: u64) -> Self {
        Duration { nanos }
    }

    /// Access the nanosecond representation.
    pub fn as_nanos(&self) -> u64 {
        self.nanos
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Instant {
    pub t_ns: u64,
}

impl Instant {
    /// Capture the current monotonic counter.
    pub fn now() -> Self {
        let ret = unsafe {
            match raw_syscall(abi::syscalls::SYSCALL_TIME_MONOTONIC_NS, 0, 0, 0, 0, 0, 0) {
                t => t,
            }
        };
        Instant { t_ns: ret }
    }

    /// Compute the duration since an earlier instant.
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration {
            nanos: self.t_ns - earlier.t_ns,
        }
    }

    /// Return the duration that has elapsed since this instant.
    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(*self)
    }

    /// Add a duration, returning `None` on overflow.
    pub fn checked_add(&self, dur: Duration) -> Option<Instant> {
        self.t_ns
            .checked_add(dur.nanos)
            .map(|t| Instant { t_ns: t })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// A system time value backed by the kernel clock.
pub struct SystemTime {
    pub ns_since_epoch: u64,
}

impl SystemTime {
    /// Capture the system time.
    pub fn now() -> Self {
        let ret = unsafe { raw_syscall(abi::syscalls::SYSCALL_TIME_SYSTEM_NS, 0, 0, 0, 0, 0, 0) };
        SystemTime {
            ns_since_epoch: ret,
        }
    }

    /// Compute the difference between two system times.
    pub fn duration_since(&self, earlier: SystemTime) -> Duration {
        Duration {
            nanos: self.ns_since_epoch - earlier.ns_since_epoch,
        }
    }
}

/// Sleep for at least `dur`.
pub fn sleep(dur: Duration) {
    unsafe {
        raw_syscall(
            abi::syscalls::SYSCALL_SLEEP_FOR_NS,
            dur.as_nanos(),
            0,
            0,
            0,
            0,
            0,
        );
    }
}

/// Yield the current thread's timeslice.
pub fn yield_now() {
    unsafe {
        raw_syscall(abi::syscalls::SYSCALL_YIELD, 0, 0, 0, 0, 0, 0);
    }
}

/// Sleep for a number of milliseconds.
pub fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms));
}
