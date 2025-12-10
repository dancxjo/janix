#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::boxed::Box;
#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(target_os = "none")]
use alloc::vec::Vec;
#[cfg(not(target_os = "none"))]
use std::string::ToString;

use abi::{
    FrameId, FrameInfo, KernelRequest, KernelResponse, MapFlags, MemorySummary, NodeId, PixelFormat,
    PropKey, PropType, PropValue, SchedulerSummary, SharedBufferInfo, ThingId, ThreadInfo,
};
use abi::graph_kinds;
use userland_rt::Sys;

pub mod alarm;
pub mod clock;
pub mod demo_shared;
pub mod time;

pub use alarm::{Alarm, sleep_until};
pub use clock::SystemClock;
pub use thing_models::{AlarmEvent, AlarmRequest, TimeSource};

pub extern crate thing_models;
pub mod thread_info {
    pub use thing_models::ThreadInfo;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysError {
    Kernel(&'static str),
    Unexpected,
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

#[derive(Clone, Debug)]
pub struct DisplayThing {
    pub id: ThingId,
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub pixel_format: Option<String>,
}

impl abi::Thing for DisplayThing {
    const KIND: &'static str = graph_kinds::KIND_DISPLAY;
    const DESCRIPTION: &'static str = "A display sink capable of scanning out a SharedBuffer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_NAME, PropValue::Str(self.name.clone())));
        out.push((graph_kinds::PROP_WIDTH, PropValue::U64(self.width)));
        out.push((graph_kinds::PROP_HEIGHT, PropValue::U64(self.height)));
        out.push((graph_kinds::PROP_STRIDE, PropValue::U64(self.stride)));
        if let Some(fmt) = &self.pixel_format {
            out.push((graph_kinds::PROP_PIXEL_FORMAT, PropValue::Str(fmt.clone())));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = None;

        for prop in props.iter().flatten() {
            match prop.0 {
                graph_kinds::PROP_NAME => {
                    if let PropValue::Str(v) = &prop.1 {
                        name = v.clone();
                    }
                }
                graph_kinds::PROP_WIDTH => {
                    if let PropValue::U64(v) = prop.1 {
                        width = v;
                    }
                }
                graph_kinds::PROP_HEIGHT => {
                    if let PropValue::U64(v) = prop.1 {
                        height = v;
                    }
                }
                graph_kinds::PROP_STRIDE => {
                    if let PropValue::U64(v) = prop.1 {
                        stride = v;
                    }
                }
                graph_kinds::PROP_PIXEL_FORMAT => {
                    if let PropValue::Str(v) = &prop.1 {
                        pixel_format = Some(v.clone());
                    }
                }
                _ => {}
            }
        }

        DisplayThing {
            id,
            name,
            width,
            height,
            stride,
            pixel_format,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_NAME, PropType::Str),
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_STRIDE, PropType::U64),
            (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct SharedBufferThing {
    pub id: ThingId,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub pixel_format: Option<String>,
}

impl abi::Thing for SharedBufferThing {
    const KIND: &'static str = graph_kinds::KIND_SHARED_BUFFER;
    const DESCRIPTION: &'static str = "Shared memory buffer exported by the kernel";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_WIDTH, PropValue::U64(self.width)));
        out.push((graph_kinds::PROP_HEIGHT, PropValue::U64(self.height)));
        out.push((graph_kinds::PROP_STRIDE, PropValue::U64(self.stride)));
        if let Some(fmt) = &self.pixel_format {
            out.push((graph_kinds::PROP_PIXEL_FORMAT, PropValue::Str(fmt.clone())));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = None;

        for prop in props.iter().flatten() {
            match prop.0 {
                graph_kinds::PROP_WIDTH => {
                    if let PropValue::U64(v) = prop.1 {
                        width = v;
                    }
                }
                graph_kinds::PROP_HEIGHT => {
                    if let PropValue::U64(v) = prop.1 {
                        height = v;
                    }
                }
                graph_kinds::PROP_STRIDE => {
                    if let PropValue::U64(v) = prop.1 {
                        stride = v;
                    }
                }
                graph_kinds::PROP_PIXEL_FORMAT => {
                    if let PropValue::Str(v) = &prop.1 {
                        pixel_format = Some(v.clone());
                    }
                }
                _ => {}
            }
        }

        SharedBufferThing {
            id,
            width,
            height,
            stride,
            pixel_format,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_STRIDE, PropType::U64),
            (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
        ]
    }
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

/// Return all neighbors reachable from `from` via `edge_kind` in insertion order.
pub fn edge_targets<S: Sys>(sys: &mut S, from: ThingId, edge_kind: &'static str) -> Vec<ThingId> {
    let mut results = Vec::new();
    let mut index = 0;
    loop {
        match sys.syscall(KernelRequest::EdgeAt {
            from,
            edge_kind,
            index,
        }) {
            KernelResponse::EdgeTarget { target: Some(id) } => {
                results.push(id);
                index += 1;
            }
            KernelResponse::EdgeTarget { target: None } => break,
            _ => break,
        }
    }
    results
}

/// Add an edge between Things via the kernel ABI.
pub fn add_edge(sys: &impl Sys, from: ThingId, edge_kind: &'static str, to: ThingId) -> bool {
    matches!(
        sys.syscall(KernelRequest::AddEdge {
            from,
            edge_kind,
            to
        }),
        KernelResponse::Success { .. }
    )
}

/// List all Things of a given `T::KIND`
///
/// This currently uses a brute-force scan of IDs 0..256.
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

pub fn create_process(sys: &impl Sys, name: &str) -> Option<u64> {
    let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
    match sys.syscall(KernelRequest::CreateProcess { name: leaked }) {
        KernelResponse::ProcessCreated { pid } => Some(pid),
        _ => None,
    }
}

pub fn create_thread(
    sys: &impl Sys,
    pid: u64,
    name: &str,
    app_id: u64,
    priority: u64,
) -> Option<u64> {
    let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
    match sys.syscall(KernelRequest::CreateThread {
        pid,
        name: leaked,
        app_id,
        priority,
    }) {
        KernelResponse::ThreadCreated { tid } => Some(tid),
        _ => None,
    }
}

pub fn scheduler_tick(sys: &impl Sys) -> Option<ThreadInfo> {
    match sys.syscall(KernelRequest::SchedulerTick) {
        KernelResponse::SchedulerTicked { current } => current,
        _ => None,
    }
}

pub fn spawn_program(sys: &mut impl Sys, boot_program_id: ThingId) -> Option<(ThingId, ThingId)> {
    match sys.syscall(KernelRequest::SpawnProgram { boot_program_id }) {
        KernelResponse::ProgramSpawned {
            process_id,
            thread_id,
        } => Some((process_id, thread_id)),
        KernelResponse::Error { message } => {
            println(sys, message);
            None
        }
        _ => None,
    }
}

pub fn shared_buffer_info(sys: &impl Sys, buffer_id: ThingId) -> Result<SharedBufferInfo, SysError> {
    match sys.syscall(KernelRequest::GetSharedBufferInfo { buffer_id }) {
        KernelResponse::SharedBufferInfoResponse { info } => Ok(info),
        KernelResponse::Error { message } => Err(SysError::Kernel(message)),
        _ => Err(SysError::Unexpected),
    }
}

pub fn shared_buffer_map(
    sys: &impl Sys,
    buffer_id: ThingId,
    flags: MapFlags,
) -> Result<(*mut u8, usize), SysError> {
    match sys.syscall(KernelRequest::MapSharedBuffer { buffer_id, flags }) {
        KernelResponse::SharedBufferMapped { vaddr, size } => {
            Ok((vaddr as *mut u8, size as usize))
        }
        KernelResponse::Error { message } => Err(SysError::Kernel(message)),
        _ => Err(SysError::Unexpected),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PrimaryDisplayBuffer {
    pub display_id: ThingId,
    pub buffer_id: ThingId,
    pub info: SharedBufferInfo,
    pub ptr: *mut u8,
    pub size: usize,
}

pub fn open_primary_display_buffer<S: Sys>(sys: &mut S) -> Result<PrimaryDisplayBuffer, SysError> {
    let displays: Vec<DisplayThing> = list_things_by_kind(sys);
    let display = displays
        .iter()
        .find(|d| d.name == "display0")
        .or_else(|| displays.first())
        .cloned()
        .ok_or(SysError::Unexpected)?;

    let mut targets = edge_targets(sys, display.id, graph_kinds::EDGE_DISPLAY_SCANOUT);
    let buffer_id = targets
        .pop()
        .ok_or(SysError::Unexpected)?;

    let info = shared_buffer_info(sys, buffer_id)?;
    let flags = MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER);
    let (ptr, size) = shared_buffer_map(sys, buffer_id, flags)?;

    Ok(PrimaryDisplayBuffer {
        display_id: display.id,
        buffer_id,
        info,
        ptr,
        size,
    })
}
