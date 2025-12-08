use kernel_core::model;
use kernel_core::log;
use limine::request::{MemoryMapRequest, MpRequest};
use limine::memory_map::EntryType;

#[used]
#[unsafe(link_section = ".requests")]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static MP_REQUEST: MpRequest = MpRequest::new();

pub fn seed_memory_graph_from_limine() {
    let Some(response) = MEMORY_MAP_REQUEST.get_response() else {
        log("No Limine memory map; skipping memory graph seeding");
        return;
    };

    for entry in response.entries() {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }

        let base = entry.base;
        let len  = entry.length;
        let frame_size = 4096;

        // For now: one pool per usable region.
        let _pool = model::create_frame_pool(base, base + len, frame_size);

        // In future, you can optionally explode this into many PhysFrame Things.
    }

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
