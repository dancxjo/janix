use super::heap_select::pick_heap_region;

#[derive(Debug, Clone, Copy)]
pub struct HeapInitInfo {
    pub phys_start: u64,
    pub virt_start: u64,
    pub size: u64,
    #[allow(dead_code)]
    pub hhdm_offset: u64,
}

pub unsafe fn init_heap_from_limine(heap_size: u64) -> HeapInitInfo {
    let memmap = boot::limine::MEMORY_MAP_REQUEST
        .get_response()
        .expect("Limine MemoryMap request failed or unimplemented")
        .entries();

    let hhdm_offset = boot::limine::HHDM_REQUEST
        .get_response()
        .expect("Limine HHDM request failed or unimplemented")
        .offset();

    let region = pick_heap_region(memmap, heap_size, 16 * 1024 * 1024, 4096)
        .expect("Failed to find a suitable heap region in Limine memory map");

    let virt_start = region.phys_start + hhdm_offset;

    // Initialize the kernel allocator
    crate::heap::init_kernel_heap(virt_start as usize, region.size as usize);

    HeapInitInfo {
        phys_start: region.phys_start,
        virt_start,
        size: region.size,
        hhdm_offset,
    }
}
