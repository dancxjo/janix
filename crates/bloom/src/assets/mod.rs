use abi::ids::ThingId;
use thing_std::memory;
use thing_std::log_info;
use alloc::format;
use core::slice;

pub mod bitmap;
pub mod cursor;
pub mod bmp;

pub fn map_bytespace_checked(bs_id: ThingId, vaddr: u64, len: u64) -> Option<&'static [u8]> {
    let result = memory::space_map(bs_id, vaddr, 0, len);
    if result != vaddr {
        // Mapping failed - result is either 0 or a different address
        log_info(&format!(
            "BLOOM: map_bytespace failed: requested {:#x}, got {:#x}", 
            vaddr, result
        ));
        return None;
    }
    
    // --- BLOOM PANIC TRAP ---
    if result == 0 {
        log_info("BLOOM BUG: map_bytespace_checked: result is 0");
        return None;
    }
    if result % 1 != 0 {
        // No-op for u8
    }

    // SAFETY: space_map succeeded, memory at vaddr is now valid
    // Check length constraint for slice::from_raw_parts
    if len > isize::MAX as u64 {
        log_info(&format!("BLOOM BUG: map_bytespace_checked: len too large {:#x}", len));
        return None;
    }
    Some(unsafe { slice::from_raw_parts(result as *const u8, len as usize) })
}
