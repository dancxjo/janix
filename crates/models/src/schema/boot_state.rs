use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BootStateBody {
    pub phase: alloc::string::String,
    pub step: u32,
    pub message: alloc::string::String,
    // pub timestamp_ns: u64, // Not yet available easily in no_std without time service? Actually loaded can use time svc. 
    // Omitting timestamp for now to keep it simple, or add it later.
    pub level: u8,
}
