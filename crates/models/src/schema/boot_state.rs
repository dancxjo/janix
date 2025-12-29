use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BootStateBody {
    pub phase: alloc::string::String,
    pub step: u32,
    pub message: alloc::string::String,
    #[serde(default)]
    pub timestamp_ns: u64,
    #[serde(default)]
    pub level: u8,
}
