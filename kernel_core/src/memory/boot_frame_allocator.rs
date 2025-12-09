#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysFrame {
    pub start_address: u64,
    pub size: u64,
}

impl PhysFrame {
    pub fn from_start_address(start_address: u64, size: u64) -> Self {
        Self {
            start_address,
            size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: u64,
    pub len: u64,
}

pub struct BootFrameAllocator {
    regions: [MemoryRegion; 64],
    region_count: usize,
    current_region: usize,
    next_free_addr: u64,
}

impl BootFrameAllocator {
    pub const fn new() -> Self {
        Self {
            regions: [MemoryRegion { start: 0, len: 0 }; 64],
            region_count: 0,
            current_region: 0,
            next_free_addr: 0,
        }
    }

    pub fn add_region(&mut self, start: u64, len: u64) {
        if self.region_count < self.regions.len() {
            self.regions[self.region_count] = MemoryRegion { start, len };
            self.region_count += 1;

            // If this is the first region, initialize next_free_addr
            if self.region_count == 1 {
                self.next_free_addr = start;
            }
        }
    }

    pub fn allocate_frame(&mut self) -> Option<PhysFrame> {
        loop {
            if self.current_region >= self.region_count {
                return None;
            }

            let region = self.regions[self.current_region];
            let end = region.start + region.len;

            if self.next_free_addr < region.start {
                self.next_free_addr = region.start;
            }

            if self.next_free_addr + 4096 <= end {
                let frame = PhysFrame::from_start_address(self.next_free_addr, 4096);
                self.next_free_addr += 4096;
                return Some(frame);
            } else {
                self.current_region += 1;
            }
        }
    }
}
