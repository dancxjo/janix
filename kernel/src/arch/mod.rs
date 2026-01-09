#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64 as imp;

#[cfg(not(target_arch = "x86_64"))]
pub mod dummy {
    pub mod paging {
        use crate::memory::boot_frame_alloc::BootFrameAllocator;
        
        pub fn init(_offset: u64) {}
        
        pub fn map_bootheap_page(_virt: u64, _phys: u64, _allocator: &mut BootFrameAllocator) {
             // For now, no-op or panic. System might crash later if it uses heap.
             // panic!("Paging not implemented for this arch"); 
        }
    }
}
#[cfg(not(target_arch = "x86_64"))]
pub use dummy as imp;
