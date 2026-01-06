use crate::input::PointerInput;
use crate::painter::{Clip, CpuPainter, Painter};
use crate::scene::Rect;
use crate::scene_cache::{apply_watch_event, SceneCache};
#[cfg(feature = "shadows")]
use crate::shadow::{ShadowMask, ShadowParams};
use crate::ui::{
    read_window_scene, render_window_scenes, window_ids_in_graph, WindowScene,
};
use crate::watch::WatchSet;
use abi::ids::ThingId;
use abi::types::WatchEvent;
use models::*;
use thing_std::graph::*;
use thing_std::*;

use crate::assets::cursor::{CursorAnimator, CursorAsset, CursorFrame};
use crate::backend::*;

const WATCH_WAIT_TIMEOUT_TICKS: u64 = 0;
const KERNEL_HANDOFF_PROGRESS: u32 = 500;
const BLOOM_FADE_DURATION_MS: u64 = 8_000;

const RENDER_MODE_IMMEDIATE: u8 = 0;
const RENDER_MODE_RECORD: u8 = 1;
const CURRENT_RENDER_MODE: u8 = RENDER_MODE_RECORD;

pub fn run() {
    thing_std::init(0);
    log_info("BLOOM: alive");

    // Warm up fonts early (SIMD-intensive parsing done before render loop)
    crate::text::ensure_font_loaded();
    log_info("BLOOM: fonts warmed up");

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
                width,
                height,
                stride_pixels: width,
            });

            let start_time_ns = monotonic_now();
            let mut current_progress = KERNEL_HANDOFF_PROGRESS;
            let mut progress_complete = false;

            let buffer_size = (width * height) as usize;
            let mut frame_buffer = alloc::vec![0u32; buffer_size];
            let mut background_cache = alloc::vec![0u32; buffer_size];
            let mut scene_buffer = alloc::vec![0u32; buffer_size];

            // Initial paint using CpuPainter
            {
                let mut painter = CpuPainter::new(frame_buffer.as_mut_slice(), width, height);
                paint_progress_background(&mut painter, current_progress);
            }
            unsafe {
                core::ptr::copy_nonoverlapping(frame_buffer.as_ptr(), background_cache.as_mut_ptr(), buffer_size);
            }
            backend.present(frame_buffer.as_slice(), DirtyRect { x: 0, y: 0, w: width, h: height });
            log_info("BLOOM: first paint complete");

            let mut animator: Option<CursorAnimator> = None;
            if let Some(asset) = load_cursor_asset() {
                animator = Some(CursorAnimator::new(asset, 1));
                log_info("BLOOM: cursor loaded");
                if !progress_complete {
                    current_progress = current_progress.saturating_add(50).min(1000);
                }
            }

            let mut input = if let Some(mouse_bs_id) = thing_find("bytespace.mouse_input") {
                let mouse_vaddr = 0x8820_0000u64;
                let mouse_size = 8192u64;
                thing_std::memory::space_map(mouse_bs_id, mouse_vaddr, 0, mouse_size);
                log_info("BLOOM: mapped bytespace.mouse_input");
                PointerInput::new(mouse_vaddr as *const u8, mouse_size as u32, width, height)
            } else {
                PointerInput::new(core::ptr::null(), 0, width, height)
            };

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
                    if !progress_complete {
                        current_progress = current_progress.saturating_add(50).min(1000);
                    }
                    set
                }
                Err(e) => {
                    log_info(&alloc::format!("BLOOM: watch graph setup failed status={}", e));
                    sched_yield();
                    continue;
                }
            };

            let mut scene_cache = SceneCache::new();
            let seeded_windows = seed_scene(
                &mut graph_client,
                windows_graph,
                &mut scene_cache,
                &mut watches,
            );
            
            if seeded_windows > 0 {
                log_info(&alloc::format!("BLOOM: seeded {} windows", seeded_windows));
            }

            let mut logged_shared_shadow = false;
            let mut prev_px = 0;
            let mut prev_py = 0;
            let mut prev_buttons: u16 = 0;
            let mut logged_window_once = false;
            let mut prev_cursor_bounds: Option<Rect> = None;
            let mut scene_dirty = true;
            let mut window_scenes: alloc::vec::Vec<WindowScene> = alloc::vec::Vec::new();
            let mut events: alloc::vec::Vec<WatchEvent> = alloc::vec::Vec::new();
            let mut last_progress = current_progress;

            loop {
                let now_ns = monotonic_now();
                
                if !progress_complete {
                    let elapsed_ms = ((now_ns - start_time_ns) / 1_000_000) as u64;
                    let time_progress = ((elapsed_ms * 500) / BLOOM_FADE_DURATION_MS).min(500) as u32;
                    current_progress = (KERNEL_HANDOFF_PROGRESS + time_progress).max(current_progress);
                    
                    if current_progress >= 1000 {
                        current_progress = 1000;
                        progress_complete = true;
                    }
                }

                if current_progress != last_progress {
                    let mut painter = CpuPainter::new(background_cache.as_mut_slice(), width, height);
                    paint_progress_background(&mut painter, current_progress);
                    scene_dirty = true;
                    last_progress = current_progress;
                }

                if let Ok(n) = watches.drain(&mut events) {
                    if n > 0 {
                        for ev in events.iter().take(n) {
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

                let now_ms = (now_ns / 1_000_000) as u64;

                let (px, py, buttons) = input.poll();
                let moved = px != prev_px || py != prev_py;
                let buttons_changed = buttons != prev_buttons;

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
                    let win_count = window_scenes.len();
                    
                    if win_count > 0 && !logged_window_once {
                        log_info(&alloc::format!("BLOOM: START render {} windows", win_count));
                    }
                    
                    scene_cache.mapping_cache.reset_frame_stats();

                    // Rebuild scene
                    if CURRENT_RENDER_MODE == RENDER_MODE_RECORD {
                         // 1. Initialize buffer with background
                        {
                            let mut bg_painter = CpuPainter::new(scene_buffer.as_mut_slice(), width, height);
                            bg_painter.copy_region(background_cache.as_slice(), width, Rect { x: 0, y: 0, w: width, h: height });
                        }
                        
                        // 2. Record commands
                        let mut recorder = crate::command_recorder::CommandRecorder::new(width, height);
                        render_window_scenes(&mut recorder, &window_scenes, &mut scene_cache.mapping_cache);
                        let cmds = recorder.finish();
                        
                        // 3. Execute
                        crate::executor::execute_cmds_into_scene(&cmds, scene_buffer.as_mut_slice(), width, height, &mut scene_cache.mapping_cache);
                    } else {
                        // Immediate Mode
                        let mut painter = CpuPainter::new(scene_buffer.as_mut_slice(), width, height);
                        // Copy background
                        painter.copy_region(background_cache.as_slice(), width, Rect { x: 0, y: 0, w: width, h: height });
                        // Render windows
                        render_window_scenes(&mut painter, &window_scenes, &mut scene_cache.mapping_cache);
                    }
                    
                    scene_cache.mapping_cache.log_frame_stats();
                    
                    if win_count > 0 && !logged_window_once {
                        log_info("BLOOM: rebuild_scene done");
                    }
                    
                    // Copy scene to frame buffer
                    {
                        let mut painter = CpuPainter::new(frame_buffer.as_mut_slice(), width, height);
                        painter.copy_region(scene_buffer.as_slice(), width, Rect { x: 0, y: 0, w: width, h: height });
                    }
                    
                    if win_count > 0 && !logged_window_once {
                        log_info("BLOOM: redraw_region done");
                    }
                    
                    merge_damage(&mut dirty, Rect { x: 0, y: 0, w: width, h: height });
                    
                    if !window_scenes.is_empty() && !logged_window_once {
                        log_info(&alloc::format!("BLOOM: rendered window {}", window_scenes[0].id.low()));
                        logged_window_once = true;
                        if !progress_complete {
                            current_progress = 1000;
                            progress_complete = true;
                            let mut painter = CpuPainter::new(background_cache.as_mut_slice(), width, height);
                            paint_progress_background(&mut painter, 1000);
                        }
                    }
                    scene_cache.clear_dirty();
                    scene_dirty = false;
                } else if cursor_changed {
                    let damage = if let Some(prev_bounds) = prev_cursor_bounds {
                        Rect::union(prev_bounds, cursor_bounds)
                    } else {
                        cursor_bounds
                    };
                    let mut painter = CpuPainter::new(frame_buffer.as_mut_slice(), width, height);
                    painter.copy_region(scene_buffer.as_slice(), width, damage);
                    merge_damage(&mut dirty, damage);
                }

                if scene_dirty || cursor_changed {
                    let mut painter = CpuPainter::new(frame_buffer.as_mut_slice(), width, height);
                    painter.set_clip(Clip::full(width, height));
                    
                    if let Some(frame) = cursor_frame {
                        #[cfg(feature = "shadows")]
                        painter.draw_shadow_mask(
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
                        painter.draw_cursor_frame(frame, px, py);
                        if !logged_shared_shadow {
                            log_info("BLOOM: shadow kernel: shared");
                            logged_shared_shadow = true;
                        }
                    } else {
                        painter.draw_fallback_cursor(px, py);
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
                    prev_px = px;
                    prev_py = py;
                    prev_buttons = buttons;
                }
                
                if WATCH_WAIT_TIMEOUT_TICKS > 0 {
                    if let Err(_e) = watches.wait(WATCH_WAIT_TIMEOUT_TICKS) {
                    }
                }
                sched_yield();
            }
        }
        sched_yield();
    }
}

fn paint_progress_background(painter: &mut dyn Painter, progress: u32) {
    let r1 = 0x05i32; let g1 = 0x05i32; let b1 = 0x05i32;
    let r2 = 0xDCi32; let g2 = 0xD0i32; let b2 = 0xFFi32;
    let p = progress.min(1000) as i32;
    let r = (r1 + (r2 - r1) * p / 1000) as u32;
    let g = (g1 + (g2 - g1) * p / 1000) as u32;
    let b = (b1 + (b2 - b1) * p / 1000) as u32;
    let color = 0xFF000000 | (r << 16) | (g << 8) | b;
    painter.clear(color);
}

fn seed_scene(
    client: &mut SyscallGraphClient,
    windows_graph: ThingId,
    scene_cache: &mut SceneCache,
    watches: &mut WatchSet,
) -> usize {
    let mut count = 0;
    for window_id in window_ids_in_graph(windows_graph) {
        if let Some(scene) = read_window_scene(client, window_id) {
            scene_cache.upsert(scene);
            count += 1;
            if let Ok(Some(window_watch)) = watches.ensure_window_watch(window_id) {
                log_info(&alloc::format!("BLOOM: watch window={} watch_id={}", window_id.low(), window_watch.0));
            }
        }
    }
    if count > 0 { scene_cache.mark_scene_dirty(); }
    count
}

fn load_cursor_asset() -> Option<CursorAsset> {
    if let Some(bs_id) = thing_find("bytespace.asset.Normal.cur") {
        let len: u64 = 256 * 1024;
        let buf = crate::assets::map_bytespace(bs_id, 0x8800_0000, len);
        if let Some(frame) = crate::assets::cursor::cur::load_cur(buf) {
            return Some(CursorAsset::static_cursor(frame));
        }
    }
    if let Some(bs_id) = thing_find("bytespace.asset.Working.ani") {
        let len: u64 = 1024 * 1024;
        let buf = crate::assets::map_bytespace(bs_id, 0x8810_0000, len);
        if let Some(asset) = crate::assets::cursor::ani::load_ani(buf) {
            return Some(asset);
        }
    }
    log_info("BLOOM: no cursor asset found, using fallback");
    None
}

fn merge_damage(into: &mut Option<Rect>, rect: Rect) {
    if rect.w == 0 || rect.h == 0 { return; }
    *into = Some(if let Some(existing) = *into { Rect::union(existing, rect) } else { rect });
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
            Rect { x: min_x, y: min_y, w: (max_x - min_x) as u32, h: (max_y - min_y) as u32 }
        }
        None => Rect { x: px, y: py, w: 10, h: 11 },
    }
}
