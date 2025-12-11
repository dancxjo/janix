extern crate alloc;

use abi::PropValue;
use kernel::graph;
use kernel::graph_kinds;
use kernel::sched_graph;
use kernel::sched_types::ThreadState;

fn init_locked() -> spin::MutexGuard<'static, ()> {
    let guard = kernel::test_lock();
    kernel::init();
    guard
}

#[test]
fn single_runnable_thread_is_marked_running() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let thread = kernel::model::create_thread(1, 5).expect("thread");

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 100).expect("thread picked");
    assert_eq!(picked, thread);

    let state = graph::get_prop(thread, "state");
    assert!(
        matches!(state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Running.as_str())
    );

    let mut buf = [None; 4];
    graph::neighbors(thread, graph_kinds::EDGE_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));
}

#[test]
fn timeslice_expiry_moves_thread_to_runnable() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let thread = kernel::model::create_thread(2, 1).expect("thread");

    let running_props = &[
        (
            "state",
            PropValue::Str(alloc::string::String::from(ThreadState::Running.as_str())),
        ),
        ("last_started_ns", PropValue::U64(0)),
    ];
    graph::update_thing(thread, running_props);
    graph::add_edge(thread, graph_kinds::EDGE_RUNS_ON, cpu);

    let mut g = graph::Graph::new();
    let result = sched_graph::sched_tick(&mut g, 0, 6_000_000);
    assert!(
        result.is_none(),
        "no thread should be selected after slice expiry without competitors"
    );

    let state = graph::get_prop(thread, "state");
    assert!(
        matches!(state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Runnable.as_str())
    );

    let runtime = graph::get_prop(thread, "runtime_ns");
    assert!(matches!(runtime, Some(PropValue::U64(v)) if v >= 6_000_000));

    let mut buf = [None; 2];
    graph::neighbors(thread, graph_kinds::EDGE_RUNS_ON, &mut buf);
    assert!(
        buf.into_iter().flatten().next().is_none(),
        "preempted thread should not hold runs_on link"
    );
}

#[test]
fn higher_priority_thread_wins() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let low = kernel::model::create_thread(10, 1).expect("low prio");
    let high = kernel::model::create_thread(11, 10).expect("high prio");

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 50).expect("picked thread");
    assert_eq!(picked, high);

    let mut buf = [None; 2];
    graph::neighbors(high, graph_kinds::EDGE_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));

    let low_state = graph::get_prop(low, "state");
    assert!(
        matches!(low_state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Runnable.as_str())
    );
}

#[test]
fn running_thread_keeps_cpu_when_slice_remaining() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let thread = kernel::model::create_thread(21, 3).expect("thread");

    graph::update_thing(
        thread,
        &[
            (
                "state",
                PropValue::Str(alloc::string::String::from(ThreadState::Running.as_str())),
            ),
            ("last_started_ns", PropValue::U64(1_000)),
            ("runtime_ns", PropValue::U64(500)),
        ],
    );
    graph::add_edge(thread, graph_kinds::EDGE_RUNS_ON, cpu);

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 3_000).expect("thread picked");
    assert_eq!(picked, thread);

    let state = graph::get_prop(thread, "state");
    assert!(
        matches!(state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Running.as_str())
    );

    let runtime = graph::get_prop(thread, "runtime_ns");
    assert!(matches!(runtime, Some(PropValue::U64(v)) if v == 2_500));

    let last_started = graph::get_prop(thread, "last_started_ns");
    assert!(matches!(last_started, Some(PropValue::U64(v)) if v == 3_000));

    let mut buf = [None; 1];
    graph::neighbors(thread, graph_kinds::EDGE_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));
}

#[test]
fn pick_prefers_lower_runtime_on_priority_tie() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let slow = kernel::model::create_thread(30, 7).expect("slow");
    let fresh = kernel::model::create_thread(31, 7).expect("fresh");

    graph::update_thing(slow, &[("runtime_ns", PropValue::U64(10_000))]);
    graph::update_thing(fresh, &[("runtime_ns", PropValue::U64(1_000))]);

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 42).expect("picked thread");
    assert_eq!(picked, fresh);

    let mut buf = [None; 1];
    graph::neighbors(fresh, graph_kinds::EDGE_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));

    let slow_state = graph::get_prop(slow, "state");
    assert!(
        matches!(slow_state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Runnable.as_str())
    );
}

#[test]
fn preempted_thread_is_skipped_for_selection() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let hog = kernel::model::create_thread(40, 9).expect("hog");
    let backup = kernel::model::create_thread(41, 1).expect("backup");

    graph::update_thing(
        hog,
        &[(
            "state",
            PropValue::Str(alloc::string::String::from(ThreadState::Running.as_str())),
        )],
    );
    graph::add_edge(hog, graph_kinds::EDGE_RUNS_ON, cpu);

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 10_000_000).expect("picked thread");
    assert_eq!(picked, backup);

    let hog_state = graph::get_prop(hog, "state");
    assert!(
        matches!(hog_state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Runnable.as_str())
    );

    let hog_runtime = graph::get_prop(hog, "runtime_ns");
    assert!(matches!(hog_runtime, Some(PropValue::U64(v)) if v >= 10_000_000));

    let mut hog_edges = [None; 1];
    graph::neighbors(hog, graph_kinds::EDGE_RUNS_ON, &mut hog_edges);
    assert!(hog_edges.into_iter().flatten().next().is_none());

    let mut backup_edges = [None; 1];
    graph::neighbors(backup, graph_kinds::EDGE_RUNS_ON, &mut backup_edges);
    assert!(backup_edges.into_iter().flatten().any(|id| id == cpu));
}

#[test]
fn sleep_event_create_and_clear() {
    let _guard = init_locked();
    let thread = kernel::model::create_thread(50, 1).expect("thread");
    let mut g = graph::Graph::new();

    let sleep = sched_graph::create_sleep_event(&mut g, thread, 1_000_000, 5).expect("sleep");
    let (kind, props) = graph::get_thing(sleep).expect("sleep thing");
    assert_eq!(kind, graph_kinds::KIND_SLEEP_EVENT);
    assert!(
        props
            .iter()
            .flatten()
            .any(|(k, v)| *k == "wake_at_ns" && matches!(v, PropValue::U64(1_000_000)))
    );
    assert!(
        props
            .iter()
            .flatten()
            .any(|(k, v)| *k == "created_at_ns" && matches!(v, PropValue::U64(5)))
    );

    let mut buf = [None; 1];
    graph::neighbors(thread, graph_kinds::EDGE_SLEEPS_UNTIL, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == sleep));

    sched_graph::clear_sleep_event(&mut g, thread);

    let mut cleared = [None; 1];
    graph::neighbors(thread, graph_kinds::EDGE_SLEEPS_UNTIL, &mut cleared);
    assert!(cleared.into_iter().flatten().next().is_none());
    assert!(graph::get_thing(sleep).is_none());
}
