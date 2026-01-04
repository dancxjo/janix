//! Timer integration with scheduler.
//!
//! This module provides the bridge between hardware timer interrupts
//! and the scheduler's preemption logic.

use crate::sched;

/// Called by the machine layer when a timer interrupt fires.
pub fn timer_tick(current_sp: u64) -> u64 {
    sched::tick(current_sp)
}
