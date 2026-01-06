use abi::ids::ThingId;
use thing_std::memory;

pub mod bmp;
pub mod cursor;


pub fn map_bytespace(bs_id: ThingId, vaddr: u64, len: u64) -> &'static [u8] {
    memory::space_map(bs_id, vaddr, 0, len);
    unsafe { core::slice::from_raw_parts(vaddr as *const u8, len as usize) }
}

pub fn load_wallpaper() -> Option<(bmp::Wallpaper, u32)> {
    let id = thing_std::thing_find("bytespace.asset.clouds.bmp")?;
    // We don't know the exact size, but we can map a generous amount or read the size from the Thing?
    // The Thing graph might have size metadata, but for now we map a safe chunk.
    // 3MB should be enough for the small clouds.bmp (it was 3MB for the big one, smaller one is ~400KB).
    // Let's map 4MB to be safe.
    let mapping = map_bytespace(id, 0x8600_0000, 4 * 1024 * 1024);
    
    if let Some(wallpaper) = bmp::parse_bmp(mapping) {
        let color = bmp::calculate_dominant_color(&wallpaper);
        Some((wallpaper, color))
    } else {
        None
    }
}
