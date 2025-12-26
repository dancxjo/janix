use abi::SymbolId;
use serde::{Deserialize, Serialize};
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogEntryBody {
    pub timestamp_ns: u64,
    pub level: u8, // 0=Trace, 1=Debug, 2=Info, 3=Warn, 4=Error, 5=Fatal
    pub message: String,
    pub subsystem: SymbolId,
    pub cpu_id: u16,
    pub thread_id: u64,
    pub process_id: u64,
    pub seq: u64,
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
    pub fault_kind: u8, // 0=PageFault, 1=GPF, etc.
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub cr2: u64,
    pub error_code: u64,
    pub access: u8, // 0=Read, 1=Write, 2=Exec
    pub address_space: u8, // 0=Kernel, 1=User
    pub kill_action: u8, // 0=Panic, 1=KillThread
}
