//! System call numbers and constants.
//!
//! Frozen for v0.5.

// Core syscalls (0-63)
pub const SYS_EXIT: u32 = 1;
pub const SYS_DEBUG_WRITE: u32 = 2;
pub const SYS_SLEEP_MS: u32 = 3;
pub const SYS_DEVICE_CALL: u32 = 4;
pub const SYS_YIELD: u32 = 5;
pub const SYS_SPAWN_THREAD: u32 = 6;

// Time/Sched (64-127) - Reserved
// pub const SYS_TIME_NOW: u32 = 64;

// Stream syscalls (128-191)
pub const SYS_STREAM_OPEN: u32 = 128;
pub const SYS_STREAM_READ: u32 = 129;
pub const SYS_STREAM_POLL: u32 = 130;

// Watch/Event syscalls (192-255)
pub const SYS_WATCH_SUBSCRIBE: u32 = 192;
pub const SYS_WATCH_READ: u32 = 193;

// Root/Graph (256+) - Reserved
