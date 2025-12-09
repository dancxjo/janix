use crate::graph::{self, Graph};
use crate::graph_kinds;
use crate::sched_types::{CpuId, ThreadState, TimeNs};
use abi::{PropValue, ThingId};
use alloc::string::String;

const TIME_SLICE_NS: TimeNs = 5_000_000;
const EDGE_BUF: usize = 4;

/// Graph-driven scheduler tick.
/// Updates runtime accounting for the current thread on `cpu` and selects the
/// next runnable thread based purely on graph state.
pub fn sched_tick(graph: &mut Graph, cpu: CpuId, now: TimeNs) -> Option<ThingId> {
    let cpu_node = find_cpu_node(cpu)?;
    let current = find_current_thread(graph, cpu_node);
    let mut preempted = None;

    if let Some(tid) = current {
        let elapsed = update_runtime(graph, tid, now);
        // If the slice is not exhausted, keep running the same thread.
        if elapsed < TIME_SLICE_NS {
            set_last_started(graph, tid, now);
            return Some(tid);
        }

        make_runnable(graph, tid, now);
        graph.remove_edge(tid, graph_kinds::EDGE_RUNS_ON, cpu_node);
        preempted = Some(tid);
    }

    let next = match pick_next_runnable(graph, preempted) {
        Some(n) => n,
        None => return None,
    };
    start_running(graph, next, cpu_node, now);
    Some(next)
}

/// Create a SleepEvent node and link it from the thread.
pub fn create_sleep_event(
    graph: &mut Graph,
    thread: ThingId,
    wake_at_ns: TimeNs,
    created_at_ns: TimeNs,
) -> Option<ThingId> {
    let props = &[
        ("wake_at_ns", PropValue::U64(wake_at_ns)),
        ("created_at_ns", PropValue::U64(created_at_ns)),
    ];
    let sleep = graph.create_thing(graph_kinds::KIND_SLEEP_EVENT, props)?;
    graph.add_edge(thread, graph_kinds::EDGE_SLEEPS_UNTIL, sleep);
    Some(sleep)
}

/// Remove the SleepEvent node for a thread, if present.
pub fn clear_sleep_event(graph: &mut Graph, thread: ThingId) {
    let mut buf = [None; EDGE_BUF];
    graph.neighbors(thread, graph_kinds::EDGE_SLEEPS_UNTIL, &mut buf);
    for ev in buf.into_iter().flatten() {
        graph.remove_edge(thread, graph_kinds::EDGE_SLEEPS_UNTIL, ev);
        let _ = graph::delete_thing(ev);
    }
}

fn find_cpu_node(cpu_index: CpuId) -> Option<ThingId> {
    let mut found = None;
    graph::iter_things(|thing| {
        if found.is_some() {
            return;
        }
        if thing.kind != graph_kinds::KIND_CPU_CORE {
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

fn find_current_thread(graph: &Graph, cpu_node: ThingId) -> Option<ThingId> {
    let mut current = None;
    graph::iter_things(|thing| {
        if current.is_some() || thing.kind != graph_kinds::KIND_THREAD {
            return;
        }
        let mut out = [None; EDGE_BUF];
        graph.neighbors(thing.id, graph_kinds::EDGE_RUNS_ON, &mut out);
        if out.into_iter().flatten().any(|cpu| cpu == cpu_node) {
            current = Some(thing.id);
        }
    });
    current
}

fn thread_state(id: ThingId) -> Option<ThreadState> {
    graph::get_prop(id, "state").and_then(|v| match v {
        PropValue::Str(s) => ThreadState::from_str(s.as_str()),
        _ => None,
    })
}

fn read_u64_prop(id: ThingId, key: &'static str) -> Option<u64> {
    graph::get_prop(id, key).and_then(|v| match v {
        PropValue::U64(v) => Some(v),
        _ => None,
    })
}

fn update_runtime(graph: &mut Graph, thread: ThingId, now: TimeNs) -> TimeNs {
    let runtime = read_u64_prop(thread, "runtime_ns").unwrap_or(0);
    let last_started = read_u64_prop(thread, "last_started_ns").unwrap_or(now);
    let delta = now.saturating_sub(last_started);
    let new_runtime = runtime.saturating_add(delta);
    graph.update_thing(
        thread,
        &[
            ("runtime_ns", PropValue::U64(new_runtime)),
            ("last_started_ns", PropValue::U64(now)),
        ],
    );
    delta
}

fn set_last_started(graph: &mut Graph, thread: ThingId, now: TimeNs) {
    graph.update_thing(
        thread,
        &[("last_started_ns", PropValue::U64(now))],
    );
}

fn make_runnable(graph: &mut Graph, thread: ThingId, now: TimeNs) {
    graph.update_thing(
        thread,
        &[
            (
                "state",
                PropValue::Str(String::from(ThreadState::Runnable.as_str())),
            ),
            ("last_started_ns", PropValue::U64(now)),
        ],
    );
}

fn start_running(graph: &mut Graph, thread: ThingId, cpu_node: ThingId, now: TimeNs) {
    clear_cpu_assignments(graph, cpu_node);
    graph.update_thing(
        thread,
        &[
            (
                "state",
                PropValue::Str(String::from(ThreadState::Running.as_str())),
            ),
            ("last_started_ns", PropValue::U64(now)),
        ],
    );
    graph.add_edge(thread, graph_kinds::EDGE_RUNS_ON, cpu_node);
}

fn clear_cpu_assignments(graph: &mut Graph, cpu_node: ThingId) {
    graph::iter_things(|thing| {
        if thing.kind != graph_kinds::KIND_THREAD {
            return;
        }
        let mut out = [None; EDGE_BUF];
        graph.neighbors(thing.id, graph_kinds::EDGE_RUNS_ON, &mut out);
        for cpu in out.into_iter().flatten() {
            if cpu == cpu_node {
                graph.remove_edge(thing.id, graph_kinds::EDGE_RUNS_ON, cpu_node);
            }
        }
    });
}

fn pick_next_runnable(_graph: &Graph, skip: Option<ThingId>) -> Option<ThingId> {
    let mut best: Option<(ThingId, u64, u64)> = None; // (id, priority, runtime)

    graph::iter_things(|thing| {
        if thing.kind != graph_kinds::KIND_THREAD {
            return;
        }

        if let Some(skip_id) = skip {
            if thing.id == skip_id {
                return;
            }
        }

        match thread_state(thing.id) {
            Some(ThreadState::Runnable) | Some(ThreadState::New) => {}
            _ => return,
        }

        let priority = read_u64_prop(thing.id, "priority").unwrap_or(0);
        let runtime = read_u64_prop(thing.id, "runtime_ns").unwrap_or(0);

        match best {
            None => best = Some((thing.id, priority, runtime)),
            Some((_, best_prio, best_runtime)) => {
                if priority > best_prio || (priority == best_prio && runtime < best_runtime) {
                    best = Some((thing.id, priority, runtime));
                }
            }
        }
    });

    best.map(|(id, _, _)| id)
}
