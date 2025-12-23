//! Display/scanout-related Thing models.

extern crate alloc;

use abi::ThingId;
use alloc::string::String;
use thing_macros::Thing;

#[derive(Thing, Clone, Debug)]
#[thing(description = "A display sink capable of scanning out a SharedBuffer")]
pub struct Display {
    pub id: ThingId,
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub active_buffer_index: i64,
}

#[derive(Thing, Clone, Debug)]
#[thing(description = "A kernel-owned shared memory buffer that can be mapped into userland")]
pub struct SharedBuffer {
    pub id: ThingId,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub pixel_format: String,
}

#[derive(Thing, Clone, Debug)]
#[thing(description = "A single frame produced by a compositor targeting a framebuffer")]
pub struct DisplayFrame {
    pub id: ThingId,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub pixel_format: String,
}
