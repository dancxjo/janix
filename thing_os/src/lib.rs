#![cfg_attr(target_os = "none", no_std)]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod sys;
pub mod heap;
pub mod console;
pub mod panic;
pub mod syscalls;

pub mod alarm;
pub mod batch;
pub mod clock;
pub mod demo_shared;
pub mod time;
// pub mod userland; // Removed
pub mod resident;
pub mod prelude;
pub mod ui; // Moved from userland/ui.rs
pub mod display;
pub use display::*;

use abi::{
    FrameInfo, KernelRequest, KernelResponse, MemorySummary, NodeId,
    SchedulerSummary, ThreadInfo, FrameId,
};
pub use abi::graph_kinds;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;
pub use alloc::string::{String, ToString};
pub use alloc::rc::Rc;
pub use alloc::sync::Arc;
pub use alloc::format;
pub use alloc::vec;
use syscalls::syscall;

pub use alarm::{Alarm, sleep_until};
pub use clock::SystemClock;
pub use thing_models::{
    AlarmEvent, AlarmRequest, Cursor, DisplayPresentRequest, MODE_INDEX_CONSOLE, Mode,
    ModeSwitchEvent, Place, RawModule, Surface, TimeSource, View, Window,
};
pub use thing_macros::main;
pub use abi::Thing;
pub use abi; // Export abi crate
pub use abi::{Predicate, PropKey, PropType, PropValue, ThingId};

/// Return the currently active `Mode` Thing, if one is marked active.
pub fn active_mode() -> Option<Mode> {
    list_things_by_kind::<Mode>()
        .into_iter()
        .find(|m| m.active)
}

/// Fallback when no mode is active yet: choose the lowest index mode.
pub fn default_mode() -> Option<Mode> {
    list_things_by_kind::<Mode>()
        .into_iter()
        .min_by_key(|m| m.index)
}

/// Convenience guard for deciding if the framebuffer console should own the screen.
pub fn is_console_mode_active() -> bool {
    active_mode()
        .map(|m| m.index == MODE_INDEX_CONSOLE)
        .unwrap_or(false)
}

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
    pub active_buffer_index: i64,
}

impl abi::Thing for DisplayThing {
    // const KIND: &'static str = graph_kinds::KIND_DISPLAY;
    const KIND: &'static str = "Display";
    const DESCRIPTION: &'static str = "A display sink capable of scanning out a SharedBuffer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_NAME, PropValue::Str(self.name.clone())));
        out.push((graph_kinds::PROP_WIDTH, PropValue::U64(self.width)));
        out.push((graph_kinds::PROP_HEIGHT, PropValue::U64(self.height)));
        out.push((graph_kinds::PROP_STRIDE, PropValue::U64(self.stride)));
        if let Some(fmt) = &self.pixel_format {
            out.push((graph_kinds::PROP_PIXEL_FORMAT, PropValue::Str(fmt.clone())));
        }
        out.push((
            graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX,
            PropValue::I64(self.active_buffer_index),
        ));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = None;
        let mut active_buffer_index = 0;

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
                graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX => {
                    if let PropValue::I64(v) = prop.1 {
                        active_buffer_index = v;
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
            active_buffer_index,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_NAME, PropType::Str),
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_STRIDE, PropType::U64),
            (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
            (graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX, PropType::I64),
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

/// Query a thing in the kernel graph and return the associated value.
pub fn graph_query(node_id: NodeId) -> Option<u64> {
    let request = KernelRequest::GraphQuery { node_id };
    match syscall(request) {
        KernelResponse::NodeData { node_id: _, value } => Some(value),
        _ => None,
    }
}

/// Create a transaction and return its ID.
pub fn create_transaction() -> Option<abi::TransactionId> {
    let request = KernelRequest::CreateTransaction;
    match syscall(request) {
        KernelResponse::TransactionCreated { tx_id } => Some(tx_id),
        _ => None,
    }
}

/// Commit an open transaction created via [`create_transaction`].
pub fn commit_transaction(tx_id: abi::TransactionId) -> bool {
    let request = KernelRequest::CommitTransaction { tx_id };
    matches!(syscall(request), KernelResponse::Success { .. })
}

/// Create a new Thing from a raw kind and property slice.
pub fn user_create_thing(
    kind: &'static str,
    props: &'static [(PropKey, PropValue)],
) -> Result<ThingId, &'static str> {
    let request = KernelRequest::ThingCreate { kind, props };
    match syscall(request) {
        KernelResponse::ThingCreated { id } => Ok(id),
        KernelResponse::Error { message } => Err(message),
        _ => Err("Unexpected response"),
    }
}

/// Update properties of an existing Thing using a property slice.
pub fn user_update_thing(
    id: ThingId,
    props: &'static [(PropKey, PropValue)],
) -> Result<(), &'static str> {
    let request = KernelRequest::ThingUpdate { id, props };
    match syscall(request) {
        KernelResponse::Success { .. } => Ok(()),
        KernelResponse::Error { message } => Err(message),
        _ => Err("Unexpected response"),
    }
}

/// Create a `Thing` value and register it with the kernel.
pub fn create_thing<T: Thing>(thing: &T) -> Option<ThingId> {
    let mut props_vec = Vec::new();
    thing.to_props(&mut props_vec);

    // Sanitize keys: Copy static strings to heap
    let mut _key_holders = Vec::new();
    let mut safe_props_vec = Vec::new();
    for (key, val) in props_vec {
        let key_str = String::from(key);
        let key_static: &'static str = unsafe { core::mem::transmute(key_str.as_str()) };
        _key_holders.push(key_str);
        safe_props_vec.push((key_static, val));
    }

    let kind_str = String::from(T::KIND);
    let kind_static: &'static str = unsafe { core::mem::transmute(kind_str.as_str()) };

    // Pass slice reference transmuted to static lifetime (safe because syscall blocks)
    let props_slice = safe_props_vec.as_slice();
    let props_static: &'static [(PropKey, PropValue)] = unsafe { core::mem::transmute(props_slice) };

    let request = KernelRequest::ThingCreate {
        kind: kind_static,
        props: props_static,
    };
    match syscall(request) {
        KernelResponse::ThingCreated { id } => Some(id),
        _ => None,
    }
}

/// Load a typed `Thing` from the kernel.
pub fn load_thing<T: Thing>(id: ThingId) -> Option<T> {
    let request = KernelRequest::ThingGet { id };
    match syscall(request) {
        KernelResponse::ThingData { id, kind, props } => {
            // Workaround: Copy T::KIND to heap for safe comparison if needed
            let kind_str = String::from(T::KIND);
            if kind != kind_str.as_str() {
                return None;
            }
            Some(T::from_props(id, props))
        }
        _ => None,
    }
}

/// Request that the kernel register the schema for `T`.
pub fn register_schema_for<T: Thing>() -> bool {
    let schema = T::schema();
    
    // Sanitize schema: Copy strings to heap
    let mut _str_holders = Vec::new();
    let mut safe_schema_vec = Vec::new();
    for (key, prop_type) in schema {
        let key_str = String::from(*key);
        let key_static: &'static str = unsafe { core::mem::transmute(key_str.as_str()) };
        _str_holders.push(key_str);
        safe_schema_vec.push((key_static, *prop_type));
    }
    
    let schema_slice = safe_schema_vec.as_slice();
    let schema_static: &'static [(&'static str, PropType)] = unsafe { core::mem::transmute(schema_slice) };

    // Workaround: Copy strings to heap to avoid potential .rodata mapping issues
    let kind_str = String::from(T::KIND);
    let desc_str = String::from(T::DESCRIPTION);
    // SAFETY: Transmute to static lifetime for syscall duration
    let kind_static: &'static str = unsafe { core::mem::transmute(kind_str.as_str()) };
    let desc_static: &'static str = unsafe { core::mem::transmute(desc_str.as_str()) };

    match syscall(KernelRequest::SchemaRegister {
        kind: kind_static,
        description: desc_static,
        props: schema_static,
    }) {
        KernelResponse::SchemaRegistered { .. } => true,
        _ => false,
    }
}

/// Search for a `Thing` that satisfies `predicate`.
pub fn find_thing<T: Thing>(predicate: impl Fn(&T) -> bool) -> Option<T> {
    list_things_by_kind().into_iter().find(predicate)
}

/// Return all neighbors reachable from `from` via `pred` in insertion order.
pub fn link_targets(src: ThingId, pred: Predicate) -> Vec<ThingId> {
    let mut results = Vec::new();
    let mut idx = 0;
    loop {
        match syscall(KernelRequest::LinkAt { src, pred, idx }) {
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

/// Add a link between Things.
pub fn add_link(src: ThingId, pred: Predicate, dst: ThingId) -> bool {
    matches!(
        syscall(KernelRequest::AddLink { src, pred, dst }),
        KernelResponse::Success { .. }
    )
}

/// List all Things of a given `T::KIND`.
pub fn list_things_by_kind<T: Thing>() -> Vec<T> {
    let mut results = Vec::new();
    let mut cursor = ThingId(u64::MAX);
    // Workaround: Copy kind to heap to avoid potential .rodata mapping issues
    let kind_str = String::from(T::KIND);
    // SAFETY: The string slice is only registered for the duration of the syscall,
    // which does not retain the pointer.
    let kind_static: &'static str = unsafe { core::mem::transmute(kind_str.as_str()) };
    loop {
        match syscall(KernelRequest::ThingList {
            kind: kind_static,
            start_after: cursor,
        }) {
            KernelResponse::ThingListEntry { id: Some(next_id) } => {
                if let Some(thing) = load_thing::<T>(next_id) {
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
pub fn update_props(id: ThingId, props: &[(PropKey, PropValue)]) -> bool {
    // Sanitize keys: Copy static strings to heap
    let mut _key_holders = Vec::new();
    let mut safe_props_vec = Vec::new();
    for (key, val) in props {
        let key_str = String::from(*key);
        let key_static: &'static str = unsafe { core::mem::transmute(key_str.as_str()) };
        _key_holders.push(key_str);
        safe_props_vec.push((key_static, val.clone()));
    }
    
    let props_slice = safe_props_vec.as_slice();
    let props_static: &'static [(PropKey, PropValue)] = unsafe { core::mem::transmute(props_slice) };

    let request = KernelRequest::ThingUpdate {
        id,
        props: props_static,
    };
    matches!(syscall(request), KernelResponse::Success { .. })
}

/// Return a summary of the current physical memory state.
pub fn memory_summary() -> Option<MemorySummary> {
    match syscall(KernelRequest::GetMemorySummary) {
        KernelResponse::MemorySummary { summary } => Some(summary),
        _ => None,
    }
}

/// Return a summary of the scheduler state exposed by the kernel.
pub fn scheduler_summary() -> Option<SchedulerSummary> {
    match syscall(KernelRequest::GetSchedulerSummary) {
        KernelResponse::SchedulerSummary { summary } => Some(summary),
        _ => None,
    }
}

/// Allocate a zero-addressed frame from the first frame pool.
pub fn alloc_frame() -> Option<FrameInfo> {
    match syscall(KernelRequest::AllocFrame { pool_index: 0 }) {
        KernelResponse::FrameAllocated { frame } => Some(frame),
        _ => None,
    }
}

/// Create a new process by spawning a known boot program.
pub fn create_process(boot_program_id: ThingId) -> Result<(ThingId, ThingId), &'static str> {
    match syscall(KernelRequest::SpawnProgram { boot_program_id }) {
        KernelResponse::ProgramSpawned {
            process_id,
            thread_id,
        } => Ok((process_id, thread_id)),
        KernelResponse::Error { message } => Err(message),
        _ => Err("Unexpected response"),
    }
}

/// Create a new thread within a process.
pub fn create_thread(
    pid: ThingId,
    name: &str,
    app_id: u64,
    priority: u64,
) -> Result<ThingId, &'static str> {
    let name_static = Box::leak(name.to_string().into_boxed_str());
    match syscall(KernelRequest::CreateThread {
        pid: pid.0,
        name: name_static,
        app_id,
        priority,
    }) {
        KernelResponse::ThreadCreated { tid } => Ok(ThingId(tid)),
        KernelResponse::Error { message } => Err(message),
        _ => Err("Unexpected response"),
    }
}

/// Free a frame by ID.
pub fn free_frame(frame_id: FrameId) -> bool {
    match syscall(KernelRequest::FreeFrame { frame_id }) {
        KernelResponse::FrameFreed { .. } => true,
        _ => false,
    }
}
