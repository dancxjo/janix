use core::alloc::Layout;
use core::ptr::null_mut;
use crate::memory::boot_frame_alloc::BootFrameAllocator;
use crate::kinfo;

// 16 MiB boot heap
pub const BOOTHEAP_VIRT_BASE: u64 = 0xFFFFFF80_40000000; // Arbitrary high kernel address
pub const BOOTHEAP_SIZE: usize = 16 * 1024 * 1024; 

pub struct BootHeap {
    allocator: Option<BootFrameAllocator>,
    
    // Virtual range state
    start: u64,
    end: u64,
    current: u64,     // Bump pointer
    committed: u64,   // How far we have backed with physical pages
    
    // Stats
    allocated_count: usize,
    allocated_bytes: usize,
}

// Global lock for the boot heap since GlobalAlloc requires Sync.
// We are single threaded in boot, but let's be safe(r) or just use a simple spinlock or similar.
// Actually, since we are strictly boot single core, a RefCell-like pattern with IrqSave is best, 
// but GlobalAlloc takes &self.
// We will use a standard spinlock wrapper in global_alloc.rs, so here we just define the inner logic.

impl BootHeap {
    pub const fn empty() -> Self {
        Self {
            allocator: None,
            start: BOOTHEAP_VIRT_BASE,
            end: BOOTHEAP_VIRT_BASE + BOOTHEAP_SIZE as u64,
            current: BOOTHEAP_VIRT_BASE,
            committed: BOOTHEAP_VIRT_BASE,
            allocated_count: 0,
            allocated_bytes: 0,
        }
    }

    pub fn init(&mut self, allocator: BootFrameAllocator) {
        self.allocator = Some(allocator);
        // We don't commit anything yet - demand commit.
        kinfo!("BootHeap initialized. Range: {:#x} - {:#x}", self.start, self.end);
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        if self.allocator.is_none() {
            // Panic or return null? Panic is better to catch early init issues.
            panic!("BootHeap::alloc called before initialization!");
        }

        let size = layout.size();
        let align = layout.align() as u64;

        // Align current pointer
        let start = align_up(self.current, align);
        let new_current = match start.checked_add(size as u64) {
            Some(x) => x,
            None => return null_mut(),
        };

        if new_current > self.end {
            panic!("BootHeap OOM! Requested {} bytes, layout {:?}, heap usage {}/{}", 
                   size, layout, self.allocated_bytes, BOOTHEAP_SIZE);
        }

        // Check if we need to commit more pages
        if new_current > self.committed {
            let needed_end = align_up(new_current, 4096);
            let current_committed = self.committed;
            
            let mut cursor = current_committed;
            while cursor < needed_end {
                let frame = self.allocator.as_mut().unwrap().alloc_frame()
                    .expect("BootFrameAllocator OOM during BootHeap expansion!");
                
                // Map the page!
                // We use the arch-specific paging implementation.
                    crate::arch::imp::paging::map_bootheap_page(cursor, frame, self.allocator.as_mut().unwrap());
                
                cursor += 4096;
            }
            self.committed = needed_end;
        }

        self.current = new_current;
        self.allocated_bytes += size;
        self.allocated_count += 1;

        start as *mut u8
    }
    
    pub fn stats(&self) {
        kinfo!("BootHeap Stats: {} allocs, {} bytes, {} committed", 
               self.allocated_count, self.allocated_bytes, self.committed - self.start);
    }
}

fn align_up(addr: u64, align: u64) -> u64 {
    (addr + align - 1) & !(align - 1)
}
