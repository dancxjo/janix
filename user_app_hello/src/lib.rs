#![no_std]

use userland::prelude::*;

pub struct AutoCounter {
    pub count: u64,
    pub active: bool,
}

impl Thing for AutoCounter {
    const KIND: &'static str = "AutoCounter";
    const DESCRIPTION: &'static str = "An automatically incrementing counter with active/inactive state";

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
                            count = *val;
                        }
                    }
                    "active" => {
                        if let PropValue::Bool(val) = v {
                            active = *val;
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
    for _ in 0..10 {
        let now = sys.time_monotonic_ns();
        let _elapsed = now - start;
        // Simple log for now
        println(sys, "hello: tick");
        sys.yield_now();
    }
    sys.exit_thread();
}
