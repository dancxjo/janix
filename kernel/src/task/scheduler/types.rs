//! Core scheduler types and data structures.

use crate::BootRuntime;
use crate::task::{Task, TaskId};
use alloc::collections::VecDeque;
use alloc::vec::Vec;

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
    pub(crate) tasks: Vec<Task<R>>,
    pub(crate) runq: VecDeque<TaskId>,
    pub(crate) wait_queue: VecDeque<TaskId>,
    pub(crate) current: Option<TaskId>,
    pub(crate) next_id: TaskId,
    pub(crate) idle_task: Option<TaskId>,
    pub(crate) preempt_disable_depth: usize,
    pub(crate) need_resched: bool,
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
            tasks: Vec::new(),
            runq: VecDeque::new(),
            wait_queue: VecDeque::new(),
            current: None,
            next_id: 1,
            idle_task: None,
            preempt_disable_depth: 0,
            need_resched: false,
            metrics: SchedulerMetrics::new(),
        }
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn current_id(&self) -> Option<TaskId> {
        self.current
    }
}
