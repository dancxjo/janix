extern crate alloc;

use core::cmp::min;
use core::mem;
use spin::Mutex;

pub use alloc::boxed::Box;
pub use alloc::format;
pub use alloc::string::{String, ToString};
pub use alloc::vec;
pub use alloc::vec::Vec;

pub use abi::{PropKey, PropType, PropValue, ThingId};

pub use userland_rt::Sys;

pub use crate::ui::{
    WindowHandle, append_window_text, create_window, ensure_ui_schemas, set_window_text,
};
pub use userland_std::{
    CpuCoreThing,
    ProcessThing,
    Thing,
    ThreadThing,
    alarm::{self, Alarm, sleep_until},
    alloc_frame,
    clock::SystemClock,
    create_process,
    // Add other common exports as needed
    create_thing,
    create_thread,
    free_frame,
    get_type_description,
    list_things_by_kind,
    load_thing,
    memory_summary,
    println,
    register_schema_for,
    scheduler_summary,
    time,
    update_props,
};

const LOG_BUFFER_LEN: usize = 256;
static LOG_BUFFER: Mutex<[u8; LOG_BUFFER_LEN]> = Mutex::new([0; LOG_BUFFER_LEN]);

/// Convenience logging helper for dynamic Strings
pub fn log_dynamic(sys: &impl Sys, msg: String) {
    let bytes = msg.as_bytes();
    let mut buffer = LOG_BUFFER.lock();
    let len = min(bytes.len(), buffer.len().saturating_sub(1));
    buffer[..len].copy_from_slice(&bytes[..len]);
    buffer[len] = 0;
    let slice = &buffer[..len];
    let temp = unsafe { core::str::from_utf8_unchecked(slice) };
    let static_str: &'static str = unsafe { mem::transmute::<&str, &'static str>(temp) };
    println(sys, static_str);
}
