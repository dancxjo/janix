//! Unified Logging System v1.0
//!
//! Human-readable format: [TIME] [LEVEL] [SOURCE] Message
//! With optional span correlation for multi-line output.

use crate::BootRuntimeBase;
use alloc::format;
use core::fmt::{self, Write};
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use spin::Mutex;

// Re-export for macros
pub use abi::logging::Level;
pub type LogLevel = Level;

static GLOBAL_LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
static IN_GRAPH_LOG: AtomicBool = AtomicBool::new(false);
static MUTE_SERIAL: AtomicBool = AtomicBool::new(false);

/// Minimum log level to output (1=Error, 2=Warn, 3=Info, 4=Debug, 5=Trace, 0=Contract-only)
/// Default is 0 (contract-only) for performance. Set to 3 for Info+ during debugging.
static MIN_LOG_LEVEL: AtomicU8 = AtomicU8::new(4);

/// Set the minimum log level for output (0=Contract-only, 1=Error+, 2=Warn+, etc.)
pub fn set_log_level(level: u8) {
    MIN_LOG_LEVEL.store(level, Ordering::Relaxed);
}

/// Get the current minimum log level
pub fn get_log_level() -> u8 {
    MIN_LOG_LEVEL.load(Ordering::Relaxed)
}

/// Global sequence counter for log ordering
static GLOBAL_SEQ: AtomicU64 = AtomicU64::new(1);

/// Current active span (0 = none)
static CURRENT_SPAN: AtomicU64 = AtomicU64::new(0);

/// Global span counter for generating unique span IDs
static SPAN_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate a new unique span ID
pub fn new_span() -> u64 {
    SPAN_COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Set the current active span for this thread/context
pub fn set_current_span(span: u64) {
    CURRENT_SPAN.store(span, Ordering::Relaxed);
}

/// Get the current active span
pub fn current_span() -> u64 {
    CURRENT_SPAN.load(Ordering::Relaxed)
}

/// Clear the current span
pub fn clear_span() {
    CURRENT_SPAN.store(0, Ordering::Relaxed);
}

/// RAII guard for log transactions (multi-line atomic output)
pub struct LogTransaction {
    #[allow(dead_code)]
    span_id: u64,
    name: &'static str,
}

impl LogTransaction {
    /// Begin a log transaction - acquires exclusive write access
    pub fn begin(name: &'static str) -> Self {
        let span_id = new_span();
        set_current_span(span_id);

        // Emit BEGIN marker (always, like contract)
        let _seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);
        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            if !MUTE_SERIAL.load(Ordering::Relaxed) {
                let ts = writer.runtime.mono_ticks();
                let _ = writeln!(writer, "[{}] [INFO] [logging] BEGIN {}", ts, name);
            }
        }
        drop(lock);

        Self { span_id, name }
    }
}

impl Drop for LogTransaction {
    fn drop(&mut self) {
        // Emit END marker (always, like contract)
        let _seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);
        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            if !MUTE_SERIAL.load(Ordering::Relaxed) {
                let ts = writer.runtime.mono_ticks();
                let _ = writeln!(writer, "[{}] [INFO] [logging] END {}", ts, self.name);
            }
        }
        drop(lock);
        clear_span();
    }
}

#[derive(Clone, Copy)]
pub struct LogMetadata {
    pub level: Level,
    pub file: &'static str,
    pub line: u32,
    pub module: &'static str,
}

pub struct Logger {
    runtime: &'static dyn BootRuntimeBase,
}

impl Logger {
    pub const fn new(runtime: &'static dyn BootRuntimeBase) -> Self {
        Self { runtime }
    }

    #[inline]
    pub fn mono_ticks(&self) -> u64 {
        self.runtime.mono_ticks()
    }
}

// Safety: BootRuntimeBase is effectively a singleton VTable provided by BRAN.
unsafe impl Sync for Logger {}
unsafe impl Send for Logger {}

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.runtime.putchar(b);
        }
        Ok(())
    }
}

pub unsafe fn init(runtime: &'static dyn BootRuntimeBase) {
    *GLOBAL_LOGGER.lock() = Some(Logger::new(runtime));
}

pub unsafe fn force_unlock() {
    // SAFETY: Only called from panic handler when logger lock may be poisoned
    unsafe {
        GLOBAL_LOGGER.force_unlock();
    }
}

/// Helper to check if graph logging is safe/ready
fn can_log_to_graph(_level: Level) -> bool {
    // TEMP: Disabled for OOM debugging
    false
    // crate::root::is_inbox_ready() && level != Level::Trace
}

/// Check if this level should be logged (considering MIN_LOG_LEVEL)
#[inline]
fn should_log(level: Level) -> bool {
    let min = MIN_LOG_LEVEL.load(Ordering::Relaxed);
    // If min is 0, only contract! messages pass (they call _log_contract directly)
    // Otherwise, check if level <= min (Error=1 is most severe, Trace=5 is least)
    min > 0 && (level as u8) <= min
}

pub fn _log_event(
    meta: LogMetadata,
    event_sym: crate::root::SymbolShell,
    msg_fmt: fmt::Arguments,
    fields: &[(&'static str, u64)],
    about: &[u64],
) {
    // Check log level filter
    if !should_log(meta.level) {
        return;
    }

    // Get sequence number first (guarantees ordering)
    let _seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);

    // 1. Serial Output - human-readable format: [TIME] [LEVEL] [SOURCE] Message
    if !MUTE_SERIAL.load(Ordering::Relaxed) {
        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            let ts = writer.runtime.mono_ticks();
            let event_str = match &event_sym {
                crate::root::SymbolShell::Str(s) => s.as_str(),
                crate::root::SymbolShell::Static(s) => s,
                crate::root::SymbolShell::Id(_) => "?",
            };

            // Human-readable format: [TIME] [LEVEL] [SOURCE] [CPUx] Message
            let _ = write!(
                writer,
                "[{}] [{}] [{}] [CPU{}] ",
                ts,
                meta.level.as_str(),
                event_str,
                writer.runtime.current_cpu_id().0
            );
            let _ = writer.write_fmt(msg_fmt);

            // Append structured fields if any
            if !fields.is_empty() {
                for (k, v) in fields {
                    let _ = write!(writer, " {}={}", k, v);
                }
            }

            let _ = writer.write_char('\n');
        }
    }

    // 2. Graph Persistence (Best Effort) - skip TRACE to reduce noise
    if can_log_to_graph(meta.level) {
        if !IN_GRAPH_LOG.swap(true, Ordering::Acquire) {
            let tid = unsafe { crate::sched::current_tid_current() };
            let timestamp = crate::runtime_base().mono_ticks();
            let message = format!("{}", msg_fmt);

            use crate::root::{LogProvenance, RootOp, SymbolShell};

            let prov = LogProvenance {
                tid,
                cpu: 0,
                module: meta.module,
                file: meta.file,
                line: meta.line,
            };

            let mut field_vec = alloc::vec::Vec::with_capacity(fields.len());
            for (k, v) in fields {
                field_vec.push((SymbolShell::Static(k), *v));
            }

            let op = RootOp::LogEvent {
                level: meta.level as u8,
                event: event_sym,
                message,
                timestamp,
                provenance: prov,
                fields: field_vec,
                about: alloc::vec::Vec::from(about),
            };

            crate::root::enqueue(op);
            IN_GRAPH_LOG.store(false, Ordering::Release);
        }
    }
}

// Backward compatibility shim for kinfo! etc
pub fn _log(meta: LogMetadata, args: fmt::Arguments) {
    _log_event(
        meta.clone(),
        crate::root::SymbolShell::Static(meta.module),
        args,
        &[],
        &[],
    );
}

/// Contract-level logging - ALWAYS outputs regardless of log level filter.
/// Use for critical boot milestones and test verification points.
pub fn _log_contract(source: &'static str, args: fmt::Arguments) {
    let _seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);

    if !MUTE_SERIAL.load(Ordering::Relaxed) {
        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            let ts = writer.runtime.mono_ticks();
            let _ = write!(
                writer,
                "[{}] [CONTRACT] [{}] [CPU{}] ",
                ts,
                source,
                writer.runtime.current_cpu_id().0
            );
            let _ = writer.write_fmt(args);
            let _ = writer.write_char('\n');
        }
    }
}

/// Log a raw string without any formatting (for kprint! compatibility)
pub fn _log_raw(args: fmt::Arguments) {
    if !MUTE_SERIAL.load(Ordering::Relaxed) {
        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            let _ = writer.write_fmt(args);
        }
    }
}

/// Contract-level logging macro - ALWAYS outputs regardless of log level.
/// Use for critical boot milestones and BDD test verification points.
#[macro_export]
macro_rules! contract {
    ($($arg:tt)*) => {
        $crate::logging::_log_contract(module_path!(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_event {
    // With fields and about
    ($lvl:expr, $event:expr, $msg:expr, { $($k:ident : $v:expr),* }, about=[$($about:expr),*]) => {
        $crate::logging::_log_event(
            $crate::logging::LogMetadata {
                level: $lvl,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            $crate::root::SymbolShell::Static($event),
            format_args!($msg),
            &[ $( (stringify!($k), $v) ),* ],
            &[ $($about),* ]
        )
    };
    // With format args, no extra fields
    ($lvl:expr, $event:expr, $($arg:tt)*) => {
        $crate::logging::_log_event(
            $crate::logging::LogMetadata {
                level: $lvl,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            $crate::root::SymbolShell::Static($event),
            format_args!($($arg)*),
            &[],
            &[]
        )
    };
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Info,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! kerror {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Error,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! kwarn {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Warn,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! kdebug {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Debug,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! ktrace {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Trace,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::logging::_log_raw(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}
