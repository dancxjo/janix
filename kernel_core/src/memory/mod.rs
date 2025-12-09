pub mod boot_frame_allocator;
pub mod frame_pool;

pub use boot_frame_allocator::{BootFrameAllocator, PhysFrame};
pub use frame_pool::{init_frame_pool, allocate_frame, free_frame, frame_stats};
