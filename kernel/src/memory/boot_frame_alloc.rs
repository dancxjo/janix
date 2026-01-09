use crate::{PhysRange, PhysRangeKind};
use crate::memory::frame_alloc::FrameAllocator;

pub struct BootFrameAllocator {
    pub map: &'static [PhysRange],
    pub current_region: usize,
    pub current_addr: u64,
    allocated_frames: usize,
}

impl BootFrameAllocator {
    pub fn new(map: &'static [PhysRange]) -> Self {
        Self {
            map,
            current_region: 0,
            current_addr: 0, 
            allocated_frames: 0,
        }
    }

    /// Allocate a 4KiB physical frame.
    /// Returns the physical address of the frame, or None if out of memory.
    pub fn alloc_frame(&mut self) -> Option<u64> {
        // Find a usable region if we aren't in one or finished the current one
        loop {
            if self.current_region >= self.map.len() {
                return None;
            }

            let region = &self.map[self.current_region];
            
            if region.kind != PhysRangeKind::Usable {
                self.current_region += 1;
                self.current_addr = 0; // Reset so we pick up start of next region
                continue;
            }

            if self.current_addr < region.start {
                self.current_addr = align_up(region.start, 4096);
            }

            // Check if we fit in the current region
            if self.current_addr + 4096 <= region.end {
                // We have a candidate frame
                let frame = self.current_addr;
                
                self.current_addr += 4096;
                self.allocated_frames += 1;
                return Some(frame);
            } else {
                // Region exhausted
                self.current_region += 1;
                self.current_addr = 0;
            }
        }
    }
    
    pub fn frames_allocated(&self) -> usize {
        self.allocated_frames
    }

    pub fn bytes_allocated(&self) -> usize {
        self.allocated_frames * 4096
    }

    pub fn transfer_state_to(&self, target: &mut FrameAllocator) {
         // Mark ranges we consumed as used.
         // 1. All PREVIOUS usable regions are fully consumed (or skipped? logic says we fill sequentially).
         // Actually `alloc_frame` skips regions if they are full.
         // So for `i < current_region`, if `map[i]` is Usable, it is FULLY consumed.
         // 2. The `current_region` is consumed up to `current_addr`.
        
        for i in 0..self.current_region {
            let r = &self.map[i];
            if r.kind == PhysRangeKind::Usable {
                target.mark_used_range(r.start, r.end);
            }
        }

        if self.current_region < self.map.len() {
            let r = &self.map[self.current_region];
            if r.kind == PhysRangeKind::Usable {
                // Current addr is the NEXT free frame.
                // So everything from start to current_addr is used.
                // Be careful if current_addr was just reset (0) -> implies start.
                let consumed_end = if self.current_addr == 0 { r.start } else { self.current_addr };
                // Clamp to region end just in case?
                if consumed_end > r.start {
                    target.mark_used_range(r.start, consumed_end);
                }
            }
        }
    }
}

fn align_up(addr: u64, align: u64) -> u64 {
    (addr + align - 1) & !(align - 1)
}
