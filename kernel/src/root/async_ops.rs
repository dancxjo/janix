use crate::root::ReplyCell;
use abi::errors::Errno;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

/// Maximum number of active asynchronous graph operations handled concurrently.
const MAX_ASYNC_OPS: usize = 2048;

struct AsyncOpState {
    cell: Arc<ReplyCell>,
    id: u64,
    waiters: crate::sched::WaitQueue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncOpPoll {
    Pending,
    Complete {
        status: i32,
        value: u64,
        p0: u64,
        p1: u64,
        p2: u64,
    },
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
                *slot = Some(AsyncOpState {
                    cell,
                    id,
                    waiters: crate::sched::WaitQueue::new(),
                });
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
            if let Some(state) = slot.take() {
                if state.id == handle {
                    state.waiters.wake_all();
                } else {
                    *slot = Some(state);
                }
            }
        }
    }
}

pub fn poll_handle(handle: u64) -> Result<AsyncOpPoll, Errno> {
    let cell = get_cell(handle).ok_or(Errno::ENOENT)?;
    if cell.done.load(Ordering::Acquire) == 0 {
        return Ok(AsyncOpPoll::Pending);
    }

    Ok(AsyncOpPoll::Complete {
        status: cell.status.load(Ordering::Relaxed),
        value: cell.value.load(Ordering::Relaxed),
        p0: cell.p0.load(Ordering::Relaxed),
        p1: cell.p1.load(Ordering::Relaxed),
        p2: cell.p2.load(Ordering::Relaxed),
    })
}

pub fn register_waiter(handle: u64, tid: u64) -> Result<(), Errno> {
    let table_lock = ASYNC_TABLE.lock();
    let table = table_lock.as_ref().ok_or(Errno::ENOENT)?;
    let index = (handle & 0xFFFF) as usize;
    let state = table
        .get(index)
        .and_then(|slot| slot.as_ref())
        .filter(|state| state.id == handle)
        .ok_or(Errno::ENOENT)?;
    state.waiters.push_back(tid);
    Ok(())
}

pub fn unregister_waiter(handle: u64, tid: u64) -> Result<(), Errno> {
    let table_lock = ASYNC_TABLE.lock();
    let table = table_lock.as_ref().ok_or(Errno::ENOENT)?;
    let index = (handle & 0xFFFF) as usize;
    let state = table
        .get(index)
        .and_then(|slot| slot.as_ref())
        .filter(|state| state.id == handle)
        .ok_or(Errno::ENOENT)?;
    state.waiters.remove(tid);
    Ok(())
}

pub fn notify_completion(cell: &Arc<ReplyCell>) {
    let table_lock = ASYNC_TABLE.lock();
    if let Some(table) = table_lock.as_ref() {
        for state in table.iter().flatten() {
            if Arc::ptr_eq(&state.cell, cell) {
                state.waiters.wake_all();
                break;
            }
        }
    }
}
