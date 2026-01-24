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
use alloc::collections::BTreeSet;

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

    let mut loop_ctrl = FrameLoop::new(60);
    let (screen_w, screen_h) = (target.width as i32, target.height as i32);
    
    // Cursor state
    let bristle_evt_handle = bristle_evt as PortHandle;
    let mut cursor = CursorState::new(screen_w / 2, screen_h / 2);
    let mut prev_cursor_bbox = cursor.bbox();
    let mut pressed_keys: BTreeSet<abi::hid::Key> = BTreeSet::new();
    let accel_cfg = MouseAccelConfig::default();
    let mut accel_state = MouseAccelState::default();
    
    // UI Root
    let mut roots = [ThingId::default(); 1];
    let ui_root = match stem::thing::sys::find(abi::schema::kinds::UI_ROOT, &mut roots) {
        Ok(count) if count > 0 => roots[0],
        _ => stem::ui::UiBuilder::create_root(),
    };

    // We watch for UI_PRESENT_EPOCH on windows/root.
    let epoch_pred = stem::thing::sys::intern(keys::UI_PRESENT_EPOCH).unwrap_or(0);
    let mut ui_watch_id = 0;
    let mut ui_watch_buf = [0u8; 4096];
    let mut ui_watch_seq = 0u64;

    if epoch_pred != 0 {
        let filter = abi::root::RootWatchFilter::predicate(epoch_pred);
        let spec = abi::types::WatchSpec {
            mode: abi::types::WatchMode::StreamOnly as u32,
            start_seq: 0,
            filter_ptr: &filter as *const _ as u64,
            filter_len: core::mem::size_of::<abi::root::RootWatchFilter>() as u64,
            ..Default::default()
        };
        if let Ok(wid) = stem::syscall::root_watch_open(&spec) {
            ui_watch_id = wid;
            let _ = stem::root_watch::watch_drain(wid, &mut ui_watch_buf, |_, _| {});
        }
    }

    loop {
        loop_ctrl.next();
        
        // Input processing
        let _old_cursor_bbox = prev_cursor_bbox;
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
        
        let new_cursor_bbox = cursor.bbox();
        prev_cursor_bbox = new_cursor_bbox;

        // Drain watches
        if ui_watch_id != 0 {
            loop {
                match stem::syscall::root_watch_next(ui_watch_id, &mut ui_watch_seq, &mut ui_watch_buf) {
                    Ok(len) if len > 0 => {},
                    Ok(_) | Err(abi::errors::Errno::EAGAIN) => break,
                    _ => break,
                }
            }
        }

        // Build DrawList from Snapshots
        let mut list = drawlist::DrawList::new();

        // 1. Wallpaper (Solid Color)
        list.clear(geometry::Color::new(16, 16, 24, 255));

        // 2. Windows
        let mut windows = [ThingId::default(); 64];
        let win_count = stem::thing::sys::find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);

        for i in 0..win_count {
            let wid = windows[i];
            if let (Ok(bs_id), Ok(w), Ok(h), Ok(x), Ok(y)) = (
                stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_BYTESPACE),
                stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_WIDTH),
                stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_HEIGHT),
                stem::thing::sys::prop_get(wid, keys::UI_X),
                stem::thing::sys::prop_get(wid, keys::UI_Y),
            ) {
                if bs_id != 0 {
                    let stride = stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_STRIDE).unwrap_or(w * 4);
                    list.commands().push(drawlist::DrawCmd::DrawSnapshot {
                        bs_id,
                        width: w as u32,
                        height: h as u32,
                        stride: stride as u32,
                        dest: geometry::Rect::new(x as i32, y as i32, w as i32, h as i32),
                    });
                } else {
                    // Placeholder
                    let w = stem::thing::sys::prop_get(wid, keys::UI_WIDTH).unwrap_or(100);
                    let h = stem::thing::sys::prop_get(wid, keys::UI_HEIGHT).unwrap_or(100);
                    let x = stem::thing::sys::prop_get(wid, keys::UI_X).unwrap_or(0);
                    let y = stem::thing::sys::prop_get(wid, keys::UI_Y).unwrap_or(0);
                    list.rect(x as i32, y as i32, w as i32, h as i32, geometry::Color::new(50, 50, 50, 255));

                    // Request paint
                    let epoch = stem::thing::sys::prop_get(wid, keys::UI_PAINT_EPOCH).unwrap_or(0);
                    let _ = stem::thing::sys::prop_set(wid, keys::UI_PAINT_EPOCH, epoch + 1);
                }
            }
        }

        // 3. Cursor
        let cursor_bs = stem::thing::sys::prop_get(ui_root, keys::UI_CURSOR_SNAPSHOT_BYTESPACE).unwrap_or(0);
        if cursor_bs != 0 {
             let w = stem::thing::sys::prop_get(ui_root, keys::UI_CURSOR_SNAPSHOT_WIDTH).unwrap_or(0);
             let h = stem::thing::sys::prop_get(ui_root, keys::UI_CURSOR_SNAPSHOT_HEIGHT).unwrap_or(0);
             let stride = stem::thing::sys::prop_get(ui_root, keys::UI_CURSOR_SNAPSHOT_STRIDE).unwrap_or(w * 4);
             let cx = cursor.x;
             let cy = cursor.y;
             // Assume hotspot is handled by Blossom painting (e.g. padding) or simple top-left for now.
             list.commands().push(drawlist::DrawCmd::DrawSnapshot {
                 bs_id: cursor_bs,
                 width: w as u32,
                 height: h as u32,
                 stride: stride as u32,
                 dest: geometry::Rect::new(cx, cy, w as i32, h as i32),
             });
        }

        // Render
        let bounds = damage::Rect::full(screen_w, screen_h);
        let damage = damage::Damage::full(bounds);

        let token = presenter.acquire_frame(
            crate::frame::FrameSpec::new(target.width, target.height, target.format),
            crate::frame::AssetGeneration::ZERO,
        );
        let mut builder = FrameBuilder::new(token);
        
        clear_surface(&mut surface, 0xFF101018);
        builder.mark_full_damage();
        
        raster::execute_with_damage(&mut surface, &list, &damage, false);

        let token = builder.finish();
        presenter.present_frame(token);
        presenter.pump();
        loop_ctrl.sleep();
    }
}
