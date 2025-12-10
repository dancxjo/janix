use abi::PropValue;
use core::sync::atomic::{AtomicUsize, Ordering};
use kernel_core::graph;
use kernel_core::graph::GraphEvent;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn node_create_and_prop_change_emit_events() {
    let _guard = TEST_LOCK.lock().unwrap();
    graph::init();
    static CREATED: AtomicUsize = AtomicUsize::new(0);
    static PROP: AtomicUsize = AtomicUsize::new(0);

    graph::subscribe_node_created("Widget", |_| {
        CREATED.fetch_add(1, Ordering::SeqCst);
    });
    graph::subscribe_prop_changed("Widget", "name", |event| {
        if let GraphEvent::PropChanged { old, new, .. } = event {
            assert_eq!(old, &None);
            assert_eq!(new, &PropValue::Str("gizmo".into()));
            PROP.fetch_add(1, Ordering::SeqCst);
        }
    });

    let id = graph::create_thing("Widget", &[]).expect("thing");
    let props = [("name", PropValue::Str("gizmo".into()))];
    assert!(graph::update_thing(id, &props));

    assert_eq!(CREATED.load(Ordering::SeqCst), 1);
    assert_eq!(PROP.load(Ordering::SeqCst), 1);
}

#[test]
fn edge_add_and_remove_are_pushed() {
    let _guard = TEST_LOCK.lock().unwrap();
    graph::init();
    static ADDED: AtomicUsize = AtomicUsize::new(0);
    static REMOVED: AtomicUsize = AtomicUsize::new(0);

    graph::subscribe_edge_added("runs_on", |_| {
        ADDED.fetch_add(1, Ordering::SeqCst);
    });
    graph::subscribe_edge_removed("runs_on", |_| {
        REMOVED.fetch_add(1, Ordering::SeqCst);
    });

    let a = graph::create_thing("Thread", &[]).expect("a");
    let b = graph::create_thing("CpuCore", &[]).expect("b");
    assert!(graph::add_edge(a, "runs_on", b));
    assert!(graph::remove_edge(a, "runs_on", b));

    assert_eq!(ADDED.load(Ordering::SeqCst), 1);
    assert_eq!(REMOVED.load(Ordering::SeqCst), 1);
}

#[test]
fn neighbors_collects_only_matching_edges() {
    let _guard = TEST_LOCK.lock().unwrap();
    graph::init();
    let a = graph::create_thing("Thread", &[]).unwrap();
    let b = graph::create_thing("CpuCore", &[]).unwrap();
    let c = graph::create_thing("CpuCore", &[]).unwrap();
    assert!(graph::add_edge(a, "runs_on", b));
    assert!(graph::add_edge(a, "runs_on", c));

    let mut out = [None; 4];
    graph::neighbors(a, "runs_on", &mut out);
    assert!(out.contains(&Some(b)));
    assert!(out.contains(&Some(c)));
}
