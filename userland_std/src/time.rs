use userland_rt::Sys;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Duration {
    nanos: u64,
}

impl Duration {
    pub fn from_secs(secs: u64) -> Self {
        Duration { nanos: secs * 1_000_000_000 }
    }
    pub fn from_millis(ms: u64) -> Self {
        Duration { nanos: ms * 1_000_000 }
    }
    pub fn as_nanos(&self) -> u64 { self.nanos }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Instant {
    pub t_ns: u64,
}

impl Instant {
    pub fn now<S: Sys>(sys: &mut S) -> Self {
        Instant { t_ns: sys.time_now_ns() }
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration { nanos: self.t_ns - earlier.t_ns }
    }

    pub fn elapsed<S: Sys>(&self, sys: &mut S) -> Duration {
        Instant::now(sys).duration_since(*self)
    }

    pub fn checked_add(&self, dur: Duration) -> Option<Instant> {
        self.t_ns
            .checked_add(dur.nanos)
            .map(|t| Instant { t_ns: t })
    }
}
