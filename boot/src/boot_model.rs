use kernel_core::log;
use kernel_core::memory::{BootFrameAllocator, init_frame_pool};
use kernel_core::model;
use limine::memory_map::EntryType;
use limine::request::{HhdmRequest, MemoryMapRequest, MpRequest};

#[used]
#[unsafe(link_section = ".requests")]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static MP_REQUEST: MpRequest = MpRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

pub fn seed_memory_graph_from_limine() {
    let Some(response) = MEMORY_MAP_REQUEST.get_response() else {
        log("No Limine memory map; skipping memory graph seeding");
        return;
    };

    let hhdm_offset = if let Some(hhdm) = HHDM_REQUEST.get_response() {
        hhdm.offset()
    } else {
        0
    };

    let mut heap_initialized = false;
    let mut boot_allocator = BootFrameAllocator::new();

    for entry in response.entries() {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }

        let mut base = entry.base;
        let mut len = entry.length;

        if !heap_initialized && len >= 2 * 1024 * 1024 {
            let heap_size = 1024 * 1024; // 1 MiB
            let heap_start_phys = base;
            let heap_start_virt = (heap_start_phys as u64 + hhdm_offset) as usize;

            unsafe {
                crate::heap::KERNEL_ALLOCATOR.init(heap_start_virt, heap_size);
            }

            log("Initialized kernel heap (1MiB)");

            base += heap_size as u64;
            len -= heap_size as u64;
            heap_initialized = true;
        }

        boot_allocator.add_region(base, len);

        let frame_size = 4096;

        // For now: one pool per usable region.
        let _pool = model::create_frame_pool(base, base + len, frame_size);

        // In future, you can optionally explode this into many PhysFrame Things.
    }

    init_frame_pool(boot_allocator);

    log("Seeded memory graph from Limine memory map");
}

pub fn seed_cpu_graph_from_limine() {
    if let Some(resp) = MP_REQUEST.get_response() {
        let cores = resp.cpus().len() as u64;
        for idx in 0..cores {
            let _ = model::create_cpu_core(idx);
        }
        log("Seeded CpuCore Things from Limine SMP");
    } else {
        // Fallback: single core
        let _ = model::create_cpu_core(0);
        log("No SMP info; created single CpuCore(0)");
    }
}

pub fn seed_boot_profile() {
    model::init_boot_profile();
}
