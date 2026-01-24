#![no_std]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
extern crate stem; // Force linkage

mod asset;
mod bmp;
mod bristle;
mod compositor;
mod cursor;
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
    
    // Spawn asset workers
    use stem::stack::{Stack, StackSpec};
    let s_spec = StackSpec { reserve_bytes: 256 * 1024, ..StackSpec::default() };
    let _ = stem::thread::spawn_on(Stack::alloc_growing_stack(s_spec).unwrap(), painter_resources::wallpaper_loader_entry);
    let _ = stem::thread::spawn_on(Stack::alloc_growing_stack(s_spec).unwrap(), painter_resources::cursor_loader_entry);
    let _ = stem::thread::spawn_on(Stack::alloc_growing_stack(s_spec).unwrap(), painter_resources::font_loader_entry);

    let mut loop_ctrl = FrameLoop::new(60);
    let (screen_w, screen_h) = (target.width as i32, target.height as i32);
    
    // Cursor state
    let bristle_evt_handle = bristle_evt as PortHandle;
    let mut cursor = CursorState::new(screen_w / 2, screen_h / 2);
    let mut prev_cursor_bbox = cursor.bbox();
    let mut pressed_keys: BTreeSet<abi::hid::Key> = BTreeSet::new();
    let accel_cfg = MouseAccelConfig::default();
    let mut accel_state = MouseAccelState::default();

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
        
        // Input processing
        let old_cursor_bbox = prev_cursor_bbox;
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
        
        if let Some(asset) = ASSETS.get_cursor() {
            cursor.set_asset(asset);
        }
        let new_cursor_bbox = cursor.bbox();
        prev_cursor_bbox = new_cursor_bbox;

        // Run UI Pipeline
        let mut list = drawlist::DrawList::new();
        
        // Render wallpaper first if available
        if let Some(wp) = ASSETS.get_wallpaper() {
            list.blit_image(&wp, 0, 0);
        } else {
            list.clear(crate::geometry::Color::from_u32(0xFF101018));
        }

        let ui_result = ui_pipeline.run(screen_w, screen_h, &mut list, &ASSETS);
        
        // Damage Tracking
        let bounds = damage::Rect::full(screen_w, screen_h);
        let mut damage = damage::Damage::empty(bounds);
        for rect in &ui_result.damage {
            damage.add_rect(*rect);
        }
        if old_cursor_bbox != new_cursor_bbox {
            damage.add_rect(old_cursor_bbox);
            damage.add_rect(new_cursor_bbox);
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
        
        // Execute drawlist (wallpaper + UI)
        raster::execute_with_damage(&mut surface, &list, &damage, ui_result.solid_text);
        
        // Always draw cursor on top (outside damage tracking for lowest latency)
        // Note: cursor.emit_drawlist already adds it to the list for damage-tracked rendering,
        // but draw_cursor_overlay was software blending into the final surface.
        // We'll rely on the DrawList-based cursor for now.

        let token = builder.finish();
        presenter.present_frame(token);
        presenter.pump();
        loop_ctrl.sleep();
    }
}
