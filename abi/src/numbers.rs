// ThingOS Syscall Numbers
//
// One single source of truth for syscall numbers.
// This file is explicitly free of any dependencies (like serde) so it can
// be shared between the kernel, the ABI, and the Platform Abstraction Layer (PAL).

// ============================================================================
// Process & Thread Management (0x1000)
// ============================================================================
pub const SYS_EXIT: u32 = 0x1000;
pub const SYS_GET_TID: u32 = 0x1001;
pub const SYS_GETPID: u32 = 0x1002;
pub const SYS_GETPPID: u32 = 0x1003;
pub const SYS_SPAWN_THREAD: u32 = 0x1004;
pub const SYS_SPAWN_PROCESS: u32 = 0x1005;
pub const SYS_SPAWN_PROCESS_EX: u32 = 0x1006;
pub const SYS_TASK_WAIT: u32 = 0x1007;
pub const SYS_TASK_KILL: u32 = 0x1008;
pub const SYS_TASK_DUMP: u32 = 0x1009;
pub const SYS_TASK_POLL: u32 = 0x100A;
pub const SYS_YIELD: u32 = 0x100B;
pub const SYS_SET_PRIORITY: u32 = 0x100C;
pub const SYS_TASK_EXEC: u32 = 0x100D;
pub const SYS_TASK_SET_TLS_BASE: u32 = 0x100E;
pub const SYS_TASK_GET_TLS_BASE: u32 = 0x100F;
pub const SYS_TASK_INTERRUPT: u32 = 0x1010;

// ============================================================================
// Process Environment (0x1100)
// ============================================================================
pub const SYS_ARGV_GET: u32 = 0x1100;
pub const SYS_ENV_GET: u32 = 0x1101;
pub const SYS_ENV_SET: u32 = 0x1102;
pub const SYS_ENV_UNSET: u32 = 0x1103;
pub const SYS_ENV_LIST: u32 = 0x1104;
pub const SYS_AUXV_GET: u32 = 0x1105;

// ============================================================================
// Time & Waiting (0x1200)
// ============================================================================
pub const SYS_SLEEP: u32 = 0x1200;
pub const SYS_SLEEP_NS: u32 = SYS_SLEEP;
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
// IPC (0x3000)
// ============================================================================
pub const SYS_PORT_CREATE: u32 = 0x3000;
pub const SYS_CHANNEL_CREATE: u32 = 0x3000;
pub const SYS_CHANNEL_SEND: u32 = 0x3001;
pub const SYS_CHANNEL_RECV: u32 = 0x3002;
pub const SYS_CHANNEL_TRY_RECV: u32 = 0x3003;
pub const SYS_CHANNEL_SEND_ALL: u32 = 0x3004;
pub const SYS_CHANNEL_SEND_HANDLE: u32 = 0x3005;
pub const SYS_CHANNEL_RECV_HANDLE: u32 = 0x3006;
pub const SYS_CHANNEL_INFO: u32 = 0x3007;
pub const SYS_CHANNEL_CLOSE: u32 = 0x3008;
pub const SYS_CHANNEL_WAIT: u32 = 0x3009;
pub const SYS_PIPE: u32 = 0x3015;

// ============================================================================
// Virtual File System (VFS) (0x4000)
// ============================================================================
pub const SYS_FS_OPEN: u32 = 0x4000;
pub const SYS_FS_CLOSE: u32 = 0x4001;
pub const SYS_FS_READ: u32 = 0x4002;
pub const SYS_FS_WRITE: u32 = 0x4003;
pub const SYS_FS_SEEK: u32 = 0x4004;
pub const SYS_FS_STAT: u32 = 0x4005;
pub const SYS_FS_READDIR: u32 = 0x4006;
pub const SYS_FS_MKDIR: u32 = 0x4007;
pub const SYS_FS_UNLINK: u32 = 0x4008;
pub const SYS_FS_MOUNT: u32 = 0x4009;
pub const SYS_FS_UMOUNT: u32 = 0x400A;
pub const SYS_FS_POLL: u32 = 0x400B;
pub const SYS_FS_DUP: u32 = 0x400C;
pub const SYS_FS_DUP2: u32 = 0x400D;
pub const SYS_FS_WATCH_FD: u32 = 0x400E;
pub const SYS_FS_WATCH_PATH: u32 = 0x400F;
pub const SYS_FS_RENAME: u32 = 0x4010;
pub const SYS_FS_DEVICE_CALL: u32 = 0x4011;
pub const SYS_FS_CHDIR: u32 = 0x4012;
pub const SYS_FS_GETCWD: u32 = 0x4013;
pub const SYS_FD_FROM_HANDLE: u32 = 0x4014;
pub const SYS_FS_NOTIFY: u32 = 0x4015;
pub const SYS_FS_ISATTY: u32 = 0x4020;
pub const SYS_FS_REALPATH: u32 = 0x4016;
pub const SYS_FS_SYNC: u32 = 0x4017;
pub const SYS_FS_FCNTL: u32 = 0x4018;

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
// System Control & Misc (0x7000)
// ============================================================================
pub const SYS_REBOOT: u32 = 0x7000;
pub const SYS_GETRANDOM: u32 = 0x7001;
pub const SYS_CONSOLE_ENABLE: u32 = 0x7002;
pub const SYS_LOG_SET_LEVEL: u32 = 0x7003;

// ============================================================================
// ABI Flags & Constants
// ============================================================================

pub mod channel_wait {
    pub const READABLE: u32 = 1 << 0;
    pub const WRITABLE: u32 = 1 << 1;
}

pub mod pipe_flags {
    pub const NONBLOCK: u32 = 1 << 0;
    pub const CLOEXEC: u32 = 1 << 1;
}

pub mod vfs_flags {
    pub const O_RDONLY: u32 = 0x0000;
    pub const O_WRONLY: u32 = 0x0001;
    pub const O_RDWR: u32 = 0x0002;
    pub const O_CREAT: u32 = 0x0040;
    pub const O_TRUNC: u32 = 0x0200;
    pub const O_APPEND: u32 = 0x0400;
    pub const O_NONBLOCK: u32 = 0x0800;
}

pub mod fcntl_cmd {
    pub const F_GETFD: u32 = 1;
    pub const F_SETFD: u32 = 2;
    pub const F_GETFL: u32 = 3;
    pub const F_SETFL: u32 = 4;
}

pub mod fd_flags {
    pub const FD_CLOEXEC: u32 = 0x1;
}

pub mod poll_flags {
    pub const POLLIN: u16 = 0x0001;
    pub const POLLOUT: u16 = 0x0004;
    pub const POLLERR: u16 = 0x0008;
    pub const POLLHUP: u16 = 0x0010;
    pub const POLLNVAL: u16 = 0x0020;
}
