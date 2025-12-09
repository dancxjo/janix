/// Maximum number of log entries
const MAX_LOG_ENTRIES: usize = 100;

/// Log entry storage
// SAFETY: LOG_BUFFER and LOG_INDEX are only accessed from single-threaded kernel context.
// In a multi-threaded environment, this would need atomic operations or locks.
static mut LOG_BUFFER: [Option<&str>; MAX_LOG_ENTRIES] = [None; MAX_LOG_ENTRIES];
static mut LOG_INDEX: usize = 0;

/// Initialize the log subsystem
/// This resets all log state for test isolation and kernel boot
pub fn init() {
    unsafe {
        // Reset counter
        LOG_INDEX = 0;

        // Clear all log entries
        let buffer = &raw mut LOG_BUFFER;
        for slot in (*buffer).iter_mut() {
            *slot = None;
        }
    }
}

/// Log a message
pub fn log_message(message: &'static str) {
    unsafe {
        if LOG_INDEX < MAX_LOG_ENTRIES {
            LOG_BUFFER[LOG_INDEX] = Some(message);
            LOG_INDEX += 1;
        }
    }
    crate::console::print(message);
    crate::console::print("\n");
}

/// Get all log entries
pub fn get_logs() -> &'static [Option<&'static str>] {
    unsafe { &LOG_BUFFER[..LOG_INDEX] }
}

/// Get the number of log entries
pub fn log_count() -> usize {
    unsafe { LOG_INDEX }
}
