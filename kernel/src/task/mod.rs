pub mod scheduler;

pub use scheduler::Scheduler;

use crate::simd::SimdState;
use crate::BootRuntime;
use crate::BootTasking;

pub type TaskId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Running,
    Blocked,
    Dead,
}

pub struct Task<R: BootRuntime> {
    pub id: TaskId,
    pub state: TaskState,

    pub kstack_base: *mut u8,
    pub kstack_size: usize,
    pub kstack_top: u64,

    pub ctx: <R::Tasking as BootTasking>::Context,
    pub aspace: <R::Tasking as BootTasking>::AddressSpace,

    pub simd: SimdState,
}

pub fn init<R: BootRuntime>() {
    scheduler::init::<R>();
}

pub fn spawn<R: BootRuntime>(entry: extern "C" fn(usize) -> !, arg: usize) -> TaskId {
    scheduler::spawn::<R>(entry, arg)
}

pub fn yield_now<R: BootRuntime>() {
    scheduler::yield_now::<R>();
}

pub fn dump_stats<R: BootRuntime>() {
    scheduler::dump_stats::<R>();
}

pub fn run_scheduler<R: BootRuntime>() -> ! {
    loop {
        yield_now::<R>();
        core::hint::spin_loop();
    }
}
