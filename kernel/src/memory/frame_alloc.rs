use crate::{PhysRange, PhysRangeKind, BootModuleDesc, kinfo};
use crate::memory::boot_frame_alloc::BootFrameAllocator;
use core::sync::atomic::{AtomicU64, Ordering};
use core::ops::Range;

pub const FRAME_SIZE: u64 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysFrame(pub u64); // 4KiB-aligned physical address

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysFrameRange {
    pub base: PhysFrame,
    pub count: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct FrameStats {
    pub total_frames: u64,
    pub free_frames: u64,
    pub used_frames: u64,
}

pub struct FrameAllocator {
    base: u64,                 // lowest managed phys addr (aligned)
    frames: u64,               // number of frames in index space
    bitmap: &'static mut [u64],// bit=1 used, bit=0 free
    next: u64,                 // next-fit cursor (frame index)
    free_count: u64,           // maintained for O(1) stats
}

impl FrameAllocator {
    pub fn new_from_boot(
        map: &'static [PhysRange],
        modules: &'static [BootModuleDesc],
        bitmap: &'static mut [u64],
    ) -> Self {
        // 1. Calculate physical logic
        let mut min_usable = u64::MAX;
        let mut max_usable = 0;

        for r in map {
            if r.kind == PhysRangeKind::Usable {
                if r.start < min_usable { min_usable = r.start; }
                if r.end > max_usable { max_usable = r.end; }
            }
        }

        // Align
        let base = align_up(min_usable, FRAME_SIZE);
        let len_bytes = max_usable.saturating_sub(base);
        let frames = len_bytes / FRAME_SIZE;
        
        // Verify bitmap size
        let words = (frames + 63) / 64;
        if (bitmap.len() as u64) < words {
             panic!("FrameAllocator: bitmap too small! Needed {} words, got {}", words, bitmap.len());
        }

        // 2. Init bitmap: All used first (safest)
        bitmap.fill(!0); // All 1s

        let mut alloc = Self {
            base,
            frames,
            bitmap,
            next: 0,
            free_count: 0, 
        };

        // 3. Free usable ranges
        for r in map {
            if r.kind == PhysRangeKind::Usable {
                alloc.mark_free_range(r.start, r.end);
            }
        }

        // 4. Mark excluded ranges
        // Re-iterate map to find non-Usable overlaps that might be inside our [base, max] window
        for r in map {
             if r.kind != PhysRangeKind::Usable {
                 alloc.mark_used_range(r.start, r.end);
             }
        }
        
        // Modules
        for m in modules {
             alloc.mark_used_range(m.phys_start, m.phys_end);
        }

        alloc
    }

    fn index(&self, addr: u64) -> Option<u64> {
        if addr < self.base { return None; }
        let offset = addr - self.base;
        let idx = offset / FRAME_SIZE;
        if idx >= self.frames { return None; }
        Some(idx)
    }

    fn addr(&self, idx: u64) -> u64 {
        self.base + idx * FRAME_SIZE
    }

    pub fn mark_used_range(&mut self, start: u64, end: u64) {
        let start_aligned = align_down(start, FRAME_SIZE);
        let end_aligned = align_up(end, FRAME_SIZE);
        
        let start_idx = match self.index(start_aligned) {
            Some(i) => i,
            None => if start_aligned < self.base { 0 } else { return } 
        };
        
        // end_aligned is exclusive end of range, so index(end_aligned - 1) + 1 gives the exclusive index bound
        let end_idx = match self.index(end_aligned.saturating_sub(1)) {
             Some(i) => i + 1, 
             None => self.frames, 
        };

        for i in start_idx..end_idx {
             self.set_bit(i, true);
        }
    }

    pub fn mark_free_range(&mut self, start: u64, end: u64) {
        let start_aligned = align_up(start, FRAME_SIZE); // Conservative: only free fully covered frames
        
        let start_idx = match self.index(start_aligned) {
            Some(i) => i,
            None => if start_aligned < self.base { 0 } else { return }
        };
        
        let end_aligned = align_down(end, FRAME_SIZE);
         let end_idx = match self.index(end_aligned.saturating_sub(1)) {
             Some(i) => i + 1,
             None => self.frames,
        };

        for i in start_idx..end_idx {
             self.set_bit(i, false);
        }
    }

    fn set_bit(&mut self, idx: u64, val: bool) {
        if idx >= self.frames { return; }
        let word_idx = (idx / 64) as usize;
        let bit_idx = (idx % 64) as usize;
        let mask = 1 << bit_idx;
        
        let old = (self.bitmap[word_idx] & mask) != 0;
        
        if val != old {
            if val {
                self.bitmap[word_idx] |= mask;
                self.free_count -= 1;
            } else {
                self.bitmap[word_idx] &= !mask;
                self.free_count += 1;
            }
        }
    }
    
    fn get_bit(&self, idx: u64) -> bool {
        if idx >= self.frames { return true; } // Out of bounds is "used"
        let word_idx = (idx / 64) as usize;
        let bit_idx = (idx % 64) as usize;
        (self.bitmap[word_idx] & (1 << bit_idx)) != 0
    }

    pub fn alloc(&mut self) -> Option<PhysFrame> {
        if self.free_count == 0 { return None; }
        
        let start_scan = self.next;
        for i in 0..self.frames {
            let idx = (start_scan + i) % self.frames;
             // We can optimize this by checking words, but single frame alloc is fine for now
             if !self.get_bit(idx) {
                 self.set_bit(idx, true);
                 self.next = (idx + 1) % self.frames;
                 return Some(PhysFrame(self.addr(idx)));
             }
        }
        None
    }

    pub fn alloc_contiguous(&mut self, count: u64) -> Option<PhysFrameRange> {
        if count == 0 { return None; }
        if count == 1 {
             let res = self.alloc().map(|f| PhysFrameRange { base: f, count: 1 });
             #[cfg(debug_assertions)]
             if let Some(ref r) = res { self.verify_allocation(r.base.0, r.count); }
             return res;
        }
        if count > self.frames { return None; }
        
        // Scan for run of length `count`
        let mut run_start = self.next;
        let mut run_len = 0;
        let mut scanned = 0;
        
        // We iterate `scanned` from 0 to `self.frames + count` (to handle wrap around and verify full buffer)
        // Wait, wrap around with contiguous is tricky.
        // "Start scan at next index (wrap around once)."
        // "Iterate over indices"
        
        let mut i = self.next;
        loop {
            if scanned > self.frames {
                 // Full circle completed and no run found
                 return None;
            }

            // Word optimization:
            // If we are aligned to 64 and run_len is 0 (or we just want to skip used words)
            // But we only want to skip if current run is broken.
            
            if i % 64 == 0 && run_len == 0 {
                 let word_idx = (i / 64) as usize;
                 if word_idx < self.bitmap.len() && self.bitmap[word_idx] == !0 {
                      // All used, skip 64
                      i += 64;
                      scanned += 64;
                      // Update next to skip used? No, self.next isn't updated during scan.
                      continue;
                 }
            }

             if !self.get_bit(i % self.frames) {
                 if run_len == 0 {
                     run_start = i;
                 }
                 run_len += 1;
                 
                 if run_len == count {
                     // Found it!
                     // Commit
                     let final_start = run_start % self.frames;
                     
                     // Check wrap-around case: if run wraps around end of buffer?
                     // Physically contiguous memory DOES NOT wrap around the end of RAM.
                     // So if (run_start % frames) + count > frames, it is NOT valid.
                     // Because index space is linear.
                     
                     if final_start + count > self.frames {
                         // This run wraps around the physical end of memory.
                         // This is NOT physically contiguous.
                         // Reset run.
                         run_len = 0;
                         // Continue scan from where we failed?
                         // We failed at `i`. The run started at `run_start`.
                         // We should reset run logic.
                     } else {
                         // Valid run.
                          let base_addr = self.addr(final_start);
                          #[cfg(debug_assertions)]
                          self.verify_allocation(base_addr, count);

                          for k in 0..count {
                              self.set_bit(final_start + k, true);
                          }
                          self.next = (final_start + count) % self.frames;
                          return Some(PhysFrameRange {
                              base: PhysFrame(base_addr),
                              count
                          });
                      }
                 }
             } else {
                 // Bit is used, reset run
                 run_len = 0;
             }
             
             i += 1;
             scanned += 1;
        }
    }

    pub fn free(&mut self, frame: PhysFrame) {
        if let Some(idx) = self.index(frame.0) {
            if !self.get_bit(idx) {
                panic!("Double free of frame {:#x}", frame.0);
            }
            self.set_bit(idx, false);
        } else {
             panic!("Freeing frame {:#x} outside managed range", frame.0);
        }
    }

    pub fn free_contiguous(&mut self, base: PhysFrame, count: u64) {
        for i in 0..count {
             self.free(PhysFrame(base.0 + i * FRAME_SIZE));
        }
    }

    pub fn stats(&self) -> FrameStats {
        FrameStats {
            total_frames: self.frames,
            free_frames: self.free_count,
            used_frames: self.frames - self.free_count,
        }
    }

    #[cfg(debug_assertions)]
    fn verify_allocation(&self, start: u64, count: u64) {
        let end = start + count * FRAME_SIZE;
        // Verify that the entire range [start, end) is covered by Usable regions in the boot map.
        // We access the authoritative map from the runtime.
        let map = crate::runtime().phys_memory_map();
        
        let mut current = start;
        while current < end {
            let mut found = false;
            for r in map {
                // We only care about Usable memory.
                if r.kind == PhysRangeKind::Usable && r.start <= current && r.end > current {
                     // Found a covering region. Advance current to the end of this region or the end of our alloc.
                     current = core::cmp::min(end, r.end);
                     found = true;
                     break;
                }
            }
            
            if !found {
                 panic!("FrameAllocator: alloc_contiguous({}) returned range [{:#x}, {:#x}) which crosses non-usable boundary at {:#x}", count, start, end, current);
            }
        }
    }
}

fn align_up(addr: u64, align: u64) -> u64 {
    (addr + align - 1) & !(align - 1)
}

fn align_down(addr: u64, align: u64) -> u64 {
    addr & !(align - 1)
}

// Wrapper for SpinLock if needed, but for now single core boot is assumed or we use a temporary lock.
// The user mentions FrameAllocatorLocked(SpinLock<FrameAllocator>).
// We don't have a SpinLock in standard yet? We have irq_disable in BootRuntime.
use core::cell::UnsafeCell;

pub struct FrameAllocatorLocked {
    inner: UnsafeCell<Option<FrameAllocator>>,
}

unsafe impl Sync for FrameAllocatorLocked {}

impl FrameAllocatorLocked {
    pub const fn new() -> Self {
        Self { inner: UnsafeCell::new(None) }
    }
    
    pub unsafe fn init(&self, alloc: FrameAllocator) {
        *self.inner.get() = Some(alloc);
    }
    
    pub fn with_lock<F, R>(&self, f: F) -> R
    where F: FnOnce(&mut FrameAllocator) -> R,
    {
         // We must disable IRQs to ensure safety on this core.
         // In a multi-core scenario, this would also need a spinlock.
         let runtime = crate::runtime();
         let irq = runtime.irq_disable();
         
         // SAFETY: 
         // 1. We disabled IRQs, so no interrupt handler can re-enter this on the same core.
         // 2. We assume single-core boot for now, or that callers respect the lock (which is just this wrapper).
         // 3. UnsafeCell usage relies on this exclusive access.
         let res = unsafe {
             if let Some(inner) = &mut *self.inner.get() {
                 f(inner)
             } else {
                 panic!("FrameAllocator not initialized");
             }
         };
         
         runtime.irq_restore(irq);
         res
    }
}
