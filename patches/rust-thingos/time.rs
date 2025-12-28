// library/std/src/sys/time/thingos.rs
use crate::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant(u64);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime(u64);

pub const UNIX_EPOCH: SystemTime = SystemTime(0);

impl Instant {
    pub fn now() -> Instant {
        let (mono, _) = get_time();
        Instant(mono)
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.0.checked_sub(other.0).map(Duration::from_nanos)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        self.0.checked_add(other.as_nanos() as u64).map(Instant)
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        self.0.checked_sub(other.as_nanos() as u64).map(Instant)
    }
}

impl SystemTime {
    pub const MAX: SystemTime = SystemTime(u64::MAX);
    pub const MIN: SystemTime = SystemTime(u64::MIN);

    pub fn now() -> SystemTime {
        let (_, sys) = get_time();
        SystemTime(sys)
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        if self.0 >= other.0 {
            Ok(Duration::from_nanos(self.0 - other.0))
        } else {
            Err(Duration::from_nanos(other.0 - self.0))
        }
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        self.0.checked_add(other.as_nanos() as u64).map(SystemTime)
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        self.0.checked_sub(other.as_nanos() as u64).map(SystemTime)
    }
}

fn get_time() -> (u64, u64) {
    let mut out: [u64; 2] = [0; 2];
    // SYSCALL_TIME = 11
    unsafe {
        let _ = syscall1(11, out.as_mut_ptr() as u64);
    }
    (out[0], out[1])
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
