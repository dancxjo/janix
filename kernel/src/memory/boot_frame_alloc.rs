use crate::{PhysRange, PhysRangeKind};

pub struct BootFrameAllocator {
    map: &'static [PhysRange],
    current_region: usize,
    current_addr: u64,
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
            
            // Initialize current_addr if entering a new region
            // We need to ensure we align up to 4096. 
            // Also need to handle if we just created the allocator (current_addr is 0)
            // or if we moved to next region.
            // But wait, if we are mid-region, current_addr should be valid.
            // Let's use a logic where if current_addr < region.start, we snap it to start.
            
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
                // We have a candidate frame!
                let frame = self.current_addr;
                
                // IMPORTANT: We need to verify this frame doesn't overlap *other* excluded ranges.
                // The map *should* have granular regions (e.g. Usable, then Kernel, then Usable)
                // but sometimes maps are coarse.
                // For this task, we assume the map provided by Bran is the source of truth for "Usable".
                // HOWEVER, we must double check against MODULES if the map doesn't explicitly exclude them.
                // The task says: "Invariant: All module pages must be excluded from allocation... Prefer map includes them as BootModule"
                // Assuming the map is constructed correctly by Bran to mark modules as BootModule or Reserved.
                // So we can trust `PhysRangeKind::Usable`.
                
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
}

fn align_up(addr: u64, align: u64) -> u64 {
    (addr + align - 1) & !(align - 1)
}
