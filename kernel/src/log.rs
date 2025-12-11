use crate::console;

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
        crate::println!("LOG_BUFFER address: {:p}", buffer);
    }
}

/// Log a message
pub fn log_message(message: &str) {
    unsafe {
        if LOG_INDEX < MAX_LOG_ENTRIES {
            let leaked = alloc::boxed::Box::leak(alloc::string::String::from(message).into_boxed_str());
            LOG_BUFFER[LOG_INDEX] = Some(leaked);
            LOG_INDEX += 1;
        } else {
             console::print("LOG BUFFER FULL\n");
        }
        if LOG_INDEX == 1 {
             console::print("LOG_BUFFER address: ");
             let ptr = &raw const LOG_BUFFER;
             // simple hex print
             // We can't use println! easily here if it recurses?
             // But console::print takes str.
             // We can use format! but that allocates.
             // Let's just print it in init()
        }
    }
    console::print(message);
    console::print("\n");
}

/// Get all log entries
pub fn get_logs() -> &'static [Option<&'static str>] {
    unsafe { &LOG_BUFFER[..LOG_INDEX] }
}

/// Get the number of log entries
pub fn log_count() -> usize {
    unsafe { LOG_INDEX }
}
