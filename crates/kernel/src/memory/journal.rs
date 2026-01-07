use abi::ids::ThingId;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use crate::memory::map::MapPerms;

pub const EVENT_BYTESPACE_CREATED_RAM: u64 = 1;
pub const EVENT_BYTESPACE_CREATED_DMA: u64 = 2;
pub const EVENT_BYTESPACE_CREATED_DEVICE: u64 = 3;
pub const EVENT_BYTESPACE_CREATED_MODULE: u64 = 4;
pub const EVENT_BYTESPACE_CREATED_FRAMEBUFFER: u64 = 5;
pub const EVENT_BYTESPACE_CREATED_KERNEL_HEAP: u64 = 6;
pub const EVENT_BYTESPACE_DESTROYED: u64 = 7;
pub const EVENT_MAP: u64 = 8;
pub const EVENT_UNMAP: u64 = 9;
pub const EVENT_HEAP_INIT: u64 = 10;
pub const EVENT_HEAP_GROW: u64 = 11;

#[derive(Clone, Copy, Debug)]
pub struct MemoryEvent {
    pub kind: u64,
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub d: u64,
}

const JOURNAL_CAPACITY: usize = 256;

struct Journal {
    buf: [MemoryEvent; JOURNAL_CAPACITY],
    head: usize,
    tail: usize,
    len: usize,
}

impl Journal {
    const fn new() -> Self {
        Self {
            buf: [MemoryEvent {
                kind: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            }; JOURNAL_CAPACITY],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    fn try_push(&mut self, ev: MemoryEvent) -> bool {
        if self.len == JOURNAL_CAPACITY {
            return false;
        }
        self.buf[self.head] = ev;
        self.head = (self.head + 1) % JOURNAL_CAPACITY;
        self.len += 1;
        true
    }

    #[allow(dead_code)]
    fn try_pop(&mut self) -> Option<MemoryEvent> {
        if self.len == 0 {
            return None;
        }
        let ev = self.buf[self.tail];
        self.tail = (self.tail + 1) % JOURNAL_CAPACITY;
        self.len -= 1;
        Some(ev)
    }
}

static JOURNAL: Mutex<Journal> = Mutex::new(Journal::new());
static DROPPED_EVENTS: AtomicU64 = AtomicU64::new(0);

pub fn journal_emit(ev: MemoryEvent) {
    if let Some(mut guard) = JOURNAL.try_lock() {
        if guard.try_push(ev) {
            return;
        }
    }
    DROPPED_EVENTS.fetch_add(1, Ordering::Relaxed);
}

pub fn journal_dropped_count() -> u64 {
    DROPPED_EVENTS.load(Ordering::Relaxed)
}

pub fn drain_memory_journal(max_events: usize) -> usize {
    let mut drained = 0;
    if let Some(mut guard) = JOURNAL.try_lock() {
        while drained < max_events {
            if guard.try_pop().is_none() {
                break;
            }
            drained += 1;
        }
    }
    drained
}

pub fn emit_bytespace_created(kind: u64, id: ThingId, size: usize, phys: u64) {
    journal_emit(MemoryEvent {
        kind,
        a: id.low(),
        b: id.high(),
        c: size as u64,
        d: phys,
    });
}

pub fn emit_bytespace_destroyed(id: ThingId) {
    journal_emit(MemoryEvent {
        kind: EVENT_BYTESPACE_DESTROYED,
        a: id.low(),
        b: id.high(),
        c: 0,
        d: 0,
    });
}

pub fn emit_map(asid: ThingId, virt: u64, phys: u64, len: usize, perms: MapPerms) {
    let kind = EVENT_MAP | ((perms.bits() as u64) << 32);
    journal_emit(MemoryEvent {
        kind,
        a: asid.low(),
        b: virt,
        c: phys,
        d: len as u64,
    });
}

pub fn emit_unmap(asid: ThingId, virt: u64, len: usize) {
    journal_emit(MemoryEvent {
        kind: EVENT_UNMAP,
        a: asid.low(),
        b: virt,
        c: len as u64,
        d: 0,
    });
}

pub fn emit_heap_init(base: u64, size: usize) {
    journal_emit(MemoryEvent {
        kind: EVENT_HEAP_INIT,
        a: base,
        b: size as u64,
        c: 0,
        d: 0,
    });
}

pub fn emit_heap_grow(task_id: u64, old_brk: u64, new_brk: u64, size: u64) {
    journal_emit(MemoryEvent {
        kind: EVENT_HEAP_GROW,
        a: task_id,
        b: old_brk,
        c: new_brk,
        d: size,
    });
}
