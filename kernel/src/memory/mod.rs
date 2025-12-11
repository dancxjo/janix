pub mod arena;
pub mod boot_frame_allocator;
pub mod frame_pool;
pub mod hhdm;

pub use boot_frame_allocator::{BootFrameAllocator, PhysFrame};
pub use frame_pool::{allocate_frame, frame_stats, free_frame, init_frame_pool};
pub use hhdm::{get_hhdm_offset, phys_to_virt, set_hhdm_offset};
