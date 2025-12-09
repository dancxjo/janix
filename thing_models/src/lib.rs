#![no_std]

extern crate alloc;

use abi::{PropKey, PropType, PropValue, Thing, ThingId};
use alloc::string::String;
use alloc::vec::Vec;

pub struct ThreadInfo {
    pub name: String,  // "hello", "heartbeat", "dashboard"
    pub state: String, // "NEW", "RUNNABLE", "RUNNING", "SLEEPING", "TERMINATED"
    pub last_run_ns: i64,
    pub total_run_ns: i64,
    pub process_thing_id: u64,
    pub scheduler_thing_id: u64,
}

impl Thing for ThreadInfo {
    const KIND: &'static str = "ThreadInfo";
    const DESCRIPTION: &'static str =
        "Runtime information about a thread including state, execution time, and owning process";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("name", PropValue::Str(self.name.clone())));
        out.push(("state", PropValue::Str(self.state.clone())));
        out.push(("last_run_ns", PropValue::I64(self.last_run_ns)));
        out.push(("total_run_ns", PropValue::I64(self.total_run_ns)));
        out.push(("process_thing_id", PropValue::U64(self.process_thing_id)));
        out.push((
            "scheduler_thing_id",
            PropValue::U64(self.scheduler_thing_id),
        ));
    }

    fn from_props(_id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        use alloc::string::String;

        let mut name = String::new();
        let mut state = String::new();
        let mut last_run_ns = 0_i64;
        let mut total_run_ns = 0_i64;
        let mut process_thing_id = 0_u64;
        let mut scheduler_thing_id = 0_u64;

        for prop in props {
            if let Some((k, v)) = prop {
                match *k {
                    "name" => {
                        if let PropValue::Str(s) = v {
                            name = s.clone();
                        }
                    }
                    "state" => {
                        if let PropValue::Str(s) = v {
                            state = s.clone();
                        }
                    }
                    "last_run_ns" => {
                        if let PropValue::I64(val) = v {
                            last_run_ns = *val;
                        }
                    }
                    "total_run_ns" => {
                        if let PropValue::I64(val) = v {
                            total_run_ns = *val;
                        }
                    }
                    "process_thing_id" => {
                        if let PropValue::U64(val) = v {
                            process_thing_id = *val;
                        }
                    }
                    "scheduler_thing_id" => {
                        if let PropValue::U64(val) = v {
                            scheduler_thing_id = *val;
                        }
                    }
                    _ => {}
                }
            }
        }

        ThreadInfo {
            name,
            state,
            last_run_ns,
            total_run_ns,
            process_thing_id,
            scheduler_thing_id,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("name", PropType::Str),
            ("state", PropType::Str),
            ("last_run_ns", PropType::I64),
            ("total_run_ns", PropType::I64),
            ("process_thing_id", PropType::U64),
            ("scheduler_thing_id", PropType::U64),
        ]
    }
}
