#![cfg_attr(target_os = "none", no_std)]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod console;
pub mod heap;
pub mod mem;
pub mod panic;
pub mod sys;
pub mod syscalls;

pub mod alarm;
pub mod batch;
pub mod clock;
pub mod demo_shared;
pub mod display;
pub mod prelude;
pub mod resident;
pub mod time;
pub mod ui;
pub use display::*;

use abi::{
    FrameId, FrameInfo, MemorySummary, NodeId, SchedulerSummary,
    syscall_defs::SymbolId,
    wire::{
        common::{UserPtr, UserSlice},
        graph::{WireProp, WirePropValue, WireSchemaProp, WireValueTag},
    },
};
pub use abi::{KernelRequest, KernelResponse};
pub use alloc::boxed::Box;
pub use alloc::format;
pub use alloc::rc::Rc;
pub use alloc::string::{String, ToString};
pub use alloc::sync::Arc;
pub use alloc::vec;
pub use alloc::vec::Vec;

pub use thing_models::graph_kinds;
pub mod graph_ops;

use syscalls::sys_symbol_intern;
use thing_models::graph_kinds::{
    KIND_SHARED_BUFFER, PROP_DISPLAY_ACTIVE_BUFFER_INDEX, PROP_HEIGHT, PROP_NAME,
    PROP_PIXEL_FORMAT, PROP_STRIDE, PROP_WIDTH,
};

use crate::sys::raw_syscall;
pub use abi; // Export abi crate
use abi::syscalls::SYSCALL_THING_GET;
pub use abi::{Predicate, ThingId};
pub use alarm::{Alarm, sleep_until};
pub use clock::SystemClock;
pub use thing_models::{DisplayFramebuffer, DisplayPowerState, DisplayPresentRequest};
pub use thing_macros::main;
pub use thing_models::Thing;
pub use thing_models::{
    AlarmEvent, AlarmRequest, Cursor, MODE_INDEX_CONSOLE, Mode, ModeSwitchEvent, Place, RawModule,
    Surface, TimeSource, View, Window,
};
pub use thing_models::{PropKey, PropType, PropValue};

/// Return the currently active `Mode` Thing, if one is marked active.
pub fn active_mode() -> Option<Mode> {
    list_things_by_kind::<Mode>().into_iter().find(|m| m.active)
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

// Re-export core models to replace shadow structs
pub use thing_models::CpuCore as CpuCoreThing;
pub use thing_models::Display as DisplayThing;
pub use thing_models::Process as ProcessThing;
pub use thing_models::SharedBuffer as SharedBufferThing;
pub use thing_models::Thread as ThreadThing;

/// Query a thing in the kernel graph and return the associated value.
pub fn graph_query(node_id: NodeId) -> Option<u64> {
    let request = KernelRequest::GraphQuery {
        node_id,
        out: UserSlice::default(),
    };
    match syscalls::syscall(request) {
        _ => None,
    }
}

/// Create a transaction and return its ID.
pub fn create_transaction() -> Option<abi::TransactionId> {
    let request = KernelRequest::CreateTransaction;
    match syscalls::syscall(request) {
        KernelResponse::TransactionCreated { tx_id } => Some(tx_id),
        _ => None,
    }
}

/// Commit an open transaction created via [`create_transaction`].
pub fn commit_transaction(tx_id: abi::TransactionId) -> bool {
    let request = KernelRequest::CommitTransaction { tx_id };
    matches!(syscalls::syscall(request), KernelResponse::Success { .. })
}

/// Create a new Thing from a raw kind and property slice.
pub fn user_create_thing(
    kind: &str,
    props: &[(PropKey, PropValue)],
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
        wire_props.push(WireProp {
            key: key_sym,
            value: wire_val,
            _pad: 0,
        });
    }

    let request = KernelRequest::ThingCreate {
        kind: kind_sym,
        props: UserSlice::from_slice(&wire_props),
    };
    match syscalls::syscall(request) {
        KernelResponse::ThingCreated { id } => Ok(id),
        KernelResponse::Error { err: _ } => Err("Error creating thing"),
        _ => Err("Unexpected response"),
    }
}

/// Update properties of an existing Thing using a property slice.
pub fn user_update_thing(
    id: ThingId,
    props: &[(PropKey, PropValue)],
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
        wire_props.push(WireProp {
            key: key_sym,
            value: wire_val,
            _pad: 0,
        });
    }

    let request = KernelRequest::ThingUpdate {
        id,
        props: UserSlice::from_slice(&wire_props),
    };
    match syscalls::syscall(request) {
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
        wire_props.push(WireProp {
            key: key_sym,
            value: wire_val,
            _pad: 0,
        });
    }

    let request = KernelRequest::ThingCreate {
        kind: kind_sym,
        props: UserSlice::from_slice(&wire_props),
    };
    match syscalls::syscall(request) {
        KernelResponse::ThingCreated { id } => Some(id),
        _ => None,
    }
}

/// Load a typed `Thing` from the kernel.
pub fn load_thing<T: Thing>(id: ThingId) -> Option<T> {
    const MAX_PROPS: usize = 32;

    let mut buf: [WireProp; MAX_PROPS] = [WireProp {
        key: SymbolId(0),
        value: WirePropValue::u64(0),
        _pad: 0,
    }; MAX_PROPS];

    let ptr = buf.as_mut_ptr() as u64;
    let len = buf.len() as u64;
    let ret = unsafe { raw_syscall(SYSCALL_THING_GET, id.0, ptr, len, 0, 0, 0) };

    if ret == u64::MAX {
        return None;
    }

    let count = core::cmp::min(ret as usize, MAX_PROPS);
    let mut props: Vec<Option<(PropKey, PropValue)>> = Vec::with_capacity(count);

    for i in 0..count {
        let wp = &buf[i];
        let mut key_buf = [0u8; 128];

        let key_req = abi::syscall_defs::SymbolResolveReq {
            id: wp.key,
            out_ptr: key_buf.as_mut_ptr() as u64,
            out_cap: key_buf.len() as u64,
        };

        let mut resp = abi::syscall_defs::SymbolResolveResp { written: 0 };
        let resolve_ret = unsafe {
            raw_syscall(
                abi::syscalls::SYSCALL_SYMBOL_RESOLVE,
                &key_req as *const _ as u64,
                &mut resp as *mut _ as u64,
                0,
                0,
                0,
                0,
            )
        };

        if resolve_ret != 0 {
            continue;
        }

        let key_len = core::cmp::min(resp.written as usize, key_buf.len());
        let key_str = core::str::from_utf8(&key_buf[..key_len]).ok()?.to_string();

        let val = match wp.value.tag {
            t if t == WireValueTag::U64 as u8 => PropValue::U64(wp.value.data_0),
            t if t == WireValueTag::I64 as u8 => PropValue::I64(wp.value.data_0 as i64),
            t if t == WireValueTag::Bool as u8 => PropValue::Bool(wp.value.data_0 != 0),
            t if t == WireValueTag::Str as u8 => {
                let sym_id = SymbolId(wp.value.data_0 as u32);

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
                        0,
                        0,
                        0,
                        0,
                    )
                };
                if str_ret == 0 {
                    let slen = core::cmp::min(str_resp.written as usize, str_buf.len());
                    let s = core::str::from_utf8(&str_buf[..slen]).ok()?.to_string();
                    PropValue::Str(s)
                } else {
                    PropValue::Str(String::new())
                }
            }
            _ => continue,
        };

        props.push(Some((key_str, val)));
    }

    Some(T::from_props(id, &props))
}

/// Check if an existing schema matches the expected schema for T.
fn schema_matches<T: Thing>(existing: &[Option<(SymbolId, PropType)>]) -> bool {
    let expected = T::schema();
    let existing_count = existing.iter().flatten().count();
    if existing_count != expected.len() {
        return false;
    }

    for (key_str, exp_pt) in expected {
        let key_sym = sys_symbol_intern(*key_str);
        let found = existing.iter().flatten().find(|(k, _)| *k == key_sym);
        let Some((_, got_pt)) = found else {
            return false;
        };
        if got_pt != exp_pt {
            return false;
        }
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
            PropType::Str => WireValueTag::Str,
            PropType::Symbol => WireValueTag::Str,
            PropType::Blob => WireValueTag::Blob,
        };
        wire_schema.push(WireSchemaProp {
            name: key_sym,
            prop_type: tag as u32,
        });
    }

    let request = KernelRequest::SchemaRegisterPackage {
        kind: kind_sym,
        description: desc_sym,
        props: UserSlice::from_slice(&wire_schema),
        links: UserSlice::default(),
    };

    match syscalls::syscall(request) {
        KernelResponse::SchemaRegistered { outcome, .. } => match outcome {
            abi::SchemaRegistryOutcome::Created => true,
            abi::SchemaRegistryOutcome::AlreadyRegisteredSame => true,
            abi::SchemaRegistryOutcome::Conflict => {
                println!("schema register conflict for kind {}", T::KIND);
                false
            }
        },
        KernelResponse::Error { err } => {
            println!("schema register failed for kind {}: {:?}", T::KIND, err);
            false
        }
        other => {
            println!(
                "schema register unexpected response for kind {}: {:?}",
                T::KIND,
                other
            );
            false
        }
    }
}

/// Ensure that the schema for `T` exists in the kernel.
/// Returns true if it exists.
pub fn ensure_schema_exists_for<T: Thing>() -> bool {
    let kind_sym = sys_symbol_intern(T::KIND);

    const MAX_SCHEMA_PROPS: usize = 16;
    let mut buf = [WireSchemaProp {
        name: SymbolId(0),
        prop_type: 0,
    }; MAX_SCHEMA_PROPS];
    let out = UserSlice::new(UserPtr::new(buf.as_mut_ptr() as u64), buf.len() as u64);

    match syscalls::syscall(KernelRequest::SchemaGet {
        kind: kind_sym,
        out,
    }) {
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
        match syscalls::syscall(KernelRequest::LinkAt { src, pred, idx }) {
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
        syscalls::syscall(KernelRequest::AddLink { src, pred, dst }),
        KernelResponse::Success { .. }
    )
}

/// List all Things of a given `T::KIND`.
pub fn list_things_by_kind<T: Thing>() -> Vec<T> {
    let mut results = Vec::new();
    let mut cursor = ThingId(0);
    let kind_sym = sys_symbol_intern(T::KIND);

    loop {
        match syscalls::syscall(KernelRequest::ThingList {
            kind: kind_sym,
            start_after: cursor,
        }) {
            KernelResponse::ThingListEntry {
                id: next_id,
                valid: 1,
            } => {
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
        wire_props.push(WireProp {
            key: key_sym,
            value: wire_val,
            _pad: 0,
        });
    }

    let request = KernelRequest::ThingUpdate {
        id,
        props: UserSlice::from_slice(&wire_props),
    };
    matches!(syscalls::syscall(request), KernelResponse::Success { .. })
}

/// Return a summary of the current physical memory state.
pub fn memory_summary() -> Option<MemorySummary> {
    match syscalls::syscall(KernelRequest::GetMemorySummary) {
        KernelResponse::MemorySummary { summary } => Some(summary),
        _ => None,
    }
}

/// Return a summary of the scheduler state exposed by the kernel.
pub fn scheduler_summary() -> Option<SchedulerSummary> {
    match syscalls::syscall(KernelRequest::GetSchedulerSummary) {
        KernelResponse::SchedulerSummary { summary } => Some(summary),
        _ => None,
    }
}

/// Allocate a zero-addressed frame from the first frame pool.
pub fn alloc_frame() -> Option<FrameInfo> {
    match syscalls::syscall(KernelRequest::AllocFrame { pool_index: 0 }) {
        KernelResponse::FrameAllocated { frame } => Some(frame),
        _ => None,
    }
}

/// Create a new process by spawning a known boot program.
pub fn create_process(boot_program_id: ThingId) -> Result<(ThingId, ThingId), &'static str> {
    match syscalls::syscall(KernelRequest::SpawnProgram { boot_program_id }) {
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
    match syscalls::syscall(KernelRequest::CreateThread {
        pid: pid.0,
        name: UserSlice::from_slice(name.as_bytes()),
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
    match syscalls::syscall(KernelRequest::FreeFrame { frame_id }) {
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

// Watch Wrappers
use abi::wire::graph::{WatchEvent, WatchSpec, WatchId};
use abi::syscall_defs::{SYSCALL_WATCH_OPEN, SYSCALL_WATCH_NEXT, SYSCALL_WATCH_CLOSE};

pub fn watch_open(spec: &WatchSpec) -> Option<WatchId> {
    let ret = unsafe { match sys::raw_syscall(SYSCALL_WATCH_OPEN, spec as *const _ as u64, 0, 0, 0, 0, 0) {
        u64::MAX => None,
        val => Some(WatchId(val)),
    }};
    ret
}

pub fn watch_next(id: WatchId, out: &mut [WatchEvent]) -> Option<usize> {
    let ret = unsafe { sys::raw_syscall(
        SYSCALL_WATCH_NEXT, 
        id.0, 
        out.as_mut_ptr() as u64, 
        (out.len() * core::mem::size_of::<WatchEvent>()) as u64, 
        0, 0, 0
    )};
    if ret == u64::MAX {
        None
    } else {
        Some(ret as usize)
    }
}

pub fn watch_close(id: WatchId) -> bool {
    let ret = unsafe { sys::raw_syscall(SYSCALL_WATCH_CLOSE, id.0, 0, 0, 0, 0, 0) };
    ret == 0
}
