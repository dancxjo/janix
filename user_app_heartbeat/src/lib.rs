#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

use abi::{PropKey, PropType, PropValue, ThingId};
use userland_rt::Sys;
use userland_std::{
    Thing, create_process, create_thread, println, register_schema_for,
    time::Instant,
};

pub struct HeartbeatThing {
    pub counter: u64,
}

impl Thing for HeartbeatThing {
    const KIND: &'static str = "Heartbeat";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("counter", PropValue::U64(self.counter)));
    }

    fn from_props(_id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut counter = 0;
        for prop in props {
            if let Some((k, v)) = prop {
                if *k == "counter" {
                    if let PropValue::U64(val) = v {
                        counter = *val;
                    }
                }
            }
        }
        HeartbeatThing { counter }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("counter", PropType::U64)]
    }
}

pub fn run<S: Sys>(sys: &mut S) {
    println(sys, "user_app_heartbeat: run() reached");

    register_schema_for::<HeartbeatThing>(sys);
    
    unsafe { START_TIME = Some(Instant::now(sys)); }

    let pid = 200;
    let tid = 201;

    if create_process(sys, pid) {
        println(sys, "user_app_heartbeat: created process 200");
        if create_thread(sys, pid, tid, 1) {
            println(sys, "user_app_heartbeat: created thread 201 in process 200");
        } else {
            println(sys, "user_app_heartbeat: failed to create thread");
        }
    } else {
        println(sys, "user_app_heartbeat: failed to create process");
    }
}

static mut START_TIME: Option<Instant> = None;

pub fn tick<S: Sys>(sys: &mut S) {
    let start = unsafe { START_TIME };
    if let Some(start) = start {
        let now = Instant::now(sys);
        let elapsed = now.duration_since(start);
        
        let msg = format!("heartbeat: {} ms since start", elapsed.as_nanos() / 1_000_000);
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        println(sys, leaked);
    }
}

