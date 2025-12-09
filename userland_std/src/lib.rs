#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::boxed::Box;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use abi::{
    FrameId, FrameInfo, KernelRequest, KernelResponse, MemorySummary, NodeId, PropKey, PropType,
    PropValue, SchedulerSummary, ThingId, ThreadInfo,
};
use userland_rt::Sys;

pub mod demo_shared;
pub mod time;

/// Print a line to the kernel log
pub fn println(sys: &impl Sys, message: &'static str) {
    let request = KernelRequest::Log { message };
    sys.syscall(request);
}

/// Query a node in the graph
pub fn graph_query(sys: &impl Sys, node_id: NodeId) -> Option<u64> {
    let request = KernelRequest::GraphQuery { node_id };
    match sys.syscall(request) {
        KernelResponse::NodeData { node_id: _, value } => Some(value),
        _ => None,
    }
}

/// Create a transaction
pub fn create_transaction(sys: &impl Sys) -> Option<abi::TransactionId> {
    let request = KernelRequest::CreateTransaction;
    match sys.syscall(request) {
        KernelResponse::TransactionCreated { tx_id } => Some(tx_id),
        _ => None,
    }
}

/// Commit a transaction
pub fn commit_transaction(sys: &impl Sys, tx_id: abi::TransactionId) -> bool {
    let request = KernelRequest::CommitTransaction { tx_id };
    matches!(sys.syscall(request), KernelResponse::Success { .. })
}

/// Create a new Thing (user wrapper)
pub fn user_create_thing(
    sys: &impl Sys,
    kind: &'static str,
    props: &'static [(PropKey, PropValue)],
) -> Result<ThingId, &'static str> {
    let request = KernelRequest::ThingCreate { kind, props };
    match sys.syscall(request) {
        KernelResponse::ThingCreated { id } => Ok(id),
        KernelResponse::Error { message } => Err(message),
        _ => Err("Unexpected response"),
    }
}

/// Update a Thing (user wrapper)
pub fn user_update_thing(
    sys: &impl Sys,
    id: ThingId,
    props: &'static [(PropKey, PropValue)],
) -> Result<(), &'static str> {
    let request = KernelRequest::ThingUpdate { id, props };
    match sys.syscall(request) {
        KernelResponse::Success { .. } => Ok(()),
        KernelResponse::Error { message } => Err(message),
        _ => Err("Unexpected response"),
    }
}

pub use abi::Thing;

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

/// Register a schema for a Thing type
pub fn register_schema_for<T: Thing>(sys: &impl Sys) -> bool {
    let schema = T::schema();
    match sys.syscall(KernelRequest::SchemaRegister {
        kind: T::KIND,
        props: schema,
    }) {
        KernelResponse::SchemaRegistered { .. } => true,
        _ => false,
    }
}

pub fn find_thing<T: Thing>(sys: &impl Sys, predicate: impl Fn(&T) -> bool) -> Option<T> {
    // Simple scan of the first 128 IDs
    for i in 0..128 {
        if let Some(thing) = load_thing::<T>(sys, ThingId(i)) {
            if predicate(&thing) {
                return Some(thing);
            }
        }
    }
    None
}

pub fn update_props(sys: &impl Sys, id: ThingId, props: &[(PropKey, PropValue)]) -> bool {
    // We must leak the props to satisfy the ABI's 'static requirement.
    let props_vec = props.to_vec();
    let props_slice = Box::leak(props_vec.into_boxed_slice());

    let request = KernelRequest::ThingUpdate {
        id,
        props: props_slice,
    };
    matches!(sys.syscall(request), KernelResponse::Success { .. })
}

pub fn memory_summary(sys: &impl Sys) -> Option<MemorySummary> {
    match sys.syscall(KernelRequest::GetMemorySummary) {
        KernelResponse::MemorySummary { summary } => Some(summary),
        _ => None,
    }
}

pub fn scheduler_summary(sys: &impl Sys) -> Option<SchedulerSummary> {
    match sys.syscall(KernelRequest::GetSchedulerSummary) {
        KernelResponse::SchedulerSummary { summary } => Some(summary),
        _ => None,
    }
}

pub fn alloc_frame(sys: &impl Sys) -> Option<FrameInfo> {
    match sys.syscall(KernelRequest::AllocFrame { pool_index: 0 }) {
        KernelResponse::FrameAllocated { frame } => Some(frame),
        _ => None,
    }
}

pub fn free_frame(sys: &impl Sys, frame_id: FrameId) -> bool {
    matches!(
        sys.syscall(KernelRequest::FreeFrame { frame_id }),
        KernelResponse::FrameFreed { .. }
    )
}

pub fn create_process(sys: &impl Sys, pid: u64) -> bool {
    matches!(
        sys.syscall(KernelRequest::CreateProcess { pid }),
        KernelResponse::ProcessCreated { .. }
    )
}

pub fn create_thread(sys: &impl Sys, pid: u64, tid: u64, priority: u64) -> bool {
    matches!(
        sys.syscall(KernelRequest::CreateThread { pid, tid, priority }),
        KernelResponse::ThreadCreated { .. }
    )
}

pub fn scheduler_tick(sys: &impl Sys) -> Option<ThreadInfo> {
    match sys.syscall(KernelRequest::SchedulerTick) {
        KernelResponse::SchedulerTicked { current } => current,
        _ => None,
    }
}
