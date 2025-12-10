#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::boxed::Box;
#[cfg(target_os = "none")]
use alloc::string::String;
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use abi::{
    FrameId, FrameInfo, KernelRequest, KernelResponse, MemorySummary, NodeId, PropKey, PropType,
    PropValue, SchedulerSummary, ThingId, ThreadInfo,
};
use userland_rt::Sys;

pub mod demo_shared;
pub mod time;

pub extern crate thing_models;
pub mod thread_info {
    pub use thing_models::ThreadInfo;
}

#[derive(Clone, Debug)]
pub struct CpuCoreThing {
    pub id: ThingId,
    pub index: u64,
}

impl abi::Thing for CpuCoreThing {
    const KIND: &'static str = "CpuCore";
    const DESCRIPTION: &'static str = "A CPU core identified by its index in the system";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("index", PropValue::U64(self.index)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut index = 0;
        for prop in props.iter().flatten() {
            if prop.0 == "index" {
                if let PropValue::U64(v) = prop.1 {
                    index = v;
                }
            }
        }
        CpuCoreThing { id, index }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("index", PropType::U64)]
    }
}

#[derive(Clone, Debug)]
pub struct ProcessThing {
    pub id: ThingId,
    pub pid: u64,
}

impl abi::Thing for ProcessThing {
    const KIND: &'static str = "Process";
    const DESCRIPTION: &'static str = "A process with process identifier (PID) and execution state";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("pid", PropValue::U64(self.pid)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut pid = 0;
        for prop in props.iter().flatten() {
            if prop.0 == "pid" {
                if let PropValue::U64(v) = prop.1 {
                    pid = v;
                }
            }
        }
        ProcessThing { id, pid }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("pid", PropType::U64)]
    }
}

#[derive(Clone, Debug)]
pub struct ThreadThing {
    pub id: ThingId,
    pub tid: u64,
    pub state: String,
    pub priority: u64,
    pub runtime_ns: u64,
    pub last_started_ns: u64,
}

impl abi::Thing for ThreadThing {
    const KIND: &'static str = "Thread";
    const DESCRIPTION: &'static str =
        "A thread of execution with thread identifier, state, priority, and runtime tracking";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("tid", PropValue::U64(self.tid)));
        out.push(("state", PropValue::Str(self.state.clone())));
        out.push(("priority", PropValue::U64(self.priority)));
        out.push(("runtime_ns", PropValue::U64(self.runtime_ns)));
        out.push(("last_started_ns", PropValue::U64(self.last_started_ns)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut tid = 0;
        let mut state = String::new();
        let mut priority = 0;
        let mut runtime_ns = 0;
        let mut last_started_ns = 0;

        for prop in props.iter().flatten() {
            match prop.0 {
                "tid" => {
                    if let PropValue::U64(v) = prop.1 {
                        tid = v;
                    }
                }
                "state" => {
                    if let PropValue::Str(ref v) = prop.1 {
                        state = v.clone();
                    }
                }
                "priority" => {
                    if let PropValue::U64(v) = prop.1 {
                        priority = v;
                    }
                }
                "runtime_ns" => {
                    if let PropValue::U64(v) = prop.1 {
                        runtime_ns = v;
                    }
                }
                "last_started_ns" => {
                    if let PropValue::U64(v) = prop.1 {
                        last_started_ns = v;
                    }
                }
                _ => {}
            }
        }

        ThreadThing {
            id,
            tid,
            state,
            priority,
            runtime_ns,
            last_started_ns,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("tid", PropType::U64),
            ("state", PropType::Str),
            ("priority", PropType::U64),
            ("runtime_ns", PropType::U64),
            ("last_started_ns", PropType::U64),
        ]
    }
}

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
        description: T::DESCRIPTION,
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

/// List all Things of a given `T::KIND`
///
/// This currently uses a brute-force scan of IDs 0..256.
pub fn list_things_by_kind<S: Sys, T: Thing>(sys: &mut S) -> Vec<T> {
    let mut results = Vec::new();
    // Scan a reasonable range of IDs. In a real system, we'd have a specific syscall.
    for i in 0..256 {
        if let Some(thing) = load_thing::<T>(sys, ThingId(i)) {
            // load_thing already checks T::KIND
            results.push(thing);
        }
    }
    results
}

/// Get the type-level description for a Thing type.
/// This is a compile-time constant that describes what the type represents.
pub fn get_type_description<T: Thing>() -> &'static str {
    T::DESCRIPTION
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
