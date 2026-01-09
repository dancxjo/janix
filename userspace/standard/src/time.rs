#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    micros: u64,
}

impl Duration {
    pub const fn from_micros(micros: u64) -> Self {
        Self { micros }
    }
    
    pub const fn from_millis(millis: u64) -> Self {
        Self { micros: millis * 1000 }
    }
    
    pub const fn from_secs(secs: u64) -> Self {
        Self { micros: secs * 1_000_000 }
    }

    pub fn as_micros(&self) -> u64 {
        self.micros
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    ticks: u64,
}

impl Instant {
    pub const fn from_ticks(ticks: u64) -> Self {
        Self { ticks }
    }
    
    pub fn ticks(&self) -> u64 {
        self.ticks
    }
}
