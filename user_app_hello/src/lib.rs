#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use abi::{PropKey, PropType, PropValue, ThingId};
use userland_rt::Sys;
use userland_std::{println, alloc_frame, free_frame, scheduler_tick, create_process, create_thread, Thing, create_thing};

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

    // Call scheduler_tick a few times
    for _ in 0..3 {
        if let Some(_thread) = scheduler_tick(sys) {
             println(sys, "user_app_hello: scheduler tick");
        } else {
             println(sys, "user_app_hello: scheduler tick (no thread)");
        }
    }

    // Create a counter to demonstrate functionality
    let counter = AutoCounter {
        count: 1,
        active: true,
    };
    
    if let Some(_) = create_thing(sys, &counter) {
        println(sys, "user_app_hello: instantiated AutoCounter");
    } else {
        println(sys, "user_app_hello: failed to instantiate AutoCounter");
    }

    println(sys, "user_app_hello: finished");
}
