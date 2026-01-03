pub mod space;

pub mod map;
pub mod bytespace;
pub mod heap;
pub use heap::init as init_heap_raw;
pub mod allocator;

/// Safely copy data from kernel to user memory.
/// Returns Ok(()) on success, Err(abi::syscall::err::EFAULT) on failure.
pub fn copy_to_user(dest: u64, src: &[u8]) -> Result<(), i32> {
    // Check overflow
    let end = dest.checked_add(src.len() as u64).ok_or(abi::syscall::err::EFAULT)?;
    
    // Check if in user range (Bit 63 must be 0)
    if (dest >> 63) != 0 || (end >> 63) != 0 {
         return Err(abi::syscall::err::EFAULT);
    }

    // Unsafe copy
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), dest as *mut u8, src.len());
    }
    
    Ok(())
}
