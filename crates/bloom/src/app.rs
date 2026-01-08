use crate::input::{PointerInput, InteractionEvent};
use crate::painter::{Clip, Painter};
use crate::scene::Rect;
use crate::scene_cache::{apply_watch_event, SceneCache};
use crate::ui::{
    read_window_scene, render_window_scenes, window_ids_in_graph, WindowScene,
};
use crate::watch::WatchSet;
use abi::ids::ThingId;
use abi::types::{WatchEvent, RelationshipRef};

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
use crate::present_loop::{PresentConfig, present_loop_entry};
use core::sync::atomic::Ordering;


use crate::assets::cursor::CursorFrame;
use crate::assets::bitmap::BitmapStore;
use crate::backend::*;

const WATCH_WAIT_TIMEOUT_TICKS: u64 = 0;

// Bloom uses a fully record-then-execute rendering pipeline.
// All scene content is recorded into DrawCmd list, then executed in a tight loop.
// The ONLY exception is cursor overlay, which is drawn immediately for responsiveness.

pub fn run() {
    trace_fn!("bloom_run");
    thing_std::init(0);
    log_info("BLOOM: alive");

    // Initialize font system (non-blocking - uses builtin font as fallback)
    crate::text::ensure_font_loaded();

    // Solid background color #2e80d2
    let background_color: u32 = CURRENT_BG_COLOR.load(core::sync::atomic::Ordering::Relaxed);

    let mut graph_client = SyscallGraphClient;

    loop {
        if let Some(display_id) = thing_find("device.display0") {
            let (width, height) =
                if let Ok(display) = DisplayDevice::read(&SyscallGraphClient, display_id) {
                     // Sanitize display dimensions to prevent overflow/panic
                     let w = if display.width > 4096 || display.width == 0 { 1280 } else { display.width };
                     let h = if display.height > 4096 || display.height == 0 { 720 } else { display.height };
                     (w, h)
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
            // configure_display(0xA000_0000u64, width, height, width);
            
            // --- START PRESENT LOOP ---
            // Allocate config on heap to ensure validity when thread reads it
            let present_config = alloc::boxed::Box::new(PresentConfig {
                fb_vaddr: 0xA000_0000,
                width,
                height,
                stride: width, // assuming stride == width for now
            });
            let config_ptr = alloc::boxed::Box::into_raw(present_config) as u64;
            
            let _present_thread = thing_std::thread::thread_spawn(present_loop_entry, config_ptr);
            log_info("BLOOM: present thread spawned");
            // --------------------------

            let buffer_size = (width * height) as usize;
            let mut frame_buffer = alloc::vec![0u32; buffer_size];
            let mut background_cache = alloc::vec![0u32; buffer_size];
            let mut scene_buffer = alloc::vec![0u32; buffer_size];
            let mut cursor_overlay = CursorOverlay::new(width, height);


            // Initial paint
            cursor_overlay.clear(frame_buffer.as_mut_slice(), background_color);
            unsafe {
                core::ptr::copy_nonoverlapping(frame_buffer.as_ptr(), background_cache.as_mut_ptr(), buffer_size);
            }
            if crate::boot_fade::WALLPAPER_READY.load(Ordering::Acquire) {
                backend.present(frame_buffer.as_slice(), DirtyRect { x: 0, y: 0, w: width, h: height });
            }
            log_info("BLOOM: first paint complete");

            let mut cursor_set = CursorSet::new();
            cursor_set.load_all(0x8900_0000);
            crate::shadow_cache::init_shadow_cache(8); // Default blur radius
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
            let mut scene_dirty = true;
            let mut window_scenes: alloc::vec::Vec<WindowScene> = alloc::vec::Vec::new();
            let mut events: alloc::vec::Vec<WatchEvent> = alloc::vec::Vec::new();
            let mut main_loop_iter: u64 = 0;  // DIAG: loop counter

            // Interaction Controller
            let mut interaction_controller = crate::interaction::InteractionController::new();
            let mut focused_window: Option<ThingId> = None;

            // Initialize Shadow Cache (radius=4 matches WindowStyle in ui.rs)
            crate::shadow_cache::init_shadow_cache(4);

            let mut current_exec: Option<ChunkedExecutor> = None;
            let mut target_damage: Option<Rect> = Some(Rect { x: 0, y: 0, w: width, h: height });

            loop {
                main_loop_iter += 1;
                if main_loop_iter <= 3 {
                    log_info(&alloc::format!("BLOOM LOOP: iter={}", main_loop_iter));
                }
                let now_ns = monotonic_now();
                if main_loop_iter <= 3 { log_info("BLOOM STEP: after monotonic_now"); }
                
                // Poll for wallpaper worker result
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
                    merge_damage(&mut target_damage, Rect { x: 0, y: 0, w: width, h: height });
                }
                if main_loop_iter <= 3 { log_info("BLOOM STEP: after wallpaper check"); }
            
                if let Ok(n) = watches.drain(&mut events) {
                    if main_loop_iter <= 3 { log_info(&alloc::format!("BLOOM STEP: after watches.drain n={}", n)); }
                    if n > 0 {
                        for ev in events.iter().take(n) {
                            if main_loop_iter <= 3 { log_info("BLOOM STEP: processing event"); }
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
                } else {
                    if main_loop_iter <= 3 { log_info("BLOOM STEP: watches.drain failed"); }
                }

                if scene_cache.is_dirty() {
                    scene_dirty = true;
                }

                if let Some(d) = scene_cache.take_damage() {
                    merge_damage(&mut target_damage, d);
                    scene_dirty = true;
                }

                let now_ms = (now_ns / 1_000_000) as u64;

                // --- Worker thread diagnostics ---
                static mut LAST_DIAG_MS: u64 = 0;
                let diag_interval = 2000; 
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

                // Try to get input from worker thread atomics
                let (mut px, mut py, mut buttons, input_seq) = INPUT_STATE.load();
                static mut LAST_INPUT_SEQ: u32 = 0;
                let input_seq_changed = unsafe {
                    let changed = input_seq != LAST_INPUT_SEQ;
                    LAST_INPUT_SEQ = input_seq;
                    changed
                };
                
                if input_seq == 0 {
                    (px, py, buttons) = fallback_input.poll();
                }
                
                let moved = px != prev_px || py != prev_py || input_seq_changed;
                let buttons_changed = buttons != prev_buttons;
                let old_focused_window = focused_window;

                // 1. Generate Interaction Events
                let mut interaction_events = alloc::vec::Vec::new();
                if buttons_changed || moved {
                    if buttons_changed {
                        let left_down = (buttons & 1) != 0;
                        let prev_left_down = (prev_buttons & 1) != 0;
                        if left_down && !prev_left_down {
                            interaction_events.push(InteractionEvent::PointerDown { x: px, y: py, buttons });
                        } else if !left_down && prev_left_down {
                            interaction_events.push(InteractionEvent::PointerUp { x: px, y: py, buttons });
                        }
                    }
                    if moved {
                         interaction_events.push(InteractionEvent::PointerMove { x: px, y: py, buttons });
                    }
                }

                // 2. Process Interaction using Controller
                for event in interaction_events {
                    use crate::interaction::{InteractionEffect, Interaction};
                    
                    let effects = match event {
                        InteractionEvent::PointerDown { x, y, .. } => 
                            interaction_controller.handle_pointer_down(x, y, &window_scenes),
                        InteractionEvent::PointerMove { x, y, .. } => 
                            interaction_controller.handle_pointer_move(x, y, &window_scenes),
                        InteractionEvent::PointerUp { x, y, .. } => 
                            interaction_controller.handle_pointer_up(x, y),
                    };

                    for effect in effects {
                        match effect {
                            InteractionEffect::SetFocus(id) => {
                                // 1. Unset old focus
                                if let Some(old_id) = focused_window {
                                    if old_id != id.unwrap_or(ThingId::from_parts(0,0)) { 
                                         if let Ok(mut win) = Window::read(&SyscallGraphClient, old_id) {
                                             if win.focused {
                                                 win.focused = false;
                                                 let _ = win.write(&mut graph_client, old_id);
                                             }
                                         }
                                    }
                                }
                                
                                // 2. Set new focus
                                if let Some(new_id) = id {
                                    if let Ok(mut win) = Window::read(&SyscallGraphClient, new_id) {
                                        if !win.focused {
                                            win.focused = true;
                                            let _ = win.write(&mut graph_client, new_id);
                                        }
                                    }
                                }
                                focused_window = id;
                            }
                            InteractionEffect::Raise(id) => {
                                // Find max Z
                                let mut max_z = 0;
                                for s in &window_scenes {
                                    if s.window.z > max_z {
                                        max_z = s.window.z;
                                    }
                                }
                                // Update Z
                                if let Ok(mut win) = Window::read(&SyscallGraphClient, id) {
                                    // Always raise to top + 1 if not already
                                    if win.z <= max_z {
                                        win.z = max_z + 1;
                                        let _ = win.write(&mut graph_client, id);
                                    }
                                }
                            }
                            InteractionEffect::Close(id) => {
                                // Unlink from graph.windows
                                let pred_contains = symbol_intern("predicate.contains");
                                let mut buf = alloc::vec![RelationshipRef {
                                    id: abi::ids::RelationshipId::from_parts(0,0),
                                    kind: abi::ids::SymbolId(0),
                                    target: abi::ids::ThingId::from_parts(0,0),
                                }; 64];
                                
                                let mut cursor = 0;
                                loop {
                                     let (count, total) = relationships_from(windows_graph, cursor, &mut buf).unwrap_or((0,0));
                                     if count == 0 { break; }
                                     
                                     for i in 0..count as usize {
                                         let r = &buf[i];
                                         if r.kind == pred_contains && r.target == id {
                                             let _ = unsafe { 
                                                 thing_std::syscall(
                                                     abi::syscall::nr::SYS_REL_DELETE, 
                                                     r.id.low(), 0, 0, 0, 0, 0
                                                 ) 
                                             };
                                         }
                                     }
                                     cursor += count;
                                     if cursor >= total { break; }
                                }
                            }
                            InteractionEffect::UpdateWindowRect { id, rect } => {
                                 if let Ok(mut win) = Window::read(&SyscallGraphClient, id) {
                                     if win.x != rect.x || win.y != rect.y || win.width != rect.w || win.height != rect.h {
                                         win.x = rect.x;
                                         win.y = rect.y;
                                         win.width = rect.w;
                                         win.height = rect.h;
                                         let _ = win.write(&mut graph_client, id);
                                     }
                                 }
                            }
                            InteractionEffect::EmitAction { window, kind, start_x, start_y, dx, dy, edges } => {
                                let _ = WindowAction {
                                    window,
                                    kind,
                                    start_x, start_y, dx, dy, edges
                                }.create(&mut graph_client);
                            }
                        }
                    }
                }

                if old_focused_window != focused_window {
                    if let Some(id) = old_focused_window {
                        if let Some(view) = scene_cache.windows.get(&id) {
                            merge_damage(&mut target_damage, view.rect);
                        }
                    }
                    if let Some(id) = focused_window {
                        if let Some(view) = scene_cache.windows.get(&id) {
                            merge_damage(&mut target_damage, view.rect);
                        }
                    }
                    scene_dirty = true;
                }

                if let crate::interaction::Interaction::Idle = interaction_controller.interaction {
                     // Cursor hover logic
                     let target = crate::ui::hittest::hit_test_scene(&window_scenes, px, py);
                     let kind = match target {
                         crate::ui::hittest::HitTarget::WindowResize { edge, .. } => match edge {
                             abi::ui::ResizeEdge::Top | abi::ui::ResizeEdge::Bottom => CursorKind::ResizeV,
                             abi::ui::ResizeEdge::Left | abi::ui::ResizeEdge::Right => CursorKind::ResizeH,
                             abi::ui::ResizeEdge::TopLeft | abi::ui::ResizeEdge::BottomRight => CursorKind::ResizeNWSE,
                             abi::ui::ResizeEdge::TopRight | abi::ui::ResizeEdge::BottomLeft => CursorKind::ResizeNESW,
                             _ => CursorKind::Default,
                         },
                         _ => CursorKind::Default,
                     };
                     cursor_set.set_cursor(kind);
                     cursor_set.set_override(None);
                } else {
                     // Dragging cursor
                    let kind = match interaction_controller.interaction {
                        crate::interaction::Interaction::DragMove { .. } => CursorKind::Move,
                        crate::interaction::Interaction::DragResize { edge, .. } => match edge {
                                 abi::ui::ResizeEdge::Top | abi::ui::ResizeEdge::Bottom => CursorKind::ResizeV,
                                 abi::ui::ResizeEdge::Left | abi::ui::ResizeEdge::Right => CursorKind::ResizeH,
                                 abi::ui::ResizeEdge::TopLeft | abi::ui::ResizeEdge::BottomRight => CursorKind::ResizeNWSE,
                                 abi::ui::ResizeEdge::TopRight | abi::ui::ResizeEdge::BottomLeft => CursorKind::ResizeNESW,
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
                let cursor_changed = moved || anim_changed || buttons_changed;

                let mut dirty: Option<Rect> = None;

                // --- SCENE EXECUTION ---
                if scene_dirty {
                    let _scene_trace_ns = trace_enter!("scene_record");
                    // Pre-execution Update
                    window_scenes = scene_cache.scenes_in_order();
                    let win_count = window_scenes.len();
                    
                    if win_count > 0 && !logged_window_once {
                        log_info(&alloc::format!("BLOOM: START render {} windows", win_count));
                    }
                    
                    scene_cache.mapping_cache.reset_frame_stats();

                    // === RECORD PHASE ===
                    // Build command list from scene state (no pixel writes allowed here)
                    let mut recorder = crate::command_recorder::CommandRecorder::new(width, height);

                    let clip_rect = target_damage.take().unwrap_or(Rect { x: 0, y: 0, w: width, h: height });
                    recorder.cmds.push(crate::draw_cmd::DrawCmd::SetClip { rect: clip_rect });
                    
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

                    let mut frame_bounds_union: Option<Rect> = None;
                    for cmd in &cmds {
                         if let Some(b) = cmd.bounds() {
                             frame_bounds_union = Some(if let Some(u) = frame_bounds_union { Rect::union(u, b) } else { b });
                         }
                    }
                    
                    // === EXECUTE PHASE (chunked) ===
                    // Start execution of recorded commands
                    current_exec = Some(ChunkedExecutor::new(cmds, width, height));
                    
                    // Clear scene_dirty now that we have captured the state into current_exec
                    // Note: If anything dirties it again while executing, we will restart at next frame
                    scene_cache.clear_dirty();
                    trace_exit!("scene_record", _scene_trace_ns);
                    scene_dirty = false;
                }

                if let Some(exec) = &mut current_exec {
                     let _exec_trace = trace_enter!("exec_chunk");
                     // Execute a chunk
                     let done = exec.step(
                         scene_buffer.as_mut_slice(), 
                         &mut scene_cache.mapping_cache, 
                         &bitmap_store, 
                         256
                     );
                     
                     // Accumulate damage from this chunk
                     if let Some(damage_rect) = exec.damage().rect {
                         merge_damage(&mut dirty, damage_rect);
                     }
                     trace_exit!("exec_chunk", _exec_trace);
                     
                     if done {
                        // Finished!
                        let output = current_exec.take().unwrap().finish(); 
                        if output.stats.bad_cmds > 0 {
                            log_info(&alloc::format!("BLOOM: Bad cmds: {}/{}", output.stats.bad_cmds, output.stats.cmds_total));
                        }
                        
                        scene_cache.mapping_cache.log_frame_stats();
                        
                         if window_scenes.len() > 0 && !logged_window_once {
                            log_info("BLOOM: rebuild_scene done");
                            log_info("BLOOM: redraw_region done");
                            logged_window_once = true;
                        }
                     }
                }

                // === COMPOSITING PHASE ===
                // 1. Copy executed scene to framebuffer (if scene changed)
                if let Some(rect) = dirty {
                     cursor_overlay.copy_region(scene_buffer.as_slice(), frame_buffer.as_mut_slice(), rect);
                }
                
                // 2. Overlay cursor (IMMEDIATE MODE - the only exception)
                // This must be immediate for responsiveness during long scene rebuilds
                if cursor_changed || dirty.is_some() {
                    // DIAG: Log buffer alignment before present
                    let scene_ptr = scene_buffer.as_ptr() as usize;
                    let frame_ptr = frame_buffer.as_ptr() as usize;
                    if scene_ptr % 4 != 0 || frame_ptr % 4 != 0 {
                        log_info(&alloc::format!("ALIGN BUG: scene={:#x}(%{}) frame={:#x}(%{})", 
                            scene_ptr, scene_ptr % 4, frame_ptr, frame_ptr % 4));
                    }
                    
                    if let Some(cursor_dirty) = cursor_overlay.present(
                        scene_buffer.as_slice(),
                        frame_buffer.as_mut_slice(),
                        px,
                        py,
                        cursor_frame,
                    ) {
                        merge_damage(&mut dirty, cursor_dirty);
                        if !logged_shared_shadow {
                            log_info("BLOOM: cursor overlay active with shadows");
                            logged_shared_shadow = true;
                        }
                    }
                }

                if let Some(rect) = dirty {
                    // DIAG: reached backend present entry
                    log_info(&alloc::format!("BLOOM DIAG: entering backend.present rect=({},{} {}x{})", 
                        rect.x, rect.y, rect.w, rect.h));
                    
                    if crate::boot_fade::WALLPAPER_READY.load(Ordering::Acquire) {
                        backend.present(
                            frame_buffer.as_mut_slice(),
                            DirtyRect {
                                x: rect.x,
                                y: rect.y,
                                w: rect.w,
                                h: rect.h,
                            },
                        );
                    }
                    
                    log_info("BLOOM DIAG: backend.present completed");
                    
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
