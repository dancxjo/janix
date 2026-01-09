use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;
use spin::Mutex;
use crate::memory::frame_alloc::{FRAME_ALLOCATOR, FRAME_SIZE};
use crate::memory::layout::{KHEAP_BASE, KHEAP_SIZE};
use crate::arch::imp::paging::{AddressSpace, tlb_flush_page};
use crate::memory::paging::PageFlags;
use crate::kinfo;

struct HeapState {
    heap_start: u64,
    heap_end_committed: u64,
}

/// A wrapper around LockedHeap that supports growing the heap on demand.
pub struct KernelHeap {
    heap: LockedHeap,
    state: Mutex<HeapState>,
}

impl KernelHeap {
    /// Create a new, empty KernelHeap.
    pub const fn empty() -> Self {
        Self {
            heap: LockedHeap::empty(),
            state: Mutex::new(HeapState {
                heap_start: KHEAP_BASE,
                heap_end_committed: KHEAP_BASE,
            }),
        }
    }

    /// Initialize the heap with a small initial size.
    pub fn init(&self, initial_pages: u64) {
        self.grow(initial_pages).expect("Failed to initialize kernel heap");
    }

    /// Grow the heap by `pages` count.
    pub fn grow(&self, pages: u64) -> Result<(), ()> {
        let mut state = self.state.lock();
        
        let phys_range = FRAME_ALLOCATOR.with_lock(|alloc| {
             alloc.alloc_contiguous(pages)
        }).ok_or(())?;
        
        let size = pages * FRAME_SIZE;
        let start_virt = state.heap_end_committed;
        
        let mut active_table = AddressSpace::active();
        let flags = PageFlags::PRESENT | PageFlags::WRITABLE | PageFlags::NO_EXECUTE | PageFlags::GLOBAL; 
        
        for i in 0..pages {
            let virt = start_virt + i * FRAME_SIZE;
            let phys = crate::memory::frame_alloc::PhysFrame(phys_range.base.0 + i * FRAME_SIZE);
            if let Err(_) = active_table.map_page(virt, phys, flags) {
                panic!("KernelHeap: Failed to map page");
            }
            tlb_flush_page(virt);
        }
        
        unsafe {
            let start_ptr = start_virt as *mut u8;
            let mut heap = self.heap.lock();
            if state.heap_end_committed == KHEAP_BASE {
                 heap.init(start_ptr, size as usize);
            } else {
                 heap.extend(size as usize);
            }
        }
        
        crate::kinfo!("kheap: grew by {} pages (phys={:#x}, virt={:#x})", pages, phys_range.base.0, start_virt);
        state.heap_end_committed += size;
        Ok(())
    }

    pub fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { self.heap.alloc(layout) }
    }

    pub fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.heap.dealloc(ptr, layout) }
    }
    
    pub fn committed_bytes(&self) -> u64 {
        let state = self.state.lock();
        state.heap_end_committed - state.heap_start
    }
    
    pub fn reserved_bytes(&self) -> u64 {
        KHEAP_SIZE
    }
    
    pub fn stats(&self) {
        let state = self.state.lock();
        kinfo!("kheap: reserved={} committed={}", KHEAP_SIZE, state.heap_end_committed - state.heap_start);
    }
}
