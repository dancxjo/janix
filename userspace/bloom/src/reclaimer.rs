//! Asset Reclaimer v0: Generation-based memory-safe eviction for Bloom compositor.
//!
//! This module provides:
//! - `InFlightFrames`: Tracks frames between acquire and present for safe eviction
//! - `AssetMetadata`: Per-asset tracking for eviction decisions
//! - `EvictedAsset`: Placeholder for future rehydration support
//! - Memory budget enforcement with LRU + reachability eviction

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use crate::frame::AssetGeneration;
use crate::asset::AssetBank;
use stem::info;

/// Default memory budget for decoded surfaces (32 MiB)
pub const DEFAULT_MEMORY_BUDGET: usize = 32 * 1024 * 1024;

/// Global memory budget (runtime-configurable)
static MEMORY_BUDGET: AtomicUsize = AtomicUsize::new(DEFAULT_MEMORY_BUDGET);

/// Global decoded bytes counter
static DECODED_BYTES: AtomicUsize = AtomicUsize::new(0);

/// Global eviction counter (for stats)
static EVICTION_COUNT: AtomicU64 = AtomicU64::new(0);

/// Set the memory budget for decoded surfaces
#[allow(dead_code)]
pub fn set_memory_budget(bytes: usize) {
    MEMORY_BUDGET.store(bytes, Ordering::Release);
    info!("[reclaimer] memory budget set to {} bytes", bytes);
}

/// Get the current memory budget
pub fn memory_budget() -> usize {
    MEMORY_BUDGET.load(Ordering::Acquire)
}

/// Get current decoded bytes usage
pub fn decoded_bytes() -> usize {
    DECODED_BYTES.load(Ordering::Acquire)
}

/// Add to decoded bytes counter (called on asset promotion)
#[allow(dead_code)]
pub fn add_decoded_bytes(bytes: usize) {
    let prev = DECODED_BYTES.fetch_add(bytes, Ordering::AcqRel);
    info!("[reclaimer] +{} bytes (total: {})", bytes, prev + bytes);
}

/// Subtract from decoded bytes counter (called on asset eviction)
#[allow(dead_code)]
pub fn sub_decoded_bytes(bytes: usize) {
    let prev = DECODED_BYTES.fetch_sub(bytes, Ordering::AcqRel);
    info!("[reclaimer] -{} bytes (total: {})", bytes, prev.saturating_sub(bytes));
}
pub fn eviction_count() -> u64 {
    EVICTION_COUNT.load(Ordering::Acquire)
}

/// Increment eviction counter
fn inc_eviction_count() {
    EVICTION_COUNT.fetch_add(1, Ordering::AcqRel);
}

/// Asset metadata for eviction decisions
#[derive(Clone, Debug)]
pub struct AssetMetadata {
    pub last_used_frame: u64,
    pub gen: AssetGeneration,
    pub decoded_bytes: usize,
    pub reachable: bool,
}

impl AssetMetadata {
    /// Compute eviction priority (lower = evict first)
    /// - Unreachable assets get priority 0
    /// - Reachable assets get priority = last_used_frame (older = lower = evict first)
    pub fn eviction_priority(&self) -> u64 {
        if self.reachable {
            // Higher frame = more recently used = higher priority = evict later
            self.last_used_frame
        } else {
            // Unreachable: always evict first
            0
        }
    }
}

/// Maximum number of in-flight frames (triple buffering)
const MAX_IN_FLIGHT: usize = 3;

/// Entry for an in-flight frame
#[derive(Clone, Copy, Debug, Default)]
struct InFlightEntry {
    frame_id: u64,
    asset_gen: u64, // Store as u64 to avoid Copy issues
    active: bool,
}

/// Tracks in-flight frames for safe eviction.
/// Assets with generation >= min_live_gen must not be evicted.
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

    /// Register a frame as in-flight (called by acquire_frame)
    fn register(&self, frame_id: u64, asset_gen: AssetGeneration) {
        // Find empty slot or oldest slot
        let mut oldest_idx = 0;
        let mut oldest_frame = u64::MAX;
        
        for i in 0..MAX_IN_FLIGHT {
            let entry = unsafe { &*self.entries[i].get() };
            if !entry.active {
                // Found empty slot
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
        
        // All slots full, replace oldest (shouldn't happen with proper complete() calls)
        info!("[reclaimer] WARNING: in-flight slots full, replacing frame {}", oldest_frame);
        unsafe {
            let ptr = self.entries[oldest_idx].get();
            (*ptr).frame_id = frame_id;
            (*ptr).asset_gen = asset_gen.0;
            (*ptr).active = true;
        }
    }

    /// Complete a frame (called by present_frame)
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

    /// Get the minimum generation among all in-flight frames.
    /// Assets with gen >= this value must NOT be evicted.
    fn min_live_gen(&self) -> AssetGeneration {
        let mut min_gen = u64::MAX;
        let mut has_any = false;
        
        for i in 0..MAX_IN_FLIGHT {
            let entry = unsafe { &*self.entries[i].get() };
            if entry.active {
                has_any = true;
                if entry.asset_gen < min_gen {
                    min_gen = entry.asset_gen;
                }
            }
        }
        
        if has_any {
            AssetGeneration(min_gen)
        } else {
            // No in-flight frames, safe to evict anything
            AssetGeneration(u64::MAX)
        }
    }

    /// Number of in-flight frames
    fn count(&self) -> usize {
        let mut count = 0;
        for i in 0..MAX_IN_FLIGHT {
            let entry = unsafe { &*self.entries[i].get() };
            if entry.active {
                count += 1;
            }
        }
        count
    }
}

/// Global in-flight frame tracker
static IN_FLIGHT: InFlightFrames = InFlightFrames::new();

/// Register a frame as in-flight (thread-safe via frame loop single-threadedness)
pub fn register_in_flight(frame_id: u64, asset_gen: AssetGeneration) {
    IN_FLIGHT.register(frame_id, asset_gen);
}

/// Complete an in-flight frame
pub fn complete_in_flight(frame_id: u64) {
    IN_FLIGHT.complete(frame_id);
}

/// Get min_live_gen from in-flight tracker
#[allow(dead_code)]
pub fn min_live_gen() -> AssetGeneration {
    IN_FLIGHT.min_live_gen()
}

/// Get in-flight frame count
pub fn in_flight_count() -> usize {
    IN_FLIGHT.count()
}


/// Check memory pressure and evict if needed.
/// Called once per frame after present.
pub fn check_memory_pressure(assets: &AssetBank) {
    let current = decoded_bytes();
    let budget = memory_budget();
    
    if current <= budget {
        return; // Under budget, no action needed
    }
    
    let min_gen = min_live_gen();
    info!("[reclaimer] memory pressure: {} > {} bytes, min_live_gen={}", 
        current, budget, min_gen.0);
    
    // Try to evict until under budget
    let mut freed_total = 0usize;
    let mut evictions = 0u32;
    
    while decoded_bytes() > budget {
        match assets.try_evict_one(min_gen) {
            Some(freed) => {
                freed_total += freed;
                evictions += 1;
                inc_eviction_count();
            }
            None => {
                // No more evictable assets
                info!("[reclaimer] no more evictable assets (freed {} bytes in {} evictions)", 
                    freed_total, evictions);
                break;
            }
        }
    }
    
    if evictions > 0 {
        info!("[reclaimer] evicted {} assets, freed {} bytes, now at {} bytes",
            evictions, freed_total, decoded_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_flight_register_complete() {
        let tracker = InFlightFrames::new();
        
        assert_eq!(tracker.count(), 0);
        assert_eq!(tracker.min_live_gen(), AssetGeneration(u64::MAX));
        
        tracker.register(1, AssetGeneration(5));
        assert_eq!(tracker.count(), 1);
        assert_eq!(tracker.min_live_gen(), AssetGeneration(5));
        
        tracker.register(2, AssetGeneration(7));
        assert_eq!(tracker.count(), 2);
        assert_eq!(tracker.min_live_gen(), AssetGeneration(5));
        
        tracker.complete(1);
        assert_eq!(tracker.count(), 1);
        assert_eq!(tracker.min_live_gen(), AssetGeneration(7));
        
        tracker.complete(2);
        assert_eq!(tracker.count(), 0);
        assert_eq!(tracker.min_live_gen(), AssetGeneration(u64::MAX));
    }

    #[test]
    fn test_asset_metadata_priority() {
        let unreachable = AssetMetadata {
            last_used_frame: 100,
            gen: AssetGeneration(1),
            decoded_bytes: 1000,
            reachable: false,
        };
        
        let old_reachable = AssetMetadata {
            last_used_frame: 50,
            gen: AssetGeneration(1),
            decoded_bytes: 1000,
            reachable: true,
        };
        
        let new_reachable = AssetMetadata {
            last_used_frame: 200,
            gen: AssetGeneration(1),
            decoded_bytes: 1000,
            reachable: true,
        };
        
        // Unreachable should have lowest priority (evict first)
        assert_eq!(unreachable.eviction_priority(), 0);
        
        // Older reachable < newer reachable
        assert!(old_reachable.eviction_priority() < new_reachable.eviction_priority());
    }
}
