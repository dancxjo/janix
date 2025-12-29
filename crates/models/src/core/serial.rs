use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SerialPortBody {
    pub port_base: u16,
    pub irq: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogEntryCompact {
    pub seq: u64,
    pub timestamp_ns: u64, // Added timestamp
    pub level: u8,
    pub message: alloc::string::String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogStreamBody {
    pub head_seq: u64,
    pub capacity: u32,
    pub dropped: u64,
    pub entries: alloc::vec::Vec<LogEntryCompact>,
}
