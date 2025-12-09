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
    register_schema_for, demo_shared::DemoState,
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

pub fn run<S: Sys>(sys: &S) {
    println(sys, "user_app_hello: run() reached");

    // Exercise alloc_frame / free_frame
    if let Some(frame) = alloc_frame(sys) {
        println(sys, "user_app_hello: allocated frame");
        if free_frame(sys, frame.id) {
            println(sys, "user_app_hello: freed frame");
        } else {
            println(sys, "user_app_hello: failed to free frame");
        }
    } else {
        println(sys, "user_app_hello: failed to allocate frame");
    }

    // Create a Process / Thread
    if create_process(sys, 100) {
        println(sys, "user_app_hello: created process 100");
        if create_thread(sys, 100, 101, 1) {
            println(sys, "user_app_hello: created thread 101 in process 100");
        } else {
            println(sys, "user_app_hello: failed to create thread");
        }
    } else {
        println(sys, "user_app_hello: failed to create process");
    }

    // Create a counter to demonstrate functionality
    register_schema_for::<AutoCounter>(sys);
    let counter = AutoCounter {
        count: 1,
        active: true,
    };

    if let Some(_) = create_thing(sys, &counter) {
        println(sys, "user_app_hello: instantiated AutoCounter");
    } else {
        println(sys, "user_app_hello: failed to instantiate AutoCounter");
    }

    println(sys, "user_app_hello: finished setup");
}

pub fn tick<S: Sys>(sys: &S) {
    // println(sys, "user_app_hello: scheduler tick");

    if let Some(demo) = DemoState::get_or_create(sys) {
        if let Some((hello, hb)) = demo.read(sys) {
            let new_hello = hello + 1;
            demo.update_hello_ticks(sys, new_hello);

            if new_hello % 10 == 0 {
                let msg = format!(
                    "user_app_hello: shared state -> hello_ticks={} heartbeat_ticks={}",
                    new_hello, hb
                );
                log_dynamic(sys, msg);
            }
        }
    }
}
