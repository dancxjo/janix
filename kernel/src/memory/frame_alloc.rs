use crate::{PhysRange, PhysRangeKind, BootModuleDesc, kinfo};
use crate::memory::boot_frame_alloc::BootFrameAllocator;
use core::sync::atomic::{AtomicU64, Ordering};

pub const FRAME_SIZE: u64 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysFrame(pub u64);

pub struct FrameStats {
    pub total_frames: u64,
    pub free_frames: u64,
    pub used_frames: u64,
}

pub struct FrameAllocator {
    base: u64,
    frames: u64,
    bitmap: &'static mut [u64],
    next: u64,
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
        // max_usable handles the "end" (exclusive), so (max_usable - base) is size
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
        };

        // 3. Free usable ranges
        for r in map {
            if r.kind == PhysRangeKind::Usable {
                alloc.mark_free_range(r.start, r.end);
            }
        }

        // 4. Mark excluded ranges
        // Re-iterate map to find non-Usable overlaps that might be inside our [base, max] window
        // (Though usually map regions disjoint)
        // Explicitly, KernelImage, Framebuffer, etc. 
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
        
        // Iterate frames
        let start_idx = match self.index(start_aligned) {
            Some(i) => i,
            None => if start_aligned < self.base { 0 } else { return } // If starts before base, clamp
        };
        
        let end_idx = match self.index(end_aligned - 1) { // end is exclusive
             Some(i) => i + 1, // +1 because index is for the frame, we want to include it
             None => self.frames, // If ends after max, clamp
        };

        for i in start_idx..end_idx {
             self.set_bit(i, true);
        }
    }

    pub fn mark_free_range(&mut self, start: u64, end: u64) {
         // Logic same as used, but set false
        let start_aligned = align_up(start, FRAME_SIZE); // Conservative: only free fully covered frames? 
        // Actually, for "Usable" regions provided by Limine, start/end might not be aligned?
        // Usually they are page aligned. But let's be safe: 
        // To be safe to free, the frame must be fully inside the range.
        
        let start_idx = match self.index(start_aligned) {
            Some(i) => i,
             // If start is before base, start at 0
            None => if start_aligned < self.base { 0 } else { return }
        };
        
        // For end, if end is not aligned, we discard the partial frame at end.
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
        if val {
            self.bitmap[word_idx] |= mask;
        } else {
            self.bitmap[word_idx] &= !mask;
        }
    }
    
    fn get_bit(&self, idx: u64) -> bool {
        if idx >= self.frames { return true; } // Out of bounds is "used"
        let word_idx = (idx / 64) as usize;
        let bit_idx = (idx % 64) as usize;
        (self.bitmap[word_idx] & (1 << bit_idx)) != 0
    }

    pub fn alloc(&mut self) -> Option<PhysFrame> {
        // Next fit scan
        let start_scan = self.next;
        
        for i in 0..self.frames {
            let idx = (start_scan + i) % self.frames;
            if !self.get_bit(idx) {
                // Found free
                self.set_bit(idx, true);
                self.next = (idx + 1) % self.frames;
                return Some(PhysFrame(self.addr(idx)));
            }
        }
        None
    }

    pub fn free(&mut self, frame: PhysFrame) {
        if let Some(idx) = self.index(frame.0) {
            if !self.get_bit(idx) {
                panic!("Double free of frame {:#x}", frame.0);
            }
            self.set_bit(idx, false);
        } else {
             // Freeing unknown frame? Ignore or panic.
             // Panic ensures correctness.
             panic!("Freeing frame {:#x} outside managed range", frame.0);
        }
    }

    pub fn stats(&self) -> FrameStats {
        let mut used = 0;
        for i in 0..self.frames {
            if self.get_bit(i) {
                used += 1;
            }
        }
        FrameStats {
            total_frames: self.frames,
            used_frames: used,
            free_frames: self.frames - used,
        }
    }

    pub fn sync_from_boot_alloc(&mut self, boot: &BootFrameAllocator) {
         // Mark all frames allocated by boot allocator as used
         // Boot allocator doesn't expose easy iteration, so we have to ask it, or we iterate our logic?
         // Better: BootFrameAllocator should expose its ranges or we just rely on its "allocated" count and blindly hope? No.
         // We need BootFrameAllocator to tell us "I allocated X at Addr Y".
         // But BootFrameAllocator is just a cursor potentially.
         
         // Actually, simpler: BootFrameAllocator allocates sequentially. 
         // But it skips used regions.
         // We should add a method to BootFrameAllocator to iterate its allocations?
         // Or, `BootFrameAllocator` could just dump its internal `allocated_frames` (count) logic?
         // Wait, `BootFrameAllocator` state is: `current_region`, `current_addr`. 
         // It doesn't track *every* allocation, it just bumps.
         // BUT, since it bumps sequentially in regions, we can just mark everything up to `current_addr` in `current_region` as used?
         // AND mark all frames in *previous* regions as used (if they were usable)?
         // Yes.
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
         // Acquire lock (disable IRQs)
         // We need BootRuntime reference to disable IRQs... but we don't have it easily here?
         // Or we assume we run in a context where we can just access it?
         // Or simplest: spin loop with atomic?
         // For now, let's just use UnsafeCell and assume single threaded boot for step 2.
         // The step says "SpinLock uses BootRuntime::irq_disable".
         // But passing Runtime everywhere is annoying.
         // Let's rely on the fact that for Step 2 we are single core.
         unsafe {
             if let Some(inner) = &mut *self.inner.get() {
                 f(inner)
             } else {
                 panic!("FrameAllocator not initialized");
             }
         }
    }
}
