#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64 as imp;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "loongarch64", target_arch = "riscv64")))]
pub mod dummy {
    pub mod paging {
        use crate::memory::boot_frame_alloc::BootFrameAllocator;
        use crate::memory::paging::PageFlags;
        use crate::memory::frame_alloc::PhysFrame;

        pub fn init(_offset: u64) {}
        
        pub fn map_bootheap_page(_virt: u64, _phys: u64, _allocator: &mut BootFrameAllocator) {
             // For now, no-op.
        }
        
        pub fn test_paging() {
            // No-op
        }

        pub fn tlb_flush_page(_virt: u64) {}

        pub struct AddressSpace;
        impl AddressSpace {
            pub fn active() -> Self { Self }
            pub fn map_page(&mut self, _virt: u64, _phys: PhysFrame, _flags: PageFlags) -> Result<(), ()> { Ok(()) }
        }
    }

    pub mod task {
        #[repr(C)]
        #[derive(Debug, Default)]
        pub struct ArchContext { _d: u64 }
        pub unsafe fn context_switch(_o: *mut ArchContext, _n: *const ArchContext) {}
        pub fn context_init(_c: &mut ArchContext, _k: u64, _e: extern "C" fn(usize)->!, _a: usize) {}
    }
}
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "loongarch64", target_arch = "riscv64")))]
pub use dummy as imp;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64 as imp;

#[cfg(target_arch = "loongarch64")]
pub mod loongarch64;
#[cfg(target_arch = "loongarch64")]
pub use loongarch64 as imp;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "riscv64")]
pub use riscv64 as imp;

