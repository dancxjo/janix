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
    total_bytes: u64,
}

impl BootFrameAllocator {
    const FRAME_SIZE: u64 = 4096;

    pub const fn new() -> Self {
        Self {
            regions: [MemoryRegion { start: 0, len: 0 }; 64],
            region_count: 0,
            current_region: 0,
            next_free_addr: 0,
            total_bytes: 0,
        }
    }

    pub fn add_region(&mut self, start: u64, len: u64) {
        if self.region_count >= self.regions.len() {
            return;
        }

        if let Some((aligned_start, aligned_len)) = Self::aligned_region(start, len) {
            if aligned_len == 0 {
                return;
            }

            self.regions[self.region_count] = MemoryRegion {
                start: aligned_start,
                len: aligned_len,
            };
            self.region_count += 1;
            self.total_bytes = self.total_bytes.saturating_add(aligned_len);

            if self.region_count == 1 {
                self.next_free_addr = aligned_start;
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

            if self.next_free_addr % Self::FRAME_SIZE != 0 {
                self.next_free_addr = Self::align_up(self.next_free_addr, Self::FRAME_SIZE);
            }

            if self.next_free_addr + Self::FRAME_SIZE <= end {
                let frame = PhysFrame::from_start_address(self.next_free_addr, Self::FRAME_SIZE);
                self.next_free_addr += Self::FRAME_SIZE;
                return Some(frame);
            } else {
                self.current_region += 1;
                self.next_free_addr = 0;
            }
        }
    }

    pub fn total_frames(&self) -> u64 {
        self.total_bytes / Self::FRAME_SIZE
    }

    fn aligned_region(start: u64, len: u64) -> Option<(u64, u64)> {
        let end = start.checked_add(len)?;
        let aligned_start = Self::align_up(start, Self::FRAME_SIZE);
        let aligned_end = Self::align_down(end, Self::FRAME_SIZE);
        if aligned_end <= aligned_start {
            return None;
        }
        Some((aligned_start, aligned_end - aligned_start))
    }

    fn align_up(addr: u64, align: u64) -> u64 {
        if align == 0 {
            return addr;
        }
        let rem = addr % align;
        if rem == 0 {
            addr
        } else {
            addr.saturating_add(align - rem)
        }
    }

    fn align_down(addr: u64, align: u64) -> u64 {
        if align == 0 {
            return addr;
        }
        addr - (addr % align)
    }
}
