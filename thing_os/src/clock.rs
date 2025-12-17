#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::vec::Vec;
#[cfg(not(target_os = "none"))]
use std::vec::Vec;

use abi::ThingId;
use thing_models::TimeSource;

use crate::{list_things_by_kind, load_thing};

/// Wraps a kernel `TimeSource` Thing so callers can read wall-clock values.
pub struct SystemClock {
    time_source_id: ThingId,
}

impl SystemClock {
    /// Discover the first published `TimeSource`.
    pub fn discover() -> Option<Self> {
        let sources: Vec<TimeSource> = list_things_by_kind();
        sources.into_iter().next().map(|source| SystemClock {
            time_source_id: source.id,
        })
    }

    /// Return the current Unix time reported by the kernel.
    pub fn now(&self) -> (i64, u32) {
        load_thing::<TimeSource>(self.time_source_id)
            .map(|ts| (ts.unix_seconds, ts.unix_nanos))
            .unwrap_or((0, 0))
    }

    /// Return the monotonic tick counter published by the time source.
    pub fn uptime_ticks(&self) -> u64 {
        load_thing::<TimeSource>(self.time_source_id)
            .map(|ts| ts.ticks_since_boot)
            .unwrap_or(0)
    }

    /// Return the tickfrequency for the underlying source.
    pub fn tick_hz(&self) -> u32 {
        load_thing::<TimeSource>(self.time_source_id)
            .map(|ts| ts.tick_hz)
            .unwrap_or(0)
    }
}
