use crate::graph::{self, Graph};
use crate::graph_kinds;
use crate::sched_types::{CpuId, ThreadState, TimeNs};
use abi::ThingId;
use abi::PropValue;
use alloc::string::String;

const TIME_SLICE_NS: TimeNs = 5_000_000;
const LINK_BUF: usize = 4;

/// Graph-driven scheduler tick.
///
/// Updates runtime accounting for the current thread on `cpu` and selects the
/// next runnable thread based purely on graph state.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let cpu = k::model::create_cpu_core(0).unwrap();
/// let thread = k::model::create_thread(1, 3).unwrap();
/// let mut g = k::graph::Graph::new();
///
/// let picked = k::sched_graph::sched_tick(&mut g, 0, 10).unwrap();
/// assert_eq!(picked, thread);
///
/// // The thread should now be marked running on the CPU.
/// let mut buf = [None; 1];
/// k::graph::neighbors(thread, k::graph_kinds::LINK_RUNS_ON, &mut buf);
/// assert!(buf.into_iter().flatten().any(|id| id == cpu));
/// ```
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
        graph.remove_link(tid, graph_kinds::LINK_RUNS_ON, cpu_node);
        preempted = Some(tid);
    }

    let next = match pick_next_runnable(graph, preempted) {
        Some(n) => n,
        None => return None,
    };
    start_running(graph, next, cpu_node, now);
    Some(next)
}

/// Update the thread's sleep_until_ns property.
///
/// Returns the thread ThingId on success.
///
/// # Examples
/// ```
/// # use kernel as k;
/// # let _guard = k::test_lock();
/// k::init();
/// let thread = k::model::create_thread(7, 1).unwrap();
/// let mut g = k::graph::Graph::new();
/// k::sched_graph::create_sleep_event(&mut g, thread, 1_000_000, 5).unwrap();
///
/// let sleep_until = k::graph::get_prop(thread, "sleep_until_ns");
/// assert!(matches!(sleep_until, Some(thing_models::PropValue::U64(1_000_000))));
/// ```
pub fn create_sleep_event(
    graph: &mut Graph,
    thread: ThingId,
    wake_at_ns: TimeNs,
    _created_at_ns: TimeNs,
) -> Option<ThingId> {
    let props = alloc::vec![
        (crate::symbols::intern("sleep_until_ns"), PropValue::U64(wake_at_ns)),
    ];
    graph.update_thing(thread, props.as_slice());
    Some(thread)
}

/// Clear the thread's sleep_until_ns property.
pub fn clear_sleep_event(graph: &mut Graph, thread: ThingId) {
    let props = alloc::vec![(crate::symbols::intern("sleep_until_ns"), PropValue::U64(0))];
    let _ = graph.update_thing(thread, props.as_slice());
}

fn find_cpu_node(cpu_index: CpuId) -> Option<ThingId> {
    let mut found = None;
    graph::iter_things(|thing| {
        if found.is_some() {
            return;
        }
        if thing.kind != crate::symbols::intern(graph_kinds::KIND_CPU_CORE) {
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
        if current.is_some() || thing.kind != crate::symbols::intern(graph_kinds::KIND_THREAD) {
            return;
        }
        let mut out = [None; LINK_BUF];
        graph.neighbors(thing.id, graph_kinds::LINK_RUNS_ON, &mut out);
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
    let props = alloc::vec![
        (crate::symbols::intern("runtime_ns"), PropValue::U64(new_runtime)),
        (crate::symbols::intern("last_started_ns"), PropValue::U64(now)),
    ];
    graph.update_thing(thread, props.as_slice());
    delta
}

fn set_last_started(graph: &mut Graph, thread: ThingId, now: TimeNs) {
    let props = alloc::vec![(crate::symbols::intern("last_started_ns"), PropValue::U64(now))];
    graph.update_thing(thread, props.as_slice());
}

fn make_runnable(graph: &mut Graph, thread: ThingId, now: TimeNs) {
    let props = alloc::vec![
        (
            crate::symbols::intern("state"),
            PropValue::Str(String::from(ThreadState::Runnable.as_str())),
        ),
        (crate::symbols::intern("last_started_ns"), PropValue::U64(now)),
    ];
    graph.update_thing(thread, props.as_slice());
}

fn start_running(graph: &mut Graph, thread: ThingId, cpu_node: ThingId, now: TimeNs) {
    clear_cpu_assignments(graph, cpu_node);
    let props = alloc::vec![
        (
            crate::symbols::intern("state"),
            PropValue::Str(String::from(ThreadState::Running.as_str())),
        ),
        (crate::symbols::intern("last_started_ns"), PropValue::U64(now)),
    ];
    graph.update_thing(thread, props.as_slice());
    graph.add_link(thread, graph_kinds::LINK_RUNS_ON, cpu_node);
}

fn clear_cpu_assignments(graph: &mut Graph, cpu_node: ThingId) {
    graph::iter_things(|thing| {
        if thing.kind != crate::symbols::intern(graph_kinds::KIND_THREAD) {
            return;
        }
        let mut out = [None; LINK_BUF];
        graph.neighbors(thing.id, graph_kinds::LINK_RUNS_ON, &mut out);
        for cpu in out.into_iter().flatten() {
            if cpu == cpu_node {
                graph.remove_link(thing.id, graph_kinds::LINK_RUNS_ON, cpu_node);
            }
        }
    });
}

fn pick_next_runnable(_graph: &Graph, skip: Option<ThingId>) -> Option<ThingId> {
    let mut best: Option<(ThingId, u64, u64)> = None; // (id, priority, runtime)

    graph::iter_things(|thing| {
        if thing.kind != crate::symbols::intern(graph_kinds::KIND_THREAD) {
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
