#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64 as imp;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub mod dummy {
    pub mod paging {
        use crate::memory::boot_frame_alloc::BootFrameAllocator;
        
        pub fn init(_offset: u64) {}
        
        pub fn map_bootheap_page(_virt: u64, _phys: u64, _allocator: &mut BootFrameAllocator) {
             // For now, no-op.
        }
        
        pub fn test_paging() {
            // No-op
        }
    }
}
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub use dummy as imp;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64 as imp;
