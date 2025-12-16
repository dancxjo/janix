use thing_os::prelude::*;
use thing_os::MODE_INDEX_CONSOLE;
use thing_os::thing_models::DisplayPresentRequest;

use crate::config::FRAME_INTERVAL_NS;
use crate::graph::{
    active_framebuffer, collect_surfaces_for_windows, collect_windows_for_place,
    console_mode_active, current_mode, handle_mode_switches, layout_policy_for_mode,
    swap_display_buffers,
};
use crate::layout::{self, StackedWindow};
use crate::model::BackgroundImage;
use crate::model::Compositor;
use crate::render::{build_display_list, render_display_list};
use abi::MapFlags;
use alloc::boxed::Box;
use alloc::format;
use thing_os::{RawModule, shared_buffer_map};

use crate::model::ConsoleBuffer;

fn draw_console<S: Sys>(sys: &mut S, compositor: &mut Compositor) {
    if compositor.console_buffer.is_none() {
        use abi::{KernelRequest, KernelResponse, PropValue, graph_kinds, ThingId};
        
        let mut found = None;
        let mut cursor = ThingId(u64::MAX);

        loop {
            match sys.syscall(KernelRequest::ThingList {
                kind: graph_kinds::KIND_SHARED_BUFFER,
                start_after: cursor,
            }) {
                KernelResponse::ThingListEntry { id: Some(next_id) } => {
                    // Check this thing
                    match sys.syscall(KernelRequest::ThingGet { id: next_id }) {
                        KernelResponse::ThingData { props, .. } => {
                            let mut is_console = false;
                            let mut width = 0;
                            let mut height = 0;
                            let mut stride = 0;

                            for item in props.iter().flatten() {
                                let (key, val) = item;
                                if *key == graph_kinds::PROP_NAME {
                                    if let PropValue::Str(s) = val {
                                        if s == "console_backend" {
                                            is_console = true;
                                        }
                                    }
                                } else if *key == graph_kinds::PROP_WIDTH {
                                     if let PropValue::U64(v) = val { width = *v as u32; }
                                } else if *key == graph_kinds::PROP_HEIGHT {
                                     if let PropValue::U64(v) = val { height = *v as u32; }
                                } else if *key == graph_kinds::PROP_STRIDE {
                                     if let PropValue::U64(v) = val { stride = *v as u32; }
                                }
                            }

                            if is_console {
                                found = Some((next_id, width, height, stride));
                            }
                        },
                        _ => {}
                    }
                    if found.is_some() { break; }
                    cursor = next_id;
                }
                _ => break,
            }
        }

        if let Some((id, w, h, stride)) = found {
             match shared_buffer_map(sys, id, MapFlags::READ.union(MapFlags::USER)) {
                 Ok((vaddr, _size)) => {
                     compositor.console_buffer = Some(ConsoleBuffer {
                         id,
                         ptr: vaddr as *const u8,
                         width: w,
                         height: h,
                         stride,
                         pixel_format: abi::PixelFormat::Bgra8888, 
                     });
                     println(sys, "compositor: mapped console buffer");
                 },
                 _ => {
                     println(sys, "compositor: failed to map console buffer");
                 }
             }
        }
    }

    if let Some(cb) = &compositor.console_buffer {
        let ptr = cb.ptr;
        let w = cb.width;
        let h = cb.height;
        let fb_w = compositor.fb.info.width;
        let fb_h = compositor.fb.info.height;
        
        let dest_x = (fb_w as i32 - w as i32) / 2;
        let dest_y = (fb_h as i32 - h as i32) / 2;
        
        let dest = compositor.fb.ptr as *mut u32;
        let dest_stride_px = (compositor.fb.info.stride / 4) as i32;
        let src_stride_px = (cb.stride / 4) as i32;
        
        // Simple blit
        for y in 0..h {
            let row_dest_y = dest_y + y as i32;
            if row_dest_y < 0 || row_dest_y >= fb_h as i32 { continue; }

            let row_src = unsafe { (ptr as *const u32).add((y as i32 * src_stride_px) as usize) };
            let row_dest = unsafe { dest.add((row_dest_y * dest_stride_px) as usize) };
            
            unsafe {
                // Bounds check horizontal
                let start_x = 0.max(-dest_x);
                let end_x = (w as i32).min(fb_w as i32 - dest_x);
                
                if end_x > start_x {
                    let count = (end_x - start_x) as usize;
                    core::ptr::copy_nonoverlapping(
                        row_src.add(start_x as usize), 
                        row_dest.add((dest_x + start_x) as usize), 
                        count
                    );
                }
            }
        }
    }
}

fn load_background_image<S: Sys>(sys: &mut S) -> Option<BackgroundImage> {
    let modules = list_things_by_kind::<S, RawModule>(sys);
    let clouds_module = modules.iter().find(|m| m.identifier == "clouds.bmp")?;

    if let Some(buffer_id) = clouds_module.framebuffer_id {
        // Map the buffer
        if let Some((vaddr, size)) =
            shared_buffer_map(sys, buffer_id, MapFlags::READ.union(MapFlags::USER)).ok()
        {
            let ptr = vaddr as *const u8;
            // Parse BMP header
            // Signature "BM" at 0
            unsafe {
                if *ptr != b'B' || *ptr.add(1) != b'M' {
                    println(sys, "clouds.bmp: invalid signature");
                    return None;
                }
                // Little endian parsing helper
                let read_u32 = |offset| {
                    let p = ptr.add(offset);
                    u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
                };
                let read_i32 = |offset| {
                    let p = ptr.add(offset);
                    i32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
                };

                let read_u16 = |offset| {
                    let p = ptr.add(offset);
                    u16::from_le_bytes([*p, *p.add(1)])
                };

                let data_offset = read_u32(0x0A);
                let width = read_i32(0x12);
                let height = read_i32(0x16);
                let bpp = read_u16(0x1C);

                let msg = format!(
                    "clouds.bmp: mapped. {}x{} offset={} bpp={}",
                    width, height, data_offset, bpp
                );
                let leaked = Box::leak(msg.into_boxed_str());
                println(sys, leaked);

                if bpp != 24 && bpp != 32 {
                    println(sys, "clouds.bmp: unsupported bpp");
                    return None;
                }

                return Some(BackgroundImage {
                    ptr: ptr.add(data_offset as usize),
                    size: size as usize,
                    width,
                    height,
                    bpp,
                });
            }
        }
    }
    println(
        sys,
        "clouds.bmp: module found but no buffer_id or map failed",
    );
    None
}

pub fn main<S: Sys>(sys: &mut S) -> ! {
    println(sys, "compositor: starting");
    ensure_ui_schemas(sys);
    let _ = register_schema_for::<DisplayPresentRequest>(sys);

    let fb = loop {
        if let Some(fb) = active_framebuffer(sys) {
            break fb;
        }
        println(sys, "compositor: waiting for primary display");
        sys.sleep_for_ns(50_000_000);
    };

    let mut compositor = Compositor::new(fb);

    if let Some(bg) = load_background_image(sys) {
        compositor.background_image = Some(bg);
    }

    // Force initial full redraw to paint background/windows
    // We must render TWICE to ensure both front and back buffers are initialized.
    compositor.add_full_damage();
    tick_once(sys, &mut compositor);
    compositor.add_full_damage();
    tick_once(sys, &mut compositor);

    let mut debug_frame_counter = 0;
    loop {
        if debug_frame_counter % 60 == 0 {
            println(sys, "compositor: tick");
        }
        debug_frame_counter += 1;
        tick_once(sys, &mut compositor);
        sys.sleep_for_ns(FRAME_INTERVAL_NS);
    }
}

pub fn tick_once<S: Sys>(sys: &mut S, compositor: &mut Compositor) {
    compositor.ensure_display_contracts(sys);

    // Process input EARLY using previous frame's layout (latency reduction)
    let prev_layout = compositor.cached_layout.clone();
    compositor.process_mouse_packets(sys, &prev_layout);

    handle_mode_switches(sys);

    if console_mode_active(sys) {
        return;
    }

    let mode = match current_mode(sys) {
        Some(mode) => mode,
        None => {
            return;
        }
    };

    if mode.index == MODE_INDEX_CONSOLE {
        draw_console(sys, compositor);
        compositor.publish_present_request(sys);
        return;
    }

    let place_id = mode.place_id.unwrap_or(ThingId(0));
    let windows = collect_windows_for_place(sys, place_id);
    if compositor.frame_counter % 60 == 0 {
         // let msg = format!("compositor: found {} windows for place {}", windows.len(), place_id.0);
         // let leaked = Box::leak(msg.into_boxed_str());
         // println(sys, leaked);
    }
    let surface_map = collect_surfaces_for_windows(sys, &windows);
    update_mapped_surfaces(sys, compositor, &surface_map);

    let fb_w = compositor.fb.info.width as i32;
    let fb_h = compositor.fb.info.height as i32;
    let policy = layout_policy_for_mode(&mode);
    let stacked: Vec<StackedWindow> = layout::apply_layout(policy, &windows, fb_w, fb_h);
    layout::persist_stack(sys, &stacked);

    // Update cached layout for next frame's input processing
    compositor.cached_layout = stacked.clone();

    compositor.sync_active_from_layout(&stacked);

    // Track layout changes for damage
    // Naive: if layout changed at all, full redraw. 
    // Ideally we diff 'prev_layout' vs 'stacked'.
    if prev_layout != stacked {
        compositor.add_full_damage();
    }

    // DEBUG: Stub input for cursor movement (remove or comment out for production)
    // let fb_w_i32 = compositor.fb.info.width as i32;
    // let fb_h_i32 = compositor.fb.info.height as i32;
    // compositor.cursor.x = (compositor.cursor.x + 2).rem_euclid(fb_w_i32);
    // compositor.cursor.y = (compositor.cursor.y + 2).rem_euclid(fb_h_i32);

    // Animate background: scroll up and left (requires incrementing offset)
    // Disabled to fix dirty rectangle glitches (we don't redraw background every frame)
    // compositor.background_offset.0 = compositor.background_offset.0.wrapping_add(1);
    // compositor.background_offset.1 = compositor.background_offset.1.wrapping_add(1);

    // Run widget layout pass
    let (widget_rects, widget_map) = run_widget_pass(sys, &stacked);

    // Damage tracking: Union all damage rects into one bounding box
    // This is the "easy" way (scissoring). 
    // A harder way is to pass multiple clip rects (region) to the renderer.
    
    // We must consider BOTH current frame damage and previous frame damage
    // because we are swapping buffers. The back buffer contains state from N-2.
    // We need to clear artifacts from N-1 (previous_damage) and draw N (current damage).
    let combined_damage_empty = compositor.damage.is_empty() && compositor.previous_damage.is_empty();

    let clip: Option<crate::widget_layout::Rect> = if combined_damage_empty {
        // No damage, do not render.
        // But we still need to swap buffers if we rendered previously?
        // Actually if nothing changed, we might not need to do anything.
        // However, strictly speaking, double buffering means we might need to copy 
        // front to back or re-render. 
        // For simplicity: if no damage, skip render.
        None
    } else {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        // Iterate over BOTH current and previous damage
        for r in compositor.damage.iter().chain(compositor.previous_damage.iter()) {
            min_x = min_x.min(r.x);
            min_y = min_y.min(r.y);
            max_x = max_x.max(r.x + r.w as i32);
            max_y = max_y.max(r.y + r.h as i32);
        }

        // Clamp to screen
        let fb_w = compositor.fb.info.width as i32;
        let fb_h = compositor.fb.info.height as i32;

        min_x = min_x.max(0);
        min_y = min_y.max(0);
        max_x = max_x.min(fb_w);
        max_y = max_y.max(fb_h);

        if max_x > min_x && max_y > min_y {
            Some(crate::widget_layout::Rect::new(min_x, min_y, (max_x - min_x) as u32, (max_y - min_y) as u32))
        } else {
            None
        }
    };

    if let Some(clip_rect) = clip {
        let ops = build_display_list(compositor, &stacked, &windows, &surface_map, &widget_rects, &widget_map);
        render_display_list(compositor, &ops, Some(clip_rect));
        
        if let Some(active_index) = swap_display_buffers(sys, compositor.fb.display_id) {
            compositor.fb.update_active_index(active_index);
        }
    }
    
    // Rotate damage history
    compositor.previous_damage = compositor.damage.clone();
    compositor.damage.clear();

    compositor.publish_present_request(sys);
}

fn run_widget_pass<S: Sys>(
    sys: &mut S,
    stacked: &[StackedWindow],
) -> (
    alloc::collections::BTreeMap<ThingId, alloc::vec::Vec<(ThingId, crate::widget_layout::Rect)>>,
    alloc::collections::BTreeMap<ThingId, crate::widgets::WidgetNode>
) {
    use crate::config::{FRAME_THICKNESS, TITLE_BAR_HEIGHT};
    use crate::widget_layout::Rect;
    use crate::widgets::{layout_children, widget_children, WidgetNode};
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;

    // 1. Load all widgets
    let all_widgets: Vec<WidgetNode> = list_things_by_kind(sys);
    let mut widget_map = BTreeMap::new();
    for w in all_widgets {
        widget_map.insert(w.id, w);
    }

    let mut results = BTreeMap::new();

    // 2. Iterate windows
    for win in stacked {
        let mut win_rects = Vec::new();

        // Window Client Area
        let client_x = win.x + FRAME_THICKNESS + 4;
        let client_y = win.y + FRAME_THICKNESS + TITLE_BAR_HEIGHT + 4;
        let client_w = (win.width - FRAME_THICKNESS * 2 - 8).max(0) as u32;
        let client_h = (win.height - FRAME_THICKNESS * 2 - TITLE_BAR_HEIGHT - 8).max(0) as u32;

        let container_rect = Rect::new(client_x, client_y, client_w, client_h);

        // Recurse function
        let mut queue = Vec::new(); // (id, rect)

        let children = widget_children(sys, win.id);
        
        let msg = format!("compositor: layout window {} children={}", win.id.0, children.len());
        let leaked = Box::leak(msg.into_boxed_str());
        println(sys, leaked);

        let root_rects = layout_children(sys, &widget_map, win.id, container_rect, &children);

        for (rid, rrect) in root_rects {
            win_rects.push((rid, rrect));
            queue.push((rid, rrect));
        }

        // Process queue
        let mut head = 0;
        while head < queue.len() {
            let (pid, prect) = queue[head];
            head += 1;

            let pchildren = widget_children(sys, pid);
            if !pchildren.is_empty() {
                let child_rects = layout_children(sys, &widget_map, pid, prect, &pchildren);
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

fn update_mapped_surfaces<S: Sys>(
    sys: &mut S,
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
            match sys.syscall(KernelRequest::MapSharedBuffer {
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
                    thing_os::println(sys, "compositor: mapped surface");
                }
                _ => {
                    thing_os::println(sys, "compositor: failed to map surface");
                }
            }
        }
    }
}
