// System Call Numbers

// Core Graph
pub const SYSCALL_GRAPH: usize = 1;

// Driver Surface
pub const SYSCALL_DRIVER_WAIT: usize = 100;
pub const SYSCALL_DRIVER_PUBLISH: usize = 101;

// Typedefs
pub const SYSCALL_TYPEDEF_REGISTER: usize = 200;
pub const SYSCALL_TYPEDEF_GET: usize = 201;

pub const SYSCALL_YIELD: usize = 2;
pub const SYS_EAGAIN: isize = -11;
