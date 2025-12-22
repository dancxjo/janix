extern crate alloc;

use alloc::vec::Vec;
use kernel::graph;
use kernel::graph_kinds;
use kernel::sched_graph;
use kernel::sched_types::ThreadState;
use thing_models::PropValue;

fn init_locked() -> spin::MutexGuard<'static, ()> {
    let guard = kernel::test_lock();
    kernel::init();
    guard
}

#[test]
#[ignore]
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
    graph::neighbors(thread, graph_kinds::LINK_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));
}

#[test]
#[ignore]
fn timeslice_expiry_moves_thread_to_runnable() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let thread = kernel::model::create_thread(2, 1).expect("thread");

    let running_props = [
        (
            kernel::symbols::intern("state"),
            PropValue::Str(alloc::string::String::from(ThreadState::Running.as_str())),
        ),
        (
            kernel::symbols::intern("last_started_ns"),
            PropValue::U64(0),
        ),
    ];
    graph::update_thing(thread, running_props.to_vec());
    graph::add_link(thread, graph_kinds::LINK_RUNS_ON, cpu);

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
    graph::neighbors(thread, graph_kinds::LINK_RUNS_ON, &mut buf);
    assert!(
        buf.into_iter().flatten().next().is_none(),
        "preempted thread should not hold runs_on link"
    );
}

#[test]
#[ignore]
fn higher_priority_thread_wins() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let low = kernel::model::create_thread(10, 1).expect("low prio");
    let high = kernel::model::create_thread(11, 10).expect("high prio");

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 50).expect("picked thread");
    assert_eq!(picked, high);

    let mut buf = [None; 2];
    graph::neighbors(high, graph_kinds::LINK_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));

    let low_state = graph::get_prop(low, "state");
    assert!(
        matches!(low_state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Runnable.as_str())
    );
}

#[test]
#[ignore]
fn running_thread_keeps_cpu_when_slice_remaining() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let thread = kernel::model::create_thread(21, 3).expect("thread");

    let props = [
        (
            kernel::symbols::intern("state"),
            PropValue::Str(alloc::string::String::from(ThreadState::Running.as_str())),
        ),
        (
            kernel::symbols::intern("last_started_ns"),
            PropValue::U64(1_000),
        ),
        (kernel::symbols::intern("runtime_ns"), PropValue::U64(500)),
    ];
    graph::update_thing(thread, props.to_vec());
    graph::add_link(thread, graph_kinds::LINK_RUNS_ON, cpu);

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
    graph::neighbors(thread, graph_kinds::LINK_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));
}

#[test]
#[ignore]
fn pick_prefers_lower_runtime_on_priority_tie() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let slow = kernel::model::create_thread(30, 7).expect("slow");
    let fresh = kernel::model::create_thread(31, 7).expect("fresh");

    let slow_props = [(
        kernel::symbols::intern("runtime_ns"),
        PropValue::U64(10_000),
    )];
    graph::update_thing(slow, slow_props.to_vec());

    let fresh_props = [(kernel::symbols::intern("runtime_ns"), PropValue::U64(1_000))];
    graph::update_thing(fresh, fresh_props.to_vec());

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 42).expect("picked thread");
    assert_eq!(picked, fresh);

    let mut buf = [None; 1];
    graph::neighbors(fresh, graph_kinds::LINK_RUNS_ON, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == cpu));

    let slow_state = graph::get_prop(slow, "state");
    assert!(
        matches!(slow_state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Runnable.as_str())
    );
}

#[test]
#[ignore]
fn preempted_thread_is_skipped_for_selection() {
    let _guard = init_locked();
    let cpu = kernel::model::create_cpu_core(0).expect("cpu");
    let hog = kernel::model::create_thread(40, 9).expect("hog");
    let backup = kernel::model::create_thread(41, 1).expect("backup");

    let hog_props = [(
        kernel::symbols::intern("state"),
        PropValue::Str(alloc::string::String::from(ThreadState::Running.as_str())),
    )];
    graph::update_thing(hog, hog_props.to_vec());
    graph::add_link(hog, graph_kinds::LINK_RUNS_ON, cpu);

    let mut g = graph::Graph::new();
    let picked = sched_graph::sched_tick(&mut g, 0, 10_000_000).expect("picked thread");
    assert_eq!(picked, backup);

    let hog_state = graph::get_prop(hog, "state");
    assert!(
        matches!(hog_state, Some(PropValue::Str(s)) if s.as_str() == ThreadState::Runnable.as_str())
    );

    let hog_runtime = graph::get_prop(hog, "runtime_ns");
    assert!(matches!(hog_runtime, Some(PropValue::U64(v)) if v >= 10_000_000));

    let mut hog_links = [None; 1];
    graph::neighbors(hog, graph_kinds::LINK_RUNS_ON, &mut hog_links);
    assert!(hog_links.into_iter().flatten().next().is_none());

    let mut backup_links = [None; 1];
    graph::neighbors(backup, graph_kinds::LINK_RUNS_ON, &mut backup_links);
    assert!(backup_links.into_iter().flatten().any(|id| id == cpu));
}

#[test]
#[ignore]
fn sleep_event_create_and_clear() {
    let _guard = init_locked();
    let thread = kernel::model::create_thread(50, 1).expect("thread");
    let mut g = graph::Graph::new();

    let sleep = sched_graph::create_sleep_event(&mut g, thread, 1_000_000, 5).expect("sleep");
    let kind_sleep = kernel::symbols::intern(graph_kinds::KIND_SLEEP_EVENT);
    let sym_wake = kernel::symbols::intern("wake_at_ns");
    let sym_created = kernel::symbols::intern("created_at_ns");

    kernel::graph::with_thing(sleep, |thing| {
        assert_eq!(thing.kind, kind_sleep);
        assert!(
            thing
                .props
                .iter()
                .any(|(k, v)| *k == sym_wake && matches!(v, PropValue::U64(1_000_000)))
        );
        assert!(
            thing
                .props
                .iter()
                .any(|(k, v)| *k == sym_created && matches!(v, PropValue::U64(5)))
        );
    })
    .expect("sleep thing");

    let mut buf = [None; 1];
    graph::neighbors(thread, graph_kinds::LINK_SLEEPS_UNTIL, &mut buf);
    assert!(buf.into_iter().flatten().any(|id| id == sleep));

    sched_graph::clear_sleep_event(&mut g, thread);

    let mut cleared = [None; 1];
    graph::neighbors(thread, graph_kinds::LINK_SLEEPS_UNTIL, &mut cleared);
    assert!(cleared.into_iter().flatten().next().is_none());
    assert!(kernel::graph::with_thing(sleep, |_| ()).is_none());
}
