#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

use abi::{PropKey, PropType, PropValue, ThingId};
use userland_rt::Sys;
use userland_std::{
    Thing, create_process, create_thing, create_thread, println, register_schema_for,
    demo_shared::DemoState,
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

pub fn run<S: Sys>(sys: &S) {
    println(sys, "user_app_heartbeat: run() reached");

    register_schema_for::<HeartbeatThing>(sys);

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

static mut COUNTER: u64 = 0;

pub fn tick<S: Sys>(sys: &S) {
    unsafe {
        COUNTER += 1;
        let c = COUNTER;
        
        if let Some(demo) = DemoState::get_or_create(sys) {
             if let Some((hello, hb)) = demo.read(sys) {
                 let new_hb = hb + 1;
                 demo.update_heartbeat_ticks(sys, new_hb);
                 
                 if new_hb % 7 == 0 {
                     let msg = format!("user_app_heartbeat: shared state -> hello_ticks={} heartbeat_ticks={}", hello, new_hb);
                     let leaked: &'static str = Box::leak(msg.into_boxed_str());
                     println(sys, leaked);
                 }
             }
        }

        if c % 10 == 0 {
            let thing = HeartbeatThing { counter: c };
            if let Some(id) = create_thing(sys, &thing) {
                let msg = format!("user_app_heartbeat: recorded heartbeat Thing {:?}", id);
                let leaked: &'static str = Box::leak(msg.into_boxed_str());
                println(sys, leaked);
            }
        }
    }
}
