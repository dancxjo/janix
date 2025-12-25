use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeNow {
    pub monotonic_ns: u64,
    pub system_ns: u64,
}
