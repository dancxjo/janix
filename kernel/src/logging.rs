use crate::BootRuntimeBase;
use core::fmt::{self, Write};
use spin::Mutex;
use alloc::format;
use core::sync::atomic::{AtomicBool, Ordering};

// Re-export for macros
pub use crate::logging::Level as LogLevel;

static GLOBAL_LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
static IN_GRAPH_LOG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Raw = 255, // No metadata, no mandatory newline
}

impl Level {
    fn as_str(&self) -> &'static str {
        match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Raw => "RAW",
        }
    }
}

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
}

// Safety: BootRuntimeBase is effectively a singleton VTable provided by BRAN.
// We assume checking console lock etc is enough.
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
    level != Level::Raw && crate::root::is_inbox_ready()
}

pub fn _log_event(
    meta: LogMetadata, 
    event_sym: &str, 
    msg_fmt: fmt::Arguments, 
    fields: &[(&str, u64)], 
    about: &[u64]
) {
    // 1. Serial Output
    {
        let mut lock = GLOBAL_LOGGER.lock();
        if let Some(writer) = lock.as_mut() {
            match meta.level {
                Level::Raw => {
                    let _ = writer.write_fmt(msg_fmt);
                },
                _ => {
                    let ticks = writer.runtime.mono_ticks();
                    // Serial format: [TICKS] [LEVEL] [EVENT] MSG
                     let _ = writer.write_fmt(format_args!("[{}] [{}] [{}] ", ticks, meta.level.as_str(), event_sym));
                    let _ = writer.write_fmt(msg_fmt);
                    let _ = writer.write_char('\n');
                }
            }
        }
    }

    // 2. Graph Persistence (Best Effort)
    if can_log_to_graph(meta.level) {
        if !IN_GRAPH_LOG.swap(true, Ordering::Acquire) {
            
            let tid = unsafe { crate::task::scheduler::current_tid_current() };
            // Note: mono_ticks requires runtime reference, we can get it from global if we had it, 
            // or just use 0 if unsafe. We'll use runtime_base() safely-ish.
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
    _log_event(meta, "log.generic", args, &[], &[]);
}

#[macro_export]
macro_rules! log_event {
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
    // No fields, no about
     ($lvl:expr, $event:expr, $msg:expr) => {
        $crate::logging::_log_event(
            $crate::logging::LogMetadata {
                level: $lvl,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            $event,
            format_args!($msg),
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
macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::logging::_log(
            $crate::logging::LogMetadata {
                level: $crate::logging::LogLevel::Raw,
                file: file!(),
                line: line!(),
                module: module_path!(),
            },
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}
