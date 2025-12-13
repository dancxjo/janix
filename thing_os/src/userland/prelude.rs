extern crate alloc;

use core::cmp::min;
use core::fmt::{self, Write};
use core::mem;
use spin::Mutex;

pub use alloc::boxed::Box;
pub use alloc::format;
pub use alloc::string::{String, ToString};
pub use alloc::vec;
pub use alloc::vec::Vec;

pub use abi::{PropKey, PropType, PropValue, ThingId};

pub use runtime::Sys;

pub use super::ui::{
    WindowHandle, append_window_text, create_window, ensure_ui_schemas, set_window_text,
};
pub use crate::{
    CpuCoreThing,
    MODE_INDEX_CONSOLE,
    ProcessThing,
    Thing,
    ThreadThing,
    active_mode,
    alarm::{self, Alarm, sleep_until},
    alloc_frame,
    clock::SystemClock,
    create_process,
    create_thing,
    create_thread,
    default_mode,
    free_frame,
    get_type_description,
    is_console_mode_active,
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

struct LogBufferWriter<'a> {
    buf: &'a mut [u8],
    written: usize,
}

impl<'a> LogBufferWriter<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, written: 0 }
    }

    fn len(&self) -> usize {
        self.written
    }
}

impl<'a> Write for LogBufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.written >= self.buf.len() {
            return Ok(());
        }
        let remaining = self.buf.len().saturating_sub(self.written);
        let bytes = s.as_bytes();
        let to_copy = min(remaining, bytes.len());
        if to_copy > 0 {
            let start = self.written;
            let end = start + to_copy;
            self.buf[start..end].copy_from_slice(&bytes[..to_copy]);
            self.written = end;
        }
        Ok(())
    }
}

/// Convenience logging helper for dynamic strings without per-call heap allocation
pub fn log_dynamic(sys: &impl Sys, args: fmt::Arguments<'_>) {
    let mut buffer = LOG_BUFFER.lock();
    let max_len = buffer.len().saturating_sub(1);
    let mut writer = LogBufferWriter::new(&mut buffer[..max_len]);
    let _ = writer.write_fmt(args);
    let len = writer.len();
    buffer[len] = 0;
    let slice = &buffer[..len];
    let temp = unsafe { core::str::from_utf8_unchecked(slice) };
    let static_str: &'static str = unsafe { mem::transmute::<&str, &'static str>(temp) };
    println(sys, static_str);
}
