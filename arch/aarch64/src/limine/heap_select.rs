use limine::memory_map::EntryType;

#[derive(Debug, Clone, Copy)]
pub struct HeapRegion {
    pub phys_start: u64,
    pub size: u64,
}

pub fn pick_heap_region(
    entries: &[&limine::memory_map::Entry],
    heap_size: u64,
    min_phys: u64,
    align: u64,
) -> Option<HeapRegion> {
    // Iterate backwards to find the highest address usable region
    for entry in entries.iter().rev() {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }

        // We want to carve from the end of the region.
        // Candidate end is the end of the region.
        let region_end = entry.base + entry.length;

        // Candidate start is end - heap_size
        let raw_start = region_end.checked_sub(heap_size)?;

        // Align down
        let aligned_start = raw_start - (raw_start % align);

        // Check bounds
        if aligned_start < entry.base {
            continue; // Doesn't fit in this region
        }

        if aligned_start < min_phys {
            continue; // Below min physical address safety floor
        }

        // Found it
        return Some(HeapRegion {
            phys_start: aligned_start,
            size: heap_size,
        });
    }

    None
}
