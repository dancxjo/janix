use crate::console;

/// Maximum number of log entries kept in memory. Older entries are
/// overwritten in a ring-buffer fashion.
const MAX_LOG_ENTRIES: usize = 100;

/// Maximum length of a single log entry. Longer messages will be truncated to keep
/// the log buffer bounded and to avoid allocating from the kernel heap. The
/// current value keeps the dashboard tidy while capturing useful context.
const MAX_LOG_LEN: usize = 256;

/// Fixed-size log entry used to store UTF-8 bytes without dynamic allocation.
#[derive(Clone, Copy)]
struct LogEntry {
    len: usize,
    data: [u8; MAX_LOG_LEN],
}

impl LogEntry {
    const fn new() -> Self {
        Self {
            len: 0,
            data: [0; MAX_LOG_LEN],
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn write_from(&mut self, message: &str) {
        let bytes = message.as_bytes();
        let len = bytes.len().min(MAX_LOG_LEN);
        self.len = len;
        self.data[..len].copy_from_slice(&bytes[..len]);
    }

    fn as_str(&'static self) -> Option<&'static str> {
        if self.is_empty() {
            None
        } else {
            // SAFETY: data[..len] is always initialized and derived from a &str.
            Some(unsafe { core::str::from_utf8_unchecked(&self.data[..self.len]) })
        }
    }
}

/// Log entry storage
// SAFETY: LOG_BUFFER and associated indices are only accessed from single-threaded
// kernel context. In a multi-threaded environment, this would need atomic operations
// or locks.
static mut LOG_BUFFER: [LogEntry; MAX_LOG_ENTRIES] = [LogEntry::new(); MAX_LOG_ENTRIES];
static mut LOG_INDEX: usize = 0;
static mut LOG_COUNT: usize = 0;
static mut LOG_TOTAL_WRITES: usize = 0;
static mut LOG_OVERWRITES: usize = 0;
static mut LOG_TRUNCATED: usize = 0;
static mut LOG_VIEW: [Option<&'static str>; MAX_LOG_ENTRIES] = [None; MAX_LOG_ENTRIES];

/// Summary of the in-kernel log buffer useful for dashboards and crash dumps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogStats {
    /// Number of entries currently retained in the ring buffer.
    pub stored_entries: usize,
    /// Total calls to [`log_message`].
    pub total_written: usize,
    /// Count of entries overwritten once the buffer was full.
    pub overwritten: usize,
    /// Number of messages that were truncated to fit in [`MAX_LOG_LEN`].
    pub truncated: usize,
}

/// Initialize the log subsystem
/// This resets all log state for test isolation and kernel boot
pub fn init() {
    unsafe {
        // Reset counter
        LOG_INDEX = 0;
        LOG_COUNT = 0;
        LOG_TOTAL_WRITES = 0;
        LOG_OVERWRITES = 0;
        LOG_TRUNCATED = 0;

        // Clear all log entries
        let buffer = &raw mut LOG_BUFFER;
        for slot in (*buffer).iter_mut() {
            *slot = LogEntry::new();
        }
        crate::println!("LOG_BUFFER address: {:p}", buffer);
    }
}

#[cfg(target_arch = "x86_64")]
use x86_64::instructions::interrupts;

#[cfg(not(target_arch = "x86_64"))]
mod interrupts {
    #[inline]
    pub fn without_interrupts<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        f()
    }
}

/// Log a message
pub fn log_message(message: &str) {
    interrupts::without_interrupts(|| unsafe {
        let truncated = message.as_bytes().len() > MAX_LOG_LEN;
        if truncated {
            LOG_TRUNCATED += 1;
        }

        if LOG_COUNT == MAX_LOG_ENTRIES {
            LOG_OVERWRITES += 1;
        } else {
            LOG_COUNT += 1;
        }

        LOG_BUFFER[LOG_INDEX].write_from(message);
        LOG_INDEX = (LOG_INDEX + 1) % MAX_LOG_ENTRIES;
        LOG_TOTAL_WRITES += 1;
    });

    #[cfg(all(feature = "debug_logging", not(test)))]
    {
        console::print(message);
        console::print("\r");
    }
}

/// Get all log entries
pub fn get_logs() -> &'static [Option<&'static str>] {
    interrupts::without_interrupts(|| unsafe {
        // Render a chronological view into LOG_VIEW to preserve the existing return
        // type without introducing heap allocations.
        let available = LOG_COUNT;
        let start = if available < MAX_LOG_ENTRIES {
            LOG_INDEX + MAX_LOG_ENTRIES - available
        } else {
            LOG_INDEX
        } % MAX_LOG_ENTRIES;

        for i in 0..available {
            let idx = (start + i) % MAX_LOG_ENTRIES;
            LOG_VIEW[i] = LOG_BUFFER[idx].as_str();
        }

        &LOG_VIEW[..available]
    })
}

/// Get the number of log entries
pub fn log_count() -> usize {
    unsafe { LOG_COUNT }
}

/// Return high-level statistics about the log buffer.
pub fn log_stats() -> LogStats {
    unsafe {
        LogStats {
            stored_entries: LOG_COUNT,
            total_written: LOG_TOTAL_WRITES,
            overwritten: LOG_OVERWRITES,
            truncated: LOG_TRUNCATED,
        }
    }
}
