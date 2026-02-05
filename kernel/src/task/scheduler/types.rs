//! Core scheduler types and data structures.

use crate::BootRuntime;
use crate::task::{Task, TaskId};
use alloc::collections::BTreeMap;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// Default time slice in ticks (~100ms at 100Hz timer)
pub const DEFAULT_TIMESLICE: u32 = 10;

/// Maximum number of CPUs supported
pub const MAX_CPUS: usize = 32;

pub struct PerCpu {
    pub runq: [VecDeque<TaskId>; 5],
    pub idle_task: Option<TaskId>,
    pub current: Option<TaskId>,
}

impl PerCpu {
    pub fn new() -> Self {
        PerCpu {
            runq: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
            idle_task: None,
            current: None,
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
    PreemptTick,
    CooperativeYield,
    SleepWait,
    BlockedOnIo,
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
    /// Maps TaskId -> ThingId for graph node lookups
    pub(crate) task_graph: BTreeMap<TaskId, u64>,
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
            tasks: Vec::new(),
            wait_queue: VecDeque::new(),
            sleep_queue: VecDeque::new(),
            per_cpu: Vec::new(),
            next_id: 1,
            preempt_disable_depth: 0,
            preempt_disable_since: 0,
            watchdog_warned: false,
            need_resched: false,
            total_cpu_count: 1,
            online_cpu_count: 1,
            bringup_in_progress: false,
            metrics: SchedulerMetrics::new(),
            task_graph: BTreeMap::new(),
        }
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
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
        self.tasks.iter().find(|t| t.id == tid).map(|t| t.priority)
    }

    /// Get the graph ThingId for a task
    pub fn graph_thing_for_tid(&self, tid: TaskId) -> Option<u64> {
        self.task_graph.get(&tid).copied()
    }

    /// Set the graph ThingId for a task
    pub fn set_graph_thing_for_tid(&mut self, tid: TaskId, thing_id: u64) {
        self.task_graph.insert(tid, thing_id);
    }

    /// Remove the graph ThingId for a task (e.g., when task is cleaned up)
    pub fn remove_graph_thing_for_tid(&mut self, tid: TaskId) -> Option<u64> {
        self.task_graph.remove(&tid)
    }
}
