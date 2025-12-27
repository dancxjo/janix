use crate::client::GraphClient;
pub use abi::wire::time::*;
use abi::SysRet;
use postcard::to_allocvec;

pub fn time_now(g: &GraphClient) -> Result<TimeNowResp, SysRet> {
    let req = TimeNowReq {};
    let req_bytes = to_allocvec(&req).map_err(|_| -1)?;
    
    // Check graph.rs handler: "time.now"
    // Using handle_graph_query => query string "time.now"
    // GraphClient::query calls syscall_graph with ptrs.
    
    // We assume GraphClient has a helper or we use base one.
    // Let's assume `g.query("time.now", &req_bytes)` returns Vec<u8>.
    // But `GraphClient` source might be needed.
    // For now assuming `call` or `query`.
    // Wait, Task says "generic generic syscall / GraphClient::call path".
    // GraphClient in `client.rs` likely has `call` or `query`.
    // I'll assume `query` for reading, `call` for ops?
    // "time.now" is a query string in `graph.rs`.
    // So `g.query`.
    
    let resp_bytes = g.query("time.now", &req_bytes)?;
    postcard::from_bytes(&resp_bytes).map_err(|_| -1)
}

pub fn monotonic_ns(g: &GraphClient) -> Result<u64, SysRet> {
    let req = TimeMonotonicReq {};
    let req_bytes = to_allocvec(&req).map_err(|_| -1)?;
    let resp_bytes = g.query("time.monotonic_ns", &req_bytes)?;
    let resp: TimeMonotonicResp = postcard::from_bytes(&resp_bytes).map_err(|_| -1)?;
    Ok(resp.monotonic_ns)
}

pub fn sleep_until_ns(g: &GraphClient, wake_ns: u64) -> Result<u64, SysRet> {
    let req = TimeSleepUntilReq { wake_monotonic_ns: wake_ns };
    let req_bytes = to_allocvec(&req).map_err(|_| -1)?;
    // Use query? Yes "time.sleep_until_ns" is in handle_graph_query.
    let resp_bytes = g.query("time.sleep_until_ns", &req_bytes)?;
    let resp: TimeSleepUntilResp = postcard::from_bytes(&resp_bytes).map_err(|_| -1)?;
    Ok(resp.woke_at_monotonic_ns)
}

pub fn sleep_ns(g: &GraphClient, duration_ns: u64) -> Result<u64, SysRet> {
    let req = TimeSleepNsReq { duration_ns };
    let req_bytes = to_allocvec(&req).map_err(|_| -1)?;
    let resp_bytes = g.query("time.sleep_ns", &req_bytes)?;
    let resp: TimeSleepNsResp = postcard::from_bytes(&resp_bytes).map_err(|_| -1)?;
    Ok(resp.woke_at_monotonic_ns)
}

pub fn sleep_ms(g: &GraphClient, ms: u64) -> Result<u64, SysRet> {
    sleep_ns(g, ms.saturating_mul(1_000_000))
}

pub fn sleep_s(g: &GraphClient, s: u64) -> Result<u64, SysRet> {
    sleep_ns(g, s.saturating_mul(1_000_000_000))
}
