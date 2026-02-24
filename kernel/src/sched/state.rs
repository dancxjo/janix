use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

pub type TaskId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Affinity {
    Any,
    Pinned(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Running,
    Blocked,
    Dead,
}

/// The state necessary to schedule a single task.
pub struct TaskSchedFields {
    pub tid: TaskId,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub base_priority: TaskPriority,
    pub timeslice_remaining: u32,
    pub affinity: Affinity,
    pub wait_ticks: u64,
    pub last_cpu: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct SleepEntry {
    pub tid: TaskId,
    pub wake_tick: u64,
}

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
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
                VecDeque::with_capacity(128),
            ],
            idle_task: None,
            current: None,
            last_switch: 0,
        }
    }
}

pub struct SchedState {
    pub tasks: Vec<TaskSchedFields>,
    pub per_cpu: Vec<PerCpu>,
    pub sleep_queue: VecDeque<SleepEntry>,
    pub wait_queue: VecDeque<TaskId>,
    pub online_cpu_count: usize,
    pub need_resched: bool,
}

impl SchedState {
    pub fn new() -> Self {
        SchedState {
            tasks: Vec::with_capacity(1024),
            per_cpu: Vec::with_capacity(32),
            sleep_queue: VecDeque::with_capacity(1024),
            wait_queue: VecDeque::with_capacity(1024),
            online_cpu_count: 1,
            need_resched: false,
        }
    }

    pub fn get_task_index(&self, tid: TaskId) -> Option<usize> {
        self.tasks.binary_search_by_key(&tid, |t| t.tid).ok()
    }

    pub fn get_task(&self, tid: TaskId) -> Option<&TaskSchedFields> {
        self.get_task_index(tid).map(|idx| &self.tasks[idx])
    }

    pub fn get_task_mut(&mut self, tid: TaskId) -> Option<&mut TaskSchedFields> {
        self.get_task_index(tid).map(move |idx| &mut self.tasks[idx])
    }

    pub fn insert_task(&mut self, fields: TaskSchedFields) {
        match self.tasks.binary_search_by_key(&fields.tid, |t| t.tid) {
            Ok(_) => panic!("Task ID {} already exists in sched", fields.tid),
            Err(idx) => self.tasks.insert(idx, fields),
        }
    }
}

