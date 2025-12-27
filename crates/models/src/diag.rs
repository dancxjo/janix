use abi::SymbolId;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Access {
    Read = 0,
    Write = 1,
    Exec = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddressSpace {
    Kernel = 0,
    User = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KillAction {
    Panic = 0,
    KillThread = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaultKind {
    PageFault = 0,
    Gpf = 1,
    DoubleFault = 2,
    InvalidOp = 3,
    DivideByZero = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogEntryBody {
    pub timestamp_ns: u64,
    pub level: LogLevel,
    pub message: String,
    pub subsystem: SymbolId,
    pub cpu_id: u16,
    pub thread_id: u64,
    pub process_id: u64,
    pub seq: u64,
}

impl LogEntryBody {
    pub fn new(level: LogLevel, subsystem: SymbolId, message: impl Into<String>) -> Self {
        Self {
            timestamp_ns: 0,
            level,
            message: message.into(),
            subsystem,
            cpu_id: 0,
            thread_id: 0,
            process_id: 0,
            seq: 0,
        }
    }

    pub const fn ts(mut self, timestamp_ns: u64) -> Self {
        self.timestamp_ns = timestamp_ns;
        self
    }

    pub const fn cpu(mut self, cpu_id: u16) -> Self {
        self.cpu_id = cpu_id;
        self
    }

    pub const fn thread(mut self, thread_id: u64) -> Self {
        self.thread_id = thread_id;
        self
    }

    pub const fn process(mut self, process_id: u64) -> Self {
        self.process_id = process_id;
        self
    }

    pub const fn seq(mut self, seq: u64) -> Self {
        self.seq = seq;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: SymbolId,
    pub message: String,
    pub severity: u8,
    pub recoverable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultBody {
    pub fault_kind: FaultKind,
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub cr2: u64,
    pub error_code: u64,
    pub access: Access,
    pub address_space: AddressSpace,
    pub kill_action: KillAction,
}
