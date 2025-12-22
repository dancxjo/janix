#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::string::String;
#[cfg(not(target_os = "none"))]
use std::string::String;

use abi::ThingId;
use thing_models::AlarmRequest;

use crate::{create_thing, load_thing};

/// Helper wrapper around an alarm Thing that was successfully requested from the kernel.
pub struct Alarm {
    pub id: ThingId,
}

impl Alarm {
    /// Request an alarm triggering at the supplied Unix timestamp.
    pub fn request_at(target_unix_seconds: i64, target_unix_nanos: u32) -> Option<Self> {
        let pending = AlarmRequest {
            id: ThingId(0),
            time_source_id: None,
            target_unix_seconds,
            target_unix_nanos,
            target_ticks: None,
            period_ticks: None,
            owner_process: ThingId(0),
            owner_thread: ThingId(0),
            armed: false,
            fired: false,
        };
        let id = create_thing(&pending)?;
        Some(Alarm { id })
    }

    /// Query the kernel for the current alarm state string.
    pub fn state(&self) -> Option<String> {
        load_thing::<AlarmRequest>(self.id).map(|req| {
            if req.fired {
                String::from("Fired")
            } else if req.armed {
                String::from("Armed")
            } else {
                String::from("Pending")
            }
        })
    }
}

/// Poll an alarm until it fires or is cancelled, sleeping the current thread.
pub fn sleep_until(target_unix_seconds: i64, target_unix_nanos: u32) {
    if let Some(alarm) = Alarm::request_at(target_unix_seconds, target_unix_nanos) {
        loop {
            if let Some(state) = alarm.state() {
                if state == "Fired" || state == "Cancelled" {
                    break;
                }
            }
            crate::time::yield_now();
        }
    }
}
