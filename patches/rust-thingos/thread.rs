// library/std/src/sys/thread/thingos.rs
use crate::ffi::CStr;
use crate::io;
use crate::time::Duration;
use crate::num::NonZero;
use crate::thread::ThreadInit;

pub struct Thread(());

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, _p: Box<ThreadInit>) -> io::Result<Thread> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "thread spawn not supported"))
    }

    pub fn join(self) {
        // No-op since we don't spawn
    }
}

pub fn yield_now() {
    // SYSCALL_YIELD = 2
    unsafe { syscall0(2); }
}

pub fn set_name(_name: &CStr) {
    // TODO: syscall to set name
}

pub fn sleep(dur: Duration) {
    // SYSCALL_SLEEP = 12
    unsafe { syscall1(12, dur.as_nanos() as u64); }
}

pub fn sleep_until(_deadline: crate::time::Instant) {
    // TODO: proper sleep until
    // For now, sleep(0)
    unsafe { syscall1(12, 0); }
}

pub fn current_os_id() -> Option<u64> {
    // TODO: syscall to get TID. For now return None to use fallback.
    None
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    unsafe { Ok(NonZero::new_unchecked(1)) }
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> { None }
    pub unsafe fn init() -> Option<Guard> { None }
}

#[inline(always)]
unsafe fn syscall0(n: u64) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline(always)]
unsafe fn syscall1(n: u64, a1: u64) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}
