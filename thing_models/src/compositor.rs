//! Compositor-related Thing models.

extern crate alloc;

use abi::ThingId;
use alloc::string::String;
use alloc::vec::Vec;
use crate::{PropKey, PropType, PropValue};
use thing_macros::Thing;

#[derive(Thing, Clone, Debug)]
#[thing(description = "User intent to render a surface frame to the screen")]
pub struct FrameRenderIntent {
    pub id: ThingId,
    pub surface_id: ThingId,
    pub frame_index: u64,
    pub width: u64,
    pub height: u64,
    pub stride: u64, // Stride of the source buffer
    pub format: String,
    pub timestamp: u64,
    pub completed: bool,
}
