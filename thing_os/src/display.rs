#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use alloc::string::{String, ToString};

use abi::{MapFlags, SharedBufferInfo, ThingId};
use crate::PropValue;
use crate::graph_kinds;
use thing_models::graph_kinds::{
    LINK_DISPLAY_HAS_FRONT_BUFFER, LINK_DISPLAY_HAS_BACK_BUFFER, PROP_DISPLAY_ACTIVE_BUFFER_INDEX
};
use crate::sys::raw_syscall;
use alloc::vec::Vec;

use crate::syscalls::syscall;
use crate::{link_targets, list_things_by_kind, update_props};
use crate::DisplayThing;

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

fn map_display_buffer(
    buffer_id: ThingId,
    flags: MapFlags,
) -> Result<SharedBufferMapping, crate::SysError> {
    let info = shared_buffer_info(buffer_id)?;
    let (ptr, size) = shared_buffer_map(buffer_id, flags)?;
    Ok(SharedBufferMapping {
        id: buffer_id,
        info,
        ptr,
        size,
    })
}

/// Query metadata for a shared buffer Thing.
pub fn shared_buffer_info(
    buffer_id: ThingId,
) -> Result<SharedBufferInfo, crate::SysError> {
    match syscall(abi::KernelRequest::GetSharedBufferInfo { buffer_id }) {
        abi::KernelResponse::SharedBufferInfoResponse { info } => Ok(info),
        abi::KernelResponse::Error { err: _ } => Err(crate::SysError::Kernel("shared_buffer_info failed")),
        _ => Err(crate::SysError::Unexpected),
    }
}

/// Map a shared buffer into the calling address space.
pub fn shared_buffer_map(
    buffer_id: ThingId,
    flags: MapFlags,
) -> Result<(*mut u8, usize), crate::SysError> {
    match syscall(abi::KernelRequest::MapSharedBuffer { buffer_id, flags }) {
        abi::KernelResponse::SharedBufferMapped { vaddr, size } => {
            if vaddr == 0 {
                return Err(crate::SysError::Kernel("Kernel returned NULL for shared buffer mapping"));
            }
            Ok((vaddr as *mut u8, size as usize))
        },
        abi::KernelResponse::Error { err: _ } => Err(crate::SysError::Kernel("shared_buffer_map failed")),
        _ => Err(crate::SysError::Unexpected),
    }
}

/// Create a new shared buffer compatible with display scanout.
pub fn create_shared_buffer(
    width: u32,
    height: u32,
    pixel_format: abi::PixelFormat,
) -> Result<ThingId, crate::SysError> {
    match syscall(abi::KernelRequest::CreateSharedBuffer {
        width,
        height,
        pixel_format,
    }) {
        abi::KernelResponse::SharedBufferCreated { buffer_id } => Ok(buffer_id),
        abi::KernelResponse::Error { err: _ } => Err(crate::SysError::Kernel("create_shared_buffer failed")),
        _ => Err(crate::SysError::Unexpected),
    }
}

/// Locate and map the kernel's primary display buffer.
pub fn open_primary_display_buffer() -> Result<PrimaryDisplayBuffer, crate::SysError> {
    let displays: Vec<crate::DisplayThing> = list_things_by_kind();
    let display = displays
        .iter()
        .find(|d| d.name == "display0")
        .or_else(|| displays.first())
        .cloned()
        .ok_or(crate::SysError::Unexpected)?;

    let mut front_targets = link_targets(display.id, LINK_DISPLAY_HAS_FRONT_BUFFER);
    let mut back_targets = link_targets(display.id, LINK_DISPLAY_HAS_BACK_BUFFER);
    let front_id = front_targets.pop().ok_or(crate::SysError::Unexpected)?;
    let back_id = back_targets.pop().ok_or(crate::SysError::Unexpected)?;

    let flags = MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER);
    let front_map = map_display_buffer(front_id, flags).map_err(|e| {
        use crate::println;
        println!("open_primary_display: failed to map front buffer {:?}: {:?}", front_id, e);
        e
    })?;
    let back_map = map_display_buffer(back_id, flags).map_err(|e| {
        use crate::println;
        println!("open_primary_display: failed to map back buffer {:?}: {:?}", back_id, e);
        e
    })?;

    if front_map.ptr.is_null() || back_map.ptr.is_null() {
         return Err(crate::SysError::Kernel("Primary display buffer mapped to NULL"));
    }

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
    use crate::println;
    println!("DEBUG: open_primary_display: front_ptr={:p} back_ptr={:p}", primary.buffers[0].ptr, primary.buffers[1].ptr);
    primary.sync_back_buffer();
    println!("DEBUG: open_primary_display: synced ptr={:p} idx={}", primary.ptr, primary.active_buffer_index);
    Ok(primary)
}

pub fn swap_display_buffers(display_id: ThingId) -> Option<i64> {
    let mut display = crate::load_thing::<crate::DisplayThing>(display_id)?;
    let current = PrimaryDisplayBuffer::clamp_active_index(display.active_buffer_index);
    let next = 1 - current;
    let updates = [(
        PROP_DISPLAY_ACTIVE_BUFFER_INDEX.to_string(),
        PropValue::I64(next),
    )];
    if update_props(display_id, &updates) {
        display.active_buffer_index = next;
        Some(next)
    } else {
        None
    }
}
