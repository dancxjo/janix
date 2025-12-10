#![cfg_attr(target_os = "none", no_std)]

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

pub struct Alarm {
    pub id: ThingId,
}

impl Alarm {
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

    pub fn state<S: Sys>(&self, sys: &mut S) -> Option<String> {
        load_thing::<AlarmRequest>(sys, self.id).map(|req| req.state)
    }
}

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
