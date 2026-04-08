//! ThingOS Syscall Constants
//!
//!
pub mod asm;
pub mod conv;

// ============================================================================
// Process & Thread Management (0x1000)
// ============================================================================
pub const SYS_EXIT: u32 = 0x1000;
pub const SYS_GET_TID: u32 = 0x1001;
pub const SYS_GETPID: u32 = 0x1002;
pub const SYS_GETPPID: u32 = 0x1003;
pub const SYS_SPAWN_THREAD: u32 = 0x1004;
pub const SYS_SPAWN_PROCESS: u32 = 0x1005;
/// Enhanced process spawn with argv, env, and stdio specification.
pub const SYS_SPAWN_PROCESS_EX: u32 = 0x1006;
pub const SYS_TASK_WAIT: u32 = 0x1007;
pub const SYS_TASK_KILL: u32 = 0x1008;
pub const SYS_TASK_DUMP: u32 = 0x1009;
pub const SYS_TASK_POLL: u32 = 0x100A;
pub const SYS_YIELD: u32 = 0x100B;
pub const SYS_SET_PRIORITY: u32 = 0x100C;

// ============================================================================
// Process Environment (0x1100)
// ============================================================================
pub const SYS_ARGV_GET: u32 = 0x1100;
pub const SYS_ENV_GET: u32 = 0x1101;
pub const SYS_ENV_SET: u32 = 0x1102;
pub const SYS_ENV_UNSET: u32 = 0x1103;
pub const SYS_ENV_LIST: u32 = 0x1104;

// ============================================================================
// Time & Waiting (0x1200)
// ============================================================================
pub const SYS_SLEEP_NS: u32 = 0x1200;
pub const SYS_SLEEP_MS: u32 = 0x1201;
pub const SYS_TIME_MONOTONIC: u32 = 0x1202;
pub const SYS_TIME_NOW: u32 = 0x1203;
pub const SYS_TIME_ANCHOR: u32 = 0x1204;
pub const SYS_WAIT_MANY: u32 = 0x1205;

// ============================================================================
// Synchronization (0x1300)
// ============================================================================
pub const SYS_FUTEX_WAIT: u32 = 0x1300;
pub const SYS_FUTEX_WAKE: u32 = 0x1301;

// ============================================================================
// Basic I/O & Console (0x1400)
// ============================================================================
pub const SYS_READ: u32 = 0x1400;
pub const SYS_WRITE: u32 = 0x1401;
pub const SYS_DEBUG_WRITE: u32 = 0x1402;
pub const SYS_LOG_WRITE: u32 = 0x1403;
pub const SYS_TRACE_READ: u32 = 0x1404;
/// Disable boot console (compositor takes over framebuffer)
pub const SYS_CONSOLE_DISABLE: u32 = 0x1405;

// ============================================================================
// Memory & Virtual Mapping (0x2000)
// ============================================================================
pub const SYS_ALLOC_STACK: u32 = 0x2000;
pub const SYS_VM_MAP: u32 = 0x2001;
pub const SYS_VM_UNMAP: u32 = 0x2002;
pub const SYS_VM_PROTECT: u32 = 0x2003;
pub const SYS_VM_ADVISE: u32 = 0x2004;
pub const SYS_VM_QUERY: u32 = 0x2005;
pub const SYS_MEMFD_CREATE: u32 = 0x2006;
pub const SYS_MEMFD_PHYS: u32 = 0x2007;

// ============================================================================
// IPC Ports & Streaming (0x3000)
// ============================================================================
pub const SYS_PORT_CREATE: u32 = 0x3000;
pub const SYS_PORT_SEND: u32 = 0x3001;
pub const SYS_PORT_RECV: u32 = 0x3002;
/// Non-blocking receive. Returns EAGAIN when no message is queued.
pub const SYS_PORT_TRY_RECV: u32 = 0x3003;
/// Send full buffer atomically (all-or-nothing). Returns EAGAIN if insufficient space.
pub const SYS_PORT_SEND_ALL: u32 = 0x3004;
/// Send a file descriptor alongside an empty or tiny payload. Args
pub const SYS_PORT_SEND_FD: u32 = 0x3005;
/// Receive a file descriptor. Args
pub const SYS_PORT_RECV_FD: u32 = 0x3006;
pub const SYS_PORT_INFO: u32 = 0x3007;
pub const SYS_PORT_CLOSE: u32 = 0x3008;
pub const SYS_PORT_WAIT: u32 = 0x3009;

pub const SYS_STREAM_OPEN: u32 = 0x300A;
pub const SYS_STREAM_LISTEN: u32 = 0x300B;
pub const SYS_STREAM_READ: u32 = 0x300C;
pub const SYS_STREAM_POLL: u32 = 0x300D;

/// Create a broadcast topic. Returns topic id.
pub const SYS_TOPIC_CREATE: u32 = 0x300E;
/// Subscribe a write port handle to a topic. Args
pub const SYS_TOPIC_SUBSCRIBE: u32 = 0x300F;
/// Publish a payload to all topic subscribers. Returns delivered subscriber count.
pub const SYS_TOPIC_PUBLISH: u32 = 0x3010;

/// Create an anonymous pipe. Args
pub const SYS_PIPE_CREATE: u32 = 0x3011;
/// Read from a pipe. Args
pub const SYS_PIPE_READ: u32 = 0x3012;
/// Write to a pipe. Args
pub const SYS_PIPE_WRITE: u32 = 0x3013;
/// Close one end of a pipe. Args
pub const SYS_PIPE_CLOSE: u32 = 0x3014;
/// Create an anonymous pipe, allocating two fds.
pub const SYS_PIPE: u32 = 0x3015;

// ============================================================================
// Virtual File System (VFS) (0x4000)
// ============================================================================
/// Open a file by path. Args
pub const SYS_FS_OPEN: u32 = 0x4000;
/// Close a file descriptor. Args
pub const SYS_FS_CLOSE: u32 = 0x4001;
/// Read from a file descriptor. Args
pub const SYS_FS_READ: u32 = 0x4002;
/// Write to a file descriptor. Args
pub const SYS_FS_WRITE: u32 = 0x4003;
/// Seek within a file descriptor.
pub const SYS_FS_SEEK: u32 = 0x4004;
/// Get file status. Args
pub const SYS_FS_STAT: u32 = 0x4005;
/// Read directory entries. Args
pub const SYS_FS_READDIR: u32 = 0x4006;
/// Create a directory by path. Args
pub const SYS_FS_MKDIR: u32 = 0x4007;
/// Remove a file or empty directory by path. Args
pub const SYS_FS_UNLINK: u32 = 0x4008;
/// Mount a userland VFS provider at a path prefix.
pub const SYS_FS_MOUNT: u32 = 0x4009;
/// Unmount the userland VFS provider at a path prefix.
pub const SYS_FS_UMOUNT: u32 = 0x400A;
/// Poll a set of file descriptors for readiness.
pub const SYS_FS_POLL: u32 = 0x400B;
/// Duplicate a file descriptor. Args
pub const SYS_FS_DUP: u32 = 0x400C;
/// Duplicate old_fd to new_fd, closing new_fd first if open.
pub const SYS_FS_DUP2: u32 = 0x400D;

// ============================================================================
// Hardware & Device Interfaces (0x5000)
// ============================================================================
pub const SYS_DEVICE_CLAIM: u32 = 0x5000;
pub const SYS_DEVICE_CALL: u32 = 0x5001;
pub const SYS_DEVICE_MAP_MMIO: u32 = 0x5002;
pub const SYS_DEVICE_ALLOC_DMA: u32 = 0x5003;
pub const SYS_DEVICE_DMA_PHYS: u32 = 0x5004;
pub const SYS_DEVICE_IOPORT_READ: u32 = 0x5005;
pub const SYS_DEVICE_IOPORT_WRITE: u32 = 0x5006;
pub const SYS_DEVICE_IRQ_SUBSCRIBE: u32 = 0x5007;
pub const SYS_DEVICE_IRQ_WAIT: u32 = 0x5008;

// ============================================================================
// Networking (0x6000)
// ============================================================================
pub const SYS_NIC_MAC: u32 = 0x6000;
pub const SYS_NIC_LINK_UP: u32 = 0x6001;
pub const SYS_NIC_POLL_RX: u32 = 0x6002;
pub const SYS_NIC_TX: u32 = 0x6003;

// ============================================================================
// System Control & Misc (0x7000)
// ============================================================================
pub const SYS_REBOOT: u32 = 0x7000;
/// Fill a user buffer with random bytes from the kernel entropy pool.
pub const SYS_GETRANDOM: u32 = 0x7001;
pub const SYS_CONSOLE_ENABLE: u32 = 0x7002;

// ============================================================================
// Unified Object Graph (0x8000)
// ============================================================================
pub const SYS_ROOT_GET_KIND: u32 = 0x8000;
pub const SYS_ROOT_PROP_GET: u32 = 0x8001;
pub const SYS_ROOT_PROP_SET: u32 = 0x8002;
pub const SYS_ROOT_LINK: u32 = 0x8003;
pub const SYS_ROOT_UNLINK: u32 = 0x8004;
pub const SYS_ROOT_INTERN: u32 = 0x8005;
pub const SYS_ROOT_CREATE_NODE: u32 = 0x8006;
pub const SYS_ROOT_FIND: u32 = 0x8007;
pub const SYS_ROOT_DESCRIBE: u32 = 0x8008;
pub const SYS_ROOT_DESCRIBE_SYMBOL: u32 = 0x8009;
pub const SYS_ROOT_RESOLVE_PATH: u32 = 0x800A;
pub const SYS_ROOT_PROPS_GET_MANY: u32 = 0x800B;
pub const SYS_ROOT_DIR_LIST: u32 = 0x800C;
pub const SYS_ROOT_WATCH_OPEN: u32 = 0x800D;
pub const SYS_ROOT_WATCH_NEXT: u32 = 0x800E;
pub const SYS_ROOT_WATCH_TRY_NEXT: u32 = 0x800F;
pub const SYS_ROOT_WATCH_CLOSE: u32 = 0x8010;
pub const SYS_ROOT_STREAM_POLL: u32 = 0x8011;
pub const SYS_ROOT_APPLY_BATCH: u32 = 0x8012;
pub const SYS_ROOT_ORPHAN_THING: u32 = 0x8013;
pub const SYS_ROOT_WATCH_SUBSCRIBE: u32 = 0x8014;

pub mod port_wait {
    pub const READABLE: u32 = 1 << 0;
    pub const WRITABLE: u32 = 1 << 1;
}

pub mod pipe_flags {
    /// Non-blocking mode: read/write return EAGAIN instead of blocking.
    pub const NONBLOCK: u32 = 1 << 0;
    /// Close-on-exec (stored but no-op until process exec support).
    pub const CLOEXEC: u32 = 1 << 1;
}

pub mod vfs_flags {
    /// Open for reading.
    pub const O_RDONLY: u32 = 0x0000;
    /// Open for writing.
    pub const O_WRONLY: u32 = 0x0001;
    /// Open for reading and writing.
    pub const O_RDWR: u32 = 0x0002;
    /// Create file if it does not exist.
    pub const O_CREAT: u32 = 0x0040;
    /// Truncate file to zero length on open.
    pub const O_TRUNC: u32 = 0x0200;
    /// Append to file on every write.
    pub const O_APPEND: u32 = 0x0400;
    /// Non-blocking I/O.
    pub const O_NONBLOCK: u32 = 0x0800;
}

/// Events/flags used by [`SYS_FS_POLL`].
pub mod poll_flags {
    /// Data available to read.
    pub const POLLIN: u16 = 0x0001;
    /// Ready to accept writes.
    pub const POLLOUT: u16 = 0x0004;
    /// An error condition has occurred.
    pub const POLLERR: u16 = 0x0008;
    /// The file descriptor was closed on the other end (hangup).
    pub const POLLHUP: u16 = 0x0010;
    /// fd is not open.
    pub const POLLNVAL: u16 = 0x0020;
}

/// Entry in the `pollfds` array passed to [`SYS_FS_POLL`].
///
/// Layout mirrors POSIX `struct pollfd` so that future libc ports can
/// alias this directly.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct PollFd {
    /// File descriptor to watch.
    pub fd: i32,
    /// Events to wait for (input, using [`poll_flags`]).
    pub events: u16,
    /// Events that occurred (output, filled by the kernel).
    pub revents: u16,
}
