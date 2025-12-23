extern crate alloc;

use crate::graph::{self, Graph};
use crate::graph_kinds;
use crate::sched::types::{CpuId, ThreadState, TimeNs};
use abi::ThingId;
use alloc::string::String;
use thing_models::PropValue;

const LINK_BUF: usize = 4;

/// Minimal scheduler tick backed by graph state.
/// For now, it keeps running the currently assigned thread (if any) or
/// promotes the first runnable Thread Thing it can find.
pub fn sched_tick(graph: &mut Graph, cpu: CpuId, now: TimeNs) {
    let Some(cpu_node) = find_cpu(cpu) else {
        return;
    };

    let current = find_thread_on_cpu(graph, cpu_node);
    if let Some(thread) = current {
        update_runtime(graph, thread, now);
    }

    let next = current.or_else(|| pick_first_runnable(graph));
    if let Some(thread) = next {
        make_thread_current(graph, thread, cpu_node, now);
    }
}

fn find_cpu(cpu_index: CpuId) -> Option<ThingId> {
    let mut found = None;
    graph::iter_things(|thing| {
        if found.is_some() || thing.kind != crate::symbols::intern(graph_kinds::KIND_CPU_CORE) {
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

fn find_thread_on_cpu(graph: &Graph, cpu_node: ThingId) -> Option<ThingId> {
    let mut current = None;
    graph::iter_things(|thing| {
        if current.is_some() || thing.kind != crate::symbols::intern(graph_kinds::KIND_THREAD) {
            return;
        }

        let mut buf = [None; LINK_BUF];
        graph.neighbors(thing.id, graph_kinds::LINK_RUNS_ON, &mut buf);
        if buf.into_iter().flatten().any(|cpu| cpu == cpu_node) {
            current = Some(thing.id);
        }
    });
    current
}

fn pick_first_runnable(_graph: &Graph) -> Option<ThingId> {
    let mut next = None;
    graph::iter_things(|thing| {
        if next.is_some() || thing.kind != crate::symbols::intern(graph_kinds::KIND_THREAD) {
            return;
        }

        let state = thread_state(thing.id);
        if matches!(
            state,
            Some(ThreadState::Running) | Some(ThreadState::Runnable) | Some(ThreadState::New)
        ) {
            next = Some(thing.id);
        }
    });
    next
}

fn thread_state(id: ThingId) -> Option<ThreadState> {
    graph::get_prop(id, "state").and_then(|v| match v {
        PropValue::Str(s) => ThreadState::from_str(s.as_str()),
        _ => None,
    })
}

fn read_u64_prop(id: ThingId, key: &'static str) -> Option<u64> {
    graph::get_prop(id, key).and_then(|v| match v {
        PropValue::U64(val) => Some(val),
        _ => None,
    })
}

fn update_runtime(graph: &mut Graph, thread: ThingId, now: TimeNs) {
    let runtime = read_u64_prop(thread, "runtime_ns").unwrap_or(0);
    let last_started = read_u64_prop(thread, "last_started_ns").unwrap_or(now);
    let delta = now.saturating_sub(last_started);
    if delta == 0 {
        return;
    }
    let new_runtime = runtime.saturating_add(delta);
    let props = alloc::vec![
        (
            crate::symbols::intern("runtime_ns"),
            PropValue::U64(new_runtime)
        ),
        (
            crate::symbols::intern("last_started_ns"),
            PropValue::U64(now)
        ),
    ];
    graph.update_thing(thread, props.as_slice());
}

fn make_thread_current(graph: &mut Graph, thread: ThingId, cpu_node: ThingId, now: TimeNs) {
    clear_cpu_assignments(graph, cpu_node, thread);
    let props = alloc::vec![
        (
            crate::symbols::intern("state"),
            PropValue::Str(String::from(ThreadState::Running.as_str())),
        ),
        (
            crate::symbols::intern("last_started_ns"),
            PropValue::U64(now)
        ),
    ];
    graph.update_thing(thread, props.as_slice());
    ensure_runs_on_link(graph, thread, cpu_node);
}

fn clear_cpu_assignments(graph: &mut Graph, cpu_node: ThingId, preserve: ThingId) {
    graph::iter_things(|thing| {
        if thing.kind != crate::symbols::intern(graph_kinds::KIND_THREAD) || thing.id == preserve {
            return;
        }
        let mut buf = [None; LINK_BUF];
        graph.neighbors(thing.id, graph_kinds::LINK_RUNS_ON, &mut buf);
        for cpu in buf.into_iter().flatten() {
            if cpu == cpu_node {
                graph.remove_link(thing.id, graph_kinds::LINK_RUNS_ON, cpu_node);
            }
        }
    });
}

fn ensure_runs_on_link(graph: &mut Graph, thread: ThingId, cpu_node: ThingId) {
    let mut buf = [None; LINK_BUF];
    graph.neighbors(thread, graph_kinds::LINK_RUNS_ON, &mut buf);
    let already = buf.into_iter().flatten().any(|cpu| cpu == cpu_node);
    if !already {
        graph.add_link(thread, graph_kinds::LINK_RUNS_ON, cpu_node);
    }
}
