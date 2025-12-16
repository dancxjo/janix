#![no_std]

extern crate alloc;

use thing_os::prelude::*;
use alloc::format;
pub use runtime::UserlandSys; // [CHANGED] Removed separate `use`, kept `pub use`
use alloc::boxed::Box;

mod text;
mod simple_window;

pub use text::draw_text_simple;
pub use simple_window::SimpleWindow;

/// Initializes logging and the ThingOS runtime.
/// Call this at the start of `main()`.
pub fn init() -> UserlandSys {
    UserlandSys::new()
}

/// Simple helper to sleep for a number of milliseconds.
pub fn sleep_ms(sys: &mut UserlandSys, ms: u64) {
    sys.sleep_for_ns(ms * 1_000_000);
}

/// Helper to log a message with a tag.
pub fn log(sys: &mut UserlandSys, tag: &str, msg: &str) {
    let s = format!("[{}] {}", tag, msg);
    // Leak the string to satisfy the ABI's static requirement for logs
    let leaked_s = Box::leak(s.into_boxed_str());
    thing_os::println(sys, leaked_s);
}
