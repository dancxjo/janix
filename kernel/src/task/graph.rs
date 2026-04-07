//! # LEGACY: Graph worker tasks for scheduler observability.
//!
//! This module is part of the legacy graph-spine observability layer.
//! It contains the background tasks that drain the per-CPU event rings
//! and flush graph work items to the Root service.
//!
//! **These tasks are optional** — the scheduler runs correctly without them.
//! They are spawned by `task::init_graph_workers()`, which is called after
//! the Root service is initialized.
//!
//! See `janix ACT I: Sever the Graph Spine` for context.

use crate::BootRuntime;

use crate::task::{flusher, graph_queue, graphify};

// Flusher batch size
const FLUSH_BATCH_SIZE: usize = 2048;

pub extern "C" fn ring_drain_task<R: BootRuntime>(_: usize) -> ! {
    let rt = crate::runtime::<R>();
    loop {
        let start = rt.mono_ticks();
        let budget_ticks = rt.mono_freq_hz() / 100; // 10ms budget
        let result = flusher::flush(FLUSH_BATCH_SIZE, budget_ticks, start);
        let actual_ticks = rt.mono_ticks().saturating_sub(start);

        let sleep_time = if result.applied >= FLUSH_BATCH_SIZE || actual_ticks > budget_ticks {
            0
        } else {
            10
        };

        crate::task::yield_now::<R>();
        if sleep_time > 0 {
            crate::sched::sleep_ms::<R>(sleep_time);
        }
    }
}

pub extern "C" fn graph_worker_task<R: BootRuntime>(_: usize) -> ! {
    crate::kinfo!("SCHED: Graph worker task started");
    loop {
        let items_processed = flush_graph_queue::<R>();
        maybe_log_profile::<R>();

        let sleep_time = if items_processed >= 32 { 0 } else { 10 };

        crate::task::yield_now::<R>();
        if sleep_time > 0 {
            crate::sched::sleep_ms::<R>(sleep_time);
        }
    }
}

use core::sync::atomic::{AtomicU64, Ordering};

static PROF_GRAPH_FLUSH_CALLS: AtomicU64 = AtomicU64::new(0);
static PROF_GRAPH_FLUSH_ITEMS: AtomicU64 = AtomicU64::new(0);
static PROF_GRAPH_FLUSH_US_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROF_GRAPH_FLUSH_SLOW: AtomicU64 = AtomicU64::new(0);
static PROF_GRAPH_FLUSH_MAX_US: AtomicU64 = AtomicU64::new(0);
static PROF_LAST_LOG_TICKS: AtomicU64 = AtomicU64::new(0);

#[inline]
pub(crate) fn update_max_u64(slot: &AtomicU64, val: u64) {
    let mut prev = slot.load(Ordering::Relaxed);
    while val > prev {
        match slot.compare_exchange_weak(prev, val, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => prev = actual,
        }
    }
}

#[inline]
pub(crate) fn ticks_to_us<R: BootRuntime>(ticks: u64) -> u64 {
    let rt = crate::runtime::<R>();
    let freq = rt.mono_freq_hz().max(1);
    ticks.saturating_mul(1_000_000) / freq
}

fn flush_graph_queue<R: BootRuntime>() -> usize {
    use crate::root::graph_anchors;
    use crate::task::graph_queue::GraphWork;

    let rt = crate::runtime::<R>();
    let t0 = rt.mono_ticks();

    let budget_ticks = 2 * rt.mono_freq_hz() / 1000;

    let sched_thing = match graph_anchors::scheduler_service() {
        Some(id) => id,
        None => return 0,
    };

    let work_items = graph_queue::drain_n(32);
    if work_items.is_empty() {
        return 0;
    }
    let item_count = work_items.len() as u64;

    let mut batch_items = alloc::vec::Vec::with_capacity(work_items.len());

    for item in work_items {
        if rt.mono_ticks().wrapping_sub(t0) > budget_ticks {
            break;
        }

        match item {
            GraphWork::CreateThread {
                tid,
                priority,
                is_user,
                name,
                parent_tid,
                spawn_arg,
            } => {
                if let Some(thing_id) = graphify::do_create_thread_node::<R>(
                    tid,
                    priority,
                    is_user,
                    name.as_deref(),
                    sched_thing,
                    spawn_arg,
                ) {
                    graphify::set_graph_thing_for_tid(tid, thing_id);
                    let parent_thing = parent_tid.and_then(|ptid| graphify::graph_thing_for_tid(ptid));
                    if let Some(parent_thing) = parent_thing {
                        graphify::do_link_parent::<R>(thing_id, parent_thing, sched_thing);
                    }
                }
            }
            GraphWork::CleanupThings { thing_id } => {
                // Fire-and-forget: ask Root to orphan/cleanup nodes owned by this task.
                crate::root::enqueue(crate::root::RootOp::CleanupTaskThings {
                    owner_thing_id: thing_id,
                });
            }
            _ => {
                let tid = match &item {
                    GraphWork::UpdateState { tid, .. } => *tid,
                    GraphWork::SetExitCode { tid, .. } => {
                        // Remove the tid→thing_id mapping from the graph layer
                        // so future lookups return None.  Push a separate
                        // CleanupThings item so Root can orphan the owned nodes;
                        // that item will be processed in a future flush iteration
                        // (not this one, since we push to the back of the queue).
                        let thing_id = graphify::remove_graph_thing_for_tid(*tid);
                        if let Some(tid_val) = thing_id {
                            graph_queue::push(GraphWork::CleanupThings { thing_id: tid_val });
                        }
                        *tid
                    }
                    GraphWork::SetPriority { tid, .. } => *tid,
                    GraphWork::SetName { tid, .. } => *tid,
                    GraphWork::SetLocation { tid, .. } => *tid,
                    GraphWork::SetAffinity { tid, .. } => *tid,
                    _ => unreachable!(),
                };
                if let Some(id) = graphify::graph_thing_for_tid(tid) {
                    batch_items.push((id, item));
                }
            }
        }
    }

    if !batch_items.is_empty() {
        graphify::do_flush_batch::<R>(&batch_items);
    }

    let elapsed_us = ticks_to_us::<R>(rt.mono_ticks().wrapping_sub(t0));
    PROF_GRAPH_FLUSH_CALLS.fetch_add(1, Ordering::Relaxed);
    PROF_GRAPH_FLUSH_ITEMS.fetch_add(item_count, Ordering::Relaxed);
    PROF_GRAPH_FLUSH_US_TOTAL.fetch_add(elapsed_us, Ordering::Relaxed);
    update_max_u64(&PROF_GRAPH_FLUSH_MAX_US, elapsed_us);
    if elapsed_us >= 5_000 {
        PROF_GRAPH_FLUSH_SLOW.fetch_add(1, Ordering::Relaxed);
    }

    item_count as usize
}

fn maybe_log_profile<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let now = rt.mono_ticks();
    let period = rt.mono_freq_hz().max(1) * 2;
    let last = PROF_LAST_LOG_TICKS.load(Ordering::Relaxed);
    if last != 0 && now.wrapping_sub(last) < period {
        return;
    }
    if PROF_LAST_LOG_TICKS
        .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        return;
    }

    let calls = PROF_GRAPH_FLUSH_CALLS.swap(0, Ordering::Relaxed);
    let items = PROF_GRAPH_FLUSH_ITEMS.swap(0, Ordering::Relaxed);
    let total_us = PROF_GRAPH_FLUSH_US_TOTAL.swap(0, Ordering::Relaxed);
    let slow = PROF_GRAPH_FLUSH_SLOW.swap(0, Ordering::Relaxed);
    let max_us = PROF_GRAPH_FLUSH_MAX_US.swap(0, Ordering::Relaxed);
    let trylock_miss = crate::sched::PROF_RESCHED_TRYLOCK_MISS.swap(0, Ordering::Relaxed);

    if trylock_miss >= crate::sched::TRYLOCK_MISS_WARN_THRESHOLD {
        crate::kwarn!(
            "SCHED: scheduler lock held too long — {} trylock misses",
            trylock_miss
        );
    }
    let q = graph_queue::stats_snapshot();
    let avg_us = if calls > 0 { total_us / calls } else { 0 };

    let ipi_tx = crate::sched::DIAG_IPI_SENT.swap(0, Ordering::Relaxed);
    let ipi_rx = crate::sched::DIAG_IPI_HANDLER.swap(0, Ordering::Relaxed);
    let hlt_w = crate::sched::DIAG_HLT_WAKE.swap(0, Ordering::Relaxed);
    let rm = crate::sched::ring::aggregate_metrics();
    let fm = flusher::metrics_snapshot_and_reset();
    let slm = crate::sched::sched_lock_metrics_snapshot_and_reset();

    let wait_blocks = graphify::WAIT_FOR_REPLY_BLOCKS.swap(0, Ordering::Relaxed);
    let wait_wakes = graphify::WAIT_FOR_REPLY_WAKES.swap(0, Ordering::Relaxed);
    let wait_calls = graphify::WAIT_FOR_REPLY_CALLS.swap(0, Ordering::Relaxed);
    let wait_us = graphify::WAIT_FOR_REPLY_US_TOTAL.swap(0, Ordering::Relaxed);
    let wait_max = graphify::WAIT_FOR_REPLY_US_MAX.swap(0, Ordering::Relaxed);
    let no_reply_batches = graphify::NO_REPLY_BATCHES_SENT.swap(0, Ordering::Relaxed);

    let wait_avg = if wait_calls > 0 {
        wait_us / wait_calls
    } else {
        0
    };
    let bc_avg = if slm.block_current.hold_calls > 0 {
        slm.block_current.hold_us_total / slm.block_current.hold_calls
    } else {
        0
    };
    let wt_avg = if slm.wake_task.hold_calls > 0 {
        slm.wake_task.hold_us_total / slm.wake_task.hold_calls
    } else {
        0
    };
    let yn_avg = if slm.yield_now.hold_calls > 0 {
        slm.yield_now.hold_us_total / slm.yield_now.hold_calls
    } else {
        0
    };
    let st_avg = if slm.sleep_ticks.hold_calls > 0 {
        slm.sleep_ticks.hold_us_total / slm.sleep_ticks.hold_calls
    } else {
        0
    };
    let ws_avg = if slm.wake_sleepers.hold_calls > 0 {
        slm.wake_sleepers.hold_us_total / slm.wake_sleepers.hold_calls
    } else {
        0
    };

    crate::kinfo!(
        "PROF: sched 2s: graph_flush calls={} items={} avg_us={} max_us={} slow={} wait=[{}us avg, {}us max] trylock_miss={} waits={} wakes={} no_replies={} qlen={} q_hwm={} q_drop_props={} q_evict={} ipi_tx={} ipi_rx={} hlt_wake={} ring_push={} ring_drop={} ring_pend={} flush_ev={} flush_tmax={}",
        calls,
        items,
        avg_us,
        max_us,
        slow,
        wait_avg,
        wait_max,
        trylock_miss,
        wait_blocks,
        wait_wakes,
        no_reply_batches,
        q.current_len,
        q.high_water_mark,
        q.dropped_non_critical,
        q.evicted_critical,
        ipi_tx,
        ipi_rx,
        hlt_w,
        rm.total_pushed,
        rm.total_dropped,
        rm.total_pending,
        fm.flush_events,
        fm.flush_ticks_max
    );
    crate::kinfo!(
        "PROF: sched_lock 2s: bc={}@{}us/{} wt={}@{}us/{} yn={}@{}us/{} st={}@{}us/{} ws={}@{}us/{}",
        slm.block_current.hold_calls,
        bc_avg,
        slm.block_current.hold_us_max,
        slm.wake_task.hold_calls,
        wt_avg,
        slm.wake_task.hold_us_max,
        slm.yield_now.hold_calls,
        yn_avg,
        slm.yield_now.hold_us_max,
        slm.sleep_ticks.hold_calls,
        st_avg,
        slm.sleep_ticks.hold_us_max,
        slm.wake_sleepers.hold_calls,
        ws_avg,
        slm.wake_sleepers.hold_us_max
    );
}
