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
    FrameId, FrameInfo, KernelRequest, KernelResponse, MapFlags, MemorySummary, NodeId,
    SchedulerSummary, SharedBufferInfo, ThreadInfo,
};
// `graph_kinds` is re-exported below as `pub use abi::graph_kinds;`
use userland_rt::{Sys, UserlandSys};

pub mod alarm;
pub mod batch;
pub mod clock;
pub mod demo_shared;
pub mod time;

pub use alarm::{Alarm, sleep_until};
pub use clock::SystemClock;
pub use thing_models::{
    AlarmEvent, AlarmRequest, DisplayPresentRequest, MODE_INDEX_CONSOLE, Mode, ModeSwitchEvent,
    Place, RawModule, Surface, TimeSource, Window, View, Cursor,
};

#[cfg(not(target_os = "none"))]
pub mod doc_helpers {
    use std::cell::RefCell;
    use std::collections::VecDeque;

    use abi::{KernelRequest, KernelResponse, PropKey, PropType, PropValue, Thing, ThingId};
    use userland_rt::Sys;

    /// Simple helper for doc tests that drives the syscall interface with canned responses.
    pub struct DocSys {
        pub requests: RefCell<Vec<KernelRequest>>,
        responses: RefCell<VecDeque<KernelResponse>>,
        pub time: RefCell<u64>,
        pub slept_for: RefCell<Vec<u64>>,
    }

    impl DocSys {
        pub fn with_responses(responses: Vec<KernelResponse>) -> Self {
            Self {
                requests: RefCell::new(Vec::new()),
                responses: RefCell::new(responses.into()),
                time: RefCell::new(0),
                slept_for: RefCell::new(Vec::new()),
            }
        }

        pub fn push_response(&self, response: KernelResponse) {
            self.responses.borrow_mut().push_back(response);
        }

        pub fn props_slice(
            props: Vec<(PropKey, PropValue)>,
        ) -> &'static [Option<(PropKey, PropValue)>] {
            Box::leak(
                props
                    .into_iter()
                    .map(Some)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            )
        }
    }

    impl Sys for DocSys {
        fn syscall(&self, request: KernelRequest) -> KernelResponse {
            self.requests.borrow_mut().push(request);
            self.responses
                .borrow_mut()
                .pop_front()
                .expect("doc helper responses exhausted")
        }

        fn time_now_ns(&mut self) -> u64 {
            let mut t = self.time.borrow_mut();
            *t += 1;
            *t
        }

        fn time_monotonic_ns(&mut self) -> u64 {
            self.time_now_ns()
        }

        fn time_system_ns(&mut self) -> u64 {
            self.time_now_ns()
        }

        fn sleep_for_ns(&mut self, delta_ns: u64) {
            self.slept_for.borrow_mut().push(delta_ns);
        }

        fn sleep_until_ns(&mut self, deadline_ns: u64) {
            self.slept_for.borrow_mut().push(deadline_ns);
        }

        fn yield_now(&mut self) {}

        fn exit_thread(&mut self) -> ! {
            panic!("doc helper exit_thread invoked")
        }
    }

    /// A lightweight Thing implementation for doc/test snippets.
    #[derive(Clone, Debug)]
    pub struct DummyThing {
        pub id: ThingId,
        pub flag: bool,
    }

    impl DummyThing {
        pub fn new(flag: bool) -> Self {
            DummyThing {
                id: ThingId(0),
                flag,
            }
        }
    }

    impl Thing for DummyThing {
        const KIND: &'static str = "DocDummy";
        const DESCRIPTION: &'static str = "Dummy Thing used in documentation snippets";

        fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
            out.push(("flag", PropValue::Bool(self.flag)));
        }

        fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
            let mut flag = false;
            for (k, v) in props.iter().flatten() {
                if *k == "flag" {
                    if let PropValue::Bool(b) = v {
                        flag = *b;
                    }
                }
            }
            DummyThing { id, flag }
        }

        fn schema() -> &'static [(&'static str, PropType)] {
            &[("flag", PropType::Bool)]
        }
    }
}

#[cfg(target_os = "none")]
pub fn entry<F>(run: F) -> !
where
    F: FnOnce(&mut UserlandSys),
{
    let mut sys = UserlandSys::new();
    run(&mut sys);
    loop {}
}

#[cfg(not(target_os = "none"))]
pub fn entry<F>(_run: F) -> !
where
    F: FnOnce(&mut UserlandSys),
{
    panic!("ThingOS userland apps only run on bare-metal targets");
}

/// Return the currently active `Mode` Thing, if one is marked active.
pub fn active_mode<S: Sys>(sys: &mut S) -> Option<Mode> {
    list_things_by_kind::<S, Mode>(sys)
        .into_iter()
        .find(|m| m.active)
}

/// Fallback when no mode is active yet: choose the lowest index mode.
pub fn default_mode<S: Sys>(sys: &mut S) -> Option<Mode> {
    list_things_by_kind::<S, Mode>(sys)
        .into_iter()
        .min_by_key(|m| m.index)
}

/// Convenience guard for deciding if the framebuffer console should own the screen.
pub fn is_console_mode_active<S: Sys>(sys: &mut S) -> bool {
    active_mode(sys)
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

/// Print a line to the kernel log.
///
/// This wraps the `Log` request, which is helpful for simple diagnostics when
/// debugging a userland program.
pub fn println(sys: &impl Sys, message: &'static str) {
    let request = KernelRequest::Log { message };
    sys.syscall(request);
}

/// Query a thing in the kernel graph and return the associated value.
///
/// If the kernel returns anything other than `NodeData`, this helper returns
/// `None`.
///
/// # Examples
///
/// ```
/// use abi::{KernelResponse, NodeId};
/// use userland_std::{doc_helpers::DocSys, graph_query};
///
/// let sys = DocSys::with_responses(vec![KernelResponse::NodeData {
///     node_id: NodeId(3),
///     value: 7,
/// }]);
/// assert_eq!(graph_query(&sys, NodeId(3)), Some(7));
/// ```
pub fn graph_query(sys: &impl Sys, node_id: NodeId) -> Option<u64> {
    let request = KernelRequest::GraphQuery { node_id };
    match sys.syscall(request) {
        KernelResponse::NodeData { node_id: _, value } => Some(value),
        _ => None,
    }
}

/// Create a transaction and return its ID.
///
/// The returned transaction can be used with other helpers such as
/// [`commit_transaction`].
///
/// # Examples
///
/// ```
/// use abi::{KernelResponse, TransactionId};
/// use userland_std::{create_transaction, doc_helpers::DocSys};
///
/// let sys = DocSys::with_responses(vec![KernelResponse::TransactionCreated {
///     tx_id: TransactionId(42),
/// }]);
/// assert_eq!(create_transaction(&sys), Some(TransactionId(42)));
/// ```
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
    props: &'static [(PropKey, PropValue)],
) -> Result<ThingId, &'static str> {
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
// Re-export commonly used ABI types for userland consumers
pub use abi::{Predicate, PropKey, PropType, PropValue, ThingId};
// Re-export graph kinds module so consumers can access it as `userland_std::graph_kinds`
pub use abi::graph_kinds;

/// Create a `Thing` value and register it with the kernel.
///
/// # Examples
///
/// ```
/// use abi::{KernelResponse, PropValue, ThingId};
/// use userland_std::{create_thing, doc_helpers::{DocSys, DummyThing}, Thing};
///
/// let sys = DocSys::with_responses(vec![KernelResponse::ThingCreated {
///     id: ThingId(1),
/// }]);
/// let dummy = DummyThing::new(true);
/// assert_eq!(create_thing(&sys, &dummy), Some(ThingId(1)));
/// ```
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
///
/// # Examples
///
/// ```
/// use abi::{KernelResponse, PropValue, ThingId};
/// use userland_std::{doc_helpers::{DocSys, DummyThing}, load_thing, Thing};
///
/// let props = DocSys::props_slice(vec![("flag", PropValue::Bool(true))]);
/// let sys = DocSys::with_responses(vec![KernelResponse::ThingData {
///     id: ThingId(1),
///     kind: DummyThing::KIND,
///     props,
/// }]);
/// let thing = load_thing::<DummyThing>(&sys, ThingId(1)).unwrap();
/// assert!(thing.flag);
/// ```
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
///
/// This attaches the description and property metadata so that users can
/// interpret Things using the schema registry.
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
///
/// This helper performs a brute-force scan over the first 128 IDs.
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

/// Return all neighbors reachable from `from` via `pred` in insertion order.
///
/// The returned IDs are ordered by the kernel's insertion order.
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
///
/// This currently uses a brute-force scan of IDs 0..256.
pub fn list_things_by_kind<S: Sys, T: Thing>(sys: &mut S) -> Vec<T> {
    let mut results = Vec::new();
    let mut cursor = ThingId(u64::MAX);
    loop {
        // println(sys, "list_things_by_kind: calling ThingList syscall");
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
/// This is a compile-time constant that describes what the type represents.
pub fn get_type_description<T: Thing>() -> &'static str {
    T::DESCRIPTION
}

/// Update the properties for the Thing with `id`.
///
/// The passed slice is leaked to satisfy the ABI's `'static` requirement.
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

/// Create a process with the provided display name.
pub fn create_process(sys: &impl Sys, name: &str) -> Option<u64> {
    let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
    match sys.syscall(KernelRequest::CreateProcess { name: leaked }) {
        KernelResponse::ProcessCreated { pid } => Some(pid),
        _ => None,
    }
}

/// Create a thread within a process.
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

/// Poll the scheduler for the currently running thread.
pub fn scheduler_tick(sys: &impl Sys) -> Option<ThreadInfo> {
    match sys.syscall(KernelRequest::SchedulerTick) {
        KernelResponse::SchedulerTicked { current } => current,
        _ => None,
    }
}

/// Spawn a boot program and return its generated process/thread IDs.
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

/// Query metadata for a shared buffer Thing.
pub fn shared_buffer_info(
    sys: &impl Sys,
    buffer_id: ThingId,
) -> Result<SharedBufferInfo, SysError> {
    match sys.syscall(KernelRequest::GetSharedBufferInfo { buffer_id }) {
        KernelResponse::SharedBufferInfoResponse { info } => Ok(info),
        KernelResponse::Error { message } => Err(SysError::Kernel(message)),
        _ => Err(SysError::Unexpected),
    }
}

/// Map a shared buffer into the calling address space.
pub fn shared_buffer_map(
    sys: &impl Sys,
    buffer_id: ThingId,
    flags: MapFlags,
) -> Result<(*mut u8, usize), SysError> {
    match sys.syscall(KernelRequest::MapSharedBuffer { buffer_id, flags }) {
        KernelResponse::SharedBufferMapped { vaddr, size } => Ok((vaddr as *mut u8, size as usize)),
        KernelResponse::Error { message } => Err(SysError::Kernel(message)),
        _ => Err(SysError::Unexpected),
    }
}

#[derive(Debug)]
pub struct SharedBufferMapping {
    pub id: ThingId,
    pub info: SharedBufferInfo,
    pub ptr: *mut u8,
    pub size: usize,
}

#[derive(Debug)]
/// Represents a mapped primary display buffer that a userland process can draw into.
pub struct PrimaryDisplayBuffer {
    pub display_id: ThingId,
    pub buffers: [SharedBufferMapping; 2],
    pub active_buffer_index: i64,
    pub info: SharedBufferInfo,
    pub ptr: *mut u8,
}

impl PrimaryDisplayBuffer {
    fn clamp_active_index(value: i64) -> i64 {
        if value == 1 { 1 } else { 0 }
    }

    fn front_index(value: i64) -> usize {
        match Self::clamp_active_index(value) {
            1 => 1,
            _ => 0,
        }
    }

    fn back_index(value: i64) -> usize {
        1 - Self::front_index(value)
    }

    fn sync_back_buffer(&mut self) {
        let idx = Self::back_index(self.active_buffer_index);
        let slot = &self.buffers[idx];
        self.ptr = slot.ptr;
        self.info = slot.info;
    }

    pub fn back_buffer(&self) -> &SharedBufferMapping {
        &self.buffers[Self::back_index(self.active_buffer_index)]
    }

    pub fn front_buffer(&self) -> &SharedBufferMapping {
        &self.buffers[Self::front_index(self.active_buffer_index)]
    }

    pub fn update_active_index(&mut self, index: i64) {
        self.active_buffer_index = Self::clamp_active_index(index);
        self.sync_back_buffer();
    }
}

fn map_display_buffer<S: Sys>(
    sys: &mut S,
    buffer_id: ThingId,
    flags: MapFlags,
) -> Result<SharedBufferMapping, SysError> {
    let info = shared_buffer_info(sys, buffer_id)?;
    let (ptr, size) = shared_buffer_map(sys, buffer_id, flags)?;
    Ok(SharedBufferMapping {
        id: buffer_id,
        info,
        ptr,
        size,
    })
}

/// Locate and map the kernel's primary display buffer.
///
/// This helper finds `display0` (or falls back to the first display), follows
/// the `scanout` link to the shared buffer, and maps it with read/write/user
/// permissions.
pub fn open_primary_display_buffer<S: Sys>(sys: &mut S) -> Result<PrimaryDisplayBuffer, SysError> {
    let displays: Vec<DisplayThing> = list_things_by_kind(sys);
    let display = displays
        .iter()
        .find(|d| d.name == "display0")
        .or_else(|| displays.first())
        .cloned()
        .ok_or(SysError::Unexpected)?;

    let mut front_targets =
        link_targets(sys, display.id, graph_kinds::LINK_DISPLAY_HAS_FRONT_BUFFER);
    let mut back_targets = link_targets(sys, display.id, graph_kinds::LINK_DISPLAY_HAS_BACK_BUFFER);
    let front_id = front_targets.pop().ok_or(SysError::Unexpected)?;
    let back_id = back_targets.pop().ok_or(SysError::Unexpected)?;

    let flags = MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER);
    let front_map = map_display_buffer(sys, front_id, flags)?;
    let back_map = map_display_buffer(sys, back_id, flags)?;

    let mut primary = PrimaryDisplayBuffer {
        display_id: display.id,
        buffers: [front_map, back_map],
        active_buffer_index: PrimaryDisplayBuffer::clamp_active_index(display.active_buffer_index),
        info: SharedBufferInfo {
            width: 0,
            height: 0,
            stride: 0,
            pixel_format: abi::PixelFormat::Rgba8888,
        },
        ptr: core::ptr::null_mut(),
    };
    primary.sync_back_buffer();
    Ok(primary)
}

pub fn swap_display_buffers<S: Sys>(sys: &mut S, display_id: ThingId) -> Option<i64> {
    let mut display = load_thing::<DisplayThing>(sys, display_id)?;
    let current = PrimaryDisplayBuffer::clamp_active_index(display.active_buffer_index);
    let next = 1 - current;
    let updates = [(
        graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX,
        PropValue::I64(next),
    )];
    if update_props(sys, display_id, &updates) {
        display.active_buffer_index = next;
        Some(next)
    } else {
        None
    }
}

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
