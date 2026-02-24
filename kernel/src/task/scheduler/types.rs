//! Core scheduler types and data structures.

use crate::BootRuntime;
use crate::task::{Task, TaskId};
use alloc::collections::BTreeMap;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// Separate lock for TaskId→ThingId graph mappings.
///
/// This is intentionally **not** inside `Scheduler` so that
/// `flush_graph_queue` can look up / insert graph IDs without
/// acquiring the main `SCHEDULER` spinlock, which is the primary
/// source of trylock-miss contention on the timer ISR path.
pub(crate) static TASK_GRAPH: spin::Mutex<BTreeMap<TaskId, u64>> =
    spin::Mutex::new(BTreeMap::new());

/// Look up the graph ThingId for a task (lock-free w.r.t. SCHEDULER).
pub(crate) fn graph_thing_for_tid(tid: TaskId) -> Option<u64> {
    TASK_GRAPH.lock().get(&tid).copied()
}

/// Set the graph ThingId for a task.
pub(crate) fn set_graph_thing_for_tid(tid: TaskId, thing_id: u64) {
    TASK_GRAPH.lock().insert(tid, thing_id);
}

/// Remove the graph ThingId for a task (e.g., when task is cleaned up).
pub(crate) fn remove_graph_thing_for_tid(tid: TaskId) -> Option<u64> {
    TASK_GRAPH.lock().remove(&tid)
}

/// Default time slice in ticks (~100ms at 100Hz timer)
pub const DEFAULT_TIMESLICE: u32 = 10;

/// Maximum number of CPUs supported
pub const MAX_CPUS: usize = 32;

/// Anti-starvation: ticks to wait before boosting priority by one level
/// At 100Hz, 500 ticks = ~5 seconds
/// 
/// This ensures low-priority tasks don't starve even when high-priority tasks
/// are continuously runnable. After waiting for AGING_THRESHOLD_TICKS, a task's
/// priority is temporarily boosted by one level until it gets scheduled.
pub const AGING_THRESHOLD_TICKS: u64 = 500;

/// Anti-starvation: maximum priority boost levels (prevents excessive boosting)
/// 
/// Limits how many priority levels a task can be boosted. For example, with
/// MAX_PRIORITY_BOOST = 2, a Low priority task can be boosted to at most High
/// priority (Low -> Normal -> High), but never to Realtime.
pub const MAX_PRIORITY_BOOST: usize = 2;

pub struct PerCpu {
    pub runq: [VecDeque<TaskId>; 5],
    pub idle_task: Option<TaskId>,
    pub current: Option<TaskId>,
    pub last_switch: u64,
}

impl PerCpu {
    pub fn new() -> Self {
        PerCpu {
            runq: [
                VecDeque::with_capacity(1024),
                VecDeque::with_capacity(1024),
                VecDeque::with_capacity(1024),
                VecDeque::with_capacity(1024),
                VecDeque::with_capacity(1024),
            ],
            idle_task: None,
            current: None,
            last_switch: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackFaultResult {
    NotStack,
    Grew,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleReason {
    /// Timer ISR path only — runs tick bookkeeping (wake sleepers, watchdog,
    /// wait-time aging, timeslice decrement).
    PreemptTick,
    CooperativeYield,
    SleepWait,
    BlockedOnIo,
    /// Used by preempt_enable(). Yields if need_resched is set but does NOT
    /// run tick bookkeeping or decrement timeslices.
    SafePoint,
    /// Used by explicit resched_if_needed() checks at syscall-return or other
    /// safe points. Same behaviour as SafePoint, semantically distinct.
    ReschedIfNeeded,
}

/// Entry in the sleep queue tracking when a task should wake
#[derive(Debug, Clone, Copy)]
pub struct SleepEntry {
    pub task_id: TaskId,
    pub wake_tick: u64, // absolute tick count when task should wake
}

pub struct SwitchParams<Ctx, AS> {
    pub from_ctx: *mut Ctx,
    pub to_ctx: *const Ctx,
    pub to_aspace: AS,
    pub from_aspace: AS,
    pub from_tid: TaskId,
    pub to_tid: TaskId,
    pub from_user: bool,
    pub to_user: bool,
}

pub(crate) struct SchedulerMetrics {
    pub yields: u64,
    pub pops: u64,
    pub pushes: u64,
    pub idle_picks: u64,
    pub last_flush: u64,
}

pub struct Scheduler<R: BootRuntime> {
    pub(crate) tasks: Vec<alloc::boxed::Box<Task<R>>>,
    pub(crate) wait_queue: VecDeque<TaskId>,
    pub(crate) sleep_queue: VecDeque<SleepEntry>, // tasks sleeping with wake times
    pub(crate) per_cpu: Vec<PerCpu>,
    pub(crate) next_id: TaskId,
    pub(crate) preempt_disable_depth: usize,
    pub(crate) preempt_disable_since: u64,
    pub(crate) watchdog_warned: bool,
    pub(crate) need_resched: bool,

    pub(crate) total_cpu_count: usize,
    pub(crate) online_cpu_count: usize,
    pub(crate) bringup_in_progress: bool,

    pub(crate) metrics: SchedulerMetrics,
}

impl SchedulerMetrics {
    pub fn new() -> Self {
        SchedulerMetrics {
            yields: 0,
            pops: 0,
            pushes: 0,
            idle_picks: 0,
            last_flush: 0,
        }
    }
}

impl<R: BootRuntime> Scheduler<R> {
    pub fn new() -> Self {
        Scheduler {
            tasks: Vec::with_capacity(1024),
            wait_queue: VecDeque::with_capacity(1024),
            sleep_queue: VecDeque::with_capacity(1024),
            per_cpu: Vec::with_capacity(32),
            next_id: 1,
            preempt_disable_depth: 0,
            preempt_disable_since: 0,
            watchdog_warned: false,
            need_resched: false,
            total_cpu_count: 1,
            online_cpu_count: 1,
            bringup_in_progress: false,
            metrics: SchedulerMetrics::new(),
        }
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn get_task_index(&self, id: TaskId) -> Option<usize> {
        self.tasks.binary_search_by_key(&id, |t| t.id).ok()
    }

    pub fn get_task_index_mut(&mut self, id: TaskId) -> Option<usize> {
        self.tasks.binary_search_by_key(&id, |t| t.id).ok()
    }

    pub fn insert_task(&mut self, task: alloc::boxed::Box<Task<R>>) {
        let id = task.id;
        match self.tasks.binary_search_by_key(&id, |t| t.id) {
            Ok(_) => panic!("Task ID {} already exists", id),
            Err(idx) => self.tasks.insert(idx, task),
        }
    }

    pub fn get_task(&self, id: TaskId) -> Option<&Task<R>> {
        self.get_task_index(id).map(|idx| &*self.tasks[idx])
    }

    pub fn get_task_mut(&mut self, id: TaskId) -> Option<&mut Task<R>> {
        self.get_task_index(id).map(move |idx| &mut *self.tasks[idx])
    }

    pub fn current_id(&self) -> Option<TaskId> {
        // This is tricky without knowing which CPU we are asking about.
        // For backwards compat logging, valid use mainly inside scheduler or per-cpu hooks.
        // We really need current_cpu_index here.
        // But Scheduler::current_id passed no index.
        // We will return None or rely on caller to use per-cpu accessors.
        // Actually, let's remove this helper or make it panic/useless?
        // Or better: `Scheduler` methods should generally task `cpu_index`?
        None
    }

    pub fn current_id_on_cpu(&self, cpu: usize) -> Option<TaskId> {
        self.per_cpu.get(cpu).and_then(|pc| pc.current)
    }

    pub fn current_priority(&self) -> Option<crate::task::TaskPriority> {
        // Also needs cpu index.
        None
    }

    pub fn current_priority_on_cpu(&self, cpu: usize) -> Option<crate::task::TaskPriority> {
        let tid = self.current_id_on_cpu(cpu)?;
        self.get_task(tid).map(|t| t.priority)
    }



    /// Returns `true` if there is at least one task in a non-idle run queue
    /// (priority levels 1–4) for the given CPU. Used by `run_scheduler` to
    /// decide whether to halt or keep spinning.
    pub fn has_runnable_work(&self, cpu_idx: usize) -> bool {
        if let Some(pc) = self.per_cpu.get(cpu_idx) {
            // Check priority queues 1 (Low) through 4 (Realtime)
            pc.runq[1..].iter().any(|q| !q.is_empty())
        } else {
            false
        }
    }
}
