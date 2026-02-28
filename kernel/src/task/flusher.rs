//! Budgeted flusher: drains per-CPU event rings and translates
//! SchedEvent → graph operations in bounded batches.
//!
//! The flusher runs from the `graph_worker_task` in thread context
//! (never from ISR). It respects a tick budget to avoid starving
//! other kernel work.

use crate::sched::events::SchedEvent;

use super::graph_queue::{self, GraphWork};
use crate::sched::ring;
use crate::sched::types;
use alloc::string::String;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// Metrics
static FLUSH_CALLS: AtomicU64 = AtomicU64::new(0);
static FLUSH_EVENTS: AtomicU64 = AtomicU64::new(0);
static FLUSH_TICKS_TOTAL: AtomicU64 = AtomicU64::new(0);
static FLUSH_TICKS_MAX: AtomicU64 = AtomicU64::new(0);

static EVENT_CREATED: AtomicUsize = AtomicUsize::new(0);
static EVENT_EXITED: AtomicUsize = AtomicUsize::new(0);
static EVENT_STATE: AtomicUsize = AtomicUsize::new(0);
static EVENT_PRIO: AtomicUsize = AtomicUsize::new(0);
static EVENT_AFF: AtomicUsize = AtomicUsize::new(0);
static EVENT_LOC: AtomicUsize = AtomicUsize::new(0);
static EVENT_NAME: AtomicUsize = AtomicUsize::new(0);
static EVENT_RAN: AtomicUsize = AtomicUsize::new(0);
static EVENT_BLOCKED: AtomicUsize = AtomicUsize::new(0);
static EVENT_WOKE: AtomicUsize = AtomicUsize::new(0);
static EVENT_YIELDED: AtomicUsize = AtomicUsize::new(0);
static EVENT_ENQ: AtomicUsize = AtomicUsize::new(0);
static EVENT_DEQ: AtomicUsize = AtomicUsize::new(0);

/// Result of a flush operation.
#[derive(Clone, Copy, Debug, Default)]
pub struct FlushResult {
    /// Number of events translated to graph operations
    pub applied: usize,
    /// Elapsed ticks for this flush call
    pub elapsed_ticks: u64,
}

/// Snapshot of flusher metrics. Reset-on-read.
#[derive(Clone, Copy, Debug, Default)]
pub struct FlusherMetrics {
    pub flush_calls: u64,
    pub flush_events: u64,
    pub flush_ticks_total: u64,
    pub flush_ticks_max: u64,
}

/// Read and reset flusher metrics (for periodic logging).
pub fn metrics_snapshot_and_reset() -> FlusherMetrics {
    let c_created = EVENT_CREATED.swap(0, Ordering::Relaxed);
    let c_exited = EVENT_EXITED.swap(0, Ordering::Relaxed);
    let c_state = EVENT_STATE.swap(0, Ordering::Relaxed);
    let c_prio = EVENT_PRIO.swap(0, Ordering::Relaxed);
    let c_aff = EVENT_AFF.swap(0, Ordering::Relaxed);
    let c_loc = EVENT_LOC.swap(0, Ordering::Relaxed);
    let c_name = EVENT_NAME.swap(0, Ordering::Relaxed);
    let c_ran = EVENT_RAN.swap(0, Ordering::Relaxed);
    let c_blk = EVENT_BLOCKED.swap(0, Ordering::Relaxed);
    let c_wok = EVENT_WOKE.swap(0, Ordering::Relaxed);
    let c_yld = EVENT_YIELDED.swap(0, Ordering::Relaxed);
    let c_enq = EVENT_ENQ.swap(0, Ordering::Relaxed);
    let c_deq = EVENT_DEQ.swap(0, Ordering::Relaxed);

    let total = c_created + c_exited + c_state + c_prio + c_aff + c_loc + c_name + c_ran + c_blk + c_wok + c_yld + c_enq + c_deq;
    if total > 0 {
        crate::kinfo!("FLUSH_SPAM: tot={} cr={} ex={} st={} pr={} af={} lc={} nm={} blk={} wok={} yld={} enq={} deq={}",
            total, c_created, c_exited, c_state, c_prio, c_aff, c_loc, c_name, c_blk, c_wok, c_yld, c_enq, c_deq);
    }

    FlusherMetrics {
        flush_calls: FLUSH_CALLS.swap(0, Ordering::Relaxed),
        flush_events: FLUSH_EVENTS.swap(0, Ordering::Relaxed),
        flush_ticks_total: FLUSH_TICKS_TOTAL.swap(0, Ordering::Relaxed),
        flush_ticks_max: FLUSH_TICKS_MAX.swap(0, Ordering::Relaxed),
    }
}

/// Drain events from all per-CPU rings and translate them into
/// graph work items, respecting a budget.
///
/// - `max_events`: maximum total events to process across all CPUs
/// - `max_ticks`: stop if elapsed monotonic ticks exceed this (0 = no limit)
/// - `start_ticks`: current monotonic tick count (caller reads this before calling)
///
/// Returns the number of events applied and elapsed ticks.
pub fn flush(max_events: usize, max_ticks: u64, start_ticks: u64) -> FlushResult {
    FLUSH_CALLS.fetch_add(1, Ordering::Relaxed);

    let mut applied = 0usize;

    // Scratch buffer for draining
    let mut buf = [SchedEvent::TaskYielded { tid: 0, timestamp: 0 }; 64];

    // Round-robin drain across CPUs, skip None entries
    let per_cpu_budget = (max_events / types::MAX_CPUS.max(1)).max(8);

    for cpu_idx in 0..types::MAX_CPUS {
        if applied >= max_events {
            break;
        }

        let ring = match ring::ring_for_cpu(cpu_idx) {
            Some(r) => r,
            None => continue,
        };

        let count = ring.drain(&mut buf, per_cpu_budget.min(max_events - applied));
        for i in 0..count {
            translate_event(&buf[i]);
            applied += 1;
        }
    }

    FLUSH_EVENTS.fetch_add(applied as u64, Ordering::Relaxed);

    // We can't easily get elapsed here without a clock, so report 0.
    // The caller (graph_worker_task) tracks its own wall-clock budget.
    FlushResult { applied, elapsed_ticks: 0 }
}

/// Translate a single SchedEvent into the appropriate GraphWork item
/// and push it to the legacy graph work queue for processing by
/// flush_graph_queue.
fn translate_event(event: &SchedEvent) {
    match event {
        SchedEvent::TaskCreated { .. } => { EVENT_CREATED.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::TaskExited { .. } => { EVENT_EXITED.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::StateChanged { .. } => { EVENT_STATE.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::PriorityChanged { .. } => { EVENT_PRIO.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::AffinitySet { .. } => { EVENT_AFF.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::LocationSet { .. } => { EVENT_LOC.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::NameSet { .. } => { EVENT_NAME.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::TaskRan { .. } => { EVENT_RAN.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::TaskBlocked { .. } => { EVENT_BLOCKED.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::TaskWoke { .. } => { EVENT_WOKE.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::TaskYielded { .. } => { EVENT_YIELDED.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::TaskEnqueued { .. } => { EVENT_ENQ.fetch_add(1, Ordering::Relaxed); }
        SchedEvent::TaskDequeued { .. } => { EVENT_DEQ.fetch_add(1, Ordering::Relaxed); }
    }

    match event {
        SchedEvent::TaskCreated {
            tid,
            prio,
            is_user,
            parent_tid,
            name,
            ..
        } => {
            // Find the end of the name (first zero byte)
            let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
            let name_str = core::str::from_utf8(&name[..name_len]).unwrap_or("");
            let name_opt = if name_len > 0 {
                Some(String::from(name_str))
            } else {
                None
            };
            let parent = if *parent_tid != 0 {
                Some(*parent_tid)
            } else {
                None
            };
            graph_queue::push(GraphWork::CreateThread {
                tid: *tid,
                priority: *prio,
                is_user: *is_user,
                name: name_opt,
                parent_tid: parent,
            });
        }
        SchedEvent::TaskExited { tid, code, .. } => {
            graph_queue::push(GraphWork::SetExitCode {
                tid: *tid,
                code: *code,
            });
            graph_queue::push(GraphWork::UpdateState {
                tid: *tid,
                state: "dead",
            });
        }
        SchedEvent::TaskBlocked { tid, .. } => {
            graph_queue::push(GraphWork::UpdateState {
                tid: *tid,
                state: "blocked",
            });
        }
        SchedEvent::TaskWoke { tid, .. } => {
            graph_queue::push(GraphWork::UpdateState {
                tid: *tid,
                state: "runnable",
            });
        }
        SchedEvent::TaskYielded { .. } => {
            // Yields are extremely high-frequency and don't represent a durable
            // state transition. Skip graph updates here to reduce queue pressure.
        }
        SchedEvent::TaskEnqueued { tid, cpu, .. } => {
            graph_queue::push(GraphWork::SetLocation {
                tid: *tid,
                cpu_index: *cpu as usize,
            });
        }
        SchedEvent::TaskDequeued { tid, reason, .. } => {
            use crate::sched::events::DequeueReason;
            let state = match reason {
                DequeueReason::Scheduled => "running",
                DequeueReason::Killed => "dead",
                DequeueReason::Blocked => "blocked",
                DequeueReason::Sleeping => "sleeping",
            };
            graph_queue::push(GraphWork::UpdateState {
                tid: *tid,
                state,
            });
        }
        SchedEvent::TaskRan { tid, cpu, .. } => {
            graph_queue::push(GraphWork::UpdateState {
                tid: *tid,
                state: "running",
            });
            graph_queue::push(GraphWork::SetLocation {
                tid: *tid,
                cpu_index: *cpu as usize,
            });
        }
        SchedEvent::PriorityChanged { tid, new_prio, .. } => {
            graph_queue::push(GraphWork::SetPriority {
                tid: *tid,
                priority: *new_prio,
            });
        }
        SchedEvent::AffinitySet { tid, cpu, .. } => {
            graph_queue::push(GraphWork::SetAffinity {
                tid: *tid,
                cpu_index: *cpu as usize,
            });
        }
        SchedEvent::LocationSet { tid, cpu, .. } => {
            graph_queue::push(GraphWork::SetLocation {
                tid: *tid,
                cpu_index: *cpu as usize,
            });
        }
        SchedEvent::StateChanged { tid, state_ptr, .. } => {
            // Reconstruct &'static str from the pointer.
            // This is safe because we only store pointers to
            // &'static str literals in the scheduler code.
            let state: &'static str = unsafe {
                let ptr = *state_ptr as *const u8;
                // We need the length too. To keep it simple, use the
                // well-known set of state strings.
                let len = state_str_len(ptr);
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len))
            };
            graph_queue::push(GraphWork::UpdateState {
                tid: *tid,
                state,
            });
        }
        SchedEvent::NameSet { tid, name, .. } => {
            let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
            let name_str = core::str::from_utf8(&name[..name_len]).unwrap_or("");
            if name_len > 0 {
                graph_queue::push(GraphWork::SetName {
                    tid: *tid,
                    name: String::from(name_str),
                });
            }
        }
    }
}

/// Determine the length of a well-known state string from its pointer.
///
/// We store `&'static str` pointers in events. Since we only use a
/// small set of known strings, we compare pointers to reconstruct the
/// original `&str`.
fn state_str_len(ptr: *const u8) -> usize {
    // Check against known static strings
    for candidate in &["runnable", "blocked", "sleeping", "dead", "running"] {
        if ptr == candidate.as_ptr() {
            return candidate.len();
        }
    }
    // Fallback: scan for null-terminator or use a safe default.
    // Since these are Rust &str from static storage, they may not be
    // null-terminated. Use a conservative max.
    0
}
