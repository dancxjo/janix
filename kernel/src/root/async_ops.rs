use crate::root::ReplyCell;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

/// Maximum number of active asynchronous graph operations handled concurrently.
const MAX_ASYNC_OPS: usize = 2048;

struct AsyncOpState {
    cell: Arc<ReplyCell>,
    id: u64,
}

static ASYNC_TABLE: Mutex<Option<Vec<Option<AsyncOpState>>>> = Mutex::new(None);
static NEXT_ASYNC_ID: AtomicU64 = AtomicU64::new(1);

pub fn init() {
    let mut vec = Vec::with_capacity(MAX_ASYNC_OPS);
    for _ in 0..MAX_ASYNC_OPS {
        vec.push(None);
    }
    *ASYNC_TABLE.lock() = Some(vec);
}

/// Allocates an opaque handle for an asynchronous reply cell.
/// Returns a 64-bit handle: high 48-bits contain a generation counter, low 16-bits the table index.
pub fn alloc_handle(cell: Arc<ReplyCell>) -> Option<u64> {
    let mut table_lock = ASYNC_TABLE.lock();
    if let Some(table) = table_lock.as_mut() {
        for (i, slot) in table.iter_mut().enumerate() {
            if slot.is_none() {
                let generation = NEXT_ASYNC_ID.fetch_add(1, Ordering::Relaxed);
                let id = (generation << 16) | (i as u64);
                *slot = Some(AsyncOpState { cell, id });
                return Some(id);
            }
        }
    }
    None
}

/// Retrieves the ReplyCell bounds to the specified opaque handle ID.
pub fn get_cell(handle: u64) -> Option<Arc<ReplyCell>> {
    let table_lock = ASYNC_TABLE.lock();
    if let Some(table) = table_lock.as_ref() {
        let index = (handle & 0xFFFF) as usize;
        if let Some(Some(state)) = table.get(index) {
            if state.id == handle {
                return Some(state.cell.clone());
            }
        }
    }
    None
}

/// Frees the handle matching the given opaque handle ID.
pub fn free_handle(handle: u64) {
    let mut table_lock = ASYNC_TABLE.lock();
    if let Some(table) = table_lock.as_mut() {
        let index = (handle & 0xFFFF) as usize;
        if let Some(slot) = table.get_mut(index) {
            if let Some(state) = slot {
                if state.id == handle {
                    *slot = None;
                }
            }
        }
    }
}
