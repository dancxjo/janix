#![no_std]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
extern crate stem; // Force linkage

mod asset;
mod bmp;
mod bristle;
mod compositor;
mod cursor;
mod cursor_rasterizer;
mod damage;
mod drawlist;
mod frame;
mod frame_loop;
pub mod geometry;
mod isa;
mod log_ratelimit;
mod logging;
mod lowered;
pub mod perf;
mod present;
mod raster;
mod reclaimer;
mod render_state;
mod surface;
mod font_graph;
mod font_client;
mod ui;
mod svg;
pub mod painter_resources;

pub use painter_resources::ASSETS;

use abi::ids::HandleId;
use stem::thing::ThingId;

use abi::display_driver_protocol::BindPayload;
use abi::schema::{keys, kinds};
use stem::syscall::PortHandle;

use crate::compositor::CompositorTarget;
use crate::frame::FrameBuilder;
use crate::frame_loop::FrameLoop;
use crate::present::{DriverPresenter, PresenterImpl};
use crate::bristle::{poll_bristle, MouseAccelConfig, MouseAccelState};
use crate::cursor::CursorState;
use crate::cursor_rasterizer::CursorRasterizer;
use crate::asset::AssetBank;
use crate::ui::{UiPipeline, FullRefreshReason};
use alloc::collections::BTreeSet;
use alloc::sync::Arc;

fn clear_surface(surface: &mut surface::Surface, color: u32) {
    let w = surface.width();
    let h = surface.height();
    for y in 0..h {
        for x in 0..w {
            surface.put_px(x, y, color);
        }
    }
}

fn clear_damage(surface: &mut surface::Surface, damage: &crate::damage::Damage, color: u32) {
    for rect in damage.iter() {
        raster::fill_rect_copy(surface, rect.x, rect.y, rect.w, rect.h, color);
    }
}

fn unpack_handle(arg: usize, index: u32) -> PortHandle {
    ((arg >> (index * 16)) & 0xFFFF) as PortHandle
}

#[cfg_attr(not(test), stem::main)]
fn main(arg: usize) -> ! {
    logging::init();
    perf::init();
    use stem::thing::sys::{bytespace_map, bytespace_unmap};
    let bs_id = ThingId::from_u64(arg as u64);
    let (mut arg_req, mut arg_resp, mut bristle_evt) = (0, 0, 0);
    if let Ok(ptr) = bytespace_map(bs_id) {
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u32, 16) };
        if slice[0] == 0xB100AA01 {
            arg_req = slice[1];
            arg_resp = slice[2];
            bristle_evt = slice[3];
        }
        let _ = bytespace_unmap(bs_id, ptr);
    } else {
        arg_req = unpack_handle(arg, 0) as u32;
        arg_resp = unpack_handle(arg, 1) as u32;
        bristle_evt = unpack_handle(arg, 2) as u32;
    }

    let target =
        CompositorTarget::discover_and_map((arg_req, arg_resp), 2000).expect("compositor discover");
    let _ = target.backend;

    let mut presenter = if target.driver_req != 0 {
        let mut d = DriverPresenter::new(target.driver_req, target.driver_resp);
        d.send_bind(&BindPayload {
            bytespace_id: target.bs_id.to_u64_lossy(),
            width: target.width,
            height: target.height,
            stride: target.stride_bytes,
            format: target.format,
        });
        PresenterImpl::Driver(d)
    } else {
        PresenterImpl::Null(present::NullPresenter)
    };

    let mut surface = unsafe {
        surface::Surface::new(
            target.ptr,
            target.size_bytes,
            target.width,
            target.height,
            target.stride_bytes,
        )
    };

    // UI Root
    let mut roots = [ThingId::default(); 1];
    let ui_root = match stem::thing::sys::find(abi::schema::kinds::UI_ROOT, &mut roots) {
        Ok(count) if count > 0 => roots[0],
        _ => stem::ui::UiBuilder::create_root(),
    };

    let mut ui_pipeline = UiPipeline::new();
    ui_pipeline.set_root(ui_root);
    
    // Spawn asset workers with diagnostic logging
    use stem::stack::{Stack, StackSpec};
    let s_spec = StackSpec { reserve_bytes: 256 * 1024, ..StackSpec::default() };
    
    match Stack::alloc_growing_stack(s_spec) {
        Ok(stack) => match stem::thread::spawn_on(stack, painter_resources::wallpaper_loader_entry) {
            Ok(tid) => stem::info!("bloom: spawned wallpaper loader (tid={})", tid),
            Err(e) => stem::error!("bloom: FAILED to spawn wallpaper loader: {:?}", e),
        },
        Err(e) => stem::error!("bloom: FAILED to alloc wallpaper stack: {:?}", e),
    }
    match Stack::alloc_growing_stack(s_spec) {
        Ok(stack) => match stem::thread::spawn_on(stack, painter_resources::cursor_loader_entry) {
            Ok(tid) => stem::info!("bloom: spawned cursor loader (tid={})", tid),
            Err(e) => stem::error!("bloom: FAILED to spawn cursor loader: {:?}", e),
        },
        Err(e) => stem::error!("bloom: FAILED to alloc cursor stack: {:?}", e),
    }
    match Stack::alloc_growing_stack(s_spec) {
        Ok(stack) => match stem::thread::spawn_on(stack, painter_resources::font_loader_entry) {
            Ok(tid) => stem::info!("bloom: spawned font loader (tid={})", tid),
            Err(e) => stem::error!("bloom: FAILED to spawn font loader: {:?}", e),
        },
        Err(e) => stem::error!("bloom: FAILED to alloc font stack: {:?}", e),
    }

    let mut loop_ctrl = FrameLoop::new(60);
    let (screen_w, screen_h) = (target.width as i32, target.height as i32);
    
    // Cursor state
    let bristle_evt_handle = bristle_evt as PortHandle;
    let mut cursor = CursorState::new(screen_w / 2, screen_h / 2);
    let mut cursor_rasterizer = CursorRasterizer::new();
    let mut pressed_keys: BTreeSet<abi::hid::Key> = BTreeSet::new();
    let accel_cfg = MouseAccelConfig::default();
    let mut accel_state = MouseAccelState::default();
    // Track previous cursor position for damage computation
    let mut prev_cursor_x = cursor.x;
    let mut prev_cursor_y = cursor.y;
    let mut prev_cursor_gen = crate::frame::AssetGeneration::ZERO;

    // Glyph Arrival Watch
    let glyph_watch_pred = stem::thing::sys::intern(kinds::FONT_GLYPH).unwrap_or(0);
    let glyph_watch = if glyph_watch_pred != 0 {
        use abi::types::{WatchSpec, WatchMode};
        use abi::root::RootWatchFilter;
        let filter = RootWatchFilter::predicate(glyph_watch_pred);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        stem::syscall::root_watch_open(&spec).ok()
    } else {
        None
    };

    // UI Window Watch - triggers dirty when windows are created/modified
    let ui_window_kind = stem::thing::sys::intern(kinds::UI_WINDOW).unwrap_or(0);
    let ui_window_watch = if ui_window_kind != 0 {
        use abi::types::{WatchSpec, WatchMode};
        use abi::root::RootWatchFilter;
        // Use kind() filter to watch for node creation, not predicate() which watches edges
        let filter = RootWatchFilter::kind(ui_window_kind);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        stem::syscall::root_watch_open(&spec).ok()
    } else {
        None
    };
    
    // UI Text Watch - triggers dirty when UI_TEXT property changes (for clock updates)
    let ui_text_key = stem::thing::sys::intern(keys::UI_TEXT).unwrap_or(0);
    let ui_text_watch = if ui_text_key != 0 {
        use abi::types::{WatchSpec, WatchMode};
        use abi::root::RootWatchFilter;
        // Watch for any SET_PROP with UI_TEXT predicate
        let filter = RootWatchFilter::predicate(ui_text_key);
        let spec = WatchSpec {
            mode: WatchMode::StreamOnly as u32,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<RootWatchFilter>() as u64,
            ..Default::default()
        };
        stem::syscall::root_watch_open(&spec).ok()
    } else {
        None
    };
    
    // Track watch event counts for diagnostics
    let mut ui_watch_events_total: u64 = 0;

    loop {
        loop_ctrl.next();
        ASSETS.publish_pending();

        // 0. Check for new glyphs in graph
        if let Some(gw) = glyph_watch {
            let mut g_seq = 0u64;
            let mut g_buf = [0u8; 1024];
            if let Ok(len) = stem::syscall::root_watch_next(gw, &mut g_seq, &mut g_buf) {
                if len > 0 {
                    crate::font_graph::mark_dirty();
                    ui_pipeline.mark_dirty_full_with_reason(FullRefreshReason::AssetChange);
                }
            }
        }
        
        // 1. Check for UI window changes
        if let Some(uw) = ui_window_watch {
            let mut w_seq = 0u64;
            let mut w_buf = [0u8; 256];
            let mut drained = 0u32;
            // Drain all pending events this frame
            while let Ok(len) = stem::syscall::root_watch_next(uw, &mut w_seq, &mut w_buf) {
                if len > 0 {
                    drained += 1;
                } else {
                    break;
                }
            }
            if drained > 0 {
                ui_watch_events_total += drained as u64;
                stem::info!("[bloom] UI watch: drained {} events (total={})", drained, ui_watch_events_total);
                ui_pipeline.mark_dirty_full_with_reason(FullRefreshReason::WatchOverflow);
            }
        }
        
        // 2. Check for UI_TEXT property changes (clock window, etc.)
        if let Some(tw) = ui_text_watch {
            let mut t_seq = 0u64;
            let mut t_buf = [0u8; 256];
            let mut drained = 0u32;
            // Drain all pending events this frame
            while let Ok(len) = stem::syscall::root_watch_next(tw, &mut t_seq, &mut t_buf) {
                if len > 0 {
                    drained += 1;
                } else {
                    break;
                }
            }
            if drained > 0 {
                stem::info!("[bloom] UI_TEXT watch: drained {} events", drained);
                // Clear cached text bytespaces AND raster cache (pre-rendered text/SVG)
                ui_pipeline.asset_cache_clear();
                ui_pipeline.raster_cache_clear();
                ui_pipeline.mark_dirty_full_with_reason(FullRefreshReason::WatchActivity);
            }
        }
        
        // Input processing (logical state only, no cursor asset handling)
        if bristle_evt_handle != 0 {
            poll_bristle(
                bristle_evt_handle, 
                &mut cursor, 
                &mut pressed_keys, 
                &accel_cfg, 
                &mut accel_state, 
                screen_w, 
                screen_h
            );
        }

        // Run UI Pipeline
        let mut list = drawlist::DrawList::new();
        
        // Render wallpaper first if available (tiled across the screen)
        if let Some(wp) = ASSETS.get_wallpaper() {
            let dest = crate::geometry::Rect::new(0, 0, screen_w, screen_h);
            list.blit_image_tiled(&wp, dest);
        } else {
            list.clear(crate::geometry::Color::from_u32(0xFF101018));
        }

        let ui_result = ui_pipeline.run(screen_w, screen_h, &mut list, &ASSETS);
        
        // Damage Tracking (cursor is now blended post-damage, does not affect window damage)
        let bounds = damage::Rect::full(screen_w, screen_h);
        let mut damage = damage::Damage::empty(bounds);
        for rect in &ui_result.damage {
            damage.add_rect(*rect);
        }
        // Cursor damage: add old + new cursor rectangles when cursor moved
        if let Some(asset) = ASSETS.get_cursor() {
            if let Some(snapshot) = cursor_rasterizer.get_snapshot(&asset) {
                let cursor_moved = cursor.x != prev_cursor_x || cursor.y != prev_cursor_y;
                let cursor_changed = snapshot.gen != prev_cursor_gen;
                
                if cursor_moved || cursor_changed {
                    let (cw, ch) = (snapshot.image.width as i32, snapshot.image.height as i32);
                    
                    // Old cursor rect (to erase)
                    let old_rect = damage::Rect::new(
                        prev_cursor_x - snapshot.hotspot_x,
                        prev_cursor_y - snapshot.hotspot_y,
                        cw, ch
                    ).expand(2).clip(bounds);
                    
                    // New cursor rect (to draw)
                    let new_rect = damage::Rect::new(
                        cursor.x - snapshot.hotspot_x,
                        cursor.y - snapshot.hotspot_y,
                        cw, ch
                    ).expand(2).clip(bounds);
                    
                    if !old_rect.is_empty() {
                        damage.add_rect(old_rect);
                    }
                    if !new_rect.is_empty() {
                        damage.add_rect(new_rect);
                    }
                    
                    // Update previous state
                    prev_cursor_x = cursor.x;
                    prev_cursor_y = cursor.y;
                    prev_cursor_gen = snapshot.gen;
                }
            }
        }

        if ui_result.changed && damage.is_empty() {
            damage = damage::Damage::full(bounds);
        }

        if damage.is_empty() {
            presenter.pump();
            loop_ctrl.sleep();
            continue;
        }

        // Render
        let token = presenter.acquire_frame(
            crate::frame::FrameSpec::new(target.width, target.height, target.format),
            ASSETS.current_generation(),
        );
        let mut builder = FrameBuilder::new(token);
        
        if damage.is_full {
             builder.mark_full_damage();
        } else {
             for rect in damage.iter() {
                 builder.add_damage(rect);
             }
        }
        
        // Execute drawlist (wallpaper + UI) - cursor is NOT in the DrawList
        raster::execute_with_damage(&mut surface, &list, &damage, ui_result.solid_text);
        
        // Cursor overlay: blend cached snapshot at cursor position (post-damage)
        // This ensures cursor movement does not trigger window repaints
        if let Some(asset) = ASSETS.get_cursor() {
            if let Some(snapshot) = cursor_rasterizer.get_snapshot(&asset) {
                let cx = cursor.x - snapshot.hotspot_x;
                let cy = cursor.y - snapshot.hotspot_y;
                raster::blit_cursor_overlay(&mut surface, &snapshot.image, cx, cy);
            }
        }

        let token = builder.finish();
        presenter.present_frame(token);
        presenter.pump();
        loop_ctrl.sleep();
    }
}
