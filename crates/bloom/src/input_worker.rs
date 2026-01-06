//! Input worker thread - drains mouse ringbuffer at high frequency, keeping cursor live.
//!
//! The worker:
//! 1. Waits for config via mailbox (blocking)
//! 2. Drains the input ringbuffer in batches
//! 3. Publishes latest pointer state via lock-free atomics
//! 4. Uses 1ms sleep backoff when no data (NOT a tight loop)
//!
//! Main thread reads INPUT_STATE to render cursor independently of scene rebuilds.

use core::sync::atomic::{AtomicI32, AtomicU16, AtomicU32, Ordering};
use thing_std::{log_info, sched_yield};

use crate::mailbox::Mailbox;

/// Shared cursor state - updated by worker (single writer), read by main (many readers).
///
/// Protocol:
/// - Worker stores x/y/buttons with Relaxed
/// - Worker then increments seq with Release (publishes the batch)
/// - Main reads seq with Acquire to get a coherent snapshot
pub struct InputState {
    pub x: AtomicI32,
    pub y: AtomicI32,
    pub buttons: AtomicU16,
    pub seq: AtomicU32,
}

impl InputState {
    pub const fn new() -> Self {
        Self {
            x: AtomicI32::new(0),
            y: AtomicI32::new(0),
            buttons: AtomicU16::new(0),
            seq: AtomicU32::new(0),
        }
    }
    
    /// Initialize with screen center
    pub fn init(&self, screen_w: u32, screen_h: u32) {
        self.x.store((screen_w / 2) as i32, Ordering::Relaxed);
        self.y.store((screen_h / 2) as i32, Ordering::Relaxed);
        self.buttons.store(0, Ordering::Relaxed);
        self.seq.store(0, Ordering::Release);
    }
    
    /// Load current state (for main thread). Uses Acquire on seq for coherence.
    #[inline]
    pub fn load(&self) -> (i32, i32, u16, u32) {
        let seq = self.seq.load(Ordering::Acquire);
        let x = self.x.load(Ordering::Relaxed);
        let y = self.y.load(Ordering::Relaxed);
        let buttons = self.buttons.load(Ordering::Relaxed);
        (x, y, buttons, seq)
    }
    
    /// Publish new state (for worker thread). Uses Release on seq after storing data.
    #[inline]
    pub fn publish(&self, x: i32, y: i32, buttons: u16) {
        self.x.store(x, Ordering::Relaxed);
        self.y.store(y, Ordering::Relaxed);
        self.buttons.store(buttons, Ordering::Relaxed);
        // Release fence: ensures x/y/buttons are visible before seq increment
        self.seq.fetch_add(1, Ordering::Release);
    }
}

/// Global shared input state
pub static INPUT_STATE: InputState = InputState::new();

/// Diagnostics: set to 1 by worker at first instruction (bypasses logging)
pub static INPUT_WORKER_STARTED: AtomicU32 = AtomicU32::new(0);

/// Diagnostics: incremented each loop iteration by worker (proves worker is alive)
pub static INPUT_WORKER_TICKS: AtomicU32 = AtomicU32::new(0);

/// Config for input worker - passed via mailbox before spawn
pub struct InputWorkerConfig {
    pub ring_ptr: *const u8,
    pub capacity: u32,
    pub screen_w: u32,
    pub screen_h: u32,
}

// SAFETY: ring_ptr points to shared memory that outlives the worker
unsafe impl Send for InputWorkerConfig {}

/// Mailbox for config (main → worker, one-shot)
pub static INPUT_CONFIG_MBX: Mailbox<InputWorkerConfig> = Mailbox::new();

/// Drain the ringbuffer and return (x, y, buttons, events_drained).
/// This is the hot path - inline aggressively.
#[inline]
fn drain_ringbuffer(
    ring_ptr: *const u8,
    capacity: u32,
    read_seq: &mut u64,
    x: &mut i32,
    y: &mut i32,
    buttons: &mut u16,
    screen_w: u32,
    screen_h: u32,
) -> u32 {
    let mut drained = 0u32;
    
    unsafe {
        if ring_ptr.is_null() {
            return 0;
        }

        const HEADER_SIZE: usize = 48;
        const RECORD_HEADER_SIZE: usize = 24;
        const EV_POINTER_DELTA: u16 = 1;

        // Validate magic
        let magic = u32::from_le_bytes([
            *ring_ptr, *ring_ptr.add(1), *ring_ptr.add(2), *ring_ptr.add(3),
        ]);
        if magic != 0x544E5645 {
            return 0;
        }

        // Read capacity from header (offset 8)
        let header_capacity = u32::from_le_bytes([
            *ring_ptr.add(8), *ring_ptr.add(9), *ring_ptr.add(10), *ring_ptr.add(11),
        ]);
        let cap = core::cmp::min(header_capacity, capacity);
        
        let ring_base = ring_ptr.add(HEADER_SIZE);
        let mut offset: u32 = 0;
        let max_iters = cap / 32;

        for _ in 0..max_iters {
            if (offset + (HEADER_SIZE as u32) + (RECORD_HEADER_SIZE as u32)) > cap {
                break;
            }

            let rec_ptr = ring_base.add(offset as usize);

            let len = u16::from_le_bytes([*rec_ptr, *rec_ptr.add(1)]);
            if len == 0 || len < RECORD_HEADER_SIZE as u16 {
                break;
            }
            if (offset + (HEADER_SIZE as u32) + (len as u32)) > cap {
                break;
            }

            let kind = u16::from_le_bytes([*rec_ptr.add(2), *rec_ptr.add(3)]);

            let seq = u64::from_le_bytes([
                *rec_ptr.add(8), *rec_ptr.add(9), *rec_ptr.add(10), *rec_ptr.add(11),
                *rec_ptr.add(12), *rec_ptr.add(13), *rec_ptr.add(14), *rec_ptr.add(15),
            ]);

            if seq > *read_seq && kind == EV_POINTER_DELTA {
                let payload_ptr = rec_ptr.add(RECORD_HEADER_SIZE);
                let dx = i16::from_le_bytes([*payload_ptr, *payload_ptr.add(1)]);
                let dy = i16::from_le_bytes([*payload_ptr.add(2), *payload_ptr.add(3)]);
                let btn = u16::from_le_bytes([*payload_ptr.add(4), *payload_ptr.add(5)]);

                *x = (*x + (dx as i32)).clamp(0, screen_w as i32 - 1);
                *y = (*y + (dy as i32)).clamp(0, screen_h as i32 - 1);
                *buttons = btn;

                *read_seq = seq;
                drained += 1;
            }

            offset += ((len as u32) + 7) & !7; // 8-byte alignment
        }
    }
    
    drained
}

/// Simple sleep helper - yields CPU for approximately `ms` milliseconds.
/// Uses scheduler yield in a loop since we don't have a proper sleep syscall.
#[inline]
fn sleep_ms(ms: u32) {
    // Each yield is roughly 1-10ms depending on scheduler quantum
    // For 1ms target, a single yield is sufficient
    for _ in 0..ms {
        sched_yield();
    }
}

/// Input worker entry point
#[unsafe(no_mangle)]
pub extern "C" fn input_worker_entry(_arg: u64) -> ! {
    // FIRST INSTRUCTION: set started flag (bypasses any logging issues)
    INPUT_WORKER_STARTED.store(1, Ordering::Release);
    
    thing_std::debug::log("BLOOM: input worker thread entry");
    
    // Wait for config (blocking poll with yield)
    let config = loop {
        if let Some(cfg) = INPUT_CONFIG_MBX.try_take() {
            break cfg;
        }
        INPUT_WORKER_TICKS.fetch_add(1, Ordering::Relaxed); // prove we're alive while waiting
        sched_yield();
    };
    
    log_info(&alloc::format!(
        "BLOOM: input worker configured (ring ptr={:#x}, cap={}, screen={}x{})",
        config.ring_ptr as u64, config.capacity, config.screen_w, config.screen_h
    ));
    
    // Initialize shared state with screen center
    INPUT_STATE.init(config.screen_w, config.screen_h);
    
    // Local state for draining
    let mut read_seq: u64 = 0;
    let mut x = (config.screen_w / 2) as i32;
    let mut y = (config.screen_h / 2) as i32;
    let mut buttons: u16 = 0;
    
    // Stats (rate-limited logging)
    let mut total_drained: u64 = 0;
    let mut last_log_seq: u64 = 0;
    
    // Main drain loop with backoff
    loop {
        // Heartbeat: proves worker is alive
        INPUT_WORKER_TICKS.fetch_add(1, Ordering::Relaxed);
        
        let drained = drain_ringbuffer(
            config.ring_ptr,
            config.capacity,
            &mut read_seq,
            &mut x,
            &mut y,
            &mut buttons,
            config.screen_w,
            config.screen_h,
        );
        
        if drained > 0 {
            // Publish to shared state
            INPUT_STATE.publish(x, y, buttons);
            total_drained += drained as u64;
            
            // Immediately loop again to catch bursts (no sleep)
        } else {
            // No data - backoff with 1ms sleep to avoid CPU spin
            sleep_ms(1);
        }
        
        // Rate-limited stats logging (every ~1000 events)
        if total_drained >= last_log_seq + 1000 {
            log_info(&alloc::format!(
                "BLOOM: input worker drained {} events total, pos=({},{})",
                total_drained, x, y
            ));
            last_log_seq = total_drained;
        }
    }
}
