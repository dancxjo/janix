use core::fmt;
use crate::debug::PortWrites;

pub trait Console {
    fn write_str(&self, s: &str);
}

pub struct StdoutConsole;

impl Console for StdoutConsole {
    fn write_str(&self, s: &str) {
        use core::fmt::Write;
        let _ = PortWrites.write_str(s);
    }
}

// Ensure it implements fmt::Write too if needed?
// The user code calls `c.write_str`.
