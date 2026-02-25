use crate::BootRuntime;

use crate::task::{flusher, graph_queue, graphify};

// Flusher batch size
const FLUSH_BATCH_SIZE: usize = 64;

/// Background task that runs periodically to drain the lock-free event rings
/// and execute synchronous graph operations.
pub extern "C" fn graph_worker_task<R: BootRuntime>(_: usize) -> ! {
    crate::kinfo!("SCHED: Graph worker task started");
    let rt = crate::runtime::<R>();

    loop {
        // Track time taken to enforce a budget (don't starve other tasks)
        let start = rt.mono_ticks();
        let budget_ticks = rt.mono_freq_hz() / 100; // 10ms budget

        let result = flusher::flush(FLUSH_BATCH_SIZE, budget_ticks, start);

        flush_graph_queue::<R>();
        maybe_log_profile::<R>();

        let actual_ticks = rt.mono_ticks().saturating_sub(start);

        // Calculate sleep time based on whether we hit the batch limit or budget
        let yield_time = if result.applied >= FLUSH_BATCH_SIZE || actual_ticks > budget_ticks {
            // We have more work, yield briefly to avoid hogging CPU
            // (1ms or next tick)
            1
        } else {
            // Ring was drained or mostly empty, sleep for a while (10ms)
            10
        };

        crate::task::yield_now::<R>(); // just let the scheduler run
        crate::sched::sleep_ms::<R>(yield_time);
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
fn update_max_u64(slot: &AtomicU64, val: u64) {
    let mut prev = slot.load(Ordering::Relaxed);
    while val > prev {
        match slot.compare_exchange_weak(prev, val, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => prev = actual,
        }
    }
}

#[inline]
fn ticks_to_us<R: BootRuntime>(ticks: u64) -> u64 {
    let rt = crate::runtime::<R>();
    let freq = rt.mono_freq_hz().max(1);
    ticks.saturating_mul(1_000_000) / freq
}

/// Process pending graph work items.
fn flush_graph_queue<R: BootRuntime>() {
    use crate::root::graph_anchors;
    use crate::sched::types;
    use crate::task::graph_queue::GraphWork;

    let rt = crate::runtime::<R>();
    let t0 = rt.mono_ticks();

    let budget_ticks = 2 * rt.mono_freq_hz() / 1000;

    let sched_thing = match graph_anchors::scheduler_service() {
        Some(id) => id,
        None => return,
    };

    let work_items = graph_queue::drain_n(32);
    if work_items.is_empty() { return; }
    let item_count = work_items.len() as u64;

    let mut batch_items = alloc::vec::Vec::with_capacity(work_items.len());

    for item in work_items {
        if rt.mono_ticks().wrapping_sub(t0) > budget_ticks { break; }

        match item {
            GraphWork::CreateThread { tid, priority, is_user, name, parent_tid } => {
                if let Some(thing_id) = graphify::do_create_thread_node(
                    tid, priority, is_user, name.as_deref(), sched_thing,
                ) {
                    types::set_graph_thing_for_tid(tid, thing_id);
                    let parent_thing = parent_tid.and_then(|ptid| types::graph_thing_for_tid(ptid));
                    if let Some(parent_thing) = parent_thing {
                        graphify::do_link_parent(thing_id, parent_thing, sched_thing);
                    }
                }
            }
            _ => {
                let tid = match &item {
                    GraphWork::UpdateState { tid, .. } => *tid,
                    GraphWork::SetExitCode { tid, .. } => *tid,
                    GraphWork::SetPriority { tid, .. } => *tid,
                    GraphWork::SetName { tid, .. } => *tid,
                    GraphWork::SetLocation { tid, .. } => *tid,
                    GraphWork::SetAffinity { tid, .. } => *tid,
                    _ => unreachable!(),
                };
                if let Some(id) = types::graph_thing_for_tid(tid) {
                    batch_items.push((id, item));
                }
            }
        }
    }

    if !batch_items.is_empty() {
        graphify::do_flush_batch(&batch_items);
    }

    let elapsed_us = ticks_to_us::<R>(rt.mono_ticks().wrapping_sub(t0));
    PROF_GRAPH_FLUSH_CALLS.fetch_add(1, Ordering::Relaxed);
    PROF_GRAPH_FLUSH_ITEMS.fetch_add(item_count, Ordering::Relaxed);
    PROF_GRAPH_FLUSH_US_TOTAL.fetch_add(elapsed_us, Ordering::Relaxed);
    update_max_u64(&PROF_GRAPH_FLUSH_MAX_US, elapsed_us);
    if elapsed_us >= 5_000 {
        PROF_GRAPH_FLUSH_SLOW.fetch_add(1, Ordering::Relaxed);
    }
}

fn maybe_log_profile<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let now = rt.mono_ticks();
    let period = rt.mono_freq_hz().max(1) * 2;
    let last = PROF_LAST_LOG_TICKS.load(Ordering::Relaxed);
    if last != 0 && now.wrapping_sub(last) < period { return; }
    if PROF_LAST_LOG_TICKS.compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed).is_err() { return; }

    let calls = PROF_GRAPH_FLUSH_CALLS.swap(0, Ordering::Relaxed);
    let items = PROF_GRAPH_FLUSH_ITEMS.swap(0, Ordering::Relaxed);
    let total_us = PROF_GRAPH_FLUSH_US_TOTAL.swap(0, Ordering::Relaxed);
    let slow = PROF_GRAPH_FLUSH_SLOW.swap(0, Ordering::Relaxed);
    let max_us = PROF_GRAPH_FLUSH_MAX_US.swap(0, Ordering::Relaxed);
    let trylock_miss = crate::sched::PROF_RESCHED_TRYLOCK_MISS.swap(0, Ordering::Relaxed);

    if trylock_miss >= crate::sched::TRYLOCK_MISS_WARN_THRESHOLD {
        crate::kwarn!("SCHED: scheduler lock held too long — {} trylock misses", trylock_miss);
    }
    let q = graph_queue::stats_snapshot();
    let avg_us = if calls > 0 { total_us / calls } else { 0 };

    let ipi_tx = crate::sched::DIAG_IPI_SENT.swap(0, Ordering::Relaxed);
    let ipi_rx = crate::sched::DIAG_IPI_HANDLER.swap(0, Ordering::Relaxed);
    let hlt_w = crate::sched::DIAG_HLT_WAKE.swap(0, Ordering::Relaxed);
    let rm = crate::sched::ring::aggregate_metrics();
    let fm = flusher::metrics_snapshot_and_reset();
    
    crate::kinfo!(
        "PROF: sched 2s: graph_flush calls={} items={} avg_us={} max_us={} slow={} trylock_miss={} qlen={} q_hwm={} q_drop_state={} q_evict={} ipi_tx={} ipi_rx={} hlt_wake={} ring_push={} ring_drop={} ring_pend={} flush_ev={} flush_tmax={}",
        calls, items, avg_us, max_us, slow, trylock_miss,
        q.current_len, q.high_water_mark, q.dropped_update_state, q.evicted_critical,
        ipi_tx, ipi_rx, hlt_w,
        rm.total_pushed, rm.total_dropped, rm.total_pending,
        fm.flush_events, fm.flush_ticks_max
    );
}
