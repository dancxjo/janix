#![no_std]

use userland::prelude::*;

pub struct AutoCounter {
    pub count: u64,
    pub active: bool,
}

impl Thing for AutoCounter {
    const KIND: &'static str = "AutoCounter";
    const DESCRIPTION: &'static str =
        "An automatically incrementing counter with active/inactive state";

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

fn log_boot_graph_view<S: Sys>(sys: &mut S) {
    log_dynamic(sys, "=== Boot graph snapshot from userland ===".into());

    let cpus: Vec<CpuCoreThing> = list_things_by_kind::<S, CpuCoreThing>(sys);
    if cpus.is_empty() {
        log_dynamic(sys, "  (no CpuCore Things found)".into());
    } else {
        for cpu in cpus {
            log_dynamic(
                sys,
                format!(
                    "  CpuCore(id={}, index={}): {}",
                    cpu.id.0,
                    cpu.index,
                    CpuCoreThing::DESCRIPTION,
                ),
            );
        }
    }

    let processes: Vec<ProcessThing> = list_things_by_kind::<S, ProcessThing>(sys);
    if processes.is_empty() {
        log_dynamic(sys, "  (no Process Things found)".into());
    } else {
        for proc in processes {
            log_dynamic(
                sys,
                format!(
                    "  Process(id={}, pid={}): {}",
                    proc.id.0,
                    proc.pid,
                    ProcessThing::DESCRIPTION,
                ),
            );
        }
    }

    let threads: Vec<ThreadThing> = list_things_by_kind::<S, ThreadThing>(sys);
    if threads.is_empty() {
        log_dynamic(sys, "  (no Thread Things found)".into());
    } else {
        for thr in threads {
            log_dynamic(
                sys,
                format!(
                    "  Thread(id={}, tid={}, state={}, prio={}, runtime_ns={}, last_started_ns={}): {}",
                    thr.id.0,
                    thr.tid,
                    thr.state,
                    thr.priority,
                    thr.runtime_ns,
                    thr.last_started_ns,
                    ThreadThing::DESCRIPTION,
                ),
            );
        }
    }
}

pub fn run<S: Sys>(sys: &mut S) {
    println(sys, "user_app_hello: run() reached");
    log_boot_graph_view(sys);
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
