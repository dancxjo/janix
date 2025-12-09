extern crate alloc;

use abi::PropValue;
use kernel_core::graph;
use kernel_core::graph_kinds;
use kernel_core::sched_graph;
use kernel_core::sched_types::ThreadState;

#[test]
fn single_runnable_thread_is_marked_running() {
    kernel_core::init();
    let cpu = kernel_core::model::create_cpu_core(0).expect("cpu");
    let thread = kernel_core::model::create_thread(1, 5).expect("thread");

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 100).expect("thread picked");
    assert_eq!(picked, thread);

    let state = graph::get_prop(thread, "state");
    assert!(matches!(state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Running.as_str()));

    let mut buf = [None; 4];
    graph::neighbors(thread, graph_kinds::EDGE_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));
}

#[test]
fn timeslice_expiry_moves_thread_to_runnable() {
    kernel_core::init();
    let cpu = kernel_core::model::create_cpu_core(0).expect("cpu");
    let thread = kernel_core::model::create_thread(2, 1).expect("thread");

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
    assert!(result.is_none(), "no thread should be selected after slice expiry without competitors");

    let state = graph::get_prop(thread, "state");
    assert!(matches!(state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Runnable.as_str()));

    let runtime = graph::get_prop(thread, "runtime_ns");
    assert!(matches!(runtime, Some(PropValue::U64(v)) if v >= 6_000_000));

    let mut buf = [None; 2];
    graph::neighbors(thread, graph_kinds::EDGE_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().next().is_none(), "preempted thread should not hold runs_on edge");
}

#[test]
fn higher_priority_thread_wins() {
    kernel_core::init();
    let cpu = kernel_core::model::create_cpu_core(0).expect("cpu");
    let low = kernel_core::model::create_thread(10, 1).expect("low prio");
    let high = kernel_core::model::create_thread(11, 10).expect("high prio");

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 50).expect("picked thread");
    assert_eq!(picked, high);

    let mut buf = [None; 2];
    graph::neighbors(high, graph_kinds::EDGE_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));

    let low_state = graph::get_prop(low, "state");
    assert!(matches!(low_state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Runnable.as_str()));
}
