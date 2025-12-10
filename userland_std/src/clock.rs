#![cfg_attr(target_os = "none", no_std)]

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

pub struct SystemClock {
    time_source_id: ThingId,
}

impl SystemClock {
    pub fn discover<S: Sys>(sys: &mut S) -> Option<Self> {
        let sources: Vec<TimeSource> = list_things_by_kind(sys);
        sources.into_iter().next().map(|source| SystemClock {
            time_source_id: source.id,
        })
    }

    pub fn now<S: Sys>(&self, sys: &mut S) -> (i64, u32) {
        load_thing::<TimeSource>(sys, self.time_source_id)
            .map(|ts| (ts.unix_seconds, ts.unix_nanos))
            .unwrap_or((0, 0))
    }

    pub fn uptime_ticks<S: Sys>(&self, sys: &mut S) -> u64 {
        load_thing::<TimeSource>(sys, self.time_source_id)
            .map(|ts| ts.ticks_since_boot)
            .unwrap_or(0)
    }

    pub fn tick_hz<S: Sys>(&self, sys: &mut S) -> u32 {
        load_thing::<TimeSource>(sys, self.time_source_id)
            .map(|ts| ts.tick_hz)
            .unwrap_or(0)
    }
}
