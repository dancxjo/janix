use crate::input::PointerInput;
use crate::painter::{Clip, CpuPainter, Painter};
use crate::scene::Rect;
use crate::scene_cache::{apply_watch_event, SceneCache};
use crate::ui::{
    read_window_scene, render_window_scenes, window_ids_in_graph, WindowScene,
};
use crate::watch::WatchSet;
use abi::ids::ThingId;
use abi::types::WatchEvent;
use models::*;
use thing_std::graph::*;
use thing_std::*;
use thing_std::{trace_fn, trace_enter, trace_exit};

use abi::ui::{HitZone, ResizeEdge};
use crate::ui::hittest::hittest_window;
use crate::cursor_manager::{CursorSet, CursorKind};
use crate::cursor_overlay::CursorOverlay;
use crate::chunked_executor::ChunkedExecutor;
use crate::wallpaper_worker::{WALLPAPER_MBX, load_wallpaper_sync};
use crate::boot_fade::{WALLPAPER_READY, CURRENT_BG_COLOR, boot_fade_entry, configure_display};
use crate::input_worker::{InputWorkerConfig, INPUT_CONFIG_MBX, INPUT_STATE, input_worker_entry, INPUT_WORKER_STARTED, INPUT_WORKER_TICKS, INPUT_EVENTS_DRAINED};
use crate::wallpaper_worker::{WALLPAPER_WORKER_STARTED, WALLPAPER_WORKER_PHASE};
use core::sync::atomic::Ordering;


use crate::assets::cursor::CursorFrame;
use crate::assets::bitmap::BitmapStore;
use crate::backend::*;

const WATCH_WAIT_TIMEOUT_TICKS: u64 = 0;

const RENDER_MODE_IMMEDIATE: u8 = 0;
const RENDER_MODE_RECORD: u8 = 1;
const CURRENT_RENDER_MODE: u8 = RENDER_MODE_RECORD;

pub fn run() {
    trace_fn!("bloom_run");
    thing_std::init(0);
    log_info("BLOOM: alive");

    // Warm up fonts early (SIMD-intensive parsing done before render loop)
    // TODO: move this to its own service process but somehow share the font cache; start loading ASAP
    crate::text::ensure_font_loaded();
    log_info("BLOOM: fonts warmed up");

    // Solid background color #2e80d2
    let background_color: u32 = CURRENT_BG_COLOR.load(core::sync::atomic::Ordering::Relaxed);

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
            
            // Initialize input state with screen center immediately (before first paint)
            INPUT_STATE.init(width, height);

            // Configure boot fade with display info
            configure_display(0xA000_0000u64, width, height, width);
            // Spawn boot fade animation thread (after display ready, avoid graph store deadlock)
            let _fade_thread = thing_std::thread::thread_spawn(boot_fade_entry, 0);
            log_info("BLOOM: boot fade thread spawned");

            let buffer_size = (width * height) as usize;
            let mut frame_buffer = alloc::vec![0u32; buffer_size];
            let mut background_cache = alloc::vec![0u32; buffer_size];
            let mut scene_buffer = alloc::vec![0u32; buffer_size];

            // Initial paint using CpuPainter
            {
                let mut painter = CpuPainter::new(frame_buffer.as_mut_slice(), width, height);
                painter.clear(background_color);
            }
            unsafe {
                core::ptr::copy_nonoverlapping(frame_buffer.as_ptr(), background_cache.as_mut_ptr(), buffer_size);
            }
            backend.present(frame_buffer.as_slice(), DirtyRect { x: 0, y: 0, w: width, h: height });
            log_info("BLOOM: first paint complete");

            let mut cursor_set = CursorSet::new();
            cursor_set.load_all(0x8900_0000);
            crate::shadow_cache::init_shadow_cache(8); // Default blur radius
            let mut cursor_overlay = CursorOverlay::new(width, height);
            log_info("BLOOM: cursor set loaded");

            let mut bitmap_store = BitmapStore::new();
            let mut wallpaper_handle: Option<(crate::assets::bitmap::BitmapHandle, u32, u32)> = None;

            // Load wallpaper synchronously (worker thread heap too slow)
            if let Some(wp) = load_wallpaper_sync() {
                log_info(&alloc::format!(
                    "BLOOM: wallpaper loaded {}x{}", wp.width, wp.height
                ));
                let bmp = crate::assets::bitmap::Bitmap {
                    w: wp.width,
                    h: wp.height,
                    pixels: alloc::sync::Arc::from(wp.pixels),
                };
                wallpaper_handle = Some((bitmap_store.add(bmp), wp.width, wp.height));
                WALLPAPER_READY.store(true, core::sync::atomic::Ordering::Release);
            }
            
            // Spawn input worker thread (drains ringbuffer, publishes atomics)
            // Also keep a fallback PointerInput for main thread in case worker fails
            let mut fallback_input = if let Some(mouse_bs_id) = thing_find("bytespace.mouse_input") {
                let mouse_vaddr = 0x8820_0000u64;
                let mouse_size = 8192u64;
                thing_std::memory::space_map(mouse_bs_id, mouse_vaddr, 0, mouse_size);
                log_info("BLOOM: mapped bytespace.mouse_input");
                
                // Send config to worker
                let cfg = InputWorkerConfig {
                    ring_ptr: mouse_vaddr as *const u8,
                    capacity: mouse_size as u32,
                    screen_w: width,
                    screen_h: height,
                };
                let _ = INPUT_CONFIG_MBX.try_send(cfg);
                
                // Spawn input worker
                let _input_worker = thing_std::thread::thread_spawn(input_worker_entry, 0);
                log_info("BLOOM: spawned input worker");
                
                PointerInput::new(mouse_vaddr as *const u8, mouse_size as u32, width, height)
            } else {
                log_info("BLOOM: no mouse input bytespace, cursor at center");
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

            // Input State
            #[derive(Debug, Clone, Copy)]
            enum DragMode {
                None,
                Move { start_wx: i32, start_wy: i32, start_px: i32, start_py: i32 },
                Resize { start_rect: Rect, start_px: i32, start_py: i32, edge: HitZone },
            }
            let mut drag_mode = DragMode::None;
            let mut captured_window: Option<ThingId> = None;
            let mut focused_window: Option<ThingId> = None;


            loop {
                let now_ns = monotonic_now();
                
                // Poll for wallpaper worker result (non-blocking)
                if let Some(wp) = WALLPAPER_MBX.try_take() {
                    log_info(&alloc::format!(
                        "BLOOM: wallpaper ready {}x{} from worker",
                        wp.width, wp.height
                    ));
                    let bmp = crate::assets::bitmap::Bitmap {
                        w: wp.width,
                        h: wp.height,
                        pixels: alloc::sync::Arc::from(wp.pixels),
                    };
                    let h = bitmap_store.add(bmp);
                    wallpaper_handle = Some((h, wp.width, wp.height));
                    scene_dirty = true;
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

                // --- Worker thread diagnostics (bypasses worker logging path) ---
                static mut LAST_DIAG_MS: u64 = 0;
                let diag_interval = 2000; // Print every 2 seconds
                unsafe {
                    if now_ms >= LAST_DIAG_MS + diag_interval {
                        let wp_started = WALLPAPER_WORKER_STARTED.load(Ordering::Acquire);
                        let inp_started = INPUT_WORKER_STARTED.load(Ordering::Acquire);
                        let inp_ticks = INPUT_WORKER_TICKS.load(Ordering::Acquire);
                        log_info(&alloc::format!(
                            "BLOOM DIAG: wp_started={} wp_phase={} inp_started={} inp_ticks={} events_drained={}",
                            wp_started, WALLPAPER_WORKER_PHASE.load(Ordering::Acquire), inp_started, inp_ticks, INPUT_EVENTS_DRAINED.load(Ordering::Relaxed)
                        ));
                        LAST_DIAG_MS = now_ms;
                    }
                }

                // Try to get input from worker thread atomics first
                let (mut px, mut py, mut buttons, input_seq) = INPUT_STATE.load();
                static mut LAST_INPUT_SEQ: u32 = 0;
                let input_seq_changed = unsafe {
                    let changed = input_seq != LAST_INPUT_SEQ;
                    LAST_INPUT_SEQ = input_seq;
                    changed
                };
                
                // Fallback: if worker isn't producing (seq==0), poll directly
                // This ensures cursor works even if threading has issues
                if input_seq == 0 {
                    (px, py, buttons) = fallback_input.poll();
                }
                
                let moved = px != prev_px || py != prev_py || input_seq_changed;
                let buttons_changed = buttons != prev_buttons;

                // Handle Input Events
                if buttons_changed || moved {
                    let left_down = (buttons & 1) != 0;
                    let prev_left_down = (prev_buttons & 1) != 0;
                    let just_pressed = left_down && !prev_left_down;
                    let just_released = !left_down && prev_left_down;

                    if just_pressed {
                        // 1. Hit Test (Top to Bottom)
                        // Note: window_scenes are in draw order (bottom to top), so iterate reverse
                        let mut hit_target = None;
                        for scene in window_scenes.iter().rev() {
                            let (zone, _lx, _ly) = hittest_window(scene, px, py);
                            if zone != HitZone::None {
                                hit_target = Some((scene.clone(), zone));
                                break;
                            }
                        }

                        if let Some((target, zone)) = hit_target {
                            // Focus & Raise (todo: graph reorder)
                            focused_window = Some(target.id);
                            
                            // Start Drag/Resize
                            match zone {
                                HitZone::Titlebar => {
                                    // Check Close Button
                                    let win_rect = crate::scene::Rect {
                                        x: target.window.x,
                                        y: target.window.y,
                                        w: target.window.width,
                                        h: target.window.height,
                                    };
                                    let close_rect = crate::ui::get_close_button_rect(win_rect);
                                    if px >= close_rect.x && px < close_rect.x + close_rect.w as i32 &&
                                       py >= close_rect.y && py < close_rect.y + close_rect.h as i32 
                                    {
                                        log_info("BLOOM: Clicked Close Button (Request Not Implemented)");
                                        // TODO: Send WindowActionKind::Close when available
                                    } else {
                                        #[cfg(feature = "window_moving")]
                                        {
                                            log_info("BLOOM: Begin Move");
                                            drag_mode = DragMode::Move { 
                                                start_wx: target.window.x, 
                                                start_wy: target.window.y, 
                                                start_px: px, 
                                                start_py: py 
                                            };
                                            captured_window = Some(target.id);
                                            let action = WindowAction {
                                                window: target.id,
                                                kind: abi::ui::WindowActionKind::BeginMove,
                                                start_x: target.window.x,
                                                start_y: target.window.y,
                                                dx: 0,
                                                dy: 0,
                                                edges: ResizeEdge::None,
                                            };
                                            log_info("BLOOM: Creating Move Action");
                                            let _ = action.create(&mut graph_client);
                                            log_info("BLOOM: Created Move Action");
                                        }
                                    }
                                }
                                HitZone::ResizeN | HitZone::ResizeS | HitZone::ResizeE | HitZone::ResizeW |
                                HitZone::ResizeNW | HitZone::ResizeNE | HitZone::ResizeSW | HitZone::ResizeSE => {
                                    log_info("BLOOM: Begin Resize");
                                    drag_mode = DragMode::Resize {
                                        start_rect: Rect { x: target.window.x, y: target.window.y, w: target.window.width, h: target.window.height },
                                        start_px: px,
                                        start_py: py,
                                        edge: zone,
                                    };
                                    captured_window = Some(target.id);
                                    let edge_flag = match zone {
                                        HitZone::ResizeN => ResizeEdge::Top,
                                        HitZone::ResizeS => ResizeEdge::Bottom,
                                        HitZone::ResizeE => ResizeEdge::Right,
                                        HitZone::ResizeW => ResizeEdge::Left,
                                        HitZone::ResizeNW => ResizeEdge::TopLeft,
                                        HitZone::ResizeNE => ResizeEdge::TopRight,
                                        HitZone::ResizeSW => ResizeEdge::BottomLeft,
                                        HitZone::ResizeSE => ResizeEdge::BottomRight,
                                        _ => ResizeEdge::None,
                                    };
                                    let _ = WindowAction {
                                        window: target.id,
                                        kind: abi::ui::WindowActionKind::BeginResize,
                                        start_x: target.window.x,
                                        start_y: target.window.y,
                                        dx: 0,
                                        dy: 0,
                                        edges: edge_flag,
                                    }.create(&mut graph_client);
                                }
                                HitZone::Content => {
                                    log_info("BLOOM: Content Click (TODO: Forward)");
                                    // captured_window = Some(target.id); // Valid for app drag/selection
                                }
                                _ => {}
                            }
                        } else {
                            // Clicked background
                            focused_window = None;
                        }
                    } else if just_released {
                        if let Some(win_id) = captured_window {
                            log_info("BLOOM: End Capture");
                            let (kind, start_x, start_y, dx, dy, edges) = match drag_mode {
                                #[cfg(feature = "window_moving")]
                                DragMode::Move { start_wx, start_wy, start_px, start_py } => 
                                    (abi::ui::WindowActionKind::EndMove, start_wx, start_wy, px - start_px, py - start_py, ResizeEdge::None),
                                DragMode::Resize { start_rect, start_px, start_py, edge } => {
                                     let edge_flag = match edge {
                                        HitZone::ResizeN => ResizeEdge::Top,
                                        HitZone::ResizeS => ResizeEdge::Bottom,
                                        HitZone::ResizeE => ResizeEdge::Right,
                                        HitZone::ResizeW => ResizeEdge::Left,
                                        HitZone::ResizeNW => ResizeEdge::TopLeft,
                                        HitZone::ResizeNE => ResizeEdge::TopRight,
                                        HitZone::ResizeSW => ResizeEdge::BottomLeft,
                                        HitZone::ResizeSE => ResizeEdge::BottomRight,
                                        _ => ResizeEdge::None,
                                    };
                                    (abi::ui::WindowActionKind::EndResize, start_rect.x, start_rect.y, px - start_px, py - start_py, edge_flag)
                                }
                                _ => (abi::ui::WindowActionKind::EndMove, 0, 0, 0, 0, ResizeEdge::None),
                            };
                            let action = WindowAction {
                                window: win_id,
                                kind,
                                start_x,
                                start_y,
                                dx,
                                dy,
                                edges,
                            };
                            log_info("BLOOM: Creating WindowAction");
                            let _ = action.create(&mut graph_client);
                            log_info("BLOOM: Created WindowAction");
                        }
                        captured_window = None;
                        drag_mode = DragMode::None;
                    } else if moved {
                         if let Some(win_id) = captured_window {
                             match drag_mode {
                                 #[cfg(feature = "window_moving")]
                                 DragMode::Move { start_wx, start_wy, start_px, start_py } => {
                                     let dx = px - start_px;
                                     let dy = py - start_py;
                                     let new_x = start_wx + dx;
                                     let new_y = start_wy + dy;
                                     
                                     // Update Graph
                                     // We need to read the Window thing, update x/y, write back
                                     // Optimization: we have the scene cached, but we should read fresh or just update safely?
                                     // Writing back to the Thing is the source of truth.
                                     if let Ok(mut win) = Window::read(&SyscallGraphClient, win_id) {
                                         if win.x != new_x || win.y != new_y {
                                             win.x = new_x;
                                             win.y = new_y;
                                             let _ = win.write(&mut graph_client, win_id);
                                             // Note: This trigger a watch event, which will update SceneCache next loop
                                         }
                                     }
                                 }
                                 DragMode::Resize { start_rect, start_px, start_py, edge } => {
                                     let dx = px - start_px;
                                     let dy = py - start_py;
                                     let mut new_rect = start_rect;
                                     
                                     const MIN_W: u32 = 50;
                                     const MIN_H: u32 = 50;

                                     match edge {
                                         HitZone::ResizeE => {
                                             let w = (start_rect.w as i32 + dx).max(MIN_W as i32);
                                             new_rect.w = w as u32;
                                         }
                                         HitZone::ResizeS => {
                                            let h = (start_rect.h as i32 + dy).max(MIN_H as i32);
                                            new_rect.h = h as u32;
                                         }
                                         HitZone::ResizeSE => {
                                             let w = (start_rect.w as i32 + dx).max(MIN_W as i32);
                                             let h = (start_rect.h as i32 + dy).max(MIN_H as i32);
                                             new_rect.w = w as u32;
                                             new_rect.h = h as u32;
                                         }
                                         // TODO: Implement other edges (requires x/y shift)
                                         _ => {}
                                     }
                                     
                                     if let Ok(mut win) = Window::read(&SyscallGraphClient, win_id) {
                                         if win.width != new_rect.w || win.height != new_rect.h {
                                             win.width = new_rect.w;
                                             win.height = new_rect.h;
                                             let _ = win.write(&mut graph_client, win_id);
                                         }
                                     }
                                 }
                                 _ => {}
                             }
                         }
                    }
                }

                // Input handling happens before scene update
                // Update active cursor based on drag mode or hit
                
                if let DragMode::None = drag_mode {
                    // Simple hit testing for cursor update
                    let (x, y) = (px, py);
                    let mut found = false;
                    let mut hover_needs_update = false;
                    
                    for scene in window_scenes.iter().rev() {
                         let (wx, wy) = (scene.window.x, scene.window.y);
                         let (ww, wh) = (scene.window.width, scene.window.height);
                         
                         if x >= wx && x < wx + ww as i32 && y >= wy && y < wy + wh as i32 {
                             // Hit this window
                             let (zone, _, _) = crate::ui::hittest::hittest_window(scene, x, y);
                             let kind = match zone {
                                 HitZone::Titlebar => {
                                     // Check for close button hover trigger
                                     // In recording mode, hover changes visual, so update dirty
                                     // Optimization: track prev hovered window/button?
                                     // For now, always dirty if hitting titlebar? Too expensive.
                                     // Just depend on movement.
                                     // DISABLED: hover_needs_update = true; // causes freeze - full scene rebuild too expensive
                                     CursorKind::Default
                                 }, 
                                 HitZone::Border => CursorKind::Default,
                                 HitZone::ResizeN | HitZone::ResizeS => CursorKind::ResizeV,
                                 HitZone::ResizeE | HitZone::ResizeW => CursorKind::ResizeH,
                                 HitZone::ResizeNW | HitZone::ResizeSE => CursorKind::ResizeNWSE,
                                 HitZone::ResizeNE | HitZone::ResizeSW => CursorKind::ResizeNESW,
                                 HitZone::Content => CursorKind::Default,
                                 _ => CursorKind::Default,
                             };
                             cursor_set.set_cursor(kind);
                             found = true;
                             break;
                         }
                    }
                    if !found {
                        cursor_set.set_cursor(CursorKind::Default);
                    }
                    cursor_set.set_override(None);
                    if false && hover_needs_update && moved { // DISABLED: causes freeze
                        scene_dirty = true;
                    }
                } else {
                    // In drag mode.
                    let kind = match drag_mode {
                        #[cfg(feature = "window_moving")]
                        DragMode::Move { .. } => CursorKind::Move,
                        DragMode::Resize { edge, .. } => match edge {
                                 HitZone::ResizeN | HitZone::ResizeS => CursorKind::ResizeV,
                                 HitZone::ResizeE | HitZone::ResizeW => CursorKind::ResizeH,
                                 HitZone::ResizeNW | HitZone::ResizeSE => CursorKind::ResizeNWSE,
                                 HitZone::ResizeNE | HitZone::ResizeSW => CursorKind::ResizeNESW,
                                 _ => CursorKind::Default,
                        },
                        _ => CursorKind::Default,
                    };
                    cursor_set.set_override(Some(kind));
                }

                let anim_changed = if let Some(animator) = cursor_set.current_animator() {
                    animator.advance(now_ms);
                    true // Assume advance always changes something for simplicity or check return
                } else {
                    false
                };

                let cursor_frame = if let Some(anim) = cursor_set.current_animator() {
                    anim.current_frame()
                } else {
                    None
                };
                let cursor_bounds = cursor_bounds(cursor_frame, px, py);
                let cursor_changed = moved || anim_changed || buttons_changed;

                let mut dirty: Option<Rect> = None;

                if scene_dirty {
                    let _scene_trace_ns = trace_enter!("scene_rebuild");
                    window_scenes = scene_cache.scenes_in_order();
                    let win_count = window_scenes.len();
                    
                    if win_count > 0 && !logged_window_once {
                        log_info(&alloc::format!("BLOOM: START render {} windows", win_count));
                    }
                    
                    scene_cache.mapping_cache.reset_frame_stats();

                    // Rebuild scene
                    if CURRENT_RENDER_MODE == RENDER_MODE_RECORD {
                         // 1. Initialize buffer with background (REMOVED in favor of TileBitmap)
                        /*
                        {
                            let mut bg_painter = CpuPainter::new(scene_buffer.as_mut_slice(), width, height);
                            bg_painter.copy_region(background_cache.as_slice(), width, Rect { x: 0, y: 0, w: width, h: height });
                        }
                        */
                        
                        // 2. Record commands
                        let mut recorder = crate::command_recorder::CommandRecorder::new(width, height);
                        
                        // Background Layer
                        if let Some((handle, w, h)) = wallpaper_handle {
                            recorder.cmds.push(crate::draw_cmd::DrawCmd::TileBitmap { 
                                dst: Rect { x: 0, y: 0, w: width, h: height }, 
                                bitmap: handle,
                                bmp_w: w,
                                bmp_h: h,
                                origin: crate::scene::Point { x: 0, y: 0 },
                                opacity: 255,
                            });
                        } else {
                            recorder.clear(background_color);
                        }

                        render_window_scenes(&mut recorder, &window_scenes, &mut scene_cache.mapping_cache, focused_window, (px, py));
                        let cmds = recorder.finish();
                        
                        // 3. Execute (monolithic for now - cursor updates after)
                        let exec_result = crate::executor::execute_cmds_into_scene(&cmds, scene_buffer.as_mut_slice(), width, height, &mut scene_cache.mapping_cache, &bitmap_store);
                        if exec_result.stats.bad_cmds > 0 {
                            log_info(&alloc::format!("BLOOM: Bad cmds: {}/{}", exec_result.stats.bad_cmds, exec_result.stats.cmds_total));
                        }
                        
                        // Merge actual damage from execution
                        if let Some(damage_rect) = exec_result.damage.rect {
                            merge_damage(&mut dirty, damage_rect);
                        } else {
                            // See comment in original file about simplified damage handling here
                        }
                    } else {
                        // Immediate Mode
                        let mut painter = CpuPainter::new(scene_buffer.as_mut_slice(), width, height);
                        // Copy background
                        painter.copy_region(background_cache.as_slice(), width, Rect { x: 0, y: 0, w: width, h: height });
                        // Render windows
                        render_window_scenes(&mut painter, &window_scenes, &mut scene_cache.mapping_cache, focused_window, (px, py));
                    }
                    
                    scene_cache.mapping_cache.log_frame_stats();
                    
                    if win_count > 0 && !logged_window_once {
                        log_info("BLOOM: rebuild_scene done");
                    }
                    
                    // Copy scene to frame buffer
                    {
                        let mut painter = CpuPainter::new(frame_buffer.as_mut_slice(), width, height);
                        // For now, full update because we did full background copy.
                        // Ideally we restrict background copy to damage too.
                        painter.copy_region(scene_buffer.as_slice(), width, Rect { x: 0, y: 0, w: width, h: height });
                    }
                    
                    if win_count > 0 && !logged_window_once {
                        log_info("BLOOM: redraw_region done");
                    }
                    
                    // Present full screen
                    merge_damage(&mut dirty, Rect { x: 0, y: 0, w: width, h: height });
                    
                    if !window_scenes.is_empty() && !logged_window_once {
                        log_info(&alloc::format!("BLOOM: rendered window {}", window_scenes[0].id.low()));
                        logged_window_once = true;
                    }
                    scene_cache.clear_dirty();
                    trace_exit!("scene_rebuild", _scene_trace_ns);
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
                    
                    if let Some(animator) = cursor_set.current_animator() {
                        if let Some(frame) = animator.current_frame() {
                            #[cfg(feature = "shadows")]
                            painter.blit_rgba_alpha(
                                px - frame.hotspot_x + frame.shadow_offset_x,
                                py - frame.hotspot_y + frame.shadow_offset_y,
                                &frame.shadow_pixels,
                                frame.width,
                                frame.height,
                            );
                            painter.draw_cursor_frame(frame, px, py);
                            // log_info("BLOOM: drew cursor");
                            if !logged_shared_shadow {
                                log_info("BLOOM: shadow kernel: shared");
                                logged_shared_shadow = true;
                            }
                        } else {
                            painter.draw_fallback_cursor(px, py);
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
