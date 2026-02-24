//! Scheduler event types for the lock-free event ring buffer.
//!
//! These are fixed-size, `Copy` types designed to be pushed into
//! per-CPU ring buffers without allocation or locking.

/// Reason a task was dequeued from the run queue.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DequeueReason {
    /// Task was selected by `prepare_schedule` to run
    Scheduled = 0,
    /// Task was killed or terminated
    Killed = 1,
    /// Task was blocked on I/O or a wait
    Blocked = 2,
    /// Task entered a timed sleep
    Sleeping = 3,
}

/// Reason a task was blocked.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockReason {
    /// Blocked on I/O (port wait, etc.)
    Io = 0,
    /// Blocked on a syscall (waiting for Root reply, etc.)
    Syscall = 1,
    /// Blocked by explicit block_current()
    Explicit = 2,
    /// Blocked waiting on a sleep timer
    Sleep = 3,
}

/// Source of a task wakeup.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeSource {
    /// Woken by sleep timer expiry
    Timer = 0,
    /// Woken by another task (e.g., message send)
    Task = 1,
    /// Woken by an interrupt / IRQ
    Interrupt = 2,
    /// Woken by explicit wake_task() call
    Explicit = 3,
}

/// A scheduler event suitable for lock-free ring buffer storage.
///
/// All variants are fixed-size and `Copy`. No heap allocation needed.
/// The `timestamp` field in each variant stores the monotonic tick
/// count when the event was emitted.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum SchedEvent {
    /// Task was added to a run queue
    TaskEnqueued {
        tid: u64,
        cpu: u16,
        prio: u8,
        timestamp: u64,
    },
    /// Task was removed from a run queue
    TaskDequeued {
        tid: u64,
        reason: DequeueReason,
        timestamp: u64,
    },
    /// Task ran for some number of ticks
    TaskRan {
        tid: u64,
        cpu: u16,
        ticks: u32,
        timestamp: u64,
    },
    /// Task transitioned to blocked state
    TaskBlocked {
        tid: u64,
        reason: BlockReason,
        timestamp: u64,
    },
    /// Task was woken up
    TaskWoke {
        tid: u64,
        source: WakeSource,
        timestamp: u64,
    },
    /// Task voluntarily yielded
    TaskYielded {
        tid: u64,
        timestamp: u64,
    },
    /// A new task was created
    TaskCreated {
        tid: u64,
        prio: u8,
        is_user: bool,
        parent_tid: u64,
        /// First 24 bytes of the task name (zero-padded)
        name: [u8; 24],
        timestamp: u64,
    },
    /// A task exited
    TaskExited {
        tid: u64,
        code: i32,
        timestamp: u64,
    },
    /// Task priority was changed
    PriorityChanged {
        tid: u64,
        old_prio: u8,
        new_prio: u8,
        timestamp: u64,
    },
    /// Task affinity was set to a specific CPU
    AffinitySet {
        tid: u64,
        cpu: u16,
        timestamp: u64,
    },
    /// Task location (RUNS_ON) was updated
    LocationSet {
        tid: u64,
        cpu: u16,
        timestamp: u64,
    },
    /// Task state changed (generic state update)
    StateChanged {
        tid: u64,
        /// Interned state name pointer (static str pointer cast to u64)
        state_ptr: u64,
        timestamp: u64,
    },
}

/// Pack a task name (up to 24 bytes) into a fixed-size array.
pub fn pack_name(name: Option<&str>) -> [u8; 24] {
    let mut buf = [0u8; 24];
    if let Some(n) = name {
        let len = n.len().min(24);
        buf[..len].copy_from_slice(&n.as_bytes()[..len]);
    }
    buf
}
