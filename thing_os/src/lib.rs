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
pub mod resident;
pub mod prelude;
pub mod ui;
pub mod display;
pub use display::*;

use abi::{
    FrameInfo, MemorySummary, NodeId,
    SchedulerSummary, FrameId, 
    wire::{graph::{WireProp, WirePropValue, WireSchemaProp, WireValueTag}, common::{UserPtr, UserSlice}},
    syscall_defs::SymbolId,
};
pub use thing_models::graph_kinds;
pub mod graph_ops;
pub use abi::{KernelRequest, KernelResponse};
use thing_models::graph_kinds::{
    PROP_NAME, PROP_WIDTH, PROP_HEIGHT, PROP_STRIDE, PROP_PIXEL_FORMAT, PROP_DISPLAY_ACTIVE_BUFFER_INDEX,
    KIND_SHARED_BUFFER
};
use thing_macros::Thing;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;
pub use alloc::string::{String, ToString};
pub use alloc::rc::Rc;
pub use alloc::sync::Arc;
pub use alloc::format;
pub use alloc::vec;
use syscalls::{syscall, sys_symbol_intern};

pub use alarm::{Alarm, sleep_until};
pub use clock::SystemClock;
pub use thing_models::{
    AlarmEvent, AlarmRequest, Cursor, DisplayPresentRequest, MODE_INDEX_CONSOLE, Mode,
    ModeSwitchEvent, Place, RawModule, Surface, TimeSource, View, Window,
};
pub use thing_macros::main;
pub use abi; // Export abi crate
pub use abi::{Predicate, ThingId};
pub use thing_models::{PropKey, PropType, PropValue};
pub use thing_models::Thing;
use crate::sys::raw_syscall;
use abi::syscalls::SYSCALL_THING_GET;

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

#[derive(Clone, Debug, Thing)]
#[thing(description = "A CPU core identified by its index in the system")]
#[thing(kind = "CpuCore")]
pub struct CpuCoreThing {
    pub id: ThingId,
    pub index: u64,
}

#[derive(Clone, Debug, Thing)]
#[thing(description = "A process with process identifier (PID) and execution state")]
#[thing(kind = "Process")]
pub struct ProcessThing {
    pub id: ThingId,
    pub pid: u64,
}

#[derive(Clone, Debug, Thing)]
#[thing(description = "A thread of execution with thread identifier, state, priority, and runtime tracking")]
#[thing(kind = "Thread")]
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

impl Thing for DisplayThing {
    // const KIND: &'static str = graph_kinds::KIND_DISPLAY;
    const KIND: &'static str = "Display";
    const DESCRIPTION: &'static str = "A display sink capable of scanning out a SharedBuffer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((PROP_NAME.to_string(), PropValue::Str(self.name.clone())));
        out.push((PROP_WIDTH.to_string(), PropValue::U64(self.width)));
        out.push((PROP_HEIGHT.to_string(), PropValue::U64(self.height)));
        out.push((PROP_STRIDE.to_string(), PropValue::U64(self.stride)));
        if let Some(fmt) = &self.pixel_format {
            out.push((PROP_PIXEL_FORMAT.to_string(), PropValue::Str(fmt.clone())));
        }
        out.push((
            PROP_DISPLAY_ACTIVE_BUFFER_INDEX.to_string(),
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
            match prop.0.as_str() {
                PROP_NAME => {
                    if let PropValue::Str(v) = &prop.1 {
                        name = v.clone();
                    }
                }
                PROP_WIDTH => {
                    if let PropValue::U64(v) = prop.1 {
                        width = v;
                    }
                }
                PROP_HEIGHT => {
                    if let PropValue::U64(v) = prop.1 {
                        height = v;
                    }
                }
                PROP_STRIDE => {
                    if let PropValue::U64(v) = prop.1 {
                        stride = v;
                    }
                }
                PROP_PIXEL_FORMAT => {
                    if let PropValue::Str(v) = &prop.1 {
                        pixel_format = Some(v.clone());
                    }
                }
                PROP_DISPLAY_ACTIVE_BUFFER_INDEX => {
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
            (PROP_NAME, PropType::Str),
            (PROP_WIDTH, PropType::U64),
            (PROP_HEIGHT, PropType::U64),
            (PROP_STRIDE, PropType::U64),
            (PROP_PIXEL_FORMAT, PropType::Str),
            (PROP_DISPLAY_ACTIVE_BUFFER_INDEX, PropType::I64),
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

impl Thing for SharedBufferThing {
    const KIND: &'static str = KIND_SHARED_BUFFER;
    const DESCRIPTION: &'static str = "Shared memory buffer exported by the kernel";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((PROP_WIDTH.to_string(), PropValue::U64(self.width)));
        out.push((PROP_HEIGHT.to_string(), PropValue::U64(self.height)));
        out.push((PROP_STRIDE.to_string(), PropValue::U64(self.stride)));
        if let Some(fmt) = &self.pixel_format {
            out.push((PROP_PIXEL_FORMAT.to_string(), PropValue::Str(fmt.clone())));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = None;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                PROP_WIDTH => {
                    if let PropValue::U64(v) = prop.1 {
                        width = v;
                    }
                }
                PROP_HEIGHT => {
                    if let PropValue::U64(v) = prop.1 {
                        height = v;
                    }
                }
                PROP_STRIDE => {
                    if let PropValue::U64(v) = prop.1 {
                        stride = v;
                    }
                }
                PROP_PIXEL_FORMAT => {
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
            (PROP_WIDTH, PropType::U64),
            (PROP_HEIGHT, PropType::U64),
            (PROP_STRIDE, PropType::U64),
            (PROP_PIXEL_FORMAT, PropType::Str),
        ]
    }
}


/// Query a thing in the kernel graph and return the associated value.
pub fn graph_query(node_id: NodeId) -> Option<u64> {
    let request = KernelRequest::GraphQuery { node_id, out: UserSlice::default() };
    match syscall(request) {
        // KernelResponse::NodeData { node_id: _, value } => {
            // // Try to interpret bytes as u64
            // if value.len() >= 8 {
            //      let mut buf = [0u8; 8];
            //      buf.copy_from_slice(&value[0..8]);
            //      Some(u64::from_le_bytes(buf))
            // } else {
            //      None
            // }
        // }
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
    let kind_sym = sys_symbol_intern(kind);
    let mut wire_props = Vec::with_capacity(props.len());
    
    for (key, val) in props {
        let key_sym = sys_symbol_intern(&key);
        let wire_val = match val {
            PropValue::U64(v) => WirePropValue::u64(*v),
            PropValue::I64(v) => WirePropValue::i64(*v),
            PropValue::Bool(v) => WirePropValue::bool(*v),
            PropValue::Str(s) => WirePropValue::sym(sys_symbol_intern(s)),
            PropValue::Symbol(id) => WirePropValue::sym(*id),
            PropValue::Blob(b) => WirePropValue::blob(b.as_ptr() as u64, b.len() as u64),
        };
        wire_props.push(WireProp { key: key_sym, value: wire_val, _pad: 0 });
    }

    let request = KernelRequest::ThingCreate {
        kind: kind_sym,
        props: UserSlice::from_slice(&wire_props),
    };
    match syscall(request) {
        KernelResponse::ThingCreated { id } => Ok(id),
        KernelResponse::Error { err: _ } => Err("Error creating thing"),
        _ => Err("Unexpected response"),
    }
}

/// Update properties of an existing Thing using a property slice.
pub fn user_update_thing(
    id: ThingId,
    props: &'static [(PropKey, PropValue)],
) -> Result<(), &'static str> {
    let mut wire_props = Vec::with_capacity(props.len());
    
    for (key, val) in props {
        let key_sym = sys_symbol_intern(&key);
        let wire_val = match val {
            PropValue::U64(v) => WirePropValue::u64(*v),
            PropValue::I64(v) => WirePropValue::i64(*v),
            PropValue::Bool(v) => WirePropValue::bool(*v),
            PropValue::Str(s) => WirePropValue::sym(sys_symbol_intern(s)),
            PropValue::Symbol(id) => WirePropValue::sym(*id),
            PropValue::Blob(b) => WirePropValue::blob(b.as_ptr() as u64, b.len() as u64),
        };
        wire_props.push(WireProp { key: key_sym, value: wire_val, _pad: 0 });
    }

    let request = KernelRequest::ThingUpdate {
        id,
        props: UserSlice::from_slice(&wire_props)
    };
    match syscall(request) {
        KernelResponse::Success { .. } => Ok(()),
        KernelResponse::Error { err: _ } => Err("Error updating thing"),
        _ => Err("Unexpected response"),
    }
}

/// Create a `Thing` value and register it with the kernel.
pub fn create_thing<T: Thing>(thing: &T) -> Option<ThingId> {
    let mut props_vec = Vec::new();
    thing.to_props(&mut props_vec);

    let kind_sym = sys_symbol_intern(T::KIND);
    let mut wire_props = Vec::with_capacity(props_vec.len());
    
    for (key, val) in &props_vec {
        let key_sym = sys_symbol_intern(key);
        let wire_val = match val {
            PropValue::U64(v) => WirePropValue::u64(*v),
            PropValue::I64(v) => WirePropValue::i64(*v),
            PropValue::Bool(v) => WirePropValue::bool(*v),
            PropValue::Str(s) => WirePropValue::sym(sys_symbol_intern(s)),
            PropValue::Symbol(id) => WirePropValue::sym(*id),
            PropValue::Blob(b) => WirePropValue::blob(b.as_ptr() as u64, b.len() as u64),
        };
        wire_props.push(WireProp { key: key_sym, value: wire_val, _pad: 0 });
    }

    let request = KernelRequest::ThingCreate {
        kind: kind_sym,
        props: UserSlice::from_slice(&wire_props),
    };
    match syscall(request) {
        KernelResponse::ThingCreated { id } => Some(id),
        _ => None,
    }
}

/// Load a typed `Thing` from the kernel.
pub fn load_thing<T: Thing>(id: ThingId) -> Option<T> {
    const MAX_PROPS: usize = 32;
    let mut buf = [WireProp { key: SymbolId(0), value: WirePropValue::u64(0), _pad: 0 }; MAX_PROPS];

    // Call syscall with ptr and len
    let ptr = buf.as_mut_ptr() as u64;
    let len = buf.len() as u64;
    let ret = unsafe { raw_syscall(SYSCALL_THING_GET, id.0, ptr, len, 0, 0, 0) };

    if ret == u64::MAX {
        return None;
    }

    let count = core::cmp::min(ret as usize, MAX_PROPS);
    let mut props: Vec<Option<(PropKey, PropValue)>> = Vec::with_capacity(count);

    // We need to resolve symbols to keys
    for i in 0..count {
        let wp = &buf[i];
        let mut key_buf = [0u8; 128];

        let key_req = abi::syscall_defs::SymbolResolveReq {
            id: wp.key,
            out_ptr: key_buf.as_mut_ptr() as u64,
            out_cap: key_buf.len() as u64,
        };

        // Resolve key
        let mut resp = abi::syscall_defs::SymbolResolveResp { written: 0 };
        let resolve_ret = unsafe {
            raw_syscall(
                abi::syscalls::SYSCALL_SYMBOL_RESOLVE,
                &key_req as *const _ as u64,
                &mut resp as *mut _ as u64,
                0, 0, 0, 0
            )
        };

        if resolve_ret != 0 { continue; }

        let key_len = core::cmp::min(resp.written as usize, key_buf.len());
        let key_str = core::str::from_utf8(&key_buf[..key_len]).ok()?.to_string();

        // Convert WirePropValue to PropValue
        let val = match wp.value.tag {
            t if t == WireValueTag::U64 as u8 => PropValue::U64(wp.value.data_0),
            t if t == WireValueTag::I64 as u8 => PropValue::I64(wp.value.data_0 as i64),
            t if t == WireValueTag::Bool as u8 => PropValue::Bool(wp.value.data_0 != 0),
            t if t == WireValueTag::Str as u8 => {
                 let sym_id = SymbolId(wp.value.data_0 as u32);
                 // Resolve string value
                 let mut str_buf = [0u8; 128];
                 let str_req = abi::syscall_defs::SymbolResolveReq {
                     id: sym_id,
                     out_ptr: str_buf.as_mut_ptr() as u64,
                     out_cap: str_buf.len() as u64,
                 };
                 let mut str_resp = abi::syscall_defs::SymbolResolveResp { written: 0 };
                 let str_ret = unsafe {
                     raw_syscall(
                         abi::syscalls::SYSCALL_SYMBOL_RESOLVE,
                         &str_req as *const _ as u64,
                         &mut str_resp as *mut _ as u64,
                         0, 0, 0, 0
                     )
                 };
                 if str_ret == 0 {
                     let slen = core::cmp::min(str_resp.written as usize, str_buf.len());
                     let s = core::str::from_utf8(&str_buf[..slen]).ok()?.to_string();
                     PropValue::Str(s)
                 } else {
                     PropValue::Str(String::new())
                 }
            },
            // Blob support omitted for brevity/safety in this context
            _ => continue,
        };

        props.push(Some((key_str, val)));
    }

    // Note: We skip checking kind string against T::KIND because we already found the thing by ID.
    // However, if strict checking is needed, we would need to fetch the kind symbol separately.
    // For now, assume if ID is valid, we trust caller knows what they loaded or `from_props` handles it.

    Some(T::from_props(id, &props))
}

/// Check if an existing schema matches the expected schema for T.
fn schema_matches<T: Thing>(existing: &[Option<(SymbolId, PropType)>]) -> bool {
    let expected = T::schema();

    // Count actual entries in existing
    let existing_count = existing.iter().flatten().count();

    if existing_count != expected.len() { 
        return false; 
    }

    // Convert existing slice to a map-like search or just simple linear scan since schemas are small.
    // Also need interned keys for comparison.
    for (key_str, exp_pt) in expected {
        let key_sym = sys_symbol_intern(*key_str);
        // Find by symbol
        let found = existing.iter().flatten().find(|(k, _)| *k == key_sym);
        let Some((_, got_pt)) = found else { return false; };
        if got_pt != exp_pt { return false; }
    }
    true
}

/// Request that the kernel register the schema for `T`.
pub fn register_schema_for<T: Thing>() -> bool {
    let schema = T::schema();
    
    let kind_sym = sys_symbol_intern(T::KIND);
    let desc_sym = sys_symbol_intern(T::DESCRIPTION);
    
    let mut wire_schema = Vec::with_capacity(schema.len());
    for (key, prop_type) in schema {
        let key_sym = sys_symbol_intern(*key);
        let tag = match prop_type {
            PropType::U64 => WireValueTag::U64,
            PropType::I64 => WireValueTag::I64,
            PropType::Bool => WireValueTag::Bool,
            PropType::Str => WireValueTag::Str, // String type
            PropType::Symbol => WireValueTag::Str, // Treated same for now?
            PropType::Blob => WireValueTag::Blob,
        };
        wire_schema.push(WireSchemaProp { name: key_sym, prop_type: tag as u32 });
    }
    
    let request = KernelRequest::SchemaRegisterPackage {
        kind: kind_sym,
        description: desc_sym,
        props: UserSlice::from_slice(&wire_schema),
    };

    match syscall(request) {
        KernelResponse::SchemaRegistered { outcome, .. } => {
            match outcome {
                abi::SchemaRegistryOutcome::Created => true,
                abi::SchemaRegistryOutcome::AlreadyRegisteredSame => true,
                abi::SchemaRegistryOutcome::Conflict => {
                    println!("schema register conflict for kind {}", T::KIND);
                    false
                }
            }
        },
        KernelResponse::Error { err } => {
            println!("schema register failed for kind {}: {:?}", T::KIND, err);
            // Check if existing schema matches what we expect
            // Assuming code 3 is PERMISSION or similar, but previously it checked string message.
            // If "Schema already registered" maps to a specific error code, we should check that.
            // For now, let's assume if it fails we check if it's already there.
            // match syscall(KernelRequest::SchemaGet { kind: kind_sym }) {
            //     KernelResponse::SchemaData { props, .. } => {
            //          schema_matches::<T>(props)
            //     }
            //     _ => false,
            // }
            false
        }
        other => {
            println!("schema register unexpected response for kind {}: {:?}", T::KIND, other);
            false
        }
    }
}

/// Ensure that the schema for `T` exists in the kernel.
/// Returns true if it exists.
pub fn ensure_schema_exists_for<T: Thing>() -> bool {
    let kind_sym = sys_symbol_intern(T::KIND);
    // Ask the kernel for the schema so we can verify shape.
    const MAX_SCHEMA_PROPS: usize = 16;
    let mut buf = [WireSchemaProp { name: SymbolId(0), prop_type: 0 }; MAX_SCHEMA_PROPS];
    let out = UserSlice::new(UserPtr::new(buf.as_mut_ptr() as u64), buf.len() as u64);

    match syscall(KernelRequest::SchemaGet { kind: kind_sym, out }) {
        KernelResponse::SchemaData { written, .. } => {
            let count = core::cmp::min(written as usize, buf.len());
            let mut props = Vec::with_capacity(count);
            for wsp in &buf[..count] {
                let pt = match wsp.prop_type {
                    0 => Some(PropType::U64),
                    1 => Some(PropType::I64),
                    2 => Some(PropType::Bool),
                    3 => Some(PropType::Str),
                    4 => Some(PropType::Blob),
                    5 => Some(PropType::Symbol),
                    _ => None,
                };
                props.push(pt.map(|p| (wsp.name, p)));
            }
            if schema_matches::<T>(&props) {
                return true;
            }
            // If the schema shape differs, try to (re)register the canonical one.
            register_schema_for::<T>()
        }
        KernelResponse::Error { .. } => register_schema_for::<T>(),
        _ => register_schema_for::<T>(),
    }
}

/// Search for a `Thing` that satisfies `predicate`.
pub fn find_thing<T: Thing>(predicate: impl Fn(&T) -> bool) -> Option<T> {
    list_things_by_kind::<T>().into_iter().find(predicate)
}

/// Return all neighbors reachable from `from` via `pred` in insertion order.
pub fn link_targets(src: ThingId, pred: Predicate) -> Vec<ThingId> {
    let mut results = Vec::new();
    let mut idx = 0;
    loop {
        match syscall(KernelRequest::LinkAt { src, pred, idx }) {
            KernelResponse::LinkTarget { target, found: 1 } => {
                results.push(target);
                idx += 1;
            }
            KernelResponse::LinkTarget { found: 0, .. } => break,
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
    let mut cursor = ThingId(0);
    let kind_sym = sys_symbol_intern(T::KIND);
    
    loop {
        match syscall(KernelRequest::ThingList {
            kind: kind_sym,
            start_after: cursor,
        }) {
            KernelResponse::ThingListEntry { id: next_id, valid: 1 } => {
                if let Some(thing) = load_thing::<T>(next_id) {
                    results.push(thing);
                }
                cursor = next_id;
            }
            KernelResponse::ThingListEntry { valid: 0, .. } => break,
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
    // This helper likely redundant with user_update_thing, but kept for consistency
    let mut wire_props = Vec::with_capacity(props.len());
    
    for (key, val) in props {
        let key_sym = sys_symbol_intern(key);
        let wire_val = match val {
            PropValue::U64(v) => WirePropValue::u64(*v),
            PropValue::I64(v) => WirePropValue::i64(*v),
            PropValue::Bool(v) => WirePropValue::bool(*v),
            PropValue::Str(s) => WirePropValue::sym(sys_symbol_intern(s)),
            PropValue::Symbol(id) => WirePropValue::sym(*id),
            PropValue::Blob(b) => WirePropValue::blob(b.as_ptr() as u64, b.len() as u64),
        };
        wire_props.push(WireProp { key: key_sym, value: wire_val, _pad: 0 });
    }

    let request = KernelRequest::ThingUpdate {
        id,
        props: UserSlice::from_slice(&wire_props),
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
        KernelResponse::Error { err: _ } => Err("Spawn failed"),
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
        name: UserSlice::from_slice(name_static.as_bytes()),
        app_id,
        priority,
    }) {
        KernelResponse::ThreadCreated { tid } => Ok(ThingId(tid)),
        KernelResponse::Error { err: _ } => Err("CreateThread failed"),
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

/// Intern a string to get its SymbolId.
pub fn intern(s: &str) -> SymbolId {
    sys_symbol_intern(s)
}

#[cfg(test)]
pub mod mock;

#[cfg(test)]
mod lib_tests;
