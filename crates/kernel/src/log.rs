//! Kernel logging subsystem
//!
//! Provides graph-native logging. Each log entry becomes a Thing in the graph.
//! All serial output goes through the kernel's internal serial drivers.

use alloc::vec::Vec;
use spin::Mutex;

use crate::boot::BootContext;
use crate::serial;
use abi::ids::{SymbolId, ThingId};
use graph::store;
use graph::symbols;

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
/// Format: [level:1][subsystem_len:2][subsystem:N][message:M]
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub level: Level,
    pub subsystem: SymbolId,
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
pub fn log_emit(level: Level, subsystem: SymbolId, message: &[u8]) -> Option<ThingId> {
    // Always output to serial for debugging
    serial_log(level, subsystem, message);

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
        message: message.to_vec(),
    };

    store::thing_set_inline_payload(id, &entry.to_payload());

    // Link to place.log
    let place_log_sym = symbols::intern(b"place.log");
    if let Some(place_log) = store::find_thing_by_name(place_log_sym) {
        let pred_contains = symbols::intern(b"predicate.contains");
        store::relationship_create(pred_contains, place_log, id);
    }

    Some(id)
}

/// Kernel log helper
pub fn klog(level: Level, subsystem: &str, message: &str) {
    let sub_sym = symbols::intern(subsystem.as_bytes());
    log_emit(level, sub_sym, message.as_bytes());
}

/// Serial output for structured logging
fn serial_log(_level: Level, subsystem: SymbolId, message: &[u8]) {
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
