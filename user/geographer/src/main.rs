#![no_std]
#![no_main]

#[cfg(target_os = "none")]
extern crate alloc;

// Panic handler is provided by thing_os::panic

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::slice;

use abi::{KernelRequest, KernelResponse, MapFlags, PixelFormat, SharedBufferInfo, ThingId};
use thing_os::prelude::*;
use thing_os::{
    add_link, create_thing, graph_kinds, list_things_by_kind, register_schema_for, update_props,
    DisplayThing, ProcessThing, Surface, Thing, Window,
};
use thing_os::{PropKey, PropType, PropValue};

mod text;
use text::draw_text_simple;

#[thing_os::main]
fn main() {
    println!("geographer: starting");

    // 1. Create a Pixel-Based Window
    let mut win = SimpleWindow::new("Geographer").expect("Failed to create window");
    println!("geographer: window created");

    let mut nodes: Vec<ThingId> = Vec::new();
    let mut links: Vec<LinkThing> = Vec::new();

    loop {
        // --- 2. Update Graph State ---
        nodes.clear();
        links.clear();

        // Simple Discovery
        let processes: Vec<ProcessThing> = list_things_by_kind();
        for p in processes {
            nodes.push(p.id);
        }

        // Just counting links for now
        let all_links: Vec<LinkThing> = list_things_by_kind();
        for l in all_links {
            links.push(l);
        }

        // --- 3. Render ---
        // Clear background
        fill_rect(
            win.buffer,
            win.stride,
            win.width,
            win.height,
            0,
            0,
            win.width as i32,
            win.height as i32,
            0xFF202020,
        );

        // Header
        let stats = format!(
            "System Graph | Nodes: {}  Links: {}",
            nodes.len(),
            links.len()
        );
        draw_text_simple(
            win.buffer, win.stride, win.width, win.height, 10, 10, &stats, 0xFFFFFFFF,
        );

        // List Nodes
        let mut y = 40;
        for (i, node) in nodes.iter().enumerate() {
            if y > win.height as i32 - 20 {
                break;
            }
            let s = format!("Process Node #{}: ID({})", i, node.0);
            draw_text_simple(
                win.buffer, win.stride, win.width, win.height, 20, y, &s, 0xFF00FF00,
            );
            y += 20;
        }

        // Present/Flush?
        // SimpleWindow logic maps buffer to surface. Surface updates might need a "present" prop or just modifying buffer is enough if kernel scans out.
        // But for composited windows, we usually need to signal update?
        // Or if it's shared buffer, compositor reads it. Compositor usually redraws on vsync.
        // We might want to sleep.

        sleep(Duration::from_millis(1000));
    }
}

// --- Minimal Graph Queries ---

pub struct LinkThing {
    pub id: ThingId,
}

impl Thing for LinkThing {
    const KIND: &'static str = "Link"; // Only if Link things exist in graph?
                                       // Actually Link is usually an edge, not a Thing.
                                       // Wait, original code had: impl Thing for LinkThing { const KIND: &'static str = "Link"; ... }
                                       // If "Link" things exist, fine. If they are edges, list_things_by_kind won't find them unless they are reified as Things.
                                       // Abi defines `KernelRequest::AddLink`. The link itself might not be a Thing unless explicitly created as one.
                                       // But original code assumed it. I'll keep it.
    const DESCRIPTION: &'static str = "";
    fn schema() -> &'static [(&'static str, PropType)] {
        &[]
    }
    fn to_props(&self, _: &mut Vec<(PropKey, PropValue)>) {}
    fn from_props(id: ThingId, _: &[Option<(PropKey, PropValue)>]) -> Self {
        LinkThing { id }
    }
}

// Reuse ProcessThing from thing_os::ProcessThing if available, or define local wrapper
// thing_os::prelude exports ProcessThing.

// --- Drawing Helper ---

fn fill_rect(
    buffer: &mut [u8],
    stride: u32,
    _width: u32,
    height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
) {
    let b_r = (color & 0xFF) as u8;
    let b_g = ((color >> 8) & 0xFF) as u8;
    let b_b = ((color >> 16) & 0xFF) as u8;
    let b_a = ((color >> 24) & 0xFF) as u8;

    for row in 0..h {
        let py = y + row;
        if py < 0 || py as u32 >= height {
            continue;
        }
        for col in 0..w {
            let px = x + col;
            if px < 0 {
                continue;
            }
            let offset = (py as u32 * stride + px as u32 * 4) as usize;
            if offset + 4 <= buffer.len() {
                buffer[offset] = b_r;
                buffer[offset + 1] = b_g;
                buffer[offset + 2] = b_b;
                buffer[offset + 3] = b_a;
            }
        }
    }
}

// --- SimpleWindow Port ---

pub struct SimpleWindow {
    pub buffer: &'static mut [u8],
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub window_id: ThingId,
}

impl SimpleWindow {
    pub fn new(title: &str) -> Option<Self> {
        println!("SimpleWindow: new called");
        // 1. Ensure schemas
        let _ = register_schema_for::<Window>();
        let _ = register_schema_for::<Surface>();

        // 2. Detect Resolution (simplistic: pick first display)
        let displays: Vec<DisplayThing> = list_things_by_kind();
        if displays.is_empty() {
            return None;
        }
        let display = &displays[0];
        let width_u32 = display.width as u32;
        let height_u32 = display.height as u32;
        println!(
            "SimpleWindow: display selected {}x{}",
            width_u32, height_u32
        );

        // 3. Find Place
        println!("SimpleWindow: finding place...");
        let place_id = thing_os::active_mode()
            .or_else(|| thing_os::default_mode())
            .and_then(|m| m.place_id)?;
        println!("SimpleWindow: place_id found {:?}", place_id);

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

        let window_id = create_thing(&window)?;
        let _ = add_link(place_id, graph_kinds::LINK_PLACE_WINDOW, window_id);

        // 5. Create SharedBuffer
        // Need raw syscall or thing_os wrapper.
        // thing_os doesn't seem to expose arbitrary syscall wrapper easily but it has `sys` module?
        // Or I can use `thing_os::syscalls::syscall`.

        let buffer_id = match thing_os::syscalls::syscall(KernelRequest::CreateSharedBuffer {
            width: width_u32,
            height: height_u32,
            pixel_format: PixelFormat::Rgba8888,
        }) {
            KernelResponse::SharedBufferCreated { buffer_id } => buffer_id,
            _ => return None,
        };

        // 6. Map SharedBuffer
        let (buffer_ptr, buffer_size) =
            match thing_os::syscalls::syscall(KernelRequest::MapSharedBuffer {
                buffer_id,
                flags: MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER),
            }) {
                KernelResponse::SharedBufferMapped { vaddr, size } => {
                    (vaddr as *mut u8, size as usize)
                }
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

        let surface_id = create_thing(&surface)?;
        let _ = add_link(window_id, graph_kinds::LINK_WINDOW_SURFACE, surface_id);

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
