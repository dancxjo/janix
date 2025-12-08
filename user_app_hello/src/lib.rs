#![no_std]

#[cfg(not(target_os = "none"))]
extern crate alloc;

#[cfg(not(target_os = "none"))]
use alloc::vec::Vec;

use abi::KernelRequest;
use userland_rt::Sys;

#[cfg(not(target_os = "none"))]
use abi::{PropKey, PropType, PropValue, ThingId};

#[cfg(not(target_os = "none"))]
use userland_std::Thing;

// Define the Thing types expected by tests
pub struct ManualCounter {
    pub count: u64,
    pub active: bool,
}

#[cfg(not(target_os = "none"))]
impl Thing for ManualCounter {
    const KIND: &'static str = "demo.ManualCounter";

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

        ManualCounter { count, active }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("count", PropType::U64), ("active", PropType::Bool)]
    }
}

pub struct AutoCounter {
    pub count: u64,
    pub active: bool,
}

#[cfg(not(target_os = "none"))]
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
    // Minimal “demo” logic using what already exists:
    // - Optionally create/load/update a Thing using the existing helpers.
    // - At minimum, prove the path works by logging a message via syscall:
    let _ = sys.syscall(KernelRequest::Log {
        message: "user_app_hello: run() reached",
    });

    // Create a counter to demonstrate functionality
    let _counter = AutoCounter {
        count: 1,
        active: true,
    };

    let _ = sys.syscall(KernelRequest::Log {
        message: "user_app_hello: instantiated AutoCounter",
    });
}
