//! System call numbers and constants.

// Core syscalls (0-63)
pub const SYS_EXIT: u32 = 1;
pub const SYS_DEBUG_WRITE: u32 = 2;
pub const SYS_SLEEP_MS: u32 = 3;

// Stream syscalls (64-127)
pub const SYS_STREAM_OPEN: u32 = 64;
pub const SYS_STREAM_READ: u32 = 65;
pub const SYS_STREAM_POLL: u32 = 66;

// Watch/Event syscalls (128-191)
pub const SYS_WATCH_SUBSCRIBE: u32 = 128;
pub const SYS_WATCH_READ: u32 = 129;

// Reserved (192-255)
