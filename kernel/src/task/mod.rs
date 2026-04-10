pub mod exec;
pub mod loader;
pub mod registry;
use crate::sched as scheduler;

pub use crate::sched::Scheduler;

use crate::BootRuntime;
use crate::BootTasking;
use crate::simd::SimdState;
use abi::types::StackInfo;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

pub type TaskId = crate::sched::state::TaskId;
pub use crate::sched::state::{Affinity, TaskPriority, TaskState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupArg {
    None,
    BootRegistry,
    DeviceId(u64),
    Raw(usize),
}

impl StartupArg {
    pub fn to_raw(self) -> usize {
        match self {
            StartupArg::None => 0,
            StartupArg::BootRegistry => 0x600000,
            StartupArg::DeviceId(id) => id as usize,
            StartupArg::Raw(val) => val,
        }
    }
}

/// Per-process identity and storage.
///
/// Shared by all threads within a process via `Arc<Mutex<ProcessInfo>>`.
/// Kernel tasks typically have `None` — only user processes created
/// by `spawn_process` get one.
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub argv: Vec<Vec<u8>>,
    pub env: BTreeMap<Vec<u8>, Vec<u8>>,
    /// File descriptor table — fds 0/1/2 are pre-populated at spawn time.
    pub fd_table: crate::vfs::fd_table::FdTable,
    /// VFS namespace for this process.
    ///
    /// **ACT III stub**: all processes share the global namespace.  Per-process
    /// divergence (sandboxing / containers) will be wired up in a later act.
    pub namespace: crate::vfs::NamespaceRef,
    /// Current working directory.
    pub cwd: alloc::string::String,
}

pub struct Task<R: BootRuntime> {
    pub id: TaskId,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub exit_code: Option<i32>,
    /// Exit notification is level-triggered: exit status stays readable after wake.
    pub exit_waiters: crate::sched::WaitQueue,
    pub is_user: bool,
    pub wake_pending: bool,
    pub affinity: Affinity,

    pub kstack_base: *mut u8,
    pub kstack_size: usize,
    pub kstack_top: u64,

    pub ctx: <R::Tasking as BootTasking>::Context,
    pub aspace: <R::Tasking as BootTasking>::AddressSpace,

    pub simd: SimdState,

    pub stack_info: Option<StackInfo>,

    pub mappings: Arc<Mutex<crate::memory::mappings::MappingList>>,

    /// Remaining time slice in ticks before preemption
    pub timeslice_remaining: u32,
    pub last_cpu: Option<usize>,

    /// Short human-readable name (e.g. "bristle", "idle/0")
    pub name: [u8; 32],
    pub name_len: u8,

    /// Per-process identity and storage (shared across threads).
    pub process_info: Option<Arc<Mutex<ProcessInfo>>>,

    /// Anti-starvation: tracks the tick when this task was last enqueued (added to a run queue).
    /// Used to calculate how long the task has been waiting: `current_tick - enqueued_at_tick`.
    /// When this exceeds `AGING_THRESHOLD_TICKS`, the task's effective priority is boosted.
    pub enqueued_at_tick: u64,

    /// Anti-starvation: base priority before any aging boost.
    /// When a task is created or its priority is changed via set_priority(), both
    /// `priority` and `base_priority` are updated. The scheduler temporarily modifies
    /// `priority` for aging, but always restores it to `base_priority` when scheduled.
    pub base_priority: TaskPriority,

    /// Per-thread user-mode TLS base (FS_BASE on x86_64).
    ///
    /// Saved on every context switch-out and restored on every context switch-in.
    /// Userspace sets/reads this via `SYS_TASK_SET_TLS_BASE` / `SYS_TASK_GET_TLS_BASE`.
    /// Initialized to 0 for all new threads; the runtime may update it later.
    pub user_fs_base: u64,
}

pub fn init<R: BootRuntime>() {
    crate::task::registry::init::<R>();
    crate::sched::init::<R>();
}
pub fn spawn<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: TaskPriority,
    affinity: Affinity,
) -> TaskId {
    crate::sched::spawn::<R>(entry, arg, priority, affinity)
}

pub fn spawn_with_priority<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: TaskPriority,
) -> TaskId {
    crate::sched::spawn_with_priority::<R>(entry, arg, priority)
}

pub unsafe fn block_current_erased() {
    unsafe {
        crate::sched::block_current_erased();
    }
}

pub unsafe fn wake_task_erased(tid: u64) {
    unsafe {
        crate::sched::wake_task_erased(tid);
    }
}

pub fn yield_now<R: BootRuntime>() -> bool {
    crate::sched::yield_now::<R>()
}

pub fn preempt_disable<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();
    {
        let lock = crate::sched::SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.preempt_disable();
        }
    }
    rt.irq_restore(irq);
}

pub fn preempt_enable<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();

    let switch_params = {
        let lock = crate::sched::SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.preempt_enable()
        } else {
            None
        }
    };

    if let Some(switch) = switch_params {
        let cr3_before = rt.debug_active_aspace_root();

        rt.tasking().activate_address_space(switch.to_aspace);

        let _cr3_after = rt.debug_active_aspace_root();

        unsafe {
            rt.tasking().switch_with_tls(
                &mut *switch.from_ctx,
                &*switch.to_ctx,
                switch.to_tid,
                switch.from_user_fs_base,
                switch.to_user_fs_base,
            );
        }
    }

    rt.irq_restore(irq);
}

pub fn resched_if_needed<R: BootRuntime>() {
    // Explicit safe-point check: yield if need_resched is set, but do NOT
    // run tick bookkeeping or decrement timeslices.
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();

    let switch_params = {
        let lock = crate::sched::SCHEDULER.lock();
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.schedule_point(crate::sched::ScheduleReason::ReschedIfNeeded)
        } else {
            None
        }
    };

    if let Some(switch) = switch_params {
        let cr3_before = rt.debug_active_aspace_root();

        rt.tasking().activate_address_space(switch.to_aspace);

        let _cr3_after = rt.debug_active_aspace_root();

        unsafe {
            rt.tasking().switch_with_tls(
                &mut *switch.from_ctx,
                &*switch.to_ctx,
                switch.to_tid,
                switch.from_user_fs_base,
                switch.to_user_fs_base,
            );
        }
    }

    rt.irq_restore(irq);
}

pub fn dump_stats<R: BootRuntime>() {
    crate::sched::dump_stats::<R>();
}

/// Bootstrap a CPU for scheduling. Must be called before the first yield
/// on any CPU that doesn't already have a current task set (e.g., secondary CPUs).
fn bootstrap_cpu<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();

    let lock = crate::sched::SCHEDULER.lock();
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        let cpu_idx = crate::sched::current_cpu_index::<R>();

        if let Some(pc) = sched.state.per_cpu.get_mut(cpu_idx) {
            if pc.current.is_none() {
                // CPU hasn't been bootstrapped yet. Set current to idle task.
                if let Some(idle_id) = pc.idle_task {
                    pc.current = Some(idle_id);
                    rt.set_current_tid(idle_id);
                    crate::kdebug!(
                        "SMP: CPU {} bootstrapped with idle task {}",
                        cpu_idx,
                        idle_id
                    );

                    // Mark the idle task as running
                    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(idle_id) {
                        task.state = TaskState::Running;
                    }
                } else {
                    crate::kerror!("SMP: CPU {} has no idle task!", cpu_idx);
                }
            }
        }
    }

    rt.irq_restore(_irq);
}

pub fn run_scheduler<R: BootRuntime>() -> ! {
    // Bootstrap this CPU if needed (sets current task for secondary CPUs)
    bootstrap_cpu::<R>();

    // Enable interrupts so this CPU can be preempted or woken from idle (HLT)
    crate::runtime::<R>().irq_restore(crate::IrqState(1));

    let mut idle_count: u64 = 0;
    loop {
        if !yield_now::<R>() {
            // No runnable work — halt until next IRQ (timer tick, device, IPI)
            crate::runtime::<R>().wait_for_interrupt();
            crate::sched::DIAG_HLT_WAKE.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

            idle_count += 1;
            if idle_count % 1000 == 0 {
                let cpu = crate::sched::current_cpu_index::<R>();
                crate::kdebug!("SCHED: CPU {} idle pulse", cpu);
            }
        }
    }
}
