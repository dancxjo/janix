//! Watch Syscalls

use abi::ids::ThingId;
use abi::syscall::err;
use abi::types::{WatchEvent, WatchEventKind, WatchKind, MAX_WATCH_EVENTS};
use abi::wire::SyscallResult;
use crate::syscall::user_mem;
use abi::cap::CapOp;

// Local zero value to seed scratch buffers without relying on Default.
const EMPTY_EVENT: WatchEvent = WatchEvent {
    kind: WatchEventKind::ThingUpdated,
    flags: 0,
    subject: ThingId(0),
    arg0: ThingId(0),
};

pub fn sys_watch_create(kind_raw: u64, target_low: u64) -> SyscallResult {
    let kind = match kind_raw as u16 {
        x if x == WatchKind::GraphMembership as u16 => WatchKind::GraphMembership,
        x if x == WatchKind::Thing as u16 => WatchKind::Thing,
        _ => return SyscallResult::new(err::EINVAL, 0, 0),
    };
    let target = ThingId(target_low as u128);
    if let Err(code) = user_mem::require_current_cap(CapOp::GraphWatch, Some(target)) {
        return SyscallResult::new(code, 0, 0);
    }
    let watch_id = crate::watch::create_watch(kind, target);
    SyscallResult::new(0, watch_id.0, 0)
}

pub fn sys_watch_poll(watch_id_raw: u64, out_ptr: u64, out_len: u64) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::GraphWatch, None) {
        return SyscallResult::new(code, 0, 0);
    }

    if out_ptr == 0 || out_len == 0 {
        return SyscallResult::new(err::EINVAL, 0, 0);
    }

    let out_len = match usize::try_from(out_len) {
        Ok(len) => len,
        Err(_) => return SyscallResult::new(err::EINVAL, 0, 0),
    };
    let count = core::cmp::min(out_len, MAX_WATCH_EVENTS);
    if count == 0 {
        return SyscallResult::new(0, 0, 0);
    }

    // Use an aligned scratch buffer in the kernel, then byte-copy to the
    // caller's buffer to avoid alignment requirements on user pointers.
    let mut scratch = [EMPTY_EVENT; MAX_WATCH_EVENTS];
    let out_slice = &mut scratch[..count];

    match crate::watch::poll_watch(abi::ids::WatchId(watch_id_raw), out_slice) {
        Ok(written) => {
            let byte_len = written * core::mem::size_of::<WatchEvent>();
            if byte_len > 0 {
                let bytes = unsafe {
                    core::slice::from_raw_parts(out_slice.as_ptr() as *const u8, byte_len)
                };
                if let Err(code) = user_mem::copy_to_user(out_ptr, bytes, byte_len) {
                    return SyscallResult::new(code, 0, 0);
                }
            }
            SyscallResult::new(0, written as u64, 0)
        }
        Err(e) => SyscallResult::new(e, 0, 0),
    }
}
