/// Interface for a low-level console sink (putchar).
/// This allows the standard crate to transparently route output
/// either to the kernel logger (if running in kernel) or a syscall (if in userspace).
pub trait ConsoleSink: Sync + Send {
    fn putchar(&self, c: u8);
}

static mut CONSOLE_SINK: Option<&'static dyn ConsoleSink> = None;

/// Sets the global console sink.
/// 
/// # Safety
/// This should only be called once during early initialization (kernel boot or user start).
pub unsafe fn set_console_sink(sink: &'static dyn ConsoleSink) {
    CONSOLE_SINK = Some(sink);
}

/// Writes a byte to the console sink, if one is registered.
#[inline]
pub fn console_putchar(c: u8) {
    if let Some(sink) = unsafe { CONSOLE_SINK } {
        sink.putchar(c);
    }
}
