// Core syscalls (0-63)
pub const SYS_EXIT: u32 = 1;
pub const SYS_DEBUG_WRITE: u32 = 2;
pub const SYS_SLEEP_MS: u32 = 3;
pub const SYS_DEVICE_CALL: u32 = 4;
pub const SYS_YIELD: u32 = 5;
pub const SYS_SPAWN_THREAD: u32 = 6;
pub const SYS_SPAWN_PROCESS: u32 = 7;
pub const SYS_TIME_MONOTONIC: u32 = 8;
pub const SYS_RTC_READ: u32 = 9;
pub const SYS_SLEEP_NS: u32 = 10;
pub const SYS_LOG_WRITE: u32 = 11;
pub const SYS_GET_TID: u32 = 12;
pub const SYS_TASK_POLL: u32 = 18;

// Device Capability Syscalls
pub const SYS_DEVICE_CLAIM: u32 = 13;
pub const SYS_DEVICE_MAP_MMIO: u32 = 14;
pub const SYS_DEVICE_IRQ_SUBSCRIBE: u32 = 15;
pub const SYS_DEVICE_IOPORT_READ: u32 = 16;
pub const SYS_DEVICE_IOPORT_WRITE: u32 = 17;


// Stream syscalls (128-191)
pub const SYS_STREAM_OPEN: u32 = 128;
pub const SYS_STREAM_READ: u32 = 129;
pub const SYS_STREAM_POLL: u32 = 130;

// Watch/Event syscalls (192-255)
pub const SYS_WATCH_SUBSCRIBE: u32 = 192;
pub const SYS_WATCH_READ: u32 = 193;

// Root/Graph (256+)
pub const SYS_ROOT_GET_KIND: u32 = 256;
pub const SYS_ROOT_BYTESPACE_CREATE: u32 = 257;
pub const SYS_ROOT_BYTESPACE_READ: u32 = 258; // NEW
pub const SYS_ROOT_WATCH_SUBSCRIBE: u32 = 259;
pub const SYS_ROOT_STREAM_POLL: u32 = 260;
pub const SYS_ROOT_PROP_SET: u32 = 261;
pub const SYS_ROOT_PROP_GET: u32 = 262;
pub const SYS_ROOT_FIND: u32 = 263;

pub const SYS_ROOT_DESCRIBE_THING: u32 = 264;
pub const SYS_ROOT_DESCRIBE_EDGE: u32 = 265;
pub const SYS_ROOT_DUMP_EDGES: u32 = 266;
pub const SYS_ROOT_LINK: u32 = 267;
pub const SYS_ROOT_INTERN: u32 = 268;
pub const SYS_ROOT_CREATE_NODE: u32 = 269;
pub const SYS_ROOT_QUERY: u32 = 270;
pub const SYS_ROOT_BYTESPACE_WRITE: u32 = 271;
pub const SYS_ROOT_DUMP_GRAPH: u32 = 272;
