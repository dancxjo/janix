use super::*;

pub fn exit(code: i32) -> ! {
    unsafe {
        syscall(nr::SYS_PROC_EXIT, code as u64, 0, 0, 0, 0, 0);
    }
    loop {}
}

pub fn spawn(name: &str) {
    unsafe {
        syscall(
            nr::SYS_PROC_SPAWN,
            name.as_ptr() as u64,
            name.len() as u64,
            0,
            0,
            0,
            0,
        );
    }
}

pub fn sched_yield() {
    unsafe {
        syscall(nr::SYS_SCHED_YIELD, 0, 0, 0, 0, 0, 0);
    }
}
