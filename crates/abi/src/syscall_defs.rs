// System Call Numbers

// Core Graph
pub const SYSCALL_GRAPH: usize = 1;

// Driver Surface
pub const SYSCALL_DRIVER_WAIT: usize = 100;
pub const SYSCALL_DRIVER_PUBLISH: usize = 101;
pub const SYSCALL_MMIO_MAP: usize = 102;
pub const SYSCALL_DMA_ALLOC: usize = 103;
pub const SYSCALL_IRQ_REGISTER: usize = 104;
pub const SYSCALL_PORT_IO: usize = 105;

// Typedefs
pub const SYSCALL_TYPEDEF_REGISTER: usize = 200;
pub const SYSCALL_TYPEDEF_GET: usize = 201;

pub const SYSCALL_BYTESPACE_CREATE: usize = 301;
pub const SYSCALL_BYTESPACE_MAP: usize = 302;
pub const SYSCALL_BYTESPACE_READ: usize = 303;
pub const SYSCALL_BYTESPACE_WRITE: usize = 304;

pub const SYSCALL_YIELD: usize = 2;
pub const SYS_EAGAIN: isize = -11;

pub const SYSCALL_LOG: usize = 10;
pub const SYSCALL_TIME: usize = 11;
pub const SYSCALL_SLEEP: usize = 12;

// Time
pub const SYSCALL_RTC_READ: usize = 300;

// Process
pub const SYSCALL_SPAWN: usize = 20;
