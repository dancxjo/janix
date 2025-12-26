use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::cell::UnsafeCell;

// Const size ring buffer. No alloc.
const RING_SIZE: usize = 1024;
pub const MSG_MAX: usize = 256;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum EntryKind {
    Log = 0,
    Error = 1,
    Fault = 2,
}

#[repr(C)]
pub struct RingEntry {
    pub ready: AtomicBool, // Write committed?
    pub kind: u8,
    pub level: u8,
    pub cpu: u16,
    pub timestamp_or_ticks: u64,
    pub payload_a: u64, // e.g. RIP
    pub payload_b: u64, // e.g. Error Code
    pub payload_c: u64, // e.g. CR2
    pub payload_d: u64, // e.g. RFlags
    pub msg_len: u16,
    pub msg_bytes: [u8; MSG_MAX],
}

impl Default for RingEntry {
    fn default() -> Self {
        Self {
            ready: AtomicBool::new(false),
            kind: 0,
            level: 0,
            cpu: 0,
            timestamp_or_ticks: 0,
            payload_a: 0,
            payload_b: 0,
            payload_c: 0,
            payload_d: 0,
            msg_len: 0,
            msg_bytes: [0; MSG_MAX],
        }
    }
}

pub struct LogRing {
    buffer: UnsafeCell<[RingEntry; RING_SIZE]>,
    head: AtomicUsize, // Write index
    tail: AtomicUsize, // Read index
}

unsafe impl Sync for LogRing {}

const INIT_ENTRY: RingEntry = RingEntry {
    ready: AtomicBool::new(false),
    kind: 0, level: 0, cpu: 0, timestamp_or_ticks: 0,
    payload_a: 0, payload_b: 0, payload_c: 0, payload_d: 0,
    msg_len: 0, msg_bytes: [0; MSG_MAX]
};

static GLOBAL_RING: LogRing = LogRing {
    buffer: UnsafeCell::new([INIT_ENTRY; RING_SIZE]),
    head: AtomicUsize::new(0),
    tail: AtomicUsize::new(0),
};

impl LogRing {
    pub fn global() -> &'static LogRing {
        &GLOBAL_RING
    }

    pub fn push(&self, kind: EntryKind, level: u8, msg: &str,
                a: u64, b: u64, c: u64, d: u64) {

        // Reserve slot
        let head = self.head.fetch_add(1, Ordering::Relaxed);
        let idx = head % RING_SIZE;

        let entry_ptr = unsafe {
            &mut (*self.buffer.get())[idx] as *mut RingEntry
        };

        unsafe {
            // Signal write in progress (if overwritting)
            (*entry_ptr).ready.store(false, Ordering::Relaxed);

            (*entry_ptr).kind = kind as u8;
            (*entry_ptr).level = level;
            (*entry_ptr).cpu = 0;
            (*entry_ptr).timestamp_or_ticks = 0;
            (*entry_ptr).payload_a = a;
            (*entry_ptr).payload_b = b;
            (*entry_ptr).payload_c = c;
            (*entry_ptr).payload_d = d;

            let bytes = msg.as_bytes();
            let len = core::cmp::min(bytes.len(), MSG_MAX);
            (*entry_ptr).msg_len = len as u16;

            // memcpy
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), (*entry_ptr).msg_bytes.as_mut_ptr(), len);

            // Commit
            (*entry_ptr).ready.store(true, Ordering::Release);
        }
    }

    pub fn drain<F>(&self, mut f: F) -> usize
    where F: FnMut(&RingEntry)
    {
        let mut count = 0;
        loop {
            let mut tail = self.tail.load(Ordering::Relaxed);
            let head = self.head.load(Ordering::Relaxed);

            // Safety catch: if we are too far behind, jump ahead.
            if head > tail + RING_SIZE {
                tail = head - RING_SIZE;
                self.tail.store(tail, Ordering::Relaxed);
            }

            if tail >= head {
                break;
            }

            let idx = tail % RING_SIZE;
            let entry = unsafe { &(*self.buffer.get())[idx] };

            // Check if ready
            if !entry.ready.load(Ordering::Acquire) {
                // Writer hasn't finished this slot yet. Stop.
                // Or if we are skipping, we might skip?
                // But if we jumped ahead, we expect valid data?
                // If head wrapped, data *should* be there unless race.
                break;
            }

            f(entry);

            // Mark consumed (optional)
            entry.ready.store(false, Ordering::Relaxed);

            self.tail.store(tail + 1, Ordering::Relaxed);
            count += 1;
        }
        count
    }
}
