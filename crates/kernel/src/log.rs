//! Kernel logging subsystem
//!
//! Provides graph-native logging. Each log entry becomes a Thing in the graph.
//! All serial output goes through the kernel's internal serial drivers.

use alloc::vec::Vec;
use spin::Mutex;

use crate::boot::BootContext;
use crate::serial;
use crate::time;
use abi::ids::{SymbolId, ThingId};
use graph::store;
use graph::symbols;
use graph::symbols::sym;

/// Log level
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }
}

/// Log entry payload format (inline bytes for now)
///
/// Format:
/// [level:1][subsystem:8][flags:1][arrival_ns?:8][msg_len:2][message:M]
/// flags bit0 set => arrival_ns present
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub level: Level,
    pub subsystem: SymbolId,
    pub arrival_mono_ns: Option<u64>,
    pub message: Vec<u8>,
}

impl LogEntry {
    /// Serialize to inline payload format
    pub fn to_payload(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.push(self.level as u8);

        // Subsystem as u64 (SymbolId)
        let sym_bytes = self.subsystem.0.to_le_bytes();
        payload.extend_from_slice(&sym_bytes);

        let mut flags = 0u8;
        if self.arrival_mono_ns.is_some() {
            flags |= 0x1;
        }
        payload.push(flags);

        if let Some(ns) = self.arrival_mono_ns {
            payload.extend_from_slice(&ns.to_le_bytes());
        }

        // Message length and content
        let len = self.message.len() as u16;
        payload.extend_from_slice(&len.to_le_bytes());
        payload.extend_from_slice(&self.message);

        payload
    }
}

/// Global context reference for logging
static CONTEXT: Mutex<Option<&'static BootContext>> = Mutex::new(None);

/// Initialize logging with boot context
pub fn init(ctx: &'static BootContext) {
    *CONTEXT.lock() = Some(ctx);

    // Check if early output is requested
    if let Some(_early_putc) = ctx.early_putc {
        // Handled in boot.rs
    }

    // Now switch to internal serial drivers for the anchor lines
    serial::write(b"LOG: serial backend installed\n");
    // "Booted." will now be printed by kernel::boot at the very end
    // serial::write(b"Booted.\n");
}

/// Write raw bytes to serial (the unified path)
fn serial_write(bytes: &[u8]) {
    serial::write(bytes);
}

/// Emit a log entry
pub(crate) fn log_emit_with_arrival(
    level: Level,
    subsystem: SymbolId,
    arrival_mono_ns: u64,
    message: &[u8],
) -> Option<ThingId> {
    // Always output to serial for debugging
    serial_log(level, subsystem, Some(arrival_mono_ns), message);

    // Create graph entry if graph is initialized AND lock is available.
    // We use is_ready_for_logging() (try_lock) to avoid deadlocks where
    // a graph operation (holding lock) triggers a log (trying to acquire lock).
    if !store::is_ready_for_logging() {
        return None;
    }

    // Optimization: Only graph Warn/Error to save memory
    if level < Level::Warn {
        return None;
    }

    let kind = symbols::intern(b"kind.LogEntry");
    let _schema = symbols::intern(b"models.core.log.LogEntry");
    // TODO: Update graph::store to accept schema/version if needed, or update this call
    // Current store::thing_create only takes kind.
    let id = store::thing_create(kind);
    // Ignoring schema/version for now as per Task 01 Simplification

    let entry = LogEntry {
        level,
        subsystem,
        arrival_mono_ns: Some(arrival_mono_ns),
        message: message.to_vec(),
    };

    store::thing_set_inline_payload(id, &entry.to_payload());

    // Link to graph.logs
    if let Some(graph_logs) = store::find_thing_by_name(sym::GRAPH_LOGS) {
        store::relationship_create(sym::PRED_CONTAINS, graph_logs, id);
    }

    Some(id)
}

/// Emit a log entry with the current monotonic time as arrival timestamp.
pub fn log_emit(level: Level, subsystem: SymbolId, message: &[u8]) -> Option<ThingId> {
    let arrival_mono_ns = time::monotonic_now();
    log_emit_with_arrival(level, subsystem, arrival_mono_ns, message)
}

/// Kernel log helper
pub fn klog(level: Level, subsystem: &str, message: &str) {
    let sub_sym = symbols::intern(subsystem.as_bytes());
    log_emit(level, sub_sym, message.as_bytes());
}

/// Serial output for structured logging
fn serial_log(_level: Level, subsystem: SymbolId, arrival_mono_ns: Option<u64>, message: &[u8]) {
    if let Some(ns) = arrival_mono_ns {
        serial::write(b"[");
        let millis = ns / 1_000_000;
        let micros = (ns / 1_000) % 1_000;
        serial_write_decimal(millis);
        serial::write(b".");
        serial_write_padded_3(micros as u16);
        serial::write(b" ms] ");
    }

    // Format: "SUBSYSTEM: message\n"
    if let Some(sub_str) = symbols::resolve(subsystem) {
        serial::write(sub_str.as_bytes());
    } else {
        serial::write(b"SYM:");
    }

    serial::write(b": ");
    serial::write(message);
    serial::write(b"\n");
}

/// Simple kernel print
pub fn kprint(message: &str) {
    serial_write(message.as_bytes());
}

/// Simple kernel println
pub fn kprintln(message: &str) {
    kprint(message);
    kprint("\n");
}

fn serial_write_decimal(mut val: u64) {
    if val == 0 {
        serial::write(b"0");
        return;
    }

    let mut buf = [0u8; 20];
    let mut idx = buf.len();

    while val > 0 {
        idx -= 1;
        buf[idx] = b'0' + (val % 10) as u8;
        val /= 10;
    }

    serial::write(&buf[idx..]);
}

fn serial_write_padded_3(val: u16) {
    let hundreds = ((val / 100) % 10) as u8;
    let tens = ((val / 10) % 10) as u8;
    let ones = (val % 10) as u8;
    let buf = [b'0' + hundreds, b'0' + tens, b'0' + ones];
    serial::write(&buf);
}
