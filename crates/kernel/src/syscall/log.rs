//! Logging Syscall

use crate::log::{self, Level, LogEntry};
use crate::time;
use alloc::vec::Vec;
use abi::ids::ThingId;
use abi::syscall::err;
use abi::wire::SyscallResult;
use graph::store;
use graph::symbols::{self, sym};

pub fn sys_log_emit(level_raw: u64, msg_ptr: u64, msg_len: u64) -> SyscallResult {
    // 1. Level
    let _level = match level_raw {
        0 => Level::Trace,
        1 => Level::Debug,
        2 => Level::Info,
        3 => Level::Warn,
        4 => Level::Error,
        _ => return SyscallResult::new(err::EINVAL, 0, 0),
    };

    // 2. Message
    // Safety: user ptr
    if msg_ptr == 0 || msg_len == 0 {
        return SyscallResult::new(err::EFAULT, 0, 0);
    }
    let msg = unsafe { core::slice::from_raw_parts(msg_ptr as *const u8, msg_len as usize) };

    let arrival_mono_ns = time::monotonic_now();
    let subsystem = symbols::intern(b"USER");

    // Log to serial for BDD/Debug visibility (with UTF-8 fallback)
    let display_msg = core::str::from_utf8(msg).unwrap_or("INVALID_UTF8");
    let logged = log::log_emit_with_arrival(_level, subsystem, arrival_mono_ns, display_msg.as_bytes());

    let entry = LogEntry {
        level: _level,
        subsystem,
        arrival_mono_ns: Some(arrival_mono_ns),
        message: Vec::from(msg),
    };

    if let Some(id) = logged {
        // Reuse the kernel-created entry but preserve the original message bytes.
        store::thing_set_inline_payload(id, &entry.to_payload());
        return SyscallResult::new(0, id.high(), id.low());
    }

    // 3. Create Log Thing (fallback for lower levels)
    let kind_log = symbols::intern(b"kind.log_entry");
    let entry_id = store::thing_create(kind_log);

    // 4. Set Payload (Inline String)
    store::thing_set_inline_payload(entry_id, &entry.to_payload());

    // 5. Link to graph.logs
    let graph_logs = store::find_thing_by_name(sym::GRAPH_LOGS).unwrap_or(ThingId(2)); // fallback root?
    store::relationship_create(sym::PRED_CONTAINS, graph_logs, entry_id);

    // 6. Link Level?
    // Optional.

    SyscallResult::new(0, entry_id.high(), entry_id.low())
}
