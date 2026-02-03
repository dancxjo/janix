//! Static storage for well-known graph ThingIds.
//!
//! This module provides a mechanism for other kernel components (like the scheduler)
//! to access known graph node IDs without performing runtime graph queries.

use core::sync::atomic::{AtomicU64, Ordering};

/// Well-known graph node IDs set during boot registration.
pub struct GraphAnchors {
    /// ThingId of the dev.Host node
    host: AtomicU64,
    /// ThingId of the svc.Scheduler node
    scheduler_service: AtomicU64,
    /// ThingId of the proc.Kernel node
    kernel_proc: AtomicU64,
    /// ThingId of the svc.Root node
    root_service: AtomicU64,
}

static ANCHORS: GraphAnchors = GraphAnchors {
    host: AtomicU64::new(0),
    scheduler_service: AtomicU64::new(0),
    kernel_proc: AtomicU64::new(0),
    root_service: AtomicU64::new(0),
};

/// Set the host ThingId (called during boot registration)
pub fn set_host(id: u64) {
    ANCHORS.host.store(id, Ordering::Release);
}

/// Get the host ThingId
pub fn host() -> Option<u64> {
    let v = ANCHORS.host.load(Ordering::Acquire);
    if v != 0 {
        Some(v)
    } else {
        None
    }
}

/// Set the scheduler service ThingId (called during boot registration)
pub fn set_scheduler_service(id: u64) {
    ANCHORS.scheduler_service.store(id, Ordering::Release);
}

/// Get the scheduler service ThingId
pub fn scheduler_service() -> Option<u64> {
    let v = ANCHORS.scheduler_service.load(Ordering::Acquire);
    if v != 0 {
        Some(v)
    } else {
        None
    }
}

/// Set the kernel proc ThingId (called during boot registration)
pub fn set_kernel_proc(id: u64) {
    ANCHORS.kernel_proc.store(id, Ordering::Release);
}

/// Get the kernel proc ThingId
pub fn kernel_proc() -> Option<u64> {
    let v = ANCHORS.kernel_proc.load(Ordering::Acquire);
    if v != 0 {
        Some(v)
    } else {
        None
    }
}

/// Set the root service ThingId (called during boot registration)
pub fn set_root_service(id: u64) {
    ANCHORS.root_service.store(id, Ordering::Release);
}

/// Get the root service ThingId
pub fn root_service() -> Option<u64> {
    let v = ANCHORS.root_service.load(Ordering::Acquire);
    if v != 0 {
        Some(v)
    } else {
        None
    }
}
