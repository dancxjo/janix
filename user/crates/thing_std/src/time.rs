use crate::graph::{GraphClient, GraphError};
pub use abi::wire::time::*;
use abi::SysRet;

// Helper to convert GraphError to SysRet
fn map_err(e: GraphError) -> SysRet {
    match e {
        GraphError::Sys(ret) => ret,
        _ => -1,
    }
}

pub fn time_now(g: &GraphClient) -> Result<TimeNowResp, SysRet> {
    let mut out = [0u8; 64];
    g.call("time.now", &TimeNowReq {}, &mut out).map_err(map_err)
}

pub fn monotonic_ns(g: &GraphClient) -> Result<u64, SysRet> {
    let mut out = [0u8; 64];
    let resp: TimeMonotonicResp = g.call("time.monotonic_ns", &TimeMonotonicReq {}, &mut out).map_err(map_err)?;
    Ok(resp.monotonic_ns)
}

pub fn sleep_until_ns(g: &GraphClient, wake_ns: u64) -> Result<u64, SysRet> {
    let mut out = [0u8; 64];
    loop {
        match g.call("time.sleep_until_ns", &TimeSleepUntilReq { wake_monotonic_ns: wake_ns }, &mut out) {
            Ok(resp) => {
                let r: TimeSleepUntilResp = resp;
                return Ok(r.woke_at_monotonic_ns);
            },
            Err(GraphError::Sys(e)) if e == -11 => { // EAGAIN
                crate::sys::sys_yield();
                continue;
            },
            Err(e) => return Err(map_err(e)),
        }
    }
}

pub fn sleep_ns(g: &GraphClient, duration_ns: u64) -> Result<u64, SysRet> {
    let mut out = [0u8; 64];
    loop {
        match g.call("time.sleep_ns", &TimeSleepNsReq { duration_ns }, &mut out) {
            Ok(resp) => {
                let r: TimeSleepNsResp = resp;
                return Ok(r.woke_at_monotonic_ns);
            },
            Err(GraphError::Sys(e)) if e == -11 => { // EAGAIN
                crate::sys::sys_yield();
                continue;
            },
            Err(e) => return Err(map_err(e)),
        }
    }
}

pub fn sleep_ms(g: &GraphClient, ms: u64) -> Result<u64, SysRet> {
    sleep_ns(g, ms.saturating_mul(1_000_000))
}

pub fn sleep_s(g: &GraphClient, s: u64) -> Result<u64, SysRet> {
    sleep_ns(g, s.saturating_mul(1_000_000_000))
}
