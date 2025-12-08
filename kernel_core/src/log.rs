/// Maximum number of log entries
const MAX_LOG_ENTRIES: usize = 100;

/// Log entry storage
static mut LOG_BUFFER: [Option<&str>; MAX_LOG_ENTRIES] = [None; MAX_LOG_ENTRIES];
static mut LOG_INDEX: usize = 0;

/// Initialize the log subsystem
pub fn init() {
    // Clear the log buffer
    unsafe {
        LOG_INDEX = 0;
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
}

/// Get all log entries
pub fn get_logs() -> &'static [Option<&'static str>] {
    unsafe { &LOG_BUFFER[..LOG_INDEX] }
}

/// Get the number of log entries
pub fn log_count() -> usize {
    unsafe { LOG_INDEX }
}
