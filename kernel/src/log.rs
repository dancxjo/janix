use crate::console;
use spin::Mutex;

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

static mut LOG_VIEW: [Option<&'static str>; MAX_LOG_ENTRIES] = [None; MAX_LOG_ENTRIES];

/// Aggregated log state guarded by a spinlock so writers on different CPUs
/// cannot stomp on each other's indices.
struct LogState {
    buffer: [LogEntry; MAX_LOG_ENTRIES],
    index: usize,
    count: usize,
    total_writes: usize,
    overwrites: usize,
    truncated: usize,
}

impl LogState {
    const fn new() -> Self {
        Self {
            buffer: [LogEntry::new(); MAX_LOG_ENTRIES],
            index: 0,
            count: 0,
            total_writes: 0,
            overwrites: 0,
            truncated: 0,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

static LOG_STATE: Mutex<LogState> = Mutex::new(LogState::new());

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
    let buffer_ptr = interrupts::without_interrupts(|| {
        let mut state = LOG_STATE.lock();
        state.reset();
        state.buffer.as_ptr()
    });

    crate::println!("LOG_BUFFER address: {:p}", buffer_ptr);
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
    interrupts::without_interrupts(|| {
        let mut state = LOG_STATE.lock();

        let truncated = message.as_bytes().len() > MAX_LOG_LEN;
        if truncated {
            state.truncated += 1;
        }

        if state.count == MAX_LOG_ENTRIES {
            state.overwrites += 1;
        } else {
            state.count += 1;
        }

        let idx = state.index;
        state.buffer[idx].write_from(message);
        state.index = (idx + 1) % MAX_LOG_ENTRIES;
        state.total_writes += 1;
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
        let state = LOG_STATE.lock();
        let available = state.count;
        let start = if available < MAX_LOG_ENTRIES {
            state.index + MAX_LOG_ENTRIES - available
        } else {
            state.index
        } % MAX_LOG_ENTRIES;

        for i in 0..available {
            let idx = (start + i) % MAX_LOG_ENTRIES;
            // SAFETY: buffer entries live for 'static because LOG_STATE is static.
            let entry: &'static LogEntry = &*(&state.buffer[idx] as *const LogEntry);
            LOG_VIEW[i] = entry.as_str();
        }

        &LOG_VIEW[..available]
    })
}

/// Get the number of log entries
pub fn log_count() -> usize {
    interrupts::without_interrupts(|| LOG_STATE.lock().count)
}

/// Return high-level statistics about the log buffer.
pub fn log_stats() -> LogStats {
    interrupts::without_interrupts(|| {
        let state = LOG_STATE.lock();
        LogStats {
            stored_entries: state.count,
            total_written: state.total_writes,
            overwritten: state.overwrites,
            truncated: state.truncated,
        }
    })
}
