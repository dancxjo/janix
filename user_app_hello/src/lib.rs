#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use alloc::boxed::Box;

use abi::{PropKey, PropType, PropValue, ThingId};
use userland_rt::Sys;
use userland_std::{
    Thing, alloc_frame, create_process, create_thing, create_thread, free_frame, println,
    register_schema_for, demo_shared::DemoState, time::SystemTime,
};

fn log_dynamic(sys: &impl Sys, msg: String) {
    // LEAK: For demo purposes only. In a real system, we'd fix the ABI to allow non-static strings.
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    println(sys, leaked);
}

pub struct AutoCounter {
    pub count: u64,
    pub active: bool,
}

impl Thing for AutoCounter {
    const KIND: &'static str = "AutoCounter";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("count", PropValue::U64(self.count)));
        out.push(("active", PropValue::Bool(self.active)));
    }

    fn from_props(_id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut count = 0;
        let mut active = false;

        for prop in props {
            if let Some((k, v)) = prop {
                match *k {
                    "count" => {
                        if let PropValue::U64(val) = v {
                            count = *val
                        }
                    }
                    "active" => {
                        if let PropValue::Bool(val) = v {
                            active = *val
                        }
                    }
                    _ => {}
                }
            }
        }

        AutoCounter { count, active }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("count", PropType::U64), ("active", PropType::Bool)]
    }
}


pub fn run<S: Sys>(sys: &mut S) {
    println(sys, "user_app_hello: run() reached");
    let start = sys.time_monotonic_ns();
    for i in 0..10 {
        let now = sys.time_monotonic_ns();
        let elapsed = now - start;
        // Simple log for now
        println(sys, "hello: tick");
        sys.yield_now();
    }
    sys.exit_thread();
}

