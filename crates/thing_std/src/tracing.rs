//! Function tracing for performance profiling
//!
//! Provides macros to trace function entry/exit with timing information.
//! All tracing is feature-gated behind `tracing` to have zero overhead in release.

use crate::time::monotonic_now;
use crate::debug::log;
use core::fmt::Write;

/// Stores the entry timestamp for duration calculation on exit
pub struct Tracepoint {
    name: &'static str,
    enter_ns: u64,
}

impl Tracepoint {
    /// Create a new tracepoint and log entry
    #[inline]
    pub fn enter(name: &'static str) -> Self {
        let enter_ns = monotonic_now();
        let mut buf = TraceBuf::new();
        let _ = write!(buf, "TRACE: {}: ENTER", name);
        buf.flush();
        Self { name, enter_ns }
    }
}

impl Drop for Tracepoint {
    fn drop(&mut self) {
        let exit_ns = monotonic_now();
        let elapsed_ns = exit_ns.saturating_sub(self.enter_ns);
        let elapsed_us = elapsed_ns / 1000;
        let elapsed_ms = elapsed_us / 1000;
        let elapsed_us_frac = elapsed_us % 1000;
        
        let mut buf = TraceBuf::new();
        let _ = write!(buf, "TRACE: {}: EXIT ({}.{:03}ms)", self.name, elapsed_ms, elapsed_us_frac);
        buf.flush();
    }
}

struct TraceBuf {
    buf: [u8; 128],
    len: usize,
}

impl TraceBuf {
    const fn new() -> Self {
        Self { buf: [0; 128], len: 0 }
    }
    
    fn flush(&self) {
        if self.len > 0 {
            if let Ok(s) = core::str::from_utf8(&self.buf[..self.len]) {
                log(s);
            }
        }
    }
}

impl Write for TraceBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let available = self.buf.len().saturating_sub(self.len);
        let to_copy = bytes.len().min(available);
        if to_copy > 0 {
            self.buf[self.len..self.len + to_copy].copy_from_slice(&bytes[..to_copy]);
            self.len += to_copy;
        }
        Ok(())
    }
}

/// Trace function entry/exit with timing. Creates RAII guard.
/// Usage: `trace_fn!("my_function");` at function start
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! trace_fn {
    ($name:expr) => {
        let _trace_guard = $crate::tracing::Tracepoint::enter($name);
    };
}

/// No-op when tracing is disabled
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! trace_fn {
    ($name:expr) => {};
}

/// Manual trace entry (for cases where RAII doesn't work well)
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! trace_enter {
    ($name:expr) => {{
        let enter_ns = $crate::time::monotonic_now();
        let mut buf = [0u8; 64];
        let msg = concat!("TRACE: ", $name, ": ENTER");
        $crate::debug::log(msg);
        enter_ns
    }};
}

#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! trace_enter {
    ($name:expr) => { 0u64 };
}

/// Manual trace exit with duration
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! trace_exit {
    ($name:expr, $enter_ns:expr) => {{
        let exit_ns = $crate::time::monotonic_now();
        let elapsed_us = (exit_ns.saturating_sub($enter_ns)) / 1000;
        let elapsed_ms = elapsed_us / 1000;
        let elapsed_us_frac = elapsed_us % 1000;
        // Use a simple buffer approach
        use core::fmt::Write;
        struct ExitBuf([u8; 64], usize);
        impl Write for ExitBuf {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                let bytes = s.as_bytes();
                let avail = self.0.len().saturating_sub(self.1);
                let n = bytes.len().min(avail);
                self.0[self.1..self.1+n].copy_from_slice(&bytes[..n]);
                self.1 += n;
                Ok(())
            }
        }
        let mut b = ExitBuf([0u8; 64], 0);
        let _ = write!(b, "TRACE: {}: EXIT ({}.{:03}ms)", $name, elapsed_ms, elapsed_us_frac);
        if let Ok(s) = core::str::from_utf8(&b.0[..b.1]) {
            $crate::debug::log(s);
        }
    }};
}

#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! trace_exit {
    ($name:expr, $enter_ns:expr) => {};
}
