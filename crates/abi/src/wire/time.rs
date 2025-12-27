use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RtcSample {
    pub year: u16,
    pub mon: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeNowReq {}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeNowResp {
    pub system_ns: u64,
    pub monotonic_ns: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeMonotonicReq {}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeMonotonicResp {
    pub monotonic_ns: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSleepNsReq {
    pub duration_ns: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSleepNsResp {
    pub woke_at_monotonic_ns: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSleepUntilReq {
    pub wake_monotonic_ns: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSleepUntilResp {
    pub woke_at_monotonic_ns: u64,
}
