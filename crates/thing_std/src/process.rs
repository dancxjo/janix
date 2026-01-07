use super::*;

pub fn exit(code: i32) -> ! {
    unsafe {
        syscall(nr::SYS_PROC_EXIT, code as u64, 0, 0, 0, 0, 0);
    }
    loop {}
}

pub fn spawn(name: &str) -> Result<ThingId, i32> {
    unsafe {
        let res = syscall(
            nr::SYS_PROC_SPAWN,
            name.as_ptr() as u64,
            name.len() as u64,
            0,
            0,
            0,
            0,
        );
        if res.status != 0 {
            Err(res.status as i32)
        } else {
            Ok(ThingId::from_parts(res.val1, res.val0)) // High is val1!
        }
    }
}

pub fn sched_yield() {
    unsafe {
        syscall(nr::SYS_SCHED_YIELD, 0, 0, 0, 0, 0, 0);
    }
}

/// Set boot progress (triggers screen color update from kernel)
pub fn boot_progress(step: u32, max_step: u32) {
    unsafe {
        crate::syscall(
            abi::syscall::nr::SYS_BOOT_PROGRESS,
            step as u64, max_step as u64, 0, 0, 0, 0
        );
    }
}
