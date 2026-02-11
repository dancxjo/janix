extern crate alloc;

#[path = "../src/discovery_state.rs"]
mod discovery_state;

use alloc::string::ToString;
use alloc::vec;
use discovery_state::{reconcile_desired, DiscoveryState};

#[test]
fn ttl_expiry_marks_instance_stale() {
    let mut state = DiscoveryState::default();
    state.observe_ptr("printer._ipp._tcp.local", 100, 1);
    assert!(!state.instance("printer._ipp._tcp.local").unwrap().stale);

    let stale = state.sweep(102);
    assert_eq!(stale, vec!["printer._ipp._tcp.local".to_string()]);
    assert!(state.instance("printer._ipp._tcp.local").unwrap().stale);
}

#[test]
fn merge_updates_do_not_duplicate_instance() {
    let mut state = DiscoveryState::default();
    state.observe_ptr("svc._http._tcp.local", 10, 120);
    state.observe_srv("svc._http._tcp.local", 11, 120);
    state.observe_txt("svc._http._tcp.local", 12, 120);

    assert_eq!(state.instance_count(), 1);
    let item = state.instance("svc._http._tcp.local").unwrap();
    assert!(item.saw_ptr);
    assert!(item.saw_srv);
    assert!(item.saw_txt);
}

#[test]
fn churn_updates_last_seen_without_new_nodes() {
    let mut state = DiscoveryState::default();
    state.observe_ptr("tv._airplay._tcp.local", 50, 30);
    state.observe_ptr("tv._airplay._tcp.local", 55, 30);

    assert_eq!(state.instance_count(), 1);
    let item = state.instance("tv._airplay._tcp.local").unwrap();
    assert_eq!(item.last_seen, 55);
    assert_eq!(item.expires_at, 85);
}

#[test]
fn desired_state_apply_calculates_publish_and_unpublish() {
    let desired = vec!["a._http._tcp.local".to_string()];
    let active = vec![];
    let (publish, unpublish) = reconcile_desired(&desired, &active);
    assert_eq!(publish, vec!["a._http._tcp.local".to_string()]);
    assert!(unpublish.is_empty());

    let desired = vec![];
    let active = vec!["a._http._tcp.local".to_string()];
    let (publish, unpublish) = reconcile_desired(&desired, &active);
    assert!(publish.is_empty());
    assert_eq!(unpublish, vec!["a._http._tcp.local".to_string()]);
}
