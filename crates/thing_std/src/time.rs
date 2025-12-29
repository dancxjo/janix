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
    let resp: TimeMonotonicResp = g
        .query("time.monotonic_ns", &req, &mut buf)
        .map_err(|_| -1)?;
    Ok(resp.monotonic_ns)
}

pub fn sleep_until_ns(g: &GraphClient, wake_ns: u64) -> Result<u64, SysRet> {
    let req = TimeSleepUntilReq {
        wake_monotonic_ns: wake_ns,
    };
    let mut buf = [0u8; 64];
    let resp: TimeSleepUntilResp = g
        .query("time.sleep_until_ns", &req, &mut buf)
        .map_err(|_| -1)?;
    Ok(resp.woke_at_monotonic_ns)
}

pub fn sleep_ns(g: &GraphClient, duration_ns: u64) -> Result<u64, SysRet> {
    // We cannot use relative sleep syscall directly because GraphClient retries on EAGAIN with the same payload.
    // Relative sleep would reset the timer on every retry, leading to infinite wait.
    // We must convert to absolute time and use sleep_until.
    let now = monotonic_ns(g)?;
    let target = now.saturating_add(duration_ns);
    sleep_until_ns(g, target)
}

pub fn sleep_ms(g: &GraphClient, ms: u64) -> Result<u64, SysRet> {
    sleep_ns(g, ms.saturating_mul(1_000_000))
}

pub fn sleep_s(g: &GraphClient, s: u64) -> Result<u64, SysRet> {
    sleep_ns(g, s.saturating_mul(1_000_000_000))
}
