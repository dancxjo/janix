//! Unified wait-many ABI.

/// Maximum number of wait specs/results accepted by the kernel in one call.
pub const WAIT_MANY_MAX_ITEMS: usize = 32;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitKind {
    Port = 1,
    RootWatch = 2,
    TaskExit = 3,
    Irq = 4,
    Timeout = 5,
    GraphOp = 6,
}

impl WaitKind {
    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            1 => Some(Self::Port),
            2 => Some(Self::RootWatch),
            3 => Some(Self::TaskExit),
            4 => Some(Self::Irq),
            5 => Some(Self::Timeout),
            6 => Some(Self::GraphOp),
            _ => None,
        }
    }
}

pub mod interest {
    pub const READABLE: u32 = 1 << 0;
    pub const WRITABLE: u32 = 1 << 1;
}

pub mod ready {
    pub const READABLE: u32 = 1 << 0;
    pub const WRITABLE: u32 = 1 << 1;
    pub const HANGUP: u32 = 1 << 2;
    pub const EXITED: u32 = 1 << 3;
    pub const TIMEOUT: u32 = 1 << 4;
    pub const OVERFLOW: u32 = 1 << 5;
    pub const ERROR: u32 = 1 << 6;
    pub const IRQ: u32 = 1 << 7;
    pub const DONE: u32 = 1 << 8;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct WaitSpec {
    pub kind: u32,
    pub flags: u32,
    pub object: u64,
    pub token: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct WaitResult {
    pub kind: u32,
    pub flags: u32,
    pub object: u64,
    pub token: u64,
    /// Optional payload:
    /// - port/watch/irq: bytes/pending count when known
    /// - task exit: exit code
    /// - error: Errno value
    pub value: i64,
    pub reserved: u64,
}
