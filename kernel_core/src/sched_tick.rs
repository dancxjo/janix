use crate::graph::{self, Graph};
use crate::graph_kinds;
use crate::println;
use crate::sched_types::{CpuId, ThreadState, TimeNs};
use abi::{PropValue, ThingId};

const EDGE_BUF: usize = 4;

/// Minimal scheduler tick that only touches the graph model.
/// It bumps the runtime of the Running thread on `cpu` and refreshes its
/// `last_started_ns` timestamp. No preemption or selection occurs here.
pub fn sched_tick(graph: &mut Graph, cpu: CpuId, now: TimeNs) {
    let Some(cpu_node) = find_cpu(cpu) else {
        return;
    };

    if let Some(thread) = find_running_thread(graph, cpu_node) {
        let runtime = read_u64_prop(thread, "runtime_ns");
        let last_started = read_u64_prop(thread, "last_started_ns");
        let delta = now.saturating_sub(last_started);
        let new_runtime = runtime.saturating_add(delta);

        graph.update_thing(
            thread,
            &[
                ("runtime_ns", PropValue::U64(new_runtime)),
                ("last_started_ns", PropValue::U64(now)),
            ],
        );

        println!(
            "sched_tick: cpu={} thread={} runtime_ns={} (+{})",
            cpu, thread.0, new_runtime, delta
        );
    }
}

fn read_u64_prop(id: ThingId, key: &'static str) -> u64 {
    graph::get_prop(id, key)
        .and_then(|v| match v {
            PropValue::U64(val) => Some(val),
            _ => None,
        })
        .unwrap_or(0)
}

fn find_cpu(cpu_index: CpuId) -> Option<ThingId> {
    let mut found = None;
    graph::iter_things(|thing| {
        if found.is_some() || thing.kind != graph_kinds::KIND_CPU_CORE {
            return;
        }
        if let Some(PropValue::U64(idx)) = graph::get_prop(thing.id, "index") {
            if idx == cpu_index {
                found = Some(thing.id);
            }
        }
    });
    found
}

fn find_running_thread(graph: &Graph, cpu_node: ThingId) -> Option<ThingId> {
    let mut running = None;
    graph::iter_things(|thing| {
        if running.is_some() || thing.kind != graph_kinds::KIND_THREAD {
            return;
        }

        if !is_running(thing.id) {
            return;
        }

        let mut buf = [None; EDGE_BUF];
        graph.neighbors(thing.id, graph_kinds::EDGE_RUNS_ON, &mut buf);
        if buf.into_iter().flatten().any(|id| id == cpu_node) {
            running = Some(thing.id);
        }
    });
    running
}

fn is_running(thread: ThingId) -> bool {
    graph::get_prop(thread, "state").map_or(false, |v| match v {
        PropValue::Str(s) => ThreadState::from_str(s.as_str()) == Some(ThreadState::Running),
        _ => false,
    })
}
