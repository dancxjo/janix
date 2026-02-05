//! Framebuffer text console for early boot logging
//!
//! Wraps `budd::display::BootUpDisplay` to provide a character-oriented interface.

use crate::framebuffer::Framebuffer;
use spin::Mutex;
use core::sync::atomic::{AtomicBool, Ordering};

/// Global console instance
pub static CONSOLE: Mutex<Option<FbConsole>> = Mutex::new(None);

/// Flag to disable console when compositor takes over
pub static CONSOLE_DISABLED: AtomicBool = AtomicBool::new(false);

pub struct FbConsole {
    display: budd::display::BootUpDisplay<Framebuffer>,
    line_buf: [u8; 256],
    line_len: usize,
}

unsafe impl Send for FbConsole {}

impl FbConsole {
    pub fn new(fb: Framebuffer) -> Self {
        Self {
            display: budd::display::BootUpDisplay::new(fb),
            line_buf: [0; 256],
            line_len: 0,
        }
    }

    pub fn put_char(&mut self, c: u8) {
        if c == b'\n' {
            if let Ok(s) = core::str::from_utf8(&self.line_buf[..self.line_len]) {
                self.display.render_log_line(s);
            }
            self.line_len = 0;
        } else if c >= 0x20 || c == b'\t' { // Skip control chars except tab (treated as space?)
             // Simple tab handling: just add a space or multiple?
             // Bud parser expects structured logs mostly.
             // Just treat as char.
             if self.line_len < self.line_buf.len() {
                 self.line_buf[self.line_len] = c;
                 self.line_len += 1;
             }
        }
    }
}

/// Initialize the global console with framebuffer
pub fn init(fb: Framebuffer) {
    let console = FbConsole::new(fb);
    *CONSOLE.lock() = Some(console);
}

/// Disable the boot console
pub fn disable() {
    CONSOLE_DISABLED.store(true, Ordering::Relaxed);
}

/// Check if console is disabled
pub fn is_disabled() -> bool {
    CONSOLE_DISABLED.load(Ordering::Relaxed)
}

/// Write a character to the console
pub fn put_char(c: u8) {
    if CONSOLE_DISABLED.load(Ordering::Relaxed) {
        return;
    }
    if let Some(ref mut console) = *CONSOLE.lock() {
        console.put_char(c);
    }
}
