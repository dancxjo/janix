use userland_rt::Sys;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Simple duration type built on nanoseconds.
pub struct Duration {
    nanos: u64,
}

impl Duration {
    /// Create a `Duration` from seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use userland_std::time::Duration;
    ///
    /// assert_eq!(Duration::from_secs(2).as_nanos(), 2_000_000_000);
    /// ```
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
    /// Capture the current monotonic counter from `sys`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use abi::{KernelRequest, KernelResponse};
    /// use userland_rt::Sys;
    /// use userland_std::time::{Duration, Instant};
    ///
    /// struct ClockSys(RefCell<u64>);
    ///
    /// impl Sys for ClockSys {
    ///     fn syscall(&self, _: KernelRequest) -> KernelResponse {
    ///         panic!("not used")
    ///     }
    ///     fn time_now_ns(&mut self) -> u64 {
    ///         let mut inner = self.0.borrow_mut();
    ///         *inner += 1;
    ///         *inner
    ///     }
    ///     fn time_monotonic_ns(&mut self) -> u64 {
    ///         self.time_now_ns()
    ///     }
    ///     fn time_system_ns(&mut self) -> u64 {
    ///         self.time_now_ns()
    ///     }
    ///     fn sleep_for_ns(&mut self, _: u64) {}
    ///     fn sleep_until_ns(&mut self, _: u64) {}
    ///     fn yield_now(&mut self) {}
    ///     fn exit_thread(&mut self) -> ! {
    ///         panic!("exit")
    ///     }
    /// }
    ///
    /// let mut sys = ClockSys(RefCell::new(0));
    /// let start = Instant::now(&mut sys);
    /// let later = Instant::now(&mut sys);
    /// assert_eq!(later.duration_since(start).as_nanos(), 1);
    /// assert_eq!(
    ///     start.checked_add(Duration::from_secs(1)).unwrap().t_ns,
    ///     start.t_ns + 1_000_000_000
    /// );
    /// ```
    pub fn now<S: Sys>(sys: &mut S) -> Self {
        Instant {
            t_ns: sys.time_now_ns(),
        }
    }

    /// Compute the duration since an earlier instant.
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration {
            nanos: self.t_ns - earlier.t_ns,
        }
    }

    /// Return the duration that has elapsed since this instant.
    pub fn elapsed<S: Sys>(&self, sys: &mut S) -> Duration {
        Instant::now(sys).duration_since(*self)
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
    /// Capture the system time from `sys`.
    ///
    /// # Examples
    ///
    /// ```
    /// use abi::{KernelRequest, KernelResponse};
    /// use userland_rt::Sys;
    /// use userland_std::time::SystemTime;
    ///
    /// struct TimeSys(u64);
    ///
    /// impl Sys for TimeSys {
    ///     fn syscall(&self, _: KernelRequest) -> KernelResponse {
    ///         panic!("syscall not used");
    ///     }
    ///     fn time_now_ns(&mut self) -> u64 {
    ///         0
    ///     }
    ///     fn time_monotonic_ns(&mut self) -> u64 {
    ///         0
    ///     }
    ///     fn time_system_ns(&mut self) -> u64 {
    ///         7
    ///     }
    ///     fn sleep_for_ns(&mut self, _: u64) {}
    ///     fn sleep_until_ns(&mut self, _: u64) {}
    ///     fn yield_now(&mut self) {}
    ///     fn exit_thread(&mut self) -> ! {
    ///         panic!("exit")
    ///     }
    /// }
    ///
    /// let mut sys = TimeSys(0);
    /// let now = SystemTime::now(&mut sys);
    /// assert_eq!(now.ns_since_epoch, 7);
    /// ```
    pub fn now<S: Sys>(sys: &mut S) -> Self {
        let ns = sys.time_system_ns();
        SystemTime { ns_since_epoch: ns }
    }

    /// Compute the difference between two system times.
    pub fn duration_since(&self, earlier: SystemTime) -> Duration {
        Duration {
            nanos: self.ns_since_epoch - earlier.ns_since_epoch,
        }
    }
}

/// Sleep for at least `dur`.
///
/// # Examples
///
/// ```
/// use std::cell::RefCell;
/// use abi::{KernelRequest, KernelResponse};
/// use userland_rt::Sys;
/// use userland_std::time::{sleep, Duration};
///
/// struct SleepSys {
///     slept: RefCell<Vec<u64>>,
/// }
///
/// impl SleepSys {
///     fn new() -> Self {
///         SleepSys {
///             slept: RefCell::new(Vec::new()),
///         }
///     }
/// }
///
/// impl Sys for SleepSys {
///     fn syscall(&self, _: KernelRequest) -> KernelResponse {
///         panic!("syscall not used");
///     }
///     fn time_now_ns(&mut self) -> u64 {
///         0
///     }
///     fn time_monotonic_ns(&mut self) -> u64 {
///         0
///     }
///     fn time_system_ns(&mut self) -> u64 {
///         0
///     }
///     fn sleep_for_ns(&mut self, delta_ns: u64) {
///         self.slept.borrow_mut().push(delta_ns);
///     }
///     fn sleep_until_ns(&mut self, _: u64) {}
///     fn yield_now(&mut self) {}
///     fn exit_thread(&mut self) -> ! {
///         panic!("exit")
///     }
/// }
///
/// let mut sys = SleepSys::new();
/// sleep(&mut sys, Duration::from_millis(2));
/// assert_eq!(sys.slept.borrow()[0], 2_000_000);
/// ```
pub fn sleep<S: Sys>(sys: &mut S, dur: Duration) {
    sys.sleep_for_ns(dur.as_nanos());
}
