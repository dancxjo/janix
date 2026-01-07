#![cfg(test)]

use crate::machine::{self, Machine, MmioMapping, MmioRange, MmioFlags, Context};
use crate::sched::{Scheduler, SCHEDULER, Task, TaskId, TaskState};
use crate::memory::space::AddressSpace;
use crate::memory::map::MapPerms;
use abi::ids::ThingId;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::{Mutex, Once};
use core::sync::atomic::{AtomicU64, Ordering};

struct MockMachine;

impl Machine for MockMachine {
    fn console_write(&self, bytes: &[u8]) -> usize {
        bytes.len()
    }

    fn mmio_map(&self, range: MmioRange, _flags: MmioFlags) -> Option<MmioMapping> {
        Some(MmioMapping { virt: range.phys, len: range.len })
    }

    fn monotonic_now(&self) -> u64 { 0 }

    fn irq_disable(&self) -> u64 { 0 }
    fn irq_restore(&self, _token: u64) {}

    fn halt(&self) -> ! {
        panic!("HALT called in test");
    }

    fn idle(&self) {}

    fn switch_to(&self, _old_ctx: &mut Context, _new_ctx: &Context) {}

    fn task_entry_stub(&self) -> u64 { 0 }

    fn virt_to_phys(&self, virt: u64) -> u64 { virt }
}

static MOCK_MACHINE: MockMachine = MockMachine;
static INIT: Once<()> = Once::new();
static TEST_LOCK: Mutex<()> = Mutex::new(());
static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1000);

pub fn setup_test_context() {
    INIT.call_once(|| {
        unsafe { machine::install(&MOCK_MACHINE) };

        // Initialize SCHEDULER
        let sched = Scheduler::new(ThingId(100), ThingId(101));
        *SCHEDULER.lock() = Some(sched);
    });
}

pub fn with_test_task<F, R>(f: F) -> R
where
    F: FnOnce(TaskId) -> R,
{
    // Serialize tests to prevent race conditions on global state
    let _lock = TEST_LOCK.lock();

    setup_test_context();

    let tid = TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed));

    {
        let mut guard = SCHEDULER.lock();
        let sched = guard.as_mut().unwrap();

        let thing = ThingId(tid.0 as u128);
        let address_space = Arc::new(AddressSpace::new().unwrap());
        let mut task = Task::new(tid, thing, 0x10000, address_space);
        task.state = TaskState::Running;
        sched.tasks.push(task);

        sched.cpu.current_task = tid;
    }

    let res = f(tid);

    // Cleanup (best effort)
    {
        let mut guard = SCHEDULER.lock();
        if let Some(sched) = guard.as_mut() {
            if sched.cpu.current_task == tid {
                sched.cpu.current_task = TaskId(0);
            }
            // Optional: remove task to keep list small
            // sched.tasks.retain(|t| t.id != tid);
        }
    }

    res
}

pub fn map_user_memory(tid: TaskId, ptr: u64, len: usize, perms: MapPerms) {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut().unwrap();
    let task = sched.tasks.iter_mut().find(|t| t.id == tid).expect("test task missing");
    task.address_space.map(ptr, ptr, len, perms).unwrap();
}

pub fn set_current_caps(tid: TaskId, caps: Vec<abi::cap::Cap>) {
    let mut guard = SCHEDULER.lock();
    let sched = guard.as_mut().unwrap();
    let task = sched.tasks.iter_mut().find(|t| t.id == tid).expect("test task missing");
    task.caps = caps;
}
