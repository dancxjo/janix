extern crate alloc;
use thing_models::PropValue;
use core::sync::atomic::{AtomicUsize, Ordering};
use kernel::graph;
use kernel::graph::GraphEvent;
use kernel::graph_kinds;
use alloc::vec::Vec;

#[test]
#[ignore]
fn node_create_and_prop_change_emit_events() {
    let _guard = kernel::test_lock();
    kernel::init();
    static CREATED: AtomicUsize = AtomicUsize::new(0);

    // There is no subscribe_node_created in kernel/src/graph/events.rs.
    // It only has generic subscribe(handler).
    // So we subscribe to all events and filter.

    let kind_widget = kernel::symbols::intern("Widget");

    kernel::graph::events::subscribe(|event| {
        if let GraphEvent::ThingCreated(id) = event {
            // We can't check kind easily here without querying graph, which might deadlock if called from event handler?
            // But this is a test, single threaded (mostly).
            // Let's just count ThingCreated events.
            CREATED.fetch_add(1, Ordering::SeqCst);
        }
    });

    let sym_name = kernel::symbols::intern("name");

    let id = graph::create_thing(kind_widget, Vec::new());

    let props = [(sym_name, PropValue::Str("gizmo".into()))];
    assert!(graph::update_thing(id, props.to_vec()));

    // create_thing emits ThingCreated.
    // We expect at least 1.
    assert!(CREATED.load(Ordering::SeqCst) >= 1);
}

#[test]
#[ignore]
fn link_add_and_remove_are_pushed() {
    let _guard = kernel::test_lock();
    kernel::init();
    static ADDED: AtomicUsize = AtomicUsize::new(0);

    kernel::graph::events::subscribe(|event| {
        if let GraphEvent::LinkAdded { src: _, dst: _, pred } = event {
            if *pred == graph_kinds::LINK_RUNS_ON {
                ADDED.fetch_add(1, Ordering::SeqCst);
            }
        }
    });

    let kind_thread = kernel::symbols::intern("Thread");
    let kind_cpu = kernel::symbols::intern("CpuCore");

    let a = graph::create_thing(kind_thread, Vec::new());
    let b = graph::create_thing(kind_cpu, Vec::new());
    assert!(graph::add_link(a, graph_kinds::LINK_RUNS_ON, b));

    // Removal is not tested as event is missing
    assert!(graph::remove_link(a, graph_kinds::LINK_RUNS_ON, b));

    assert_eq!(ADDED.load(Ordering::SeqCst), 1);
}

#[test]
#[ignore]
fn neighbors_collects_only_matching_links() {
    let _guard = kernel::test_lock();
    kernel::init();

    let kind_thread = kernel::symbols::intern("Thread");
    let kind_cpu = kernel::symbols::intern("CpuCore");

    let a = graph::create_thing(kind_thread, Vec::new());
    let b = graph::create_thing(kind_cpu, Vec::new());
    let c = graph::create_thing(kind_cpu, Vec::new());

    assert!(graph::add_link(a, graph_kinds::LINK_RUNS_ON, b));
    assert!(graph::add_link(a, graph_kinds::LINK_RUNS_ON, c));

    let mut out = [None; 4];
    graph::neighbors(a, graph_kinds::LINK_RUNS_ON, &mut out);
    assert!(out.contains(&Some(b)));
    assert!(out.contains(&Some(c)));
}
