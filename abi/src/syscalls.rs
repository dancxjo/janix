

// Time & Scheduling
pub const SYSCALL_YIELD: u64 = 0;
pub const SYSCALL_SLEEP_FOR_NS: u64 = 1;
pub const SYSCALL_SLEEP_UNTIL: u64 = 2;
pub const SYSCALL_TIME_MONOTONIC_NS: u64 = 3;
pub const SYSCALL_TIME_SYSTEM_NS: u64 = 4;
pub const SYSCALL_TIME_NOW: u64 = 5;

// Diagnostics
pub const SYSCALL_LOG: u64 = 6;

// Thread & Process
pub const SYSCALL_EXIT_THREAD: u64 = 7;
pub const SYSCALL_CREATE_PROCESS: u64 = 10;
pub const SYSCALL_CREATE_THREAD: u64 = 11;
pub const SYSCALL_SPAWN_PROGRAM: u64 = 12;

// Memory
pub const SYSCALL_ALLOC_FRAME: u64 = 8;
pub const SYSCALL_FREE_FRAME: u64 = 9;

// Graph Operations
pub const SYSCALL_THING_CREATE: u64 = 13;
pub const SYSCALL_THING_GET: u64 = 14;
pub const SYSCALL_THING_UPDATE: u64 = 15;
pub const SYSCALL_THING_LIST: u64 = 16;
pub const SYSCALL_ADD_LINK: u64 = 17;
pub const SYSCALL_LINK_AT: u64 = 18;
pub const SYSCALL_GRAPH_QUERY: u64 = 20;
pub const SYSCALL_CREATE_TRANSACTION: u64 = 21;
pub const SYSCALL_COMMIT_TRANSACTION: u64 = 22;

// Schema
pub const SYSCALL_SCHEMA_REGISTER_PACKAGE: u64 = 19;
pub const SYSCALL_SCHEMA_GET: u64 = 33;

// Shared Buffer
pub const SYSCALL_MAP_SHARED_BUFFER: u64 = 23;
pub const SYSCALL_CREATE_SHARED_BUFFER: u64 = 24;
pub const SYSCALL_GET_SHARED_BUFFER_INFO: u64 = 25;

// Resident Memory
pub const SYSCALL_RESIDENT_ALLOC: u64 = 26;
pub const SYSCALL_RESIDENT_MAP: u64 = 27;
pub const SYSCALL_RESIDENT_UNMAP: u64 = 28;
pub const SYSCALL_THING_REST: u64 = 29;

// Symbols (New)
pub const SYSCALL_SYMBOL_INTERN: u64 = 30;
pub const SYSCALL_SYMBOL_RESOLVE: u64 = 31;
pub const SYSCALL_THING_BATCH_UPDATE: u64 = 32;

// Devices
pub const SYSCALL_DEV_OPEN: u64 = 64; // 0x40
pub const SYSCALL_DEV_READ: u64 = 65; // 0x41
