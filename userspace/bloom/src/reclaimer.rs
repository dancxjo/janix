//! Simplified reclaimer for Bloom (Compositor only)
//! Tracks in-flight frames but does not handle asset eviction.

use core::cell::UnsafeCell;
use crate::frame::AssetGeneration;

/// Maximum number of in-flight frames (triple buffering)
const MAX_IN_FLIGHT: usize = 3;

/// Entry for an in-flight frame
#[derive(Clone, Copy, Debug, Default)]
struct InFlightEntry {
    frame_id: u64,
    asset_gen: u64,
    active: bool,
}

struct InFlightFrames {
    entries: [UnsafeCell<InFlightEntry>; MAX_IN_FLIGHT],
}

unsafe impl Sync for InFlightFrames {}

impl InFlightFrames {
    const fn new() -> Self {
        Self {
            entries: [
                UnsafeCell::new(InFlightEntry { frame_id: 0, asset_gen: 0, active: false }),
                UnsafeCell::new(InFlightEntry { frame_id: 0, asset_gen: 0, active: false }),
                UnsafeCell::new(InFlightEntry { frame_id: 0, asset_gen: 0, active: false }),
            ],
        }
    }

    fn register(&self, frame_id: u64, asset_gen: AssetGeneration) {
        let mut oldest_idx = 0;
        let mut oldest_frame = u64::MAX;
        
        for i in 0..MAX_IN_FLIGHT {
            let entry = unsafe { &*self.entries[i].get() };
            if !entry.active {
                unsafe {
                    let ptr = self.entries[i].get();
                    (*ptr).frame_id = frame_id;
                    (*ptr).asset_gen = asset_gen.0;
                    (*ptr).active = true;
                }
                return;
            }
            if entry.frame_id < oldest_frame {
                oldest_frame = entry.frame_id;
                oldest_idx = i;
            }
        }
        
        unsafe {
            let ptr = self.entries[oldest_idx].get();
            (*ptr).frame_id = frame_id;
            (*ptr).asset_gen = asset_gen.0;
            (*ptr).active = true;
        }
    }

    fn complete(&self, frame_id: u64) {
        for i in 0..MAX_IN_FLIGHT {
            let entry = unsafe { &*self.entries[i].get() };
            if entry.active && entry.frame_id == frame_id {
                unsafe {
                    (*self.entries[i].get()).active = false;
                }
                return;
            }
        }
    }
}

static IN_FLIGHT: InFlightFrames = InFlightFrames::new();

pub fn register_in_flight(frame_id: u64, asset_gen: AssetGeneration) {
    IN_FLIGHT.register(frame_id, asset_gen);
}

pub fn complete_in_flight(frame_id: u64) {
    IN_FLIGHT.complete(frame_id);
}

// Stubs for stats used by present.rs logging (if any)
pub fn decoded_bytes() -> usize { 0 }
pub fn memory_budget() -> usize { 0 }
pub fn eviction_count() -> u64 { 0 }
pub fn in_flight_count() -> usize { 0 }
pub fn min_live_gen() -> AssetGeneration { AssetGeneration::ZERO }
