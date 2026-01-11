use crate::BootRuntimeBase;
use core::fmt::{self, Write};

static mut WRITER: Option<Logger> = None;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
}

impl Level {
    fn as_str(&self) -> &'static str {
        match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
        }
    }
}

pub struct Logger {
    runtime: &'static dyn BootRuntimeBase,
}

impl Logger {
    pub const fn new(runtime: &'static dyn BootRuntimeBase) -> Self {
        Self { runtime }
    }
}

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.runtime.putchar(b);
        }
        Ok(())
    }
}

pub unsafe fn init(runtime: &'static dyn BootRuntimeBase) {
    unsafe {
        WRITER = Some(Logger::new(runtime));
    }
}

pub fn _log(level: Level, args: fmt::Arguments) {
    unsafe {
        if let Some(writer) = &mut *core::ptr::addr_of_mut!(WRITER) {
            let ticks = writer.runtime.mono_ticks();
            let _ = writer.write_fmt(format_args!("[{}] ", ticks));
            let _ = writer.write_fmt(format_args!("[{}] ", level.as_str()));
            let _ = writer.write_fmt(args);
            let _ = writer.write_char('\n');
        }
    }
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {
        $crate::logging::_log($crate::logging::Level::Info, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! kerror {
    ($($arg:tt)*) => {
        $crate::logging::_log($crate::logging::Level::Error, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! kwarn {
    ($($arg:tt)*) => {
        $crate::logging::_log($crate::logging::Level::Warn, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! kdebug {
    ($($arg:tt)*) => {
        $crate::logging::_log($crate::logging::Level::Debug, format_args!($($arg)*));
    };
}


pub fn _print(args: fmt::Arguments) {
    unsafe {
        if let Some(writer) = &mut *core::ptr::addr_of_mut!(WRITER) {
            let _ = writer.write_fmt(args);
        }
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::logging::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}
