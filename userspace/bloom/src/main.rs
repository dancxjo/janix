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
mod render_graph;
mod render_state;
mod surface;
mod font_graph;
mod font_client;
mod ui;
mod svg;
mod blossom_client;
mod window_manager;
pub mod painter_resources;
mod paint_vm;
mod ui_events;

pub use painter_resources::ASSETS;

use abi::hid::Key;
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
use crate::paint_vm::PaintPipeline;
use alloc::collections::BTreeSet;
use alloc::sync::Arc;

const BLOSSOM_BORDER: i32 = 2;
const BLOSSOM_TITLE_BAR_HEIGHT: i32 = 24;

#[derive(Clone, Copy)]
struct WindowHit {
    id: ThingId,
    rect: crate::geometry::Rect,
    z: i32,
}

#[derive(Clone, Copy)]
struct DragState {
    window_id: ThingId,
    start_mouse: (i32, i32),
    start_rect: crate::geometry::Rect,
}

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

fn window_rect_from_props(window_id: ThingId, screen_w: i32, screen_h: i32) -> crate::geometry::Rect {
    let w = stem::thing::sys::prop_get(window_id, keys::UI_WIDTH).unwrap_or(0) as i32;
    let h = stem::thing::sys::prop_get(window_id, keys::UI_HEIGHT).unwrap_or(0) as i32;
    if w <= 0 || h <= 0 {
        return crate::geometry::Rect::new(0, 0, 0, 0);
    }
    let mut x = stem::thing::sys::prop_get(window_id, keys::UI_X).unwrap_or(0) as i32;
    let mut y = stem::thing::sys::prop_get(window_id, keys::UI_Y).unwrap_or(0) as i32;
    let inset_right = stem::thing::sys::prop_get(window_id, keys::UI_INSET_RIGHT).unwrap_or(0) as i32;
    let inset_bottom = stem::thing::sys::prop_get(window_id, keys::UI_INSET_BOTTOM).unwrap_or(0) as i32;
    if inset_right > 0 {
        x = screen_w - inset_right - w;
    }
    if inset_bottom > 0 {
        y = screen_h - inset_bottom - h;
    }
    crate::geometry::Rect::new(x, y, w, h)
}

fn top_window_at_point(
    x: i32,
    y: i32,
    screen_w: i32,
    screen_h: i32,
) -> Option<WindowHit> {
    let mut windows = [ThingId::default(); 128];
    let count = stem::thing::sys::find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);
    let mut best: Option<WindowHit> = None;
    for win in windows.iter().take(count) {
        if stem::thing::sys::prop_get(*win, keys::UI_HIDDEN).unwrap_or(0) != 0 {
            continue;
        }
        let rect = window_rect_from_props(*win, screen_w, screen_h);
        if rect.width() <= 0 || rect.height() <= 0 {
            continue;
        }
        if x < rect.x() || y < rect.y() || x >= rect.x() + rect.width() || y >= rect.y() + rect.height() {
            continue;
        }
        let z = stem::thing::sys::prop_get(*win, keys::UI_Z_INDEX).unwrap_or(0) as i32;
        if best.map(|b| z >= b.z).unwrap_or(true) {
            best = Some(WindowHit { id: *win, rect, z });
        }
    }
    best
}

fn in_title_bar(rect: crate::geometry::Rect, x: i32, y: i32) -> bool {
    let local_y = y - rect.y();
    let title_top = BLOSSOM_BORDER;
    let title_bottom = BLOSSOM_BORDER + BLOSSOM_TITLE_BAR_HEIGHT;
    local_y >= title_top && local_y < title_bottom
}

fn in_client_area(rect: crate::geometry::Rect, x: i32, y: i32) -> bool {
    let left = rect.x() + BLOSSOM_BORDER;
    let right = rect.x() + rect.width() - BLOSSOM_BORDER;
    let top = rect.y() + BLOSSOM_BORDER + BLOSSOM_TITLE_BAR_HEIGHT;
    let bottom = rect.y() + rect.height() - BLOSSOM_BORDER;
    x >= left && x < right && y >= top && y < bottom
}

fn clamp_window_rect(rect: crate::geometry::Rect, screen_w: i32, screen_h: i32) -> crate::geometry::Rect {
    let mut r = rect;
    let min_visible = BLOSSOM_TITLE_BAR_HEIGHT;
    if r.y() + min_visible < 0 {
        r.origin.y = -min_visible + 1;
    }
    if r.y() > screen_h - min_visible {
        r.origin.y = screen_h - min_visible;
    }
    if r.x() + r.width() < min_visible {
        r.origin.x = min_visible - r.width();
    }
    if r.x() > screen_w - min_visible {
        r.origin.x = screen_w - min_visible;
    }
    r
}

fn raise_window(window_id: ThingId) {
    let mut windows = [ThingId::default(); 128];
    let count = stem::thing::sys::find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);
    let mut max_z = 0i32;
    for win in windows.iter().take(count) {
        if *win == window_id {
            continue;
        }
        let z = stem::thing::sys::prop_get(*win, keys::UI_Z_INDEX).unwrap_or(0) as i32;
        if z > max_z {
            max_z = z;
        }
    }
    let _ = stem::thing::sys::prop_set(window_id, keys::UI_Z_INDEX, (max_z as u64).saturating_add(1));
}

fn set_focus(focused_window: &mut Option<ThingId>, target: Option<ThingId>) {
    if *focused_window == target {
        return;
    }
    if let Some(prev) = focused_window.take() {
        let _ = stem::thing::sys::prop_set(prev, keys::UI_FOCUSED, 0);
    }
    if let Some(next) = target {
        let _ = stem::thing::sys::prop_set(next, keys::UI_FOCUSED, 1);
        *focused_window = Some(next);
    }
}

fn build_window_cycle_order() -> (alloc::vec::Vec<ThingId>, i32) {
    let mut windows = [ThingId::default(); 128];
    let count = stem::thing::sys::find(kinds::UI_WINDOW, &mut windows).unwrap_or(0);
    if count == 0 {
        return (alloc::vec::Vec::new(), 0);
    }

    let mut list: alloc::vec::Vec<(ThingId, i32)> = alloc::vec::Vec::new();
    for id in windows.iter().take(count) {
        if stem::thing::sys::prop_get(*id, keys::UI_HIDDEN).unwrap_or(0) != 0 {
            continue;
        }
        let z = stem::thing::sys::prop_get(*id, keys::UI_Z_INDEX).unwrap_or(0) as i32;
        list.push((*id, z));
    }

    if list.is_empty() {
        return (alloc::vec::Vec::new(), 0);
    }

    list.sort_by(|(a_id, a_z), (b_id, b_z)| {
        b_z.cmp(a_z)
            .then(a_id.to_u64_lossy().cmp(&b_id.to_u64_lossy()))
    });

    let max_z = list.iter().map(|(_, z)| *z).max().unwrap_or(0);
    let order = list.into_iter().map(|(id, _)| id).collect();
    (order, max_z)
}

fn cycle_windows_in_order(
    order: &[ThingId],
    current: Option<ThingId>,
    reverse: bool,
    max_z: &mut i32,
) -> Option<ThingId> {
    if order.is_empty() {
        return None;
    }

    let target_idx = match current.and_then(|id| order.iter().position(|wid| *wid == id)) {
        Some(idx) => {
            if reverse {
                if idx == 0 { order.len() - 1 } else { idx - 1 }
            } else {
                (idx + 1) % order.len()
            }
        }
        None => {
            if reverse { order.len() - 1 } else { 0 }
        }
    };
    let target = order[target_idx];

    // Publish focus status to graph
    if let Some(prev) = current {
        let _ = stem::thing::sys::prop_set(prev, keys::UI_FOCUSED, 0);
    }
    let _ = stem::thing::sys::prop_set(target, keys::UI_FOCUSED, 1);

    *max_z = max_z.saturating_add(1);
    let _ = stem::thing::sys::prop_set(target, keys::UI_Z_INDEX, *max_z as u64);
    Some(target)
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

    let _ = ui_root;
    let mut paint_pipeline = PaintPipeline::new();
    
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
    match Stack::alloc_growing_stack(s_spec) {
        Ok(stack) => match stem::thread::spawn_on(stack, painter_resources::icon_loader_entry) {
            Ok(tid) => stem::info!("bloom: spawned icon loader (tid={})", tid),
            Err(e) => stem::error!("bloom: FAILED to spawn icon loader: {:?}", e),
        },
        Err(e) => stem::error!("bloom: FAILED to alloc icon stack: {:?}", e),
    }

    let mut loop_ctrl = FrameLoop::new(60);
    let (screen_w, screen_h) = (target.width as i32, target.height as i32);
    
    // Cursor state
    let bristle_evt_handle = bristle_evt as PortHandle;
    let mut cursor = CursorState::new(screen_w / 2, screen_h / 2);
    let mut cursor_rasterizer = CursorRasterizer::new();
    let mut pressed_keys: BTreeSet<Key> = BTreeSet::new();
    let mut prev_keys: BTreeSet<Key> = BTreeSet::new();
    let mut prev_cursor_buttons = cursor.buttons();
    let mut ui_dispatch = ui_events::UiEventDispatcher::new();
    let mut focused_window: Option<ThingId> = None;
    let mut alt_cycle_order: alloc::vec::Vec<ThingId> = alloc::vec::Vec::new();
    let mut alt_cycle_max_z: i32 = 0;
    let mut alt_prev_down = false;
    let accel_cfg = MouseAccelConfig::default();
    let mut accel_state = MouseAccelState::default();
    // Track previous cursor position for damage computation
    let mut prev_cursor_x = cursor.x;
    let mut prev_cursor_y = cursor.y;
    let mut prev_cursor_gen = crate::frame::AssetGeneration::ZERO;
    let mut drag_state: Option<DragState> = None;

    // Window Manager disabled in paint pipeline (no legacy chrome/hit testing)

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
    
    // UI Paint Watch - triggers dirty when paint generation changes
    let ui_paint_gen_key = stem::thing::sys::intern(keys::UI_PAINT_GEN).unwrap_or(0);
    let ui_paint_watch = if ui_paint_gen_key != 0 {
        use abi::types::{WatchSpec, WatchMode};
        use abi::root::RootWatchFilter;
        let filter = RootWatchFilter::predicate(ui_paint_gen_key);
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
    let mut force_full_damage = false;

    loop {
        loop_ctrl.next();
        force_full_damage = false;
        ASSETS.publish_pending();

        // 0. Check for new glyphs in graph
        if let Some(gw) = glyph_watch {
            let mut g_seq = 0u64;
            let mut g_buf = [0u8; 1024];
            if let Ok(len) = stem::syscall::root_watch_next(gw, &mut g_seq, &mut g_buf) {
                if len > 0 {
                    crate::font_graph::mark_dirty();
                    force_full_damage = true;
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
                force_full_damage = true;
            }
        }
        
        // 2. Check for UI_PAINT updates
        if let Some(pw) = ui_paint_watch {
            let mut p_seq = 0u64;
            let mut p_buf = [0u8; 256];
            let mut drained = 0u32;
            while let Ok(len) = stem::syscall::root_watch_next(pw, &mut p_seq, &mut p_buf) {
                if len > 0 {
                    drained += 1;
                } else {
                    break;
                }
            }
            if drained > 0 {
                force_full_damage = true;
            }
        }
        
        // Input processing with window management
        if bristle_evt_handle != 0 {
            prev_keys = pressed_keys.clone();
            {
                crate::trace_span!("bloom.loop.poll_bristle");
                poll_bristle(
                    bristle_evt_handle,
                    &mut cursor,
                    &mut pressed_keys,
                    &accel_cfg,
                    &mut accel_state,
                    screen_w,
                    screen_h,
                )
            }
            let current_buttons = cursor.buttons();
            let left_down = (current_buttons & 1) != 0;
            let left_prev = (prev_cursor_buttons & 1) != 0;
            prev_cursor_buttons = current_buttons;
            let cursor_moved = cursor.x != prev_cursor_x || cursor.y != prev_cursor_y;

            if left_down && !left_prev {
                if let Some(hit) = top_window_at_point(cursor.x, cursor.y, screen_w, screen_h) {
                    set_focus(&mut focused_window, Some(hit.id));
                    if in_title_bar(hit.rect, cursor.x, cursor.y) {
                        let inset_right =
                            stem::thing::sys::prop_get(hit.id, keys::UI_INSET_RIGHT).unwrap_or(0);
                        let inset_bottom =
                            stem::thing::sys::prop_get(hit.id, keys::UI_INSET_BOTTOM).unwrap_or(0);
                        if inset_right == 0 && inset_bottom == 0 {
                            drag_state = Some(DragState {
                                window_id: hit.id,
                                start_mouse: (cursor.x, cursor.y),
                                start_rect: hit.rect,
                            });
                            raise_window(hit.id);
                        }
                    } else if in_client_area(hit.rect, cursor.x, cursor.y) {
                        raise_window(hit.id);
                        ui_dispatch.dispatch_click(cursor.x, cursor.y, screen_w, screen_h);
                    }
                } else {
                    set_focus(&mut focused_window, None);
                }
            }

            if !left_down && left_prev {
                drag_state = None;
            }

            if let Some(drag) = drag_state {
                set_focus(&mut focused_window, Some(drag.window_id));
                if left_down && cursor_moved {
                    let delta_x = cursor.x - drag.start_mouse.0;
                    let delta_y = cursor.y - drag.start_mouse.1;
                    let mut next_rect = crate::geometry::Rect::new(
                        drag.start_rect.x() + delta_x,
                        drag.start_rect.y() + delta_y,
                        drag.start_rect.width(),
                        drag.start_rect.height(),
                    );
                    next_rect = clamp_window_rect(next_rect, screen_w, screen_h);
                    let _ = stem::thing::sys::prop_set(
                        drag.window_id,
                        keys::UI_X,
                        next_rect.x() as u64,
                    );
                    let _ = stem::thing::sys::prop_set(
                        drag.window_id,
                        keys::UI_Y,
                        next_rect.y() as u64,
                    );
                }
            } else {
                let hovered = top_window_at_point(cursor.x, cursor.y, screen_w, screen_h).map(|h| h.id);
                set_focus(&mut focused_window, hovered);
            }

            let alt_down = pressed_keys.contains(&Key::LeftAlt) || pressed_keys.contains(&Key::RightAlt);
            let shift_down = pressed_keys.contains(&Key::LeftShift) || pressed_keys.contains(&Key::RightShift);
            let tab_pressed = pressed_keys.contains(&Key::Tab) && !prev_keys.contains(&Key::Tab);
            let alt_pressed = alt_down && !alt_prev_down;
            let alt_released = !alt_down && alt_prev_down;
            if alt_pressed {
                let (order, max_z) = build_window_cycle_order();
                alt_cycle_order = order;
                alt_cycle_max_z = max_z;
            }
            if alt_released {
                alt_cycle_order.clear();
            }
            if alt_down && tab_pressed {
                if alt_cycle_order.is_empty() {
                    let (order, max_z) = build_window_cycle_order();
                    alt_cycle_order = order;
                    alt_cycle_max_z = max_z;
                }
                if let Some(next) = cycle_windows_in_order(
                    &alt_cycle_order,
                    focused_window,
                    shift_down,
                    &mut alt_cycle_max_z,
                ) {
                    focused_window = Some(next);
                    force_full_damage = true;
                }
            }
            alt_prev_down = alt_down;
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

        let paint_result = {
            crate::trace_span!("bloom.loop.paint_pipeline");
            paint_pipeline.run(screen_w, screen_h, &mut list)
        };
        
        // Damage Tracking (cursor is now blended post-damage, does not affect window damage)
        let bounds = damage::Rect::full(screen_w, screen_h);
        let mut damage = damage::Damage::empty(bounds);
        for rect in &paint_result.damage {
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

        if force_full_damage && damage.is_empty() {
            damage = damage::Damage::full(bounds);
        }

        {
            crate::trace_span!("bloom.loop.damage");
            // damage calculation trace (already mostly done but wrapping ensures consistency)
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
        {
            crate::trace_span!("bloom.loop.raster");
            raster::execute_with_damage(&mut surface, &list, &damage, false);
        }
        
        // Cursor overlay: blend cached snapshot at cursor position (post-damage)
        // This ensures cursor movement does not trigger window repaints
        if let Some(asset) = ASSETS.get_cursor() {
            if let Some(snapshot) = cursor_rasterizer.get_snapshot(&asset) {
                let cx = cursor.x - snapshot.hotspot_x;
                let cy = cursor.y - snapshot.hotspot_y;
                raster::blit_cursor_overlay(&mut surface, &snapshot.image, cx, cy);
            }
        }

        {
            crate::trace_span!("bloom.loop.present");
            let token = builder.finish();
            presenter.present_frame(token);
            presenter.pump();
        }
        loop_ctrl.sleep();
    }
}
