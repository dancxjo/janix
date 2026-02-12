//! ThingOS Syscall Constants
//!
//!
pub mod asm;
pub mod conv;

// ============================================================================
// Block 0x00–0x0F: Process lifecycle + identity
// ============================================================================
pub const SYS_EXIT: u32 = 0x01;
pub const SYS_GET_TID: u32 = 0x02;
pub const SYS_SPAWN_THREAD: u32 = 0x03;
pub const SYS_SPAWN_PROCESS: u32 = 0x04;
pub const SYS_TASK_WAIT: u32 = 0x05;
pub const SYS_REBOOT: u32 = 0x06;
pub const SYS_TASK_KILL: u32 = 0x07;
pub const SYS_GETPID: u32 = 0x08;
pub const SYS_GETPPID: u32 = 0x09;
pub const SYS_ARGV_GET: u32 = 0x0A;
pub const SYS_ENV_GET: u32 = 0x0B;
pub const SYS_ENV_SET: u32 = 0x0C;
pub const SYS_ENV_UNSET: u32 = 0x0D;
pub const SYS_ENV_LIST: u32 = 0x0E;
/// Enhanced process spawn with argv, env, and stdio specification.
/// Args: req_ptr (SpawnProcessExReq), resp_ptr (SpawnProcessExResp).
pub const SYS_SPAWN_PROCESS_EX: u32 = 0x0F;

// ============================================================================
// Block 0x10–0x1F: Scheduling + sleep/yield
// ============================================================================
pub const SYS_YIELD: u32 = 0x10;
pub const SYS_SLEEP_NS: u32 = 0x11;
pub const SYS_SLEEP_MS: u32 = 0x12;
pub const SYS_TASK_POLL: u32 = 0x13;
pub const SYS_SET_PRIORITY: u32 = 0x14;
pub const SYS_TASK_DUMP: u32 = 0x15;
pub const SYS_FUTEX_WAIT: u32 = 0x16;
pub const SYS_FUTEX_WAKE: u32 = 0x17;

// ============================================================================
// Block 0x20–0x2F: Time
// ============================================================================
pub const SYS_TIME_MONOTONIC: u32 = 0x20;
pub const SYS_TIME_NOW: u32 = 0x21;
pub const SYS_TIME_ANCHOR: u32 = 0x22;

// ============================================================================
// Block 0x30–0x3F: Logging / debug output
// ============================================================================
pub const SYS_DEBUG_WRITE: u32 = 0x30;
pub const SYS_LOG_WRITE: u32 = 0x31;
pub const SYS_TRACE_READ: u32 = 0x32;
/// Disable boot console (compositor takes over framebuffer)
pub const SYS_CONSOLE_DISABLE: u32 = 0x33;

// ============================================================================
// Block 0x40–0x4F: Memory / stacks
// ============================================================================
pub const SYS_ALLOC_STACK: u32 = 0x40;
pub const SYS_VM_MAP: u32 = 0x41;
pub const SYS_VM_UNMAP: u32 = 0x42;
pub const SYS_VM_PROTECT: u32 = 0x43;
pub const SYS_VM_ADVISE: u32 = 0x44;
pub const SYS_VM_QUERY: u32 = 0x45;

// ============================================================================
// Block 0x80–0x8F: Port IPC
// ============================================================================
pub const SYS_PORT_CREATE: u32 = 0x80;
pub const SYS_PORT_SEND: u32 = 0x81;
pub const SYS_PORT_RECV: u32 = 0x82;
pub const SYS_PORT_CLOSE: u32 = 0x83;
pub const SYS_PORT_WAIT: u32 = 0x84;
/// Send full buffer atomically (all-or-nothing). Returns EAGAIN if insufficient space.
pub const SYS_PORT_SEND_ALL: u32 = 0x85;
pub const SYS_PORT_INFO: u32 = 0x86;
/// Create a broadcast topic. Returns topic id.
pub const SYS_TOPIC_CREATE: u32 = 0x87;
/// Subscribe a write port handle to a topic. Args: topic_id, write_port_handle.
pub const SYS_TOPIC_SUBSCRIBE: u32 = 0x88;
/// Publish a payload to all topic subscribers. Returns delivered subscriber count.
pub const SYS_TOPIC_PUBLISH: u32 = 0x89;

pub mod port_wait {
    pub const READABLE: u32 = 1 << 0;
    pub const WRITABLE: u32 = 1 << 1;
}

// ============================================================================
// Block 0x90–0x9F: Streams
// ============================================================================
pub const SYS_STREAM_OPEN: u32 = 0x90;
pub const SYS_STREAM_LISTEN: u32 = 0x93; // Added
pub const SYS_STREAM_READ: u32 = 0x91;
pub const SYS_STREAM_POLL: u32 = 0x92;

// ============================================================================
// Block 0xC0–0xCF: Device claim + generic device call
// ============================================================================
pub const SYS_DEVICE_CLAIM: u32 = 0xC0;
pub const SYS_DEVICE_CALL: u32 = 0xC1;

// ============================================================================
// Block 0xD0–0xDF: Device resources (MMIO/IRQ/IOPORT)
// ============================================================================
pub const SYS_DEVICE_MAP_MMIO: u32 = 0xD0;
pub const SYS_DEVICE_IRQ_SUBSCRIBE: u32 = 0xD1;
pub const SYS_DEVICE_IOPORT_READ: u32 = 0xD2;
pub const SYS_DEVICE_IOPORT_WRITE: u32 = 0xD3;
pub const SYS_DEVICE_ALLOC_DMA: u32 = 0xD4;
pub const SYS_DEVICE_DMA_PHYS: u32 = 0xD5;
pub const SYS_DEVICE_IRQ_WAIT: u32 = 0xD6;

// ============================================================================
// Block 0x100–0x10F: Watches/events
// ============================================================================
pub const SYS_WATCH_SUBSCRIBE: u32 = 0x100;
pub const SYS_WATCH_READ: u32 = 0x101;

// ============================================================================
// Block 0x140–0x1FF: Root/Graph
// ============================================================================
pub const SYS_ROOT_GET_KIND: u32 = 0x140;
pub const SYS_ROOT_INTERN: u32 = 0x141;
pub const SYS_ROOT_CREATE_NODE: u32 = 0x142;
pub const SYS_ROOT_LINK: u32 = 0x143;
pub const SYS_ROOT_APPLY_BATCH: u32 = 0x144;

// 0x148-0x14F: Properties
pub const SYS_ROOT_PROP_GET: u32 = 0x148;
pub const SYS_ROOT_PROP_SET: u32 = 0x149;
/// Bulk property fetch: get multiple properties for a single node in one syscall
/// Args: node_id, keys_ptr, keys_len, out_ptr, out_len
/// Returns: number of properties found
pub const SYS_ROOT_PROPS_GET_MANY: u32 = 0x14A;

// 0x150-0x157: Query/Find
pub const SYS_ROOT_FIND: u32 = 0x150;
pub const SYS_ROOT_QUERY: u32 = 0x151;

// 0x158-0x15F: Debug/Describe
pub const SYS_ROOT_DESCRIBE_THING: u32 = 0x158;
pub const SYS_ROOT_DESCRIBE_EDGE: u32 = 0x159;
pub const SYS_ROOT_DESCRIBE_SYMBOL: u32 = 0x15D; // Added
pub const SYS_ROOT_DUMP_EDGES: u32 = 0x15A;
pub const SYS_ROOT_DUMP_GRAPH: u32 = 0x15B;
pub const SYS_ROOT_GET_EDGES: u32 = 0x15C;
pub const SYS_ROOT_GET_PROPS: u32 = 0x15E;

// 0x160-0x167: Bytespace
pub const SYS_ROOT_BYTESPACE_CREATE: u32 = 0x160;
pub const SYS_ROOT_BYTESPACE_READ: u32 = 0x161;
pub const SYS_ROOT_BYTESPACE_WRITE: u32 = 0x162;
pub const SYS_ROOT_BYTESPACE_INFO: u32 = 0x163;
pub const SYS_ROOT_BYTESPACE_MAP: u32 = 0x164;
pub const SYS_ROOT_BYTESPACE_UNMAP: u32 = 0x165;
pub const SYS_ROOT_BYTESPACE_PHYS: u32 = 0x166;
pub const SYS_ROOT_BYTESPACE_TRUNCATE: u32 = 0x167;

// 0x168-0x16F: Watch/Stream (Root-specific)
pub const SYS_ROOT_WATCH_SUBSCRIBE: u32 = 0x168;
pub const SYS_ROOT_STREAM_POLL: u32 = 0x169;

pub const SYS_ROOT_WATCH_OPEN: u32 = 0x16A;
pub const SYS_ROOT_WATCH_NEXT: u32 = 0x16B;
pub const SYS_ROOT_WATCH_CLOSE: u32 = 0x16C;

// ============================================================================
// Block 0x170–0x17F: Filesystem (path resolution)
// ============================================================================
pub const SYS_ROOT_RESOLVE_PATH: u32 = 0x170;
pub const SYS_ROOT_UNLINK: u32 = 0x171;
pub const SYS_ROOT_DIR_LIST: u32 = 0x172;

// ============================================================================
// Block 0x1A0–0x1AF: Network
// ============================================================================
pub const SYS_NIC_MAC: u32 = 0x1A0;
pub const SYS_NIC_LINK_UP: u32 = 0x1A1;
pub const SYS_NIC_POLL_RX: u32 = 0x1A2;
pub const SYS_NIC_TX: u32 = 0x1A3;

// ============================================================================
// Block 0x200–0x20F: Pipes
// ============================================================================
/// Create an anonymous pipe. Args: capacity, flags. Returns pipe_id.
/// Read handle = pipe_id, write handle = pipe_id (distinguished by syscall used).
pub const SYS_PIPE_CREATE: u32 = 0x200;
/// Read from a pipe. Args: pipe_id, buf_ptr, buf_len. Returns bytes read.
pub const SYS_PIPE_READ: u32 = 0x201;
/// Write to a pipe. Args: pipe_id, buf_ptr, buf_len. Returns bytes written.
pub const SYS_PIPE_WRITE: u32 = 0x202;
/// Close one end of a pipe. Args: pipe_id, end (0=read, 1=write).
pub const SYS_PIPE_CLOSE: u32 = 0x203;

pub mod pipe_flags {
    /// Non-blocking mode: read/write return EAGAIN instead of blocking.
    pub const NONBLOCK: u32 = 1 << 0;
    /// Close-on-exec (stored but no-op until process exec support).
    pub const CLOEXEC: u32 = 1 << 1;
}

// ============================================================================
// Block 0x210–0x21F: Entropy
// ============================================================================
/// Fill a user buffer with random bytes from the kernel entropy pool.
/// Args: buf_ptr, buf_len. Returns 0 on success.
pub const SYS_GETRANDOM: u32 = 0x210;
