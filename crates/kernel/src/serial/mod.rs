//! Kernel serial output routed through the machine interface.

use crate::machine;

pub fn putc(c: u8) {
    machine::machine().console_write(&[c]);
}

pub fn init() {
    // Machine backend performs its own hardware setup on first use.
}

pub fn write(bytes: &[u8]) {
    machine::machine().console_write(bytes);
}
