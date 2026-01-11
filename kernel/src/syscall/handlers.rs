use abi::errors::{Errno, SysResult};
use abi::device::{DeviceCall, DeviceKind};
use super::validate::{validate_user_range, copyin};
use crate::task::scheduler::{yield_now, sleep_ms};

pub fn sys_exit(code: i32) -> SysResult<usize> {
    crate::kprintln!("SYSCALL EXIT: code={}", code);
    // TODO: Actually terminate the task.
    // For now, just spin or panic to show we got here.
    // panic!("Task exited with code {}", code); 
    // Or simpler:
    loop { core::hint::spin_loop(); }
}

pub fn sys_debug_write(ptr: usize, len: usize) -> SysResult<usize> {
    // Limit max write to avoid huge buffers
    if len > 1024 {
        return Err(Errno::EINVAL);
    }
    
    // Check range
    validate_user_range(ptr, len, false)?;
    
    // Copy to stack buffer (chunked if needed, but we limited to 1024)
    // We'll use a small buffer for safety
    let mut buf = [0u8; 128];
    let mut offset = 0;
    
    while offset < len {
        let chunk_len = core::cmp::min(len - offset, buf.len());
        unsafe { copyin(&mut buf[..chunk_len], ptr + offset)?; }
        
        // Print it (lossy utf8 check is fine for debug)
        if let Ok(s) = core::str::from_utf8(&buf[..chunk_len]) {
             crate::kprint!("{}", s);
        } else {
             crate::kprint!("<invalid utf8>");
        }
        
        offset += chunk_len;
    }
    
    Ok(len)
}

pub fn sys_yield() -> SysResult<usize> {
    // Co-operative yield
    // Since R is not easily available here, we need help.
    // Actually, handlers.rs doesn't know about R.
    // But `yield_now` in `kernel::task::scheduler` requires `<R>`.
    // Wait, the scheduler functions are generic over R.
    // The syscall dispatch needs to know R.
    // Currently `sys_sleep_ms` stub used `yield_now_current`.
    // We should use `crate::task::yield_now::<R>()` but we don't have R.
    // Solution: We need to use `yield_now_current` which uses the hook, OR `crate::task::scheduler::SCHEDULER`.
    // But `get_scheduler()` is generic.
    // `yield_now_current` is the safe-ish wrapper around the hook.
    // We already have `yield_now_current` exposed in `scheduler.rs`.
    // Let's use that for now since we are in a non-generic context.
    
    // Actually, `crate::task::yield_now_current()` calls `YIELD_HOOK` which is typed to `yield_now::<R>`.
    // So this works perfectly for non-generic handlers.
    
    unsafe { crate::task::scheduler::yield_now_current(); }
    Ok(0)
}

pub fn sys_sleep_ms(ms: u64) -> SysResult<usize> {
    // We need a hook for sleep too if we want to avoid generics here, 
    // OR we expose a non-generic `sleep_ms_current`.
    // `sleep_ms` in `scheduler.rs` is generic R.
    // We should add a hook for it too or just use `yield_now_current` loop here for v0?
    // User plan said: "sys_sleep_ms(ms): calls sleep_until(now+ms)".
    // The `sleep_ms` I wrote in scheduler.rs is `sleep_ms<R>`.
    // I can't call it easily from here.
    
    // Option A: Add `SLEEP_HOOK` to scheduler.rs.
    // Option B: Just loop yield_now_current here for now (Plan v0 allowed yield-loop).
    // Option C: Make handlers generic (huge change).
    
    // Let's go with Option B for "v0" correctness but maybe add a TODO or helper.
    // Actually, `sleep_until` logic is: loop { check time; yield; }.
    // I can implement that here using `runtime_base().mono_ticks()` and `yield_now_current()`.
    // It duplicates the logic but avoids the generic mess.
    
    let rt = crate::runtime_base(); // This returns &dyn BootRuntime? No, it returns ... wait.
    // `crate::runtime::<R>()` returns `&R`.
    // `crate::runtime_base()` ? 
    // Let's check kernel/src/lib.rs or mod.rs. 
    // Actually `runtime()` is commonly used.
    // If I don't have R, I can't get the runtime easily unless there is a global or trait object.
    // `BootRuntime` is a trait.
    
    // Wait, `sys_sleep_ms` in existing code used `crate::runtime_base().mono_ticks()`.
    // Does `runtime_base` exist? 
    // Line 50 of original file: `let start = crate::runtime_base().mono_ticks();`
    // So yes, it exists. It probably returns `&dyn BootRuntime` or similar? 
    // Or maybe `runtime_base()` isn't generic?
    // Let's rely on it.
    
    // V0 implementation: simple loop in handler.
    let start = crate::runtime_base().mono_ticks();
    let freq = crate::runtime_base().mono_freq_hz();
    let ticks = (ms * freq) / 1000;
    let deadline = start + ticks;
    
    loop {
        let now = crate::runtime_base().mono_ticks();
        if now >= deadline {
            break;
        }
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
    
    Ok(0)
}

pub fn sys_device_call(call_ptr: usize) -> SysResult<usize> {
    // Validate the DeviceCall struct itself
    let size = core::mem::size_of::<DeviceCall>();
    validate_user_range(call_ptr, size, true)?;
    
    // Copy it in
    let mut call: DeviceCall = unsafe { core::mem::zeroed() };
    let slice = unsafe { 
        core::slice::from_raw_parts_mut(
            &mut call as *mut _ as *mut u8, 
            size
        ) 
    };
    unsafe { copyin(slice, call_ptr)?; }
    
    // Dispatch based on kind
    match call.kind {
        DeviceKind::RtcCmos => {
            // Stub: return NotSupported
            Err(Errno::NotSupported)
        }
        _ => Err(Errno::NotSupported)
    }
}
