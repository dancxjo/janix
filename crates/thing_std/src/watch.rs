use abi::ids::{ThingId, WatchId};
use abi::syscall::nr;
use abi::types::{WaitFlags, WakeReason, WakeReasonCode, WatchEvent, WatchKind};

use crate::syscall;

pub fn watch_create(kind: WatchKind, target: ThingId) -> Result<WatchId, i32> {
    let res = unsafe {
        syscall(
            nr::SYS_WATCH_CREATE,
            kind as u16 as u64,
            target.low(),
            0,
            0,
            0,
            0,
        )
    };
    if res.status == 0 {
        Ok(WatchId(res.val0))
    } else {
        Err(res.status as i32)
    }
}

pub fn watch_poll(id: WatchId, out: &mut [WatchEvent]) -> Result<usize, i32> {
    if out.is_empty() {
        return Ok(0);
    }
    let res = unsafe {
        syscall(
            nr::SYS_WATCH_POLL,
            id.0,
            out.as_mut_ptr() as u64,
            out.len() as u64,
            0,
            0,
            0,
        )
    };
    if res.status == 0 {
        Ok(res.val0 as usize)
    } else {
        Err(res.status as i32)
    }
}

pub fn watch_wait(
    watches: &[WatchId],
    flags: WaitFlags,
    timeout_ticks: u64,
) -> Result<WakeReason, i32> {
    let ptr = if watches.is_empty() {
        core::ptr::null()
    } else {
        watches.as_ptr() as *const u64
    };

    let res = unsafe {
        syscall(
            nr::SYS_WAIT,
            ptr as u64,
            watches.len() as u64,
            flags.bits() as u64,
            timeout_ticks,
            0,
            0,
        )
    };

    if res.status != 0 {
        return Err(res.status as i32);
    }

    let reason = match res.val0 as u32 {
        x if x == WakeReasonCode::Watch as u32 => WakeReasonCode::Watch,
        x if x == WakeReasonCode::Timeout as u32 => WakeReasonCode::Timeout,
        _ => WakeReasonCode::Cancelled,
    };

    Ok(WakeReason {
        reason,
        which: WatchId(res.val1),
        val0: 0,
        val1: 0,
    })
}
