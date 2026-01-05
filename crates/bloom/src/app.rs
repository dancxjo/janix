use crate::input::PointerInput;
use crate::pixels::*;
use crate::scene::Rect;
use crate::scene_cache::{apply_watch_event, SceneCache};
use crate::shadow::{draw_shadow_from_mask, ShadowMask, ShadowParams};
use crate::ui::{
    read_window_scene, render_window_scenes, window_ids_in_graph, WindowScene,
};
use crate::watch::WatchSet;
use abi::ids::ThingId;
use abi::types::{WatchEvent, WatchEventKind};
use models::*;
use thing_std::graph::*;
use thing_std::*;

use crate::assets::cursor::{CursorAnimator, CursorAsset, CursorFrame};
use crate::backend::*;
use crate::assets::bmp::Wallpaper;

const WATCH_WAIT_TIMEOUT_TICKS: u64 = 2;

pub fn run() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    let mut graph_client = SyscallGraphClient;

    loop {
        if let Some(display_id) = thing_find("device.display0") {
            let (width, height) =
                if let Ok(display) = DisplayDevice::read(&SyscallGraphClient, display_id) {
                    (display.width, display.height)
                } else {
                    (1280u32, 720u32)
                };

            let mut backend = CpuBytespaceBackend::new(
                thing_find("bytespace.display0").expect("bytespace not found"),
                0xA000_0000u64,
            );
            backend.configure(SurfaceDesc {
                width: width,
                height: height,
                stride_pixels: width,
            });

            // Map mouse input bytespace
            let mut input = if let Some(mouse_bs_id) = thing_find("bytespace.mouse_input") {
                let mouse_vaddr = 0x8820_0000u64;
                let mouse_size = 8192u64;
                thing_std::memory::space_map(mouse_bs_id, mouse_vaddr, 0, mouse_size);
                log_info("BLOOM: mapped bytespace.mouse_input");
                PointerInput::new(mouse_vaddr as *const u8, mouse_size as u32, width, height)
            } else {
                PointerInput::new(core::ptr::null(), 0, width, height)
            };

            let buffer_size = (width * height) as usize;
            let mut frame_buffer = alloc::vec![0u32; buffer_size];
            let mut scene_buffer = alloc::vec![0u32; buffer_size];
            let mut wallpaper_cache = alloc::vec![0u32; buffer_size];

            let wallpaper = thing_find("bytespace.asset.clouds.bmp").and_then(|id| {
                let len = 128 * 1024 * 1024;
                let buf = crate::assets::map_bytespace(id, 0x8000_0000, len);
                crate::assets::bmp::parse_bmp(buf)
            });

            if let Some(ref wp) = wallpaper {
                log_info(&alloc::format!(
                    "BLOOM: wallpaper clouds.bmp {}x{}",
                    wp.width,
                    wp.height
                ));
                render_wallpaper_full(wallpaper_cache.as_mut_ptr(), width, height, wp);
            } else {
                log_info("BLOOM: no wallpaper asset found");
            }

            let windows_graph = if let Some(id) = thing_find("graph.windows") {
                id
            } else {
                log_info("BLOOM: graph.windows NOT FOUND");
                sched_yield();
                continue;
            };

            let mut watches = match WatchSet::new(windows_graph) {
                Ok(set) => {
                    log_info(&alloc::format!(
                        "BLOOM: watch graph={} watch_id={}",
                        windows_graph.low(),
                        set.graph_watch.0
                    ));
                    set
                }
                Err(e) => {
                    log_info(&alloc::format!("BLOOM: watch graph setup failed status={}", e));
                    sched_yield();
                    continue;
                }
            };

            let mut scene_cache = SceneCache::new();
            seed_scene(
                &mut graph_client,
                windows_graph,
                &mut scene_cache,
                &mut watches,
            );

            // Load cursor assets
            let cursor_asset = load_cursor_asset();

            // Use milliseconds for animation timing (monotonic_now returns nanoseconds)
            let mut animator = cursor_asset.map(|asset| CursorAnimator::new(asset, 1));
            let mut logged_shared_shadow = false;
            let mut prev_px = 0;
            let mut prev_py = 0;
            let mut prev_buttons: u16 = 0;
            let mut logged_window_once = false;
            let mut prev_cursor_bounds: Option<Rect> = None;
            let mut scene_dirty = true;
            let mut window_scenes: alloc::vec::Vec<WindowScene> = scene_cache.scenes_in_order();
            let mut events: alloc::vec::Vec<WatchEvent> = alloc::vec::Vec::new();

            loop {
                if let Err(e) = watches.wait(WATCH_WAIT_TIMEOUT_TICKS) {
                    log_info(&alloc::format!("BLOOM: watch wait error status={}", e));
                }

                if let Ok(n) = watches.drain(&mut events) {
                    if n > 0 {
                        log_info(&alloc::format!("BLOOM: events n={}", n));
                        for ev in events.iter().take(n) {
                            log_info(&alloc::format!(
                                "BLOOM: event kind={} subject={} arg0={}",
                                event_kind_name(ev.kind),
                                ev.subject.low(),
                                ev.arg0.low()
                            ));
                            if let Some((win_id, watch_id)) =
                                apply_watch_event(ev, &mut scene_cache, &mut watches, &mut graph_client)
                            {
                                log_info(&alloc::format!(
                                    "BLOOM: watch window={} watch_id={}",
                                    win_id.low(),
                                    watch_id.0
                                ));
                            }
                        }
                        events.clear();
                    }
                }

                if scene_cache.is_dirty() {
                    scene_dirty = true;
                }

                // Get current time in milliseconds
                let now_ms = (monotonic_now() / 1_000_000) as u64;

                let (px, py, buttons) = input.poll();
                let moved = px != prev_px || py != prev_py;
                let buttons_changed = buttons != prev_buttons;

                // Animation update
                let anim_changed = if let Some(anim) = animator.as_mut() {
                    anim.advance(now_ms)
                } else {
                    false
                };

                let cursor_frame = if let Some(anim) = animator.as_ref() {
                    anim.current_frame()
                } else {
                    None
                };
                let cursor_bounds = cursor_bounds(cursor_frame, px, py);
                let cursor_changed = moved || anim_changed || buttons_changed;

                let mut dirty: Option<Rect> = None;

                if scene_dirty {
                    window_scenes = scene_cache.scenes_in_order();
                    rebuild_scene(
                        scene_buffer.as_mut_slice(),
                        wallpaper_cache.as_slice(),
                        width,
                        height,
                        &window_scenes,
                    );
                    unsafe {
                        redraw_region(
                            frame_buffer.as_mut_ptr(),
                            scene_buffer.as_ptr(),
                            width,
                            height,
                            Rect {
                                x: 0,
                                y: 0,
                                w: width,
                                h: height,
                            },
                        );
                    }
                    merge_damage(&mut dirty, Rect { x: 0, y: 0, w: width, h: height });
                    if !window_scenes.is_empty() && !logged_window_once {
                        let win_id = window_scenes[0].id.low();
                        log_info(&alloc::format!("BLOOM: rendered window {}", win_id));
                        logged_window_once = true;
                    }
                    log_info(&alloc::format!(
                        "BLOOM: redraw windows={} dirty={}",
                        window_scenes.len(),
                        scene_cache.dirty_count()
                    ));
                    scene_cache.clear_dirty();
                } else if cursor_changed {
                    let damage = if let Some(prev_bounds) = prev_cursor_bounds {
                        Rect::union(prev_bounds, cursor_bounds)
                    } else {
                        cursor_bounds
                    };
                    unsafe {
                        redraw_region(
                            frame_buffer.as_mut_ptr(),
                            scene_buffer.as_ptr(),
                            width,
                            height,
                            damage,
                        );
                    }
                    merge_damage(&mut dirty, damage);
                }

                if scene_dirty || cursor_changed {
                    if let Some(frame) = cursor_frame {
                        unsafe {
                            draw_shadow_from_mask(
                                frame_buffer.as_mut_ptr(),
                                width,
                                height,
                                px - frame.hotspot_x,
                                py - frame.hotspot_y,
                                ShadowMask::SpriteAlpha {
                                    pixels: &frame.shadow_pixels,
                                    width: frame.width,
                                    height: frame.height,
                                },
                                ShadowParams {
                                    offset_x: frame.shadow_offset_x,
                                    offset_y: frame.shadow_offset_y,
                                    blur_radius: 0,
                                    color: 0xAA000000,
                                },
                            );
                            draw_cursor_frame(
                                frame_buffer.as_mut_ptr(),
                                width,
                                height,
                                frame,
                                px,
                                py,
                            );
                        }
                        if !logged_shared_shadow {
                            log_info("BLOOM: shadow kernel: shared");
                            logged_shared_shadow = true;
                        }
                    } else {
                        unsafe {
                            draw_fallback_cursor(
                                frame_buffer.as_mut_ptr(),
                                width,
                                height,
                                px,
                                py,
                            );
                        }
                    }
                    merge_damage(&mut dirty, cursor_bounds);
                    prev_cursor_bounds = Some(cursor_bounds);
                }

                if let Some(rect) = dirty {
                    backend.present(
                        frame_buffer.as_mut_slice(),
                        DirtyRect {
                            x: rect.x,
                            y: rect.y,
                            w: rect.w,
                            h: rect.h,
                        },
                    );
                    scene_dirty = false;
                    prev_px = px;
                    prev_py = py;
                    prev_buttons = buttons;
                }
                sched_yield();
            }
        }
        sched_yield();
    }
}

fn seed_scene(
    client: &mut SyscallGraphClient,
    windows_graph: ThingId,
    scene_cache: &mut SceneCache,
    watches: &mut WatchSet,
) {
    for window_id in window_ids_in_graph(windows_graph) {
        if let Some(scene) = read_window_scene(client, window_id) {
            scene_cache.upsert(scene);
            if let Ok(Some(window_watch)) = watches.ensure_window_watch(window_id) {
                log_info(&alloc::format!(
                    "BLOOM: watch window={} watch_id={}",
                    window_id.low(),
                    window_watch.0
                ));
            }
        }
    }
    scene_cache.mark_scene_dirty();
}

fn event_kind_name(kind: WatchEventKind) -> &'static str {
    match kind {
        WatchEventKind::GraphMemberAdded => "GRAPH_MEMBER_ADDED",
        WatchEventKind::GraphMemberRemoved => "GRAPH_MEMBER_REMOVED",
        WatchEventKind::ThingUpdated => "THING_UPDATED",
        WatchEventKind::ThingDeleted => "THING_DELETED",
    }
}

fn load_cursor_asset() -> Option<CursorAsset> {
    if let Some(bs_id) = thing_find("bytespace.asset.Normal.cur") {
        if let Some(asset) = load_cur_asset(bs_id, 0x8800_0000) {
            log_info("BLOOM: loaded Normal.cur");
            return Some(asset);
        }
    }
    if let Some(bs_id) = thing_find("bytespace.asset.Working.ani") {
        if let Some(asset) = load_ani_asset(bs_id, 0x8810_0000) {
            log_info("BLOOM: loaded Working.ani");
            return Some(asset);
        }
    }
    log_info("BLOOM: no cursor asset found, using fallback");
    None
}

fn load_cur_asset(bs_id: ThingId, vaddr: u64) -> Option<CursorAsset> {
    let len: u64 = 256 * 1024;
    let buf = crate::assets::map_bytespace(bs_id, vaddr, len);
    crate::assets::cursor::cur::load_cur(buf).map(CursorAsset::static_cursor)
}

fn load_ani_asset(bs_id: ThingId, vaddr: u64) -> Option<CursorAsset> {
    let len: u64 = 1024 * 1024;
    let buf = crate::assets::map_bytespace(bs_id, vaddr, len);
    crate::assets::cursor::ani::load_ani(buf)
}

fn render_wallpaper_full(dest: *mut u32, dest_w: u32, dest_h: u32, wp: &Wallpaper) {
    for y in 0..dest_h {
        let src_y = y % wp.height;
        let actual_src_y = if wp.bottom_up {
            wp.height - 1 - src_y
        } else {
            src_y
        };
        let row_ptr = unsafe { wp.data_ptr.add(actual_src_y as usize * wp.row_stride) };
        let dest_row = unsafe { dest.add((y * dest_w) as usize) };
        unsafe {
            for x in 0..dest_w {
                let src_x = x % wp.width;
                let src_ptr = row_ptr.add(src_x as usize * wp.bytes_per_pixel);
                let b = *src_ptr;
                let g = *src_ptr.add(1);
                let r = *src_ptr.add(2);
                *dest_row.add(x as usize) =
                    0xFF000000u32 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            }
        }
    }
}

fn rebuild_scene(
    scene_buffer: &mut [u32],
    wallpaper_cache: &[u32],
    width: u32,
    height: u32,
    scenes: &[WindowScene],
) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            wallpaper_cache.as_ptr(),
            scene_buffer.as_mut_ptr(),
            scene_buffer.len(),
        );
    }
    render_window_scenes(scene_buffer.as_mut_ptr(), width, height, scenes);
}

fn merge_damage(into: &mut Option<Rect>, rect: Rect) {
    if rect.w == 0 || rect.h == 0 {
        return;
    }
    *into = Some(if let Some(existing) = *into {
        Rect::union(existing, rect)
    } else {
        rect
    });
}

fn cursor_bounds(frame: Option<&CursorFrame>, px: i32, py: i32) -> Rect {
    match frame {
        Some(frame) => {
            let sprite_x = px - frame.hotspot_x;
            let sprite_y = py - frame.hotspot_y;
            let shadow_x = sprite_x + frame.shadow_offset_x;
            let shadow_y = sprite_y + frame.shadow_offset_y;

            let min_x = sprite_x.min(shadow_x);
            let min_y = sprite_y.min(shadow_y);
            let max_x = (sprite_x + frame.width as i32).max(shadow_x + frame.width as i32);
            let max_y = (sprite_y + frame.height as i32).max(shadow_y + frame.height as i32);

            Rect {
                x: min_x,
                y: min_y,
                w: (max_x - min_x) as u32,
                h: (max_y - min_y) as u32,
            }
        }
        None => Rect {
            x: px,
            y: py,
            w: 10,
            h: 11,
        },
    }
}
