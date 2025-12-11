use crate::memory::boot_frame_allocator::{BootFrameAllocator, PhysFrame};
use spin::Mutex;

pub struct FramePool {
    allocator: BootFrameAllocator,
    total: u64,
    used: u64,
}

impl FramePool {
    pub unsafe fn new(allocator: BootFrameAllocator) -> Self {
        let total = allocator.total_frames();
        Self {
            allocator,
            total,
            used: 0,
        }
    }

    pub fn alloc_frame(&mut self) -> Option<PhysFrame> {
        if let Some(frame) = self.allocator.allocate_frame() {
            self.used += 1;
            Some(frame)
        } else {
            None
        }
    }

    pub fn free_frame(&mut self, _frame: PhysFrame) {
        // For now, just decrement used. We don't support actual freeing yet.
        if self.used > 0 {
            self.used -= 1;
        }
    }

    pub fn stats(&self) -> (u64, u64, u64) {
        let free = self.total.saturating_sub(self.used);
        (self.total, self.used, free)
    }
}

static FRAME_POOL: Mutex<Option<FramePool>> = Mutex::new(None);

pub fn init_frame_pool(allocator: BootFrameAllocator) {
    unsafe {
        *FRAME_POOL.lock() = Some(FramePool::new(allocator));
    }
}

pub fn allocate_frame() -> Option<PhysFrame> {
    FRAME_POOL.lock().as_mut()?.alloc_frame()
}

pub fn free_frame(frame: PhysFrame) {
    if let Some(pool) = FRAME_POOL.lock().as_mut() {
        pool.free_frame(frame);
    }
}

pub fn frame_stats() -> (u64, u64, u64) {
    FRAME_POOL
        .lock()
        .as_ref()
        .map(|p| p.stats())
        .unwrap_or((0, 0, 0))
}
