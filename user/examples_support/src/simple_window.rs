use alloc::string::ToString;
use core::slice;
use thing_os::prelude::*;
use abi::{PixelFormat, MapFlags, KernelRequest, KernelResponse, graph_kinds};
use thing_os::{Window, Surface, create_thing, list_things_by_kind, register_schema_for};
use thing_os::DisplayThing;

pub struct SimpleWindow {
    pub buffer: &'static mut [u8],
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub window_id: ThingId,
}

impl SimpleWindow {
    pub fn new<S: Sys>(sys: &mut S, title: &str) -> Option<Self> {
        // 1. Ensure schemas
        let _ = register_schema_for::<Window>(sys);
        let _ = register_schema_for::<Surface>(sys);

        // 2. Detect Resolution (simplistic: pick first display)
        let displays: alloc::vec::Vec<DisplayThing> = list_things_by_kind(sys);
        if displays.is_empty() {
            return None;
        }
        let display = &displays[0];
        let width_u32 = display.width as u32;
        let height_u32 = display.height as u32;

        // 3. Find Place
        let place_id = thing_os::active_mode(sys)
            .or_else(|| thing_os::default_mode(sys))
            .and_then(|m| m.place_id)?;

        // 4. Create Window
        let window = Window {
            id: ThingId(0),
            place_id,
            x: 0,
            y: 0,
            width: width_u32 as i32,
            height: height_u32 as i32,
            z_index: -1, // Background layer
            active: true,
            title: title.to_string(),
            draggable: false,
            resizable: false,
            closable: false,
            minimizable: false,
        };

        let window_id = create_thing(sys, &window)?;
        let _ = thing_os::add_link(sys, place_id, graph_kinds::LINK_PLACE_WINDOW, window_id);

        // 5. Create SharedBuffer
        let buffer_id = match sys.syscall(KernelRequest::CreateSharedBuffer {
            width: width_u32,
            height: height_u32,
            pixel_format: PixelFormat::Rgba8888,
        }) {
            KernelResponse::SharedBufferCreated { buffer_id } => buffer_id,
            _ => return None,
        };

        // 6. Map SharedBuffer
        let (buffer_ptr, buffer_size) = match sys.syscall(KernelRequest::MapSharedBuffer {
            buffer_id,
            flags: MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER),
        }) {
            KernelResponse::SharedBufferMapped { vaddr, size } => (vaddr as *mut u8, size as usize),
            _ => return None,
        };

        // 7. Create Surface wrapping this buffer
        let surface = Surface {
            id: ThingId(0),
            window_id,
            kind: "raw_buffer".to_string(),
            text: "".to_string(),
            width: width_u32 as u64,
            height: height_u32 as u64,
            stride: (width_u32 * 4) as u64,
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
        let buffer = unsafe { slice::from_raw_parts_mut(buffer_ptr, buffer_size) };
        buffer.fill(0);

        Some(SimpleWindow {
            buffer,
            width: width_u32,
            height: height_u32,
            stride: width_u32 * 4,
            window_id,
        })
    }
}
