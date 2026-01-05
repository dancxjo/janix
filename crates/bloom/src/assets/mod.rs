use thing_std::*;
use models::*;

pub mod bmp;
pub mod cursor;

pub fn map_bytespace(bs_id: ThingId, vaddr: u64, len: u64) -> &'static [u8] {
    thing_std::memory::space_map(bs_id, vaddr, 0, len);
    unsafe { core::slice::from_raw_parts(vaddr as *const u8, len as usize) }
}
