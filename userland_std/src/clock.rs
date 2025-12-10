#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::vec::Vec;
#[cfg(not(target_os = "none"))]
use std::vec::Vec;

use abi::ThingId;
use thing_models::TimeSource;
use userland_rt::Sys;

use crate::{list_things_by_kind, load_thing};

/// Wraps a kernel `TimeSource` Thing so callers can read wall-clock values.
pub struct SystemClock {
    time_source_id: ThingId,
}

impl SystemClock {
    /// Discover the first published `TimeSource`.
    ///
    /// # Examples
    ///
    /// ```
    /// use abi::{KernelResponse, PropValue, ThingId};
    /// use thing_models::TimeSource;
    /// use userland_std::{clock::SystemClock, doc_helpers::DocSys, Thing};
    ///
    /// let props = DocSys::props_slice(vec![
    ///     ("ticks_since_boot", PropValue::U64(100)),
    ///     ("tick_hz", PropValue::U64(1000)),
    ///     ("unix_seconds", PropValue::I64(42)),
    ///     ("unix_nanos", PropValue::U64(10)),
    /// ]);
    /// let mut sys = DocSys::with_responses(vec![
    ///     KernelResponse::ThingListEntry { id: Some(ThingId(1)) },
    ///     KernelResponse::ThingData {
    ///         id: ThingId(1),
    ///         kind: TimeSource::KIND,
    ///         props,
    ///     },
    ///     KernelResponse::ThingListEntry { id: None },
    ///     KernelResponse::ThingData {
    ///         id: ThingId(1),
    ///         kind: TimeSource::KIND,
    ///         props,
    ///     },
    /// ]);
    /// let clock = SystemClock::discover(&mut sys).unwrap();
    /// assert_eq!(clock.now(&mut sys), (42, 10));
    /// ```
    pub fn discover<S: Sys>(sys: &mut S) -> Option<Self> {
        let sources: Vec<TimeSource> = list_things_by_kind(sys);
        sources.into_iter().next().map(|source| SystemClock {
            time_source_id: source.id,
        })
    }

    /// Return the current Unix time reported by the kernel.
    pub fn now<S: Sys>(&self, sys: &mut S) -> (i64, u32) {
        load_thing::<TimeSource>(sys, self.time_source_id)
            .map(|ts| (ts.unix_seconds, ts.unix_nanos))
            .unwrap_or((0, 0))
    }

    /// Return the monotonic tick counter published by the time source.
    pub fn uptime_ticks<S: Sys>(&self, sys: &mut S) -> u64 {
        load_thing::<TimeSource>(sys, self.time_source_id)
            .map(|ts| ts.ticks_since_boot)
            .unwrap_or(0)
    }

    /// Return the tickfrequency for the underlying source.
    pub fn tick_hz<S: Sys>(&self, sys: &mut S) -> u32 {
        load_thing::<TimeSource>(sys, self.time_source_id)
            .map(|ts| ts.tick_hz)
            .unwrap_or(0)
    }
}
