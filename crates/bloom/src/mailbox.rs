//! One-shot SPSC mailbox for inter-thread communication.
//!
//! This is a simple channel-like abstraction that supports exactly one message.
//! The worker sends once, main takes once. No locks, no blocking.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU32, Ordering};

/// Mailbox states
const STATE_EMPTY: u32 = 0;
const STATE_WRITING: u32 = 1;
const STATE_READY: u32 = 2;
const STATE_TAKEN: u32 = 3;

/// A one-shot single-producer single-consumer mailbox.
///
/// # Safety
/// This is safe because:
/// - Only one thread can transition Empty→Writing (producer)
/// - Only one thread can transition Ready→Taken (consumer)
/// - The payload is only accessed when in the correct state
pub struct Mailbox<T> {
    state: AtomicU32,
    payload: UnsafeCell<Option<T>>,
}

// SAFETY: Mailbox uses atomic state to synchronize access
unsafe impl<T: Send> Send for Mailbox<T> {}
unsafe impl<T: Send> Sync for Mailbox<T> {}

impl<T> Mailbox<T> {
    /// Create a new empty mailbox.
    pub const fn new() -> Self {
        Self {
            state: AtomicU32::new(STATE_EMPTY),
            payload: UnsafeCell::new(None),
        }
    }

    /// Try to send a value. Returns Err(val) if already sent or being written.
    ///
    /// This is a one-shot operation - can only succeed once.
    pub fn try_send(&self, val: T) -> Result<(), T> {
        // Try to transition Empty → Writing
        match self.state.compare_exchange(
            STATE_EMPTY,
            STATE_WRITING,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                // We own the payload now
                unsafe {
                    *self.payload.get() = Some(val);
                }
                // Transition Writing → Ready (release the data)
                self.state.store(STATE_READY, Ordering::Release);
                Ok(())
            }
            Err(_) => Err(val), // Already sent or being written
        }
    }

    /// Try to take the value. Returns Some(val) if ready, None otherwise.
    ///
    /// This is a one-shot operation - can only succeed once.
    pub fn try_take(&self) -> Option<T> {
        // Try to transition Ready → Taken
        match self.state.compare_exchange(
            STATE_READY,
            STATE_TAKEN,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                // We own the payload now
                unsafe { (*self.payload.get()).take() }
            }
            Err(_) => None, // Not ready yet or already taken
        }
    }

    /// Check if the mailbox has a ready message without taking it.
    pub fn is_ready(&self) -> bool {
        self.state.load(Ordering::Acquire) == STATE_READY
    }
}
