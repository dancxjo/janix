//! Kernel logging subsystem
//!
//! Provides graph-native logging. Each log entry becomes a Thing in the graph.
//! All serial output goes through Architecture::serial_write for consistency.

use alloc::vec::Vec;
use spin::Mutex;

use crate::graph::{self, ThingId};
use crate::machine::Machine;
use crate::symbols::{self, SymbolId};

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

/// Global machine reference for logging
static MACHINE: Mutex<Option<&'static dyn Machine>> = Mutex::new(None);

/// Initialize logging with machine reference
pub fn init(machine: &'static dyn Machine) {
    *MACHINE.lock() = Some(machine);
    
    // Print the BDD anchor line - this is what the tests look for
    serial_write(b"Booted.\n");
    
    // Log that the serial backend is installed
    serial_write(b"LOG: serial backend installed\n");
}

/// Write raw bytes to serial (the unified path)
fn serial_write(bytes: &[u8]) {
    let guard = MACHINE.lock();
    if let Some(machine) = *guard {
        machine.arch().serial_write(bytes);
    }
}

/// Emit a log entry
/// 
/// Creates a LogEntry Thing in the graph and outputs to serial.
pub fn log_emit(level: Level, subsystem: SymbolId, message: &[u8]) -> Option<ThingId> {
    // Always output to serial for debugging
    serial_log(level, subsystem, message);
    
    // Create graph entry if graph is initialized
    let kind = symbols::sym_log_entry();
    if kind == SymbolId::INVALID {
        return None;
    }
    
    let schema = symbols::well_known(b"models.core.log.LogEntry");
    let id = graph::thing_create(kind, schema, 1);
    
    let entry = LogEntry {
        level,
        subsystem,
        message: message.to_vec(),
    };
    
    graph::thing_set_inline_payload(id, &entry.to_payload());
    
    Some(id)
}

/// Kernel log helper - logs with "KERNEL" subsystem
pub fn klog(level: Level, subsystem: &str, message: &str) {
    let sub_sym = symbols::intern(subsystem.as_bytes());
    log_emit(level, sub_sym, message.as_bytes());
}

/// Serial output for structured logging
fn serial_log(_level: Level, subsystem: SymbolId, message: &[u8]) {
    let guard = MACHINE.lock();
    if let Some(machine) = *guard {
        let arch = machine.arch();
        
        // Format: "SUBSYSTEM: message\n"
        if let Some(sub_str) = symbols::resolve(subsystem) {
            arch.serial_write(sub_str.as_bytes());
        } else {
            arch.serial_write(b"SYM:");
        }
        
        arch.serial_write(b": ");
        arch.serial_write(message);
        arch.serial_write(b"\n");
    }
}

/// Simple kernel print (bypasses graph, direct to serial)
pub fn kprint(message: &str) {
    serial_write(message.as_bytes());
}

/// Simple kernel println (bypasses graph, direct to serial)
pub fn kprintln(message: &str) {
    kprint(message);
    kprint("\n");
}
