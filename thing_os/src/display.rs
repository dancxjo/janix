#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use alloc::string::{String, ToString};

use abi::{MapFlags, SharedBufferInfo, ThingId};
use alloc::vec::Vec;


use crate::syscalls::syscall;
use crate::{link_targets, list_things_by_kind};
use crate::graph_kinds;

#[derive(Debug)]
pub struct SharedBufferMapping {
    pub id: ThingId,
    pub info: SharedBufferInfo,
    pub ptr: *mut u8,
    pub size: usize,
}

#[derive(Debug)]
/// Represents a mapped primary display buffer (scanout) that a userland process can draw into.
/// Single-buffered configuration.
pub struct PrimaryDisplayBuffer {
    pub display_id: ThingId,
    pub buffer: SharedBufferMapping,
    pub info: SharedBufferInfo,
    pub ptr: *mut u8,
}

impl PrimaryDisplayBuffer {
    // Legacy helper: update_active_index is now a no-op since there's only one buffer
    pub fn update_active_index(&mut self, _index: i64) {
        // No-op
    }

    pub fn front_buffer(&self) -> &SharedBufferMapping {
        &self.buffer
    }

    // Legacy support: back_buffer returns the same buffer
    pub fn back_buffer(&self) -> &SharedBufferMapping {
        &self.buffer
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
pub fn shared_buffer_info(buffer_id: ThingId) -> Result<SharedBufferInfo, crate::SysError> {
    match syscall(abi::KernelRequest::GetSharedBufferInfo { buffer_id }) {
        abi::KernelResponse::SharedBufferInfoResponse { info } => Ok(info),
        abi::KernelResponse::Error { err: _ } => {
            Err(crate::SysError::Kernel("shared_buffer_info failed"))
        }
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
                return Err(crate::SysError::Kernel(
                    "Kernel returned NULL for shared buffer mapping",
                ));
            }
            Ok((vaddr as *mut u8, size as usize))
        }
        abi::KernelResponse::Error { err: _ } => {
            Err(crate::SysError::Kernel("shared_buffer_map failed"))
        }
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
        abi::KernelResponse::Error { err: _ } => {
            Err(crate::SysError::Kernel("create_shared_buffer failed"))
        }
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

    // Try finding the scanout directly (preferred for single-buffer/compositor-owned backbuffer mode)
    let mut targets = link_targets(display.id, graph_kinds::LINK_DISPLAY_SCANOUT);
    
    let buffer_id = targets.pop().ok_or({
        use crate::println;
        println!("open_primary_display: no scanout buffer found for display {}", display.id.0);
        crate::SysError::Unexpected
    })?;

    let flags = MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER);
    let mapping = map_display_buffer(buffer_id, flags).map_err(|e| {
        use crate::println;
        println!(
            "open_primary_display: failed to map buffer {:?}: {:?}",
            buffer_id, e
        );
        e
    })?;

    if mapping.ptr.is_null() {
        return Err(crate::SysError::Kernel(
            "Primary display buffer mapped to NULL",
        ));
    }

    let ptr = mapping.ptr;
    let info = mapping.info;

    use crate::println;
    println!(
        "DEBUG: open_primary_display: ptr={:p} w={} h={}",
        ptr, info.width, info.height
    );

    Ok(PrimaryDisplayBuffer {
        display_id: display.id,
        buffer: mapping,
        info,
        ptr,
    })
}

// Deprecated/No-op
pub fn swap_display_buffers(_display_id: ThingId) -> Option<i64> {
    // Always index 0
    Some(0)
}
