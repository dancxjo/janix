use crate::client::GraphClient;
pub use abi::wire::time::*;
use abi::SysRet;

pub fn time_now(g: &GraphClient) -> Result<TimeNowResp, SysRet> {
    let req = TimeNowReq {};
    let mut buf = [0u8; 64];
    g.query("time.now", &req, &mut buf).map_err(|_| -1)
}

pub fn monotonic_ns(g: &GraphClient) -> Result<u64, SysRet> {
    let req = TimeMonotonicReq {};
    let mut buf = [0u8; 64];
    let resp: TimeMonotonicResp = g.query("time.monotonic_ns", &req, &mut buf).map_err(|_| -1)?;
    Ok(resp.monotonic_ns)
}

pub fn sleep_until_ns(g: &GraphClient, wake_ns: u64) -> Result<u64, SysRet> {
    let req = TimeSleepUntilReq { wake_monotonic_ns: wake_ns };
    let mut buf = [0u8; 64];
    let resp: TimeSleepUntilResp = g.query("time.sleep_until_ns", &req, &mut buf).map_err(|_| -1)?;
    Ok(resp.woke_at_monotonic_ns)
}

pub fn sleep_ns(g: &GraphClient, duration_ns: u64) -> Result<u64, SysRet> {
    let req = TimeSleepNsReq { duration_ns };
    let mut buf = [0u8; 64];
    let resp: TimeSleepNsResp = g.query("time.sleep_ns", &req, &mut buf).map_err(|_| -1)?;
    Ok(resp.woke_at_monotonic_ns)
}

pub fn sleep_ms(g: &GraphClient, ms: u64) -> Result<u64, SysRet> {
    sleep_ns(g, ms.saturating_mul(1_000_000))
}

pub fn sleep_s(g: &GraphClient, s: u64) -> Result<u64, SysRet> {
    sleep_ns(g, s.saturating_mul(1_000_000_000))
}
