#[cfg(target_os = "none")]
use alloc::boxed::Box;
#[cfg(target_os = "none")]
use alloc::string::ToString;
#[cfg(target_os = "none")]
use alloc::vec::Vec;
#[cfg(not(target_os = "none"))]
use std::string::ToString;

use abi::{
    FrameId, FrameInfo, KernelRequest, KernelResponse, MapFlags, MemorySummary, NodeId,
    SchedulerSummary, SharedBufferInfo, ThreadInfo,
};
use runtime::Sys;

/// Print a line to the kernel log.
pub fn println(sys: &impl Sys, message: &'static str) {
    let request = KernelRequest::Log { message };
    sys.syscall(request);
}

/// Query a node in the kernel graph and return the associated value.
pub fn graph_query(sys: &impl Sys, node_id: NodeId) -> Option<u64> {
    let request = KernelRequest::GraphQuery { node_id };
    match sys.syscall(request) {
        KernelResponse::NodeData { node_id: _, value } => Some(value),
        _ => None,
    }
}

/// Create a transaction and return its ID.
pub fn create_transaction(sys: &impl Sys) -> Option<abi::TransactionId> {
    let request = KernelRequest::CreateTransaction;
    match sys.syscall(request) {
        KernelResponse::TransactionCreated { tx_id } => Some(tx_id),
        _ => None,
    }
}

/// Commit an open transaction created via [`create_transaction`].
pub fn commit_transaction(sys: &impl Sys, tx_id: abi::TransactionId) -> bool {
    let request = KernelRequest::CommitTransaction { tx_id };
    matches!(sys.syscall(request), KernelResponse::Success { .. })
}

/// Create a new Thing from a raw kind and property slice.
pub fn user_create_thing(
    sys: &impl Sys,
    kind: &'static str,
    props: &'static [(abi::PropKey, abi::PropValue)],
) -> Result<abi::ThingId, &'static str> {
    let request = KernelRequest::ThingCreate { kind, props };
    match sys.syscall(request) {
        KernelResponse::ThingCreated { id } => Ok(id),
        KernelResponse::Error { message } => Err(message),
        _ => Err("Unexpected response"),
    }
}

/// Update properties of an existing Thing using a property slice.
pub fn user_update_thing(
    sys: &impl Sys,
    id: abi::ThingId,
    props: &'static [(abi::PropKey, abi::PropValue)],
) -> Result<(), &'static str> {
    let request = KernelRequest::ThingUpdate { id, props };
    match sys.syscall(request) {
        KernelResponse::Success { .. } => Ok(()),
        KernelResponse::Error { message } => Err(message),
        _ => Err("Unexpected response"),
    }
}

pub use abi::Thing;
pub use abi::graph_kinds;
pub use abi::{Predicate, PropKey, PropType, PropValue, ThingId};

/// Create a `Thing` value and register it with the kernel.
pub fn create_thing<T: Thing>(sys: &impl Sys, thing: &T) -> Option<ThingId> {
    let mut props_vec = Vec::new();
    thing.to_props(&mut props_vec);
    let props_slice = Box::leak(props_vec.into_boxed_slice());

    let request = KernelRequest::ThingCreate {
        kind: T::KIND,
        props: props_slice,
    };
    match sys.syscall(request) {
        KernelResponse::ThingCreated { id } => Some(id),
        _ => None,
    }
}

/// Load a typed `Thing` from the kernel.
pub fn load_thing<T: Thing>(sys: &impl Sys, id: ThingId) -> Option<T> {
    let request = KernelRequest::ThingGet { id };
    match sys.syscall(request) {
        KernelResponse::ThingData { id, kind, props } => {
            if kind != T::KIND {
                return None;
            }
            Some(T::from_props(id, props))
        }
        _ => None,
    }
}

/// Request that the kernel register the schema for `T`.
pub fn register_schema_for<T: Thing>(sys: &impl Sys) -> bool {
    let schema = T::schema();
    match sys.syscall(KernelRequest::SchemaRegister {
        kind: T::KIND,
        description: T::DESCRIPTION,
        props: schema,
    }) {
        KernelResponse::SchemaRegistered { .. } => true,
        _ => false,
    }
}

/// Search for a `Thing` that satisfies `predicate`.
pub fn find_thing<T: Thing>(sys: &impl Sys, predicate: impl Fn(&T) -> bool) -> Option<T> {
    for i in 0..128 {
        if let Some(thing) = load_thing::<T>(sys, ThingId(i)) {
            if predicate(&thing) {
                return Some(thing);
            }
        }
    }
    None
}

/// Return all neighbors reachable from `from` via `pred` in insertion order.
pub fn link_targets<S: Sys>(sys: &mut S, src: ThingId, pred: Predicate) -> Vec<ThingId> {
    let mut results = Vec::new();
    let mut idx = 0;
    loop {
        match sys.syscall(KernelRequest::LinkAt { src, pred, idx }) {
            KernelResponse::LinkTarget { target: Some(id) } => {
                results.push(id);
                idx += 1;
            }
            KernelResponse::LinkTarget { target: None } => break,
            _ => break,
        }
    }
    results
}

/// Add a link between Things via the kernel ABI.
pub fn add_link(sys: &impl Sys, src: ThingId, pred: Predicate, dst: ThingId) -> bool {
    matches!(
        sys.syscall(KernelRequest::AddLink { src, pred, dst }),
        KernelResponse::Success { .. }
    )
}

/// List all Things of a given `T::KIND`.
pub fn list_things_by_kind<S: Sys, T: Thing>(sys: &mut S) -> Vec<T> {
    let mut results = Vec::new();
    let mut cursor = ThingId(u64::MAX);
    loop {
        match sys.syscall(KernelRequest::ThingList {
            kind: T::KIND,
            start_after: cursor,
        }) {
            KernelResponse::ThingListEntry { id: Some(next_id) } => {
                if let Some(thing) = load_thing::<T>(sys, next_id) {
                    results.push(thing);
                }
                cursor = next_id;
            }
            KernelResponse::ThingListEntry { id: None } => break,
            KernelResponse::Error { .. } => break,
            _ => break,
        }
    }
    results
}

/// Get the type-level description for a `Thing` type.
pub fn get_type_description<T: Thing>() -> &'static str {
    T::DESCRIPTION
}

/// Update the properties for the Thing with `id`.
pub fn update_props(sys: &impl Sys, id: ThingId, props: &[(PropKey, PropValue)]) -> bool {
    let props_vec = props.to_vec();
    let props_slice = Box::leak(props_vec.into_boxed_slice());

    let request = KernelRequest::ThingUpdate {
        id,
        props: props_slice,
    };
    matches!(sys.syscall(request), KernelResponse::Success { .. })
}

/// Return a summary of the current physical memory state.
pub fn memory_summary(sys: &impl Sys) -> Option<MemorySummary> {
    match sys.syscall(KernelRequest::GetMemorySummary) {
        KernelResponse::MemorySummary { summary } => Some(summary),
        _ => None,
    }
}

/// Return a summary of the scheduler state exposed by the kernel.
pub fn scheduler_summary(sys: &impl Sys) -> Option<SchedulerSummary> {
    match sys.syscall(KernelRequest::GetSchedulerSummary) {
        KernelResponse::SchedulerSummary { summary } => Some(summary),
        _ => None,
    }
}

/// Allocate a zero-addressed frame from the first frame pool.
pub fn alloc_frame(sys: &impl Sys) -> Option<FrameInfo> {
    match sys.syscall(KernelRequest::AllocFrame { pool_index: 0 }) {
        KernelResponse::FrameAllocated { frame } => Some(frame),
        _ => None,
    }
}

/// Return a previously-allocated frame to the kernel.
pub fn free_frame(sys: &impl Sys, frame_id: FrameId) -> bool {
    matches!(
        sys.syscall(KernelRequest::FreeFrame { frame_id }),
        KernelResponse::FrameFreed { .. }
    )
}

/// Query a thing and return name+props (utility for debugging).
pub fn get_thing<S: Sys>(sys: &mut S, id: ThingId) -> Option<(String, Vec<(String, PropValue)>)> {
    match sys.syscall(KernelRequest::ThingGet { id }) {
        KernelResponse::ThingData { kind, props, .. } => {
            let props_vec = props
                .iter()
                .flatten()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect();
            Some((kind.to_string(), props_vec))
        }
        _ => None,
    }
}
