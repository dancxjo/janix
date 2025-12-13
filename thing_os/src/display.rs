#[cfg(target_os = "none")]
use alloc::string::String;
#[cfg(not(target_os = "none"))]
use std::string::String;

use abi::{MapFlags, PropValue, SharedBufferInfo, ThingId};
use runtime::Sys;

use crate::syscalls::{link_targets, list_things_by_kind, update_props};
use crate::things::DisplayThing;

#[derive(Debug)]
pub struct SharedBufferMapping {
    pub id: ThingId,
    pub info: SharedBufferInfo,
    pub ptr: *mut u8,
    pub size: usize,
}

#[derive(Debug)]
/// Represents a mapped primary display buffer that a userland process can draw into.
pub struct PrimaryDisplayBuffer {
    pub display_id: ThingId,
    pub buffers: [SharedBufferMapping; 2],
    pub active_buffer_index: i64,
    pub info: SharedBufferInfo,
    pub ptr: *mut u8,
}

impl PrimaryDisplayBuffer {
    fn clamp_active_index(value: i64) -> i64 {
        if value == 1 { 1 } else { 0 }
    }

    fn front_index(value: i64) -> usize {
        match Self::clamp_active_index(value) {
            1 => 1,
            _ => 0,
        }
    }

    fn back_index(value: i64) -> usize {
        1 - Self::front_index(value)
    }

    fn sync_back_buffer(&mut self) {
        let idx = Self::back_index(self.active_buffer_index);
        let slot = &self.buffers[idx];
        self.ptr = slot.ptr;
        self.info = slot.info;
    }

    pub fn back_buffer(&self) -> &SharedBufferMapping {
        &self.buffers[Self::back_index(self.active_buffer_index)]
    }

    pub fn front_buffer(&self) -> &SharedBufferMapping {
        &self.buffers[Self::front_index(self.active_buffer_index)]
    }

    pub fn update_active_index(&mut self, index: i64) {
        self.active_buffer_index = Self::clamp_active_index(index);
        self.sync_back_buffer();
    }
}

fn map_display_buffer<S: Sys>(
    sys: &mut S,
    buffer_id: ThingId,
    flags: MapFlags,
) -> Result<SharedBufferMapping, crate::things::SysError> {
    let info = shared_buffer_info(sys, buffer_id)?;
    let (ptr, size) = shared_buffer_map(sys, buffer_id, flags)?;
    Ok(SharedBufferMapping {
        id: buffer_id,
        info,
        ptr,
        size,
    })
}

/// Query metadata for a shared buffer Thing.
pub fn shared_buffer_info(
    sys: &impl Sys,
    buffer_id: ThingId,
) -> Result<SharedBufferInfo, crate::things::SysError> {
    match sys.syscall(abi::KernelRequest::GetSharedBufferInfo { buffer_id }) {
        abi::KernelResponse::SharedBufferInfoResponse { info } => Ok(info),
        abi::KernelResponse::Error { message } => Err(crate::things::SysError::Kernel(message)),
        _ => Err(crate::things::SysError::Unexpected),
    }
}

/// Map a shared buffer into the calling address space.
pub fn shared_buffer_map(
    sys: &impl Sys,
    buffer_id: ThingId,
    flags: MapFlags,
) -> Result<(*mut u8, usize), crate::things::SysError> {
    match sys.syscall(abi::KernelRequest::MapSharedBuffer { buffer_id, flags }) {
        abi::KernelResponse::SharedBufferMapped { vaddr, size } => Ok((vaddr as *mut u8, size as usize)),
        abi::KernelResponse::Error { message } => Err(crate::things::SysError::Kernel(message)),
        _ => Err(crate::things::SysError::Unexpected),
    }
}

/// Locate and map the kernel's primary display buffer.
pub fn open_primary_display_buffer<S: Sys>(sys: &mut S) -> Result<PrimaryDisplayBuffer, crate::things::SysError> {
    let displays: Vec<DisplayThing> = list_things_by_kind(sys);
    let display = displays
        .iter()
        .find(|d| d.name == "display0")
        .or_else(|| displays.first())
        .cloned()
        .ok_or(crate::things::SysError::Unexpected)?;

    let mut front_targets = link_targets(sys, display.id, abi::graph_kinds::LINK_DISPLAY_HAS_FRONT_BUFFER);
    let mut back_targets = link_targets(sys, display.id, abi::graph_kinds::LINK_DISPLAY_HAS_BACK_BUFFER);
    let front_id = front_targets.pop().ok_or(crate::things::SysError::Unexpected)?;
    let back_id = back_targets.pop().ok_or(crate::things::SysError::Unexpected)?;

    let flags = MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER);
    let front_map = map_display_buffer(sys, front_id, flags)?;
    let back_map = map_display_buffer(sys, back_id, flags)?;

    let mut primary = PrimaryDisplayBuffer {
        display_id: display.id,
        buffers: [front_map, back_map],
        active_buffer_index: PrimaryDisplayBuffer::clamp_active_index(display.active_buffer_index),
        info: SharedBufferInfo {
            width: 0,
            height: 0,
            stride: 0,
            pixel_format: abi::PixelFormat::Rgba8888,
        },
        ptr: core::ptr::null_mut(),
    };
    primary.sync_back_buffer();
    Ok(primary)
}

pub fn swap_display_buffers<S: Sys>(sys: &mut S, display_id: ThingId) -> Option<i64> {
    let mut display = crate::syscalls::load_thing::<DisplayThing>(sys, display_id)?;
    let current = PrimaryDisplayBuffer::clamp_active_index(display.active_buffer_index);
    let next = 1 - current;
    let updates = [(
        abi::graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX,
        PropValue::I64(next),
    )];
    if update_props(sys, display_id, &updates) {
        display.active_buffer_index = next;
        Some(next)
    } else {
        None
    }
}
