use abi::ids::ThingId;
use thing_std::memory;

pub mod bmp;
pub mod cursor;
pub mod bitmap;

/// Map a bytespace and return a slice to its contents.
/// Returns None if mapping fails.
pub fn map_bytespace_checked(bs_id: ThingId, vaddr: u64, len: u64) -> Option<&'static [u8]> {
    let result = memory::space_map(bs_id, vaddr, 0, len);
    if result != vaddr {
        // Mapping failed - result is either 0 or a different address
        thing_std::log_info(&alloc::format!(
            "BLOOM: map_bytespace failed: requested {:#x}, got {:#x}", 
            vaddr, result
        ));
        return None;
    }
    // SAFETY: space_map succeeded, memory at vaddr is now valid
    Some(unsafe { core::slice::from_raw_parts(vaddr as *const u8, len as usize) })
}

/// Map a bytespace and return a slice to its contents.
/// Panics if mapping fails (use map_bytespace_checked for graceful handling).
#[deprecated(note = "use map_bytespace_checked for safer mapping")]
pub fn map_bytespace(bs_id: ThingId, vaddr: u64, len: u64) -> &'static [u8] {
    map_bytespace_checked(bs_id, vaddr, len)
        .expect("map_bytespace: space_map failed")
}

pub fn load_wallpaper() -> Option<(bmp::Wallpaper, u32)> {
    let id = thing_std::thing_find(theme::current::WALLPAPER_BYTESPACE)?;
    // We don't know the exact size, but we can map a generous amount or read the size from the Thing?
    // The Thing graph might have size metadata, but for now we map a safe chunk.
    // 3MB should be enough for the small clouds.bmp (it was 3MB for the big one, smaller one is ~400KB).
    // Let's map 4MB to be safe.
    let mapping = map_bytespace_checked(id, 0x8600_0000, 4 * 1024 * 1024)?;
    
    if let Some(wallpaper) = bmp::parse_bmp(mapping) {
        let color = bmp::calculate_dominant_color(&wallpaper);
        Some((wallpaper, color))
    } else {
        None
    }
}
