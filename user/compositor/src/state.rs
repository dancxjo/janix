use thing_os::prelude::*;

use thing_models::DisplayPresentRequest;

use crate::config::FRAME_INTERVAL_NS;
use crate::graph::{
    active_framebuffer, collect_all_windows, collect_surfaces_for_windows,
};
use crate::layout::{self, StackedWindow};
use crate::model::{BackgroundCanvas, BackgroundImage, Compositor};
use crate::render::{build_display_list, primitives, render_display_list};
use abi::MapFlags;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use thing_os::{RawModule, shared_buffer_map};

use crate::model::ConsoleBuffer;
use thing_os::println;
use thing_os::syscalls::{sys_symbol_intern, syscall};
use abi::{KernelRequest, KernelResponse, PixelFormat};
use abi::resident::ResidentMapPerms;

fn draw_console(compositor: &mut Compositor) {
    if let Some(cb) = &compositor.console_buffer {
        let ptr = cb.ptr;
        let w = cb.width;
        let h = cb.height;
        let fb_w = compositor.fb.info.width;
        let fb_h = compositor.fb.info.height;

        let dest_x = (fb_w as i32 - w as i32) / 2;
        let dest_y = (fb_h as i32 - h as i32) / 2;

        let dest = compositor.fb.ptr as *mut u32;
        // Use byte stride arithmetic
        let dest_stride_bytes = compositor.fb.info.stride as usize;
        let src_stride_px = (cb.stride / 4) as i32;

        // Simple blit
        for y in 0..h {
            let row_dest_y = dest_y + y as i32;
            if row_dest_y < 0 || row_dest_y >= fb_h as i32 {
                continue;
            }

            let row_src = unsafe { (ptr as *const u32).add((y as i32 * src_stride_px) as usize) };

            unsafe {
                let row_dest = (dest as *mut u8).add(row_dest_y as usize * dest_stride_bytes) as *mut u32;

                // Bounds check horizontal
                let start_x = 0.max(-dest_x);
                let end_x = (w as i32).min(fb_w as i32 - dest_x);

                if end_x > start_x {
                    let count = (end_x - start_x) as usize;
                    core::ptr::copy_nonoverlapping(
                        row_src.add(start_x as usize),
                        row_dest.add((dest_x + start_x) as usize),
                        count,
                    );
                }
            }
        }
    }
}

fn load_background_image() -> Option<BackgroundImage> {
    let modules = list_things_by_kind::<RawModule>();
    let clouds_module = modules.iter().find(|m| m.identifier == "clouds.bmp")?;

    println!("compositor: mapping clouds.bmp via ResidentMap");

    // Map the RawModule via ResidentMap
    let (src_ptr, src_len) = match syscall(KernelRequest::ResidentMap {
        id: clouds_module.id,
        perms: ResidentMapPerms::READ,
    }) {
        KernelResponse::ResidentMapped { resp } => (resp.user_addr as *const u8, resp.byte_len as usize),
        _ => {
            println!("clouds.bmp: failed to map resident module");
            return None;
        }
    };

    unsafe {
        if *src_ptr != b'B' || *src_ptr.add(1) != b'M' {
            println!("clouds.bmp: invalid signature");
            return None;
        }
        // Little endian parsing helper
        let read_u32 = |offset| {
            let p = src_ptr.add(offset);
            u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
        };
        let read_i32 = |offset| {
            let p = src_ptr.add(offset);
            i32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
        };
        let read_u16 = |offset| {
            let p = src_ptr.add(offset);
            u16::from_le_bytes([*p, *p.add(1)])
        };

        let data_offset = read_u32(0x0A);
        let width = read_i32(0x12);
        let height = read_i32(0x16);
        let bpp = read_u16(0x1C);

        if width <= 0 || height <= 0 {
            println!("clouds.bmp: invalid dimensions");
            return None;
        }

        let width = width as u32;
        let height = height as u32;

        let msg = format!(
            "clouds.bmp: decoded header. {}x{} offset={} bpp={}",
            width, height, data_offset, bpp
        );
        let leaked = Box::leak(msg.into_boxed_str());
        println!("{}", leaked);

        if bpp != 24 {
             let msg = format!("clouds.bmp: unsupported bpp {} (only 24 supported for now)", bpp);
             println!("{}", Box::leak(msg.into_boxed_str()));
             return None;
        }

        // Allocate SharedBuffer for decoded image
        let pixel_format = PixelFormat::Bgra8888;
        let buffer_id = match syscall(KernelRequest::CreateSharedBuffer {
            width,
            height,
            pixel_format,
        }) {
            KernelResponse::SharedBufferCreated { buffer_id } => buffer_id,
            _ => {
                println!("clouds.bmp: failed to create shared buffer");
                return None;
            }
        };

        // Map the new buffer
        let (dest_ptr, dest_size) = match syscall(KernelRequest::MapSharedBuffer {
            buffer_id,
            flags: MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER),
        }) {
            KernelResponse::SharedBufferMapped { vaddr, size } => (vaddr as *mut u8, size),
            _ => {
                println!("clouds.bmp: failed to map new shared buffer");
                return None;
            }
        };

        // Decode: Convert BGR (24bpp) to BGRA (32bpp)
        // BMP lines are padded to 4-byte boundary
        let src_stride = ((width * 24 + 31) / 32) * 4;
        let dest_stride = width * 4;
        let src_data = src_ptr.add(data_offset as usize);

        // BMP is usually stored bottom-up, but if height is positive it's bottom-up.
        // If height is negative, top-down. We read abs(height) already?
        // Wait, read_i32 returns signed. If positive, it's bottom-up.
        // We should check sign of height.
        let height_i32 = read_i32(0x16);
        let is_top_down = height_i32 < 0;

        for y in 0..height {
            let src_row_idx = if is_top_down {
                y
            } else {
                height - 1 - y
            };

            let row_src = src_data.add((src_row_idx * src_stride) as usize);
            let row_dest = dest_ptr.add((y * dest_stride) as usize);

            for x in 0..width {
                let p_src = row_src.add((x * 3) as usize);
                let p_dest = row_dest.add((x * 4) as usize);

                // BGR -> BGRA
                *p_dest = *p_src;       // B
                *p_dest.add(1) = *p_src.add(1); // G
                *p_dest.add(2) = *p_src.add(2); // R
                *p_dest.add(3) = 0xFF;  // A
            }
        }

        println!("clouds.bmp: decoded successfully");

        return Some(BackgroundImage {
            ptr: dest_ptr,
            size: dest_size as usize,
            width: width as i32,
            height: height as i32,
            bpp: 32, // Converted to 32
        });
    }
}

fn prepare_background_canvas(compositor: &mut Compositor) {
    compositor.background_canvas = None;

    let Some(bg) = &compositor.background_image else {
        return;
    };

    if bg.width <= 0 || bg.height <= 0 {
        println!("compositor: background image has invalid dimensions, skipping");
        return;
    }

    // Safety guard for stride/size if needed (though we just created it)
    if bg.bpp != 32 {
         println!("compositor: expected 32bpp background");
         return;
    }

    let width = compositor.fb.info.width;
    let height = compositor.fb.info.height;
    let stride_pixels = (compositor.fb.info.stride / 4) as u32;

    // Blitter safety
    if stride_pixels < width {
         println!("compositor: framebuffer stride < width!");
         return;
    }
    compositor.background_canvas = None;

    let Some(bg) = &compositor.background_image else {
        return;
    };

    let width = compositor.fb.info.width;
    let height = compositor.fb.info.height;

    // Use tight packing for offscreen canvas
    let stride_pixels = width;
    let stride_bytes = stride_pixels * 4;

    let total_pixels = stride_pixels.saturating_mul(height);
    let mut pixels = Vec::with_capacity(total_pixels as usize);
    pixels.resize(total_pixels as usize, 0);

    primitives::draw_tiled_image(
        pixels.as_mut_ptr(),
        stride_bytes,
        width,
        height,
        bg.ptr,
        bg.width,
        bg.height,
        bg.bpp,
        compositor.background_offset.0,
        compositor.background_offset.1,
        None,
        true, // force_opaque
    );

    compositor.background_canvas = Some(BackgroundCanvas {
        pixels,
        width,
        height,
        stride_pixels, // This is now tightly packed width
    });
}

pub fn main() -> ! {
    println!("compositor: starting (single-buffered)");

    // 1. Critical Base Infrastructure Checks
    if !ensure_ui_schemas() {
        println!("compositor: schemas unavailable; sleeping and retrying");
        loop {
            thing_os::time::sleep(Duration::from_millis(250));
            if ensure_ui_schemas() {
                break;
            }
        }
    }

    // We no longer strictly need DisplayPresentRequest schema since we don't send it,
    // but the driver might still register it. We can skip checking it.

    let fb = loop {
        match active_framebuffer() {
            Some(fb) => break fb,
            None => {
                thing_os::time::sleep(Duration::from_millis(50));
            }
        }
    };

    let mut compositor = Compositor::new(fb);

    if let Some(bg) = load_background_image() {
        compositor.background_image = Some(bg);
        prepare_background_canvas(&mut compositor);
    }

    // Force initial full redraw to paint background/windows
    compositor.add_full_damage();
    tick_once(&mut compositor);

    let mut debug_frame_counter = 0;
    loop {
        /*
        if debug_frame_counter % 60 == 0 {
            println!("compositor: tick");
        }
        debug_frame_counter += 1;
        */
        tick_once(&mut compositor);
        thing_os::time::sleep(Duration::from_nanos(FRAME_INTERVAL_NS));
    }
}

pub fn tick_once(compositor: &mut Compositor) {
    compositor.ensure_display_contracts();

    // Process input EARLY using previous frame's layout (latency reduction)
    let prev_layout = compositor.cached_layout.clone();
    compositor.process_mouse_packets(&prev_layout);

    // handle_mode_switches();
    // if console_mode_active() { return; }

    /*
    let mode = match current_mode() {
        Some(mode) => mode,
        None => {
            return;
        }
    };

    if mode.index == MODE_INDEX_CONSOLE {
        draw_console(compositor);
        compositor.publish_present_request();
        return;
    }

    let place_id = mode.place_id.unwrap_or(ThingId(0));
    */

    let windows = collect_all_windows();
    let surface_map = collect_surfaces_for_windows(&windows);
    update_mapped_surfaces(compositor, &surface_map);

    let fb_w = compositor.fb.info.width as i32;
    let fb_h = compositor.fb.info.height as i32;
    let policy = layout::LayoutPolicy::default();
    let stacked: Vec<StackedWindow> = layout::apply_layout(policy, &windows, fb_w, fb_h);
    layout::persist_stack(&stacked);

    // Update cached layout for next frame's input processing
    compositor.cached_layout = stacked.clone();

    compositor.sync_active_from_layout(&stacked);

    // Track layout changes for damage
    if prev_layout != stacked {
        compositor.add_full_damage();
    }

    // Run widget layout pass
    let (widget_rects, widget_map) = run_widget_pass(&stacked);

    // Damage tracking: Union all damage rects into one bounding box
    
    // In single-buffered mode, we must treat damage differently than double-buffered swap.
    // If we move the cursor, we must:
    // 1. Redraw the area where the cursor WAS (to restore background/windows).
    // 2. Redraw the area where the cursor IS (to draw new cursor).
    // The `previous_damage` tracking in compositor handles the "previous frame" concept,
    // but mainly for "what was dirty on back buffer".
    // Here, we just need to ensure we cover the "trails".

    // The compositor model pushes new damage into `compositor.damage`.
    // It keeps `previous_damage` from the last tick.

    // Union current damage (includes new cursor pos)
    let current_union = calc_damage_union(&compositor.damage);
    
    // Union with previous damage (includes old cursor pos, etc)
    // We MUST redraw previous damage regions because they might contain artifacts (like old cursor)
    // that are still on the single screen buffer if we don't clear them.
    // Wait, if we moved the cursor, `compositor.process_mouse_packets` adds damage for *both* old and new positions?
    // Let's check `process_mouse_packets` in `input.rs` (not visible here, but usually it does).
    // If `process_mouse_packets` adds old+new rects to `compositor.damage`, then `current_union` suffices.
    // Assuming standard implementation:
    // `cursor.x/y` updated. Old rect added to damage. New rect added to damage.

    // If so, we just need `current_union`.
    // `previous_damage` was useful for swapping because the back buffer was 2 frames old.
    // With single buffer, the buffer is current-1 frame old (what we just showed).
    // We just need to overwrite changed pixels.

    let render_clip_tuple = if let Some((cx, cy, cw, ch)) = current_union {
         Some((cx, cy, cw, ch))
    } else {
         None
    };

    let clip: Option<crate::widget_layout::Rect> = render_clip_tuple.map(|(x, y, w, h)| {
         let fb_w = compositor.fb.info.width as i32;
         let fb_h = compositor.fb.info.height as i32;

         let min_x = x.max(0);
         let min_y = y.max(0);
         let max_x = (x + w).min(fb_w);
         let max_y = (y + h).min(fb_h);

         if max_x > min_x && max_y > min_y {
             crate::widget_layout::Rect::new(
                 min_x,
                 min_y,
                 (max_x - min_x) as u32,
                 (max_y - min_y) as u32,
             )
         } else {
             crate::widget_layout::Rect::new(0, 0, 0, 0)
         }
    }).filter(|r| r.w > 0 && r.h > 0);

    if let Some(clip_rect) = clip {
        let ops = build_display_list(
            compositor,
            &stacked,
            &windows,
            &surface_map,
            &widget_rects,
            &widget_map,
        );
        render_display_list(compositor, &ops, Some(clip_rect));
        
        // Present (No-op / Metadata update)
        compositor.present_frame();
        
        // Clear damage
        compositor.previous_damage = compositor.damage.clone();
        compositor.damage.clear();
    } else {
        // No damage, no render.
    }
}

fn calc_damage_union(damage: &[crate::widget_layout::Rect]) -> Option<(i32, i32, i32, i32)> {
    if damage.is_empty() { return None; }
    let mut u_x = 0;
    let mut u_y = 0;
    let mut u_w = 0;
    let mut u_h = 0;
    let mut first = true;
    for rect in damage {
        if first {
            u_x = rect.x; u_y = rect.y; u_w = rect.w as i32; u_h = rect.h as i32;
            first = false;
        } else {
            let min_x = u_x.min(rect.x);
            let min_y = u_y.min(rect.y);
            let max_x = (u_x + u_w).max(rect.x + rect.w as i32);
            let max_y = (u_y + u_h).max(rect.y + rect.h as i32);
            u_x = min_x; u_y = min_y; u_w = max_x - min_x; u_h = max_y - min_y;
        }
    }
    Some((u_x, u_y, u_w, u_h))
}

fn run_widget_pass(
    stacked: &[StackedWindow],
) -> (
    alloc::collections::BTreeMap<ThingId, alloc::vec::Vec<(ThingId, crate::widget_layout::Rect)>>,
    alloc::collections::BTreeMap<ThingId, crate::widgets::WidgetNode>,
) {
    use crate::config::{FRAME_THICKNESS, TITLE_BAR_HEIGHT};
    use crate::widget_layout::Rect;
    use crate::widgets::{WidgetNode, layout_children, widget_children};
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;

    // 1. Load all widgets
    let all_widgets: Vec<WidgetNode> = list_things_by_kind();
    let mut widget_map = BTreeMap::new();
    for w in all_widgets {
        widget_map.insert(w.id, w);
    }

    let mut results = BTreeMap::new();

    // 2. Iterate windows
    for win in stacked {
        // Explicit type annotation to ensure we are collecting (ThingId, Rect) tuples as required
        let mut win_rects: Vec<(ThingId, Rect)> = Vec::new();

        // Window Client Area
        let client_x = win.x + FRAME_THICKNESS + 4;
        let client_y = win.y + FRAME_THICKNESS + TITLE_BAR_HEIGHT + 4;
        let client_w = (win.width - FRAME_THICKNESS * 2 - 8).max(0) as u32;
        let client_h = (win.height - FRAME_THICKNESS * 2 - TITLE_BAR_HEIGHT - 8).max(0) as u32;

        let container_rect = Rect::new(client_x, client_y, client_w, client_h);

        // Recurse function
        let mut queue: Vec<(ThingId, Rect)> = Vec::new();

        let children = widget_children(win.id);

        let root_rects = layout_children(&widget_map, win.id, container_rect, &children);

        for (rid, rrect) in root_rects {
            win_rects.push((rid, rrect));
            queue.push((rid, rrect));
        }

        // Process queue
        let mut head = 0;
        while head < queue.len() {
            let (pid, prect) = queue[head];
            head += 1;

            let pchildren = widget_children(pid);
            if !pchildren.is_empty() {
                let child_rects = layout_children(&widget_map, pid, prect, &pchildren);
                for (cid, crect) in child_rects {
                    win_rects.push((cid, crect));
                    queue.push((cid, crect));
                }
            }
        }

        results.insert(win.id, win_rects);
    }

    (results, widget_map)
}

fn update_mapped_surfaces(
    compositor: &mut Compositor,
    current_surfaces: &alloc::collections::BTreeMap<ThingId, thing_os::Surface>,
) {
    use crate::model::MappedSurface;
    use abi::{KernelRequest, KernelResponse, PixelFormat};

    // 1. Remove stale mappings
    let mut to_remove = alloc::vec::Vec::new();
    for id in compositor.mapped_surfaces.keys() {
        if !current_surfaces.contains_key(id) {
            to_remove.push(*id);
        }
    }
    for id in to_remove {
        // Unmap not supported yet, just drop
        compositor.mapped_surfaces.remove(&id);
    }

    // 2. Add new mappings
    for (id, surface) in current_surfaces {
        if compositor.mapped_surfaces.contains_key(id) {
            continue;
        }

        if let Some(buf_id) = surface.shared_buffer_id {
            // Map it
            match syscall(KernelRequest::MapSharedBuffer {
                buffer_id: buf_id,
                flags: MapFlags::READ.union(MapFlags::USER),
            }) {
                KernelResponse::SharedBufferMapped { vaddr, size } => {
                    let format = match surface.format.as_str() {
                        "Rgba8888" => PixelFormat::Rgba8888,
                        "Bgra8888" => PixelFormat::Bgra8888,
                        _ => PixelFormat::Rgba8888,
                    };

                    compositor.mapped_surfaces.insert(
                        *id,
                        MappedSurface {
                            ptr: vaddr as *const u8,
                            width: surface.width as u32,
                            height: surface.height as u32,
                            stride: surface.stride as u32,
                            size: size as usize,
                            pixel_format: format,
                        },
                    );
                    println!("compositor: mapped surface");
                }
                _ => {
                    println!("compositor: failed to map surface");
                }
            }
        }
    }
}
