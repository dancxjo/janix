#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::string::String;
#[cfg(not(target_os = "none"))]
use std::string::String;

use abi::ThingId;
use thing_models::AlarmRequest;
use userland_rt::Sys;

use crate::{create_thing, load_thing};

/// Helper wrapper around an alarm Thing that was successfully requested from the kernel.
pub struct Alarm {
    pub id: ThingId,
}

impl Alarm {
    /// Request an alarm triggering at the supplied Unix timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use abi::{KernelResponse, ThingId};
    /// use userland_std::{alarm::Alarm, doc_helpers::DocSys};
    ///
    /// let mut sys = DocSys::with_responses(vec![KernelResponse::ThingCreated {
    ///     id: ThingId(2),
    /// }]);
    /// let alarm = Alarm::request_at(&mut sys, 0, 0);
    /// assert_eq!(alarm.unwrap().id, ThingId(2));
    /// ```
    pub fn request_at<S: Sys>(
        sys: &mut S,
        target_unix_seconds: i64,
        target_unix_nanos: u32,
    ) -> Option<Self> {
        let pending = AlarmRequest {
            id: ThingId(0),
            target_unix_seconds,
            target_unix_nanos,
            owner_process: ThingId(0),
            owner_thread: ThingId(0),
            state: String::from("Pending"),
            target_ticks: None,
        };
        let id = create_thing(sys, &pending)?;
        Some(Alarm { id })
    }

    /// Query the kernel for the current alarm state string.
    ///
    /// # Examples
    ///
    /// ```
    /// use abi::{KernelResponse, PropValue, ThingId};
    /// use thing_models::AlarmRequest;
    /// use userland_std::{alarm::Alarm, doc_helpers::DocSys, Thing};
    ///
    /// let props = DocSys::props_slice(vec![("state", PropValue::Str("Fired".into()))]);
    /// let mut sys = DocSys::with_responses(vec![KernelResponse::ThingData {
    ///     id: ThingId(3),
    ///     kind: AlarmRequest::KIND,
    ///     props,
    /// }]);
    /// let alarm = Alarm { id: ThingId(3) };
    /// assert_eq!(alarm.state(&mut sys), Some("Fired".to_string()));
    /// ```
    pub fn state<S: Sys>(&self, sys: &mut S) -> Option<String> {
        load_thing::<AlarmRequest>(sys, self.id).map(|req| req.state)
    }
}

/// Poll an alarm until it fires or is cancelled, sleeping the current thread.
pub fn sleep_until<S: Sys>(sys: &mut S, target_unix_seconds: i64, target_unix_nanos: u32) {
    if let Some(alarm) = Alarm::request_at(sys, target_unix_seconds, target_unix_nanos) {
        loop {
            if let Some(state) = alarm.state(sys) {
                if state == "Fired" || state == "Cancelled" {
                    break;
                }
            }
            sys.yield_now();
        }
    }
}
