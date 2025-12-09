#![no_std]

use userland::prelude::*;

/// Optional: an example Thing you can delete or repurpose.
pub struct ExampleThing {
    pub counter: u64,
    pub active: bool,
}

impl Thing for ExampleThing {
    const KIND: &'static str = "ExampleThing";
    const DESCRIPTION: &'static str = "An example Thing demonstrating counter and active state tracking";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("counter", PropValue::U64(self.counter)));
        out.push(("active", PropValue::Bool(self.active)));
    }

    fn from_props(_id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut counter = 0;
        let mut active = false;

        for prop in props {
            if let Some((k, v)) = prop {
                match *k {
                    "counter" => {
                        if let PropValue::U64(val) = v {
                            counter = *val;
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

        ExampleThing { counter, active }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("counter", PropType::U64),
            ("active", PropType::Bool),
        ]
    }
}

/// Entry point for this userland app.
///
/// The kernel will create a process + thread that calls this.
pub fn run<S: Sys>(sys: &mut S) {
    println(sys, "{{ crate_name }}: run() reached");

    // Example of time usage
    let start = sys.time_monotonic_ns();
    for i in 0..10 {
        let now = sys.time_monotonic_ns();
        let elapsed_ms = (now - start) / 1_000_000;

        log_dynamic(
            sys,
            format!(
                "{{ crate_name }}: tick {} ({} ms since start)",
                i,
                elapsed_ms
            ),
        );

        sys.yield_now();
    }

    sys.exit_thread();
}
