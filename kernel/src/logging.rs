//! Unified Logging System v1.0
//!
//! Provides a canonical log format with:
//! - ts= monotonic timestamp
//! - lvl= level (ERROR/WARN/INFO/DEBUG/TRACE)
//! - cpu= cpu id
//! - tid= kernel thread id
//! - pid= userspace process id (or - for kernel)
//! - src= module path
//! - span= correlation id (optional)
//! - seq= global sequence number

use crate::BootRuntimeBase;
use core::fmt::{self, Write};
use spin::Mutex;
use alloc::format;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// Re-export for macros
pub use abi::logging::Level;
pub type LogLevel = Level;

static GLOBAL_LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
static IN_GRAPH_LOG: AtomicBool = AtomicBool::new(false);

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
    span_id: u64,
    name: &'static str,
}

impl LogTransaction {
    /// Begin a log transaction - acquires exclusive write access
    pub fn begin(name: &'static str) -> Self {
        let span_id = new_span();
        set_current_span(span_id);
        
        // Emit BEGIN marker
        let seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);
        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            let ts = writer.runtime.mono_ticks();
            let tid = unsafe { crate::task::scheduler::current_tid_current() };
            let _ = writeln!(writer, 
                "[ts={} lvl=INFO cpu=0 tid={} pid=- src=kernel::logging span={}#{} seq={}] BEGIN {}",
                ts, tid, name, span_id, seq, name
            );
        }
        drop(lock);
        
        Self { span_id, name }
    }
}

impl Drop for LogTransaction {
    fn drop(&mut self) {
        // Emit END marker
        let seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);
        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            let ts = writer.runtime.mono_ticks();
            let tid = unsafe { crate::task::scheduler::current_tid_current() };
            let _ = writeln!(writer, 
                "[ts={} lvl=INFO cpu=0 tid={} pid=- src=kernel::logging span={}#{} seq={}] END {}",
                ts, tid, self.name, self.span_id, seq, self.name
            );
        }
        drop(lock);
        clear_span();
    }
}

#[derive(Clone, Copy)]
pub struct LogMetadata<'a> {
    pub level: Level,
    pub file: &'a str,
    pub line: u32,
    pub module: &'a str,
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
    
    /// Write a complete log line atomically
    pub fn write_line(&mut self, s: &str) {
        for b in s.bytes() {
            self.runtime.putchar(b);
        }
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
    GLOBAL_LOGGER.force_unlock();
}

/// Helper to check if graph logging is safe/ready
fn can_log_to_graph(level: Level) -> bool {
    crate::root::is_inbox_ready() && level != Level::Trace
}

/// Format PID for display (- for kernel context)
fn format_pid(pid: u64) -> alloc::string::String {
    if pid == abi::logging::PID_KERNEL || pid == 0 {
        alloc::string::String::from("-")
    } else {
        format!("{}", pid)
    }
}

pub fn _log_event(
    meta: LogMetadata, 
    event_sym: &str, 
    msg_fmt: fmt::Arguments, 
    fields: &[(&str, u64)], 
    about: &[u64]
) {
    // Get sequence number first (guarantees ordering)
    let seq = GLOBAL_SEQ.fetch_add(1, Ordering::Relaxed);
    let span = current_span();
    
    // 1. Serial Output - build complete line then emit atomically
    {
        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            let ts = writer.runtime.mono_ticks();
            let tid = unsafe { crate::task::scheduler::current_tid_current() };
            
            // Build prefix
            let span_part = if span != 0 {
                format!(" span={}", span)
            } else {
                alloc::string::String::new()
            };
            
            // Unified format: [ts=N lvl=L cpu=0 tid=T pid=P src=M seq=S (span=X)?] msg
            let _ = write!(writer, 
                "[ts={} lvl={} cpu=0 tid={} pid=-{} src={} seq={}] ",
                ts, meta.level.as_str(), tid, span_part, event_sym, seq
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
            let tid = unsafe { crate::task::scheduler::current_tid_current() };
            let timestamp = crate::runtime_base().mono_ticks();
            let message = format!("{}", msg_fmt);
            
            use crate::root::{RootOp, SymbolShell, LogProvenance};
            
            let prov = LogProvenance {
                tid,
                cpu: 0, 
                module: alloc::string::String::from(meta.module),
                file: alloc::string::String::from(meta.file),
                line: meta.line,
            };
            
            let mut field_vec = alloc::vec::Vec::with_capacity(fields.len());
            for (k, v) in fields {
                field_vec.push((SymbolShell::Str(alloc::string::String::from(*k)), *v));
            }

            let op = RootOp::LogEvent {
                level: meta.level as u8, 
                event: SymbolShell::Str(alloc::string::String::from(event_sym)),
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
    _log_event(meta.clone(), meta.module, args, &[], &[]);
}

/// Log a raw string without any formatting (for kprint! compatibility)
pub fn _log_raw(args: fmt::Arguments) {
    let mut lock = GLOBAL_LOGGER.lock();
    if let Some(writer) = lock.as_mut() {
        let _ = writer.write_fmt(args);
    }
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
            $event,
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
            $event,
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
