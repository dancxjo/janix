use alloc::string::ToString;
use core::slice;
use thing_os::prelude::*;
use abi::{PixelFormat, MapFlags, KernelRequest, KernelResponse, graph_kinds};
use thing_os::{Window, Surface, create_thing, list_things_by_kind, register_schema_for};

pub struct WindowContext {
    pub buffer_ptr: *mut u8,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

use thing_os::DisplayThing;

pub fn create_geographer_window<S: Sys>(sys: &mut S) -> Option<WindowContext> {
    // 1. Ensure schemas
    let _ = register_schema_for::<Window>(sys);
    let _ = register_schema_for::<Surface>(sys);

    // 2. Detect Resolution
    let displays: alloc::vec::Vec<DisplayThing> = list_things_by_kind(sys);
    if displays.is_empty() {
        return None;
    }
    let display = &displays[0];
    let width_u32 = display.width as u32;
    let height_u32 = display.height as u32;
    let width_i32 = width_u32 as i32;
    let height_i32 = height_u32 as i32;

    // 3. Find Place
    let place_id = thing_os::active_mode(sys)
        .or_else(|| thing_os::default_mode(sys))
        .and_then(|m| m.place_id);
    
    let place_id = match place_id {
        Some(pid) if pid.0 != 0 => pid,
        _ => return None,
    };

    // 4. Create Window
    let window = Window {
        id: ThingId(0),
        place_id,
        x: 0,
        y: 0,
        width: width_i32,
        height: height_i32,
        z_index: -1, // Keep behind application windows

        active: true,
        title: "geographer".to_string(),
        draggable: false,
        resizable: false,
        closable: false,
        minimizable: false,
    };

    let window_id = create_thing(sys, &window)?;
    let _ = thing_os::add_link(sys, place_id, graph_kinds::LINK_PLACE_WINDOW, window_id);

    // 3. Create SharedBuffer
    let buffer_id = match sys.syscall(KernelRequest::CreateSharedBuffer {
        width: width_u32,
        height: height_u32,
        pixel_format: PixelFormat::Rgba8888,
    }) {
        KernelResponse::SharedBufferCreated { buffer_id } => buffer_id,
        _ => return None,
    };

    // 4. Map SharedBuffer
    let (buffer_ptr, _buffer_size) = match sys.syscall(KernelRequest::MapSharedBuffer {
        buffer_id,
        flags: MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER),
    }) {
        KernelResponse::SharedBufferMapped { vaddr, size } => (vaddr as *mut u8, size as usize),
        _ => return None,
    };

    // 5. Create Surface wrapping this buffer
    let surface = Surface {
        id: ThingId(0),
        window_id,
        kind: "raw_buffer".to_string(),
        text: "".to_string(),
        width: width_u32 as u64,
        height: height_u32 as u64,
        stride: (width_u32 * 4) as u64, // Assume tight packing for now as shared buffer usually is
        format: "Rgba8888".to_string(),
        shared_buffer_id: Some(buffer_id),
        refresh_interval_ns: None,
        frames_presented: None,
        last_present_ns: None,
        power_state: None,
    };

    let surface_id = create_thing(sys, &surface)?;
    let _ = thing_os::add_link(sys, window_id, graph_kinds::LINK_WINDOW_SURFACE, surface_id);

    // Initial clear
    unsafe {
        let slice = slice::from_raw_parts_mut(buffer_ptr, _buffer_size);
        slice.fill(0);
    }

    Some(WindowContext {
        buffer_ptr,
        width: width_u32,
        height: height_u32,
        stride: width_u32 * 4,
    })
}
