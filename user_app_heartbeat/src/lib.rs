#![no_std]

use userland::prelude::*;

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
    let start = sys.time_monotonic_ns();
    for _ in 0..10 {
        let now = sys.time_monotonic_ns();
        let _elapsed = now - start;
        println(sys, "heartbeat: tick");
        sys.yield_now();
    }
    sys.exit_thread();
}
