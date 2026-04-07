//! Lock-free single-producer / single-consumer ring buffer for scheduler events.
//!
//! The producer (scheduler tick path) pushes events without locking.
//! The consumer (graph flusher) drains events in bounded batches.
//!
//! Overflow policy: drop the event and increment a counter. Never block.

use crate::sched::events::{BlockReason, DequeueReason, SchedEvent, WakeSource};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Power-of-two capacity for the ring buffer.
/// 256 events ≈ 12-16 KiB depending on SchedEvent size.
pub const RING_CAPACITY: usize = 256;

/// Compile-time assertion that capacity is a power of two.
const _: () = assert!(
    RING_CAPACITY.is_power_of_two(),
    "RING_CAPACITY must be a power of two"
);

/// Mask for wrapping indices.
const RING_MASK: u32 = (RING_CAPACITY - 1) as u32;

/// A fixed-size, single-producer / single-consumer ring buffer.
///
/// - `head`: next write position (only modified by producer)
/// - `tail`: next read position (only modified by consumer)
/// - Buffer is full when `head - tail == RING_CAPACITY`
/// - Buffer is empty when `head == tail`
pub struct EventRing {
    /// Ring buffer storage. Indices are `head % RING_CAPACITY` and
    /// `tail % RING_CAPACITY`.
    buf: [MaybeUninit<SchedEvent>; RING_CAPACITY],
    /// Next write position (producer-owned, consumer reads)
    head: AtomicU32,
    /// Next read position (consumer-owned, producer reads)
    tail: AtomicU32,
    /// Total events successfully pushed
    events_pushed: AtomicU64,
    /// Total events dropped due to full ring
    events_dropped: AtomicU64,
}

impl EventRing {
    /// Create a new empty ring buffer.
    pub const fn new() -> Self {
        // SAFETY: MaybeUninit does not require initialization
        Self {
            buf: unsafe { MaybeUninit::uninit().assume_init() },
            head: AtomicU32::new(0),
            tail: AtomicU32::new(0),
            events_pushed: AtomicU64::new(0),
            events_dropped: AtomicU64::new(0),
        }
    }

    /// Push an event into the ring buffer.
    ///
    /// Returns `true` if the event was stored, `false` if the ring was
    /// full and the event was dropped.
    ///
    /// # Safety contract
    /// Must only be called from the single producer (scheduler on this CPU).
    #[inline]
    pub fn push(&self, event: SchedEvent) -> bool {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        if head.wrapping_sub(tail) >= RING_CAPACITY as u32 {
            // Ring is full — drop the event
            self.events_dropped.fetch_add(1, Ordering::Relaxed);
            return false;
        }

        let idx = (head & RING_MASK) as usize;
        // SAFETY: idx is always < RING_CAPACITY. We are the sole producer,
        // and this slot is not readable by the consumer until we advance head.
        unsafe {
            let slot =
                &self.buf[idx] as *const MaybeUninit<SchedEvent> as *mut MaybeUninit<SchedEvent>;
            (*slot).write(event);
        }

        // Make the write visible to the consumer
        self.head.store(head.wrapping_add(1), Ordering::Release);
        self.events_pushed.fetch_add(1, Ordering::Relaxed);
        true
    }

    /// Pop a single event from the ring buffer.
    ///
    /// Returns `None` if the ring is empty.
    ///
    /// # Safety contract
    /// Must only be called from the single consumer (flusher task).
    #[inline]
    pub fn pop(&self) -> Option<SchedEvent> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        if tail == head {
            return None;
        }

        let idx = (tail & RING_MASK) as usize;
        // SAFETY: idx < RING_CAPACITY, and the producer has written this slot
        // (head > tail means this slot is populated).
        let event = unsafe { self.buf[idx].assume_init_read() };

        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Some(event)
    }

    /// Drain up to `max` events into the provided buffer slice.
    ///
    /// Returns the number of events drained.
    ///
    /// # Safety contract
    /// Must only be called from the single consumer (flusher task).
    pub fn drain(&self, buf: &mut [SchedEvent], max: usize) -> usize {
        let limit = max.min(buf.len());
        let mut count = 0;

        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        let available = head.wrapping_sub(tail) as usize;
        let to_read = available.min(limit);

        for i in 0..to_read {
            let idx = (tail.wrapping_add(i as u32) & RING_MASK) as usize;
            // SAFETY: these slots are populated (head > tail + i)
            buf[i] = unsafe { self.buf[idx].assume_init_read() };
            count += 1;
        }

        if count > 0 {
            self.tail
                .store(tail.wrapping_add(count as u32), Ordering::Release);
        }

        count
    }

    /// Number of events currently in the ring.
    #[inline]
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        head.wrapping_sub(tail) as usize
    }

    /// Whether the ring is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Total events successfully pushed since creation.
    #[inline]
    pub fn events_pushed(&self) -> u64 {
        self.events_pushed.load(Ordering::Relaxed)
    }

    /// Total events dropped due to a full ring since creation.
    #[inline]
    pub fn events_dropped(&self) -> u64 {
        self.events_dropped.load(Ordering::Relaxed)
    }
}

// SAFETY: EventRing is designed for single-producer / single-consumer use.
// The atomics ensure proper memory ordering between the two threads.
unsafe impl Sync for EventRing {}
unsafe impl Send for EventRing {}

// ============================================================================
// Per-CPU ring access
// ============================================================================

use core::sync::atomic::AtomicPtr;

/// Per-CPU event ring storage. Initialized during `scheduler::init()`.
/// Each entry is an `AtomicPtr<EventRing>`.
static EVENT_RINGS: [AtomicPtr<EventRing>; super::types::MAX_CPUS] = {
    // Array init workaround for non-Copy types in statics:
    const PTR: AtomicPtr<EventRing> = AtomicPtr::new(core::ptr::null_mut());
    [PTR; super::types::MAX_CPUS]
};

/// Initialize the event ring for a CPU. Allocates a ring on the heap
/// and leaks it to get a `'static` reference.
pub fn init_ring(cpu_index: usize) {
    let ring = alloc::boxed::Box::new(EventRing::new());
    let ring_ptr = alloc::boxed::Box::into_raw(ring);
    EVENT_RINGS[cpu_index].store(ring_ptr, Ordering::Release);
}

/// Get the event ring for a CPU, if initialized.
#[inline]
pub fn ring_for_cpu(cpu_index: usize) -> Option<&'static EventRing> {
    let ptr = EVENT_RINGS[cpu_index].load(Ordering::Acquire);
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &*ptr })
    }
}

/// Push an event to the current CPU's ring buffer.
/// Returns `false` if the ring is full or not initialized.
#[inline]
pub fn push_event(cpu_index: usize, event: SchedEvent) -> bool {
    if let Some(ring) = ring_for_cpu(cpu_index) {
        ring.push(event)
    } else {
        false
    }
}

/// Snapshot of ring metrics across all CPUs.
#[derive(Clone, Copy, Debug, Default)]
pub struct RingMetrics {
    pub total_pushed: u64,
    pub total_dropped: u64,
    pub total_pending: usize,
}

/// Collect aggregate metrics across all CPU rings.
pub fn aggregate_metrics() -> RingMetrics {
    let mut m = RingMetrics::default();
    for i in 0..super::types::MAX_CPUS {
        if let Some(ring) = ring_for_cpu(i) {
            m.total_pushed += ring.events_pushed();
            m.total_dropped += ring.events_dropped();
            m.total_pending += ring.len();
        }
    }
    m
}

pub fn push_task_state<R: crate::BootRuntime>(tid: u64, state: &'static str) {
    let cpu = crate::sched::current_cpu_index::<R>();
    push_event(
        cpu,
        crate::sched::events::SchedEvent::StateChanged {
            tid,
            state_ptr: state.as_ptr() as u64,
            timestamp: crate::runtime::<R>().mono_ticks(),
        },
    );
}

pub fn push_task_exited<R: crate::BootRuntime>(tid: u64, code: i32) {
    let cpu = crate::sched::current_cpu_index::<R>();
    push_event(
        cpu,
        crate::sched::events::SchedEvent::TaskExited {
            tid,
            code,
            timestamp: crate::runtime::<R>().mono_ticks(),
        },
    );
}

pub fn push_task_priority<R: crate::BootRuntime>(tid: u64, priority: u8) {
    let cpu = crate::sched::current_cpu_index::<R>();
    push_event(
        cpu,
        crate::sched::events::SchedEvent::PriorityChanged {
            tid,
            old_prio: 0, // not really needed for graph update
            new_prio: priority,
            timestamp: crate::runtime::<R>().mono_ticks(),
        },
    );
}

pub fn push_task_affinity<R: crate::BootRuntime>(tid: u64, target_cpu: usize) {
    let cpu = crate::sched::current_cpu_index::<R>();
    push_event(
        cpu,
        crate::sched::events::SchedEvent::AffinitySet {
            tid,
            cpu: target_cpu as u16,
            timestamp: crate::runtime::<R>().mono_ticks(),
        },
    );
}

pub fn push_task_location<R: crate::BootRuntime>(tid: u64, target_cpu: usize) {
    let cpu = crate::sched::current_cpu_index::<R>();
    push_event(
        cpu,
        crate::sched::events::SchedEvent::LocationSet {
            tid,
            cpu: target_cpu as u16,
            timestamp: crate::runtime::<R>().mono_ticks(),
        },
    );
}

pub fn push_task_created<R: crate::BootRuntime>(
    tid: u64,
    priority: u8,
    is_user: bool,
    name: Option<&str>,
    parent_tid: Option<u64>,
    spawn_arg: u64,
) {
    let cpu = crate::sched::current_cpu_index::<R>();
    push_event(
        cpu,
        crate::sched::events::SchedEvent::TaskCreated {
            tid,
            prio: priority,
            is_user,
            parent_tid: parent_tid.unwrap_or(0),
            name: crate::sched::events::pack_name(name),
            spawn_arg,
            timestamp: crate::runtime::<R>().mono_ticks(),
        },
    );
}

pub fn push_task_name<R: crate::BootRuntime>(tid: u64, name: Option<&str>) {
    let cpu = crate::sched::current_cpu_index::<R>();
    push_event(
        cpu,
        crate::sched::events::SchedEvent::NameSet {
            tid,
            name: crate::sched::events::pack_name(name),
            timestamp: crate::runtime::<R>().mono_ticks(),
        },
    );
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sched::events::*;

    fn make_event(tid: u64) -> SchedEvent {
        SchedEvent::TaskYielded { tid, timestamp: 0 }
    }

    #[test]
    fn test_push_pop_basic() {
        let ring = EventRing::new();
        assert!(ring.is_empty());
        assert_eq!(ring.len(), 0);

        assert!(ring.push(make_event(1)));
        assert!(ring.push(make_event(2)));
        assert!(ring.push(make_event(3)));
        assert_eq!(ring.len(), 3);

        // Pop in FIFO order
        if let Some(SchedEvent::TaskYielded { tid, .. }) = ring.pop() {
            assert_eq!(tid, 1);
        } else {
            panic!("expected TaskYielded");
        }
        if let Some(SchedEvent::TaskYielded { tid, .. }) = ring.pop() {
            assert_eq!(tid, 2);
        } else {
            panic!("expected TaskYielded");
        }
        if let Some(SchedEvent::TaskYielded { tid, .. }) = ring.pop() {
            assert_eq!(tid, 3);
        } else {
            panic!("expected TaskYielded");
        }
        assert!(ring.pop().is_none());
        assert!(ring.is_empty());
    }

    #[test]
    fn test_ring_full_drops() {
        let ring = EventRing::new();

        // Fill the ring
        for i in 0..RING_CAPACITY {
            assert!(ring.push(make_event(i as u64)));
        }
        assert_eq!(ring.len(), RING_CAPACITY);
        assert_eq!(ring.events_pushed(), RING_CAPACITY as u64);
        assert_eq!(ring.events_dropped(), 0);

        // One more should be dropped
        assert!(!ring.push(make_event(999)));
        assert_eq!(ring.events_dropped(), 1);
        assert_eq!(ring.events_pushed(), RING_CAPACITY as u64); // didn't increment
        assert_eq!(ring.len(), RING_CAPACITY); // still full
    }

    #[test]
    fn test_drain_partial() {
        let ring = EventRing::new();
        for i in 0..10 {
            ring.push(make_event(i));
        }
        assert_eq!(ring.len(), 10);

        let mut buf = [make_event(0); 5];
        let drained = ring.drain(&mut buf, 5);
        assert_eq!(drained, 5);
        assert_eq!(ring.len(), 5);

        // Verify order
        for (i, ev) in buf.iter().enumerate() {
            if let SchedEvent::TaskYielded { tid, .. } = ev {
                assert_eq!(*tid, i as u64);
            } else {
                panic!("expected TaskYielded");
            }
        }
    }

    #[test]
    fn test_empty_pop() {
        let ring = EventRing::new();
        assert!(ring.pop().is_none());
        assert_eq!(ring.events_pushed(), 0);
        assert_eq!(ring.events_dropped(), 0);
    }

    #[test]
    fn test_wrap_around() {
        let ring = EventRing::new();

        // Fill and drain multiple times to exercise wrapping
        for round in 0..3u64 {
            for i in 0..RING_CAPACITY {
                assert!(ring.push(make_event(round * 1000 + i as u64)));
            }
            assert_eq!(ring.len(), RING_CAPACITY);

            for i in 0..RING_CAPACITY {
                if let Some(SchedEvent::TaskYielded { tid, .. }) = ring.pop() {
                    assert_eq!(tid, round * 1000 + i as u64);
                } else {
                    panic!("expected TaskYielded at round={} i={}", round, i);
                }
            }
            assert!(ring.is_empty());
        }

        assert_eq!(ring.events_pushed(), (RING_CAPACITY * 3) as u64);
        assert_eq!(ring.events_dropped(), 0);
    }

    #[test]
    fn test_drain_more_than_available() {
        let ring = EventRing::new();
        ring.push(make_event(1));
        ring.push(make_event(2));

        let mut buf = [make_event(0); 10];
        let drained = ring.drain(&mut buf, 10);
        assert_eq!(drained, 2);
        assert!(ring.is_empty());
    }
}
