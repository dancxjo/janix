use crate::requests::{MEMORY_MAP_REQUEST, HHDM_REQUEST, FRAMEBUFFER_REQUEST, SMP_REQUEST, MODULE_REQUEST};

mod simd;
mod serial;
mod runtime;

pub use runtime::{Runtime, hcf};
