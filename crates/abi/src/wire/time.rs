use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TimeNowReq {}

#[derive(Serialize, Deserialize)]
pub struct TimeNowResp {
    pub system_ns: u64,
    pub monotonic_ns: u64,
}
