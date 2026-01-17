//! Logging Syscall

use crate::log::{self, Level, LogEntry};
use crate::time;
use alloc::vec::Vec;
use abi::ids::ThingId;
use abi::syscall::err;
use abi::wire::SyscallResult;
use graph::store;
use graph::symbols::{self, sym};
use crate::syscall::user_mem;
use abi::cap::CapOp;

pub fn sys_log_emit(level_raw: u64, msg_ptr: u64, msg_len: u64) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::Log, None) {
        return SyscallResult::new(code, 0, 0);
    }

    // 1. Level
    let level = match level_raw {
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
    let msg_len = match usize::try_from(msg_len) {
        Ok(len) => len,
        Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
    };

    // Optimization: Use stack buffer for small messages to avoid initial Vec allocation.
    // We still allocate a Vec for the LogEntry, but we avoid the double allocation
    // (temporary buffer + LogEntry buffer) and duplicated graph ops.
    const STACK_BUF_SIZE: usize = 256;
    let mut stack_buf = [0u8; STACK_BUF_SIZE];

    let msg_vec = if msg_len <= STACK_BUF_SIZE {
        if let Err(code) = user_mem::copy_from_user(&mut stack_buf[..msg_len], msg_ptr, msg_len) {
            return SyscallResult::new(code, 0, 0);
        }
        Vec::from(&stack_buf[..msg_len])
    } else {
        let mut buf = alloc::vec![0u8; msg_len];
        if let Err(code) = user_mem::copy_from_user(&mut buf, msg_ptr, msg_len) {
            return SyscallResult::new(code, 0, 0);
        }
        buf
    };

    let arrival_mono_ns = time::monotonic_now();
    let subsystem = symbols::intern(b"USER");

    // Log to serial for BDD/Debug visibility (with UTF-8 fallback)
    // We use the same formatting as kernel logs, but we do it manually here
    // to avoid the double-graph-write that log::log_emit_with_arrival would do.
    let display_msg = core::str::from_utf8(&msg_vec).unwrap_or("INVALID_UTF8");
    log::serial_log(level, subsystem, Some(arrival_mono_ns), display_msg.as_bytes());

    // 3. Create Log Thing
    // We use "kind.LogEntry" to match the kernel logger (crates/kernel/src/log.rs).
    let kind_log = symbols::intern(b"kind.LogEntry");
    let entry_id = store::thing_create(kind_log);

    let entry = LogEntry {
        level,
        subsystem,
        arrival_mono_ns: Some(arrival_mono_ns),
        message: msg_vec, // Move the vector, avoiding copy/allocation
    };

    // 4. Set Payload (Inline String)
    store::thing_set_inline_payload(entry_id, &entry.to_payload());

    // 5. Link to graph.logs
    let graph_logs = store::find_thing_by_name(sym::GRAPH_LOGS).unwrap_or(ThingId(2));
    store::relationship_create(sym::PRED_CONTAINS, graph_logs, entry_id);

    SyscallResult::new(0, entry_id.high(), entry_id.low())
}
