
use crate::ffi::CStr;
use crate::io;
use crate::time::Duration;

pub struct Thread(());

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new<F>(_stack: usize, _p: F) -> io::Result<Thread> {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    }

    pub fn join(self) {}
}

pub fn yield_now() {}

pub fn set_name(_name: &CStr) {}

pub fn sleep(_dur: Duration) {}

pub fn current_os_id() -> Option<u64> { Some(0) }

pub fn available_parallelism() -> io::Result<crate::num::NonZeroUsize> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}
