#![no_std]
#![no_main]

extern crate alloc;
use thing_std as std;
use thing_std::{StdoutConsole, Console, GraphClient};
use core::fmt::Write;
use thing_std::debug::PortWrites;
use abi::wire::graph::{GraphOp, GraphReply};
use abi::ids::ThingId;
use thing_models::builtins::core_kinds::DisplayFramebufferBody;
use thing_models::builtins::ids::{
    THING_BOOT_ROOT, THING_HAS_DEVICE_KIND, THING_DISPLAY_FRAMEBUFFER_KIND,
    THING_MODULE_KIND, THING_POINTER_EVENT_STREAM_KIND, THING_WINDOW_KIND,
    THING_EMITS_KIND, THING_MOUSE_KIND,
    THING_MOUSE_SCHEMA, THING_POINTER_EVENT_STREAM_SCHEMA, THING_MODULE_SCHEMA,
    THING_BOOT_STATE_KIND, THING_BOOT_STATE_SCHEMA
};
use thing_models::schema::bitmap::BitmapBody;
use thing_models::core::input::PointerEventStreamBody;
use thing_models::builtins::core_kinds::ModuleBody;
use thing_models::schema::boot_state::BootStateBody; // Added
use crate::boot_scene::BootScene;

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 32 * 1024 * 1024); }
    std::init();

    let c = StdoutConsole;
    let _ = c.write_str("COMPOSITOR: Starting...\n");

    let g = GraphClient::new();
    let mut buf = [0u8; 8192];
    let mut fb_thing_id: Option<ThingId> = None;

    let mut cursor_x: i32 = 512;
    let mut cursor_y: i32 = 384;
    let mut cursor_bitmap: Option<BitmapBody> = None;
    let mut cursor_frames: Option<alloc::vec::Vec<cursor_parser::Frame>> = None;
    let mut cursor_frame_idx = 0;
    let mut cursor_frame_timer = 0;
    let mut pointer_stream_id: Option<ThingId> = None;
    let mut last_seq: u64 = 0;
    
    // Boot State Tracking
    let mut boot_state_id: Option<ThingId> = None;

    'scan: loop {
        c.write_str("COMPOSITOR: Scanning...\n");
        let op = GraphOp::ScanLinks {
            from: Some(THING_BOOT_ROOT),
            to: None,
            kind: None // Scan all links from root to find devices AND boot state
        };
        
        if let Ok(GraphReply::Links(list)) = g.call_op(&op, &mut buf) {
             for (_, target, kind) in list {
                 let get_op = GraphOp::GetThing { id: target };
                 if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&get_op, &mut buf) {
                     let type_id = tb.type_id.0 as u128;
                     
                     if type_id == THING_DISPLAY_FRAMEBUFFER_KIND.0 as u128 {
                         if fb_thing_id.is_none() {
                             fb_thing_id = Some(target);
                             c.write_str("COMPOSITOR: Found DisplayFramebuffer!\n");
                         }
                     } else if type_id == THING_BOOT_STATE_SCHEMA.0 as u128 {
                         if boot_state_id.is_none() {
                             boot_state_id = Some(target);
                             c.write_str("COMPOSITOR: Found BootState!\n");
                         }
                     } else if kind == THING_BOOT_STATE_KIND { // Check Link Kind too just in case
                         if boot_state_id.is_none() {
                             boot_state_id = Some(target);
                             c.write_str("COMPOSITOR: Found BootState via Link!\n");
                         }
                     } else if type_id == THING_MODULE_SCHEMA.0 as u128 {
                         if cursor_bitmap.is_none() && cursor_frames.is_none() {
                             if let Ok(module) = postcard::from_bytes::<ModuleBody>(&tb.bytes) {
                                 // Prefer Working.ani or Normal.cur
                                 if module.path.ends_with("Normal.cur") || module.path.ends_with("Working.ani") || module.path.ends_with("cursor.bmp") {
                                      if let Some(parsed) = cursor_parser::parse(&module.data) {
                                          match parsed {
                                              cursor_parser::CursorType::Static(bmp) => {
                                                  cursor_bitmap = Some(bmp);
                                                  c.write_str("COMPOSITOR: Parsed Static Cursor!\n");
                                              }
                                              cursor_parser::CursorType::Animated(frames) => {
                                                  cursor_frames = Some(frames);
                                                  c.write_str("COMPOSITOR: Parsed Animated Cursor!\n");
                                              }
                                          }
                                      } else if module.path.ends_with("cursor.bmp") {
                                         // Fallback to BMP parser
                                         if let Some(bmp) = bitmap_parser::parse_bmp(&module.data) {
                                             cursor_bitmap = Some(bmp);
                                             c.write_str("COMPOSITOR: Parsed Cursor Bitmap!\n");
                                         }
                                      }
                                 }
                             }
                         }
                     } else if type_id == THING_MOUSE_SCHEMA.0 as u128 {
                         // Mouse logic ...
                         let sub_op = GraphOp::ScanLinks {
                             from: Some(target),
                             to: None,
                             kind: Some(THING_EMITS_KIND)
                         };
                         let mut sub_buf = [0u8; 1024];
                         if let Ok(GraphReply::Links(sub_list)) = g.call_op(&sub_op, &mut sub_buf) {
                             for (_, stream_target, _) in sub_list {
                                 let get_stream = GraphOp::GetThing { id: stream_target };
                                 if let Ok(GraphReply::TypedValue(stb)) = g.call_op(&get_stream, &mut sub_buf) {
                                     if stb.type_id.0 as u128 == THING_POINTER_EVENT_STREAM_SCHEMA.0 as u128 {
                                         if pointer_stream_id.is_none() {
                                             pointer_stream_id = Some(stream_target);
                                             c.write_str("COMPOSITOR: Found Pointer Stream!\n");
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
            }
        }
        
        if fb_thing_id.is_some() && pointer_stream_id.is_some() {
            c.write_str("COMPOSITOR: FB and Stream found! (BootState optional but we proceed)\n");
            break 'scan;
        }

        c.write_str("COMPOSITOR: Sleeping...\n");
        for _ in 0..100000 { unsafe { core::arch::asm!("nop"); } }
    }

    // 2. Main Loop
    let mut last_boot_step: u32 = 0;
    
    // Boot Scene
    let mut boot_scene = BootScene::new();
    
    let mut frame_count: u64 = 0;
    c.write_str("COMPOSITOR: Entering Main Loop.\n");

    loop {
        frame_count += 1;
        if frame_count % 60 == 0 {
             let _ = PortWrites.write_fmt(format_args!("COMPOSITOR: Main Loop Cycle {}\n", frame_count));
        }

        // Lazy Load Cursor
        if cursor_bitmap.is_none() && cursor_frames.is_none() {
             let scan_op = GraphOp::ScanLinks {
                 from: Some(THING_BOOT_ROOT),
                 to: None,
                 kind: Some(THING_MODULE_SCHEMA)
             };
             if let Ok(GraphReply::Links(list)) = g.call_op(&scan_op, &mut buf) {
                 for (_, target, _) in list {
                     let get_op = GraphOp::GetThing { id: target };
                     if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&get_op, &mut buf) {
                         if let Ok(module) = postcard::from_bytes::<ModuleBody>(&tb.bytes) {
                              if module.path.ends_with("Normal.cur") || module.path.ends_with("Working.ani") {
                                  if let Some(parsed) = cursor_parser::parse(&module.data) {
                                      match parsed {
                                          cursor_parser::CursorType::Static(bmp) => {
                                              cursor_bitmap = Some(bmp);
                                          }
                                          cursor_parser::CursorType::Animated(frames) => {
                                              cursor_frames = Some(frames);
                                          }
                                      }
                                  }
                              }
                         }
                     }
                 }
             }
        }

        // Animation update
        if let Some(ref frames) = cursor_frames {
            // Assume ~16ms per loop iteration (sleep + overhead)
            // Or use proper time. For now, simple counter.
            cursor_frame_timer += 16;
            let current_frame = &frames[cursor_frame_idx];
            if cursor_frame_timer >= current_frame.duration_ms {
                cursor_frame_timer = 0;
                cursor_frame_idx = (cursor_frame_idx + 1) % frames.len();
            }
        }


        // Lazy Load Cursor Bitmap if missing
        // ... (existing lazy load code omitted for brevity but preserved in replacement) ...
        
        // Pointer Stream Logic ...
        if let Some(pid) = pointer_stream_id {
            // ... (existing pointer logic) ...
            let op = GraphOp::GetThing { id: pid };
             match g.call_op(&op, &mut buf) {
                Ok(GraphReply::TypedValue(tb)) => {
                    if let Ok(stream) = postcard::from_bytes::<PointerEventStreamBody>(&tb.bytes) {
                        let count = stream.events.len() as u64;
                        let start_seq = stream.head_seq.saturating_sub(count).saturating_add(1);

                        for (i, event) in stream.events.iter().enumerate() {
                            let seq = start_seq + i as u64;
                            if seq > last_seq {
                                cursor_x += event.dx as i32;
                                cursor_y += event.dy as i32;
                                last_seq = seq;
                            }
                        }
                    }
                }
                _ => {}
             }
        }

        // Lazy Find BootState if missing
        if boot_state_id.is_none() {
             let scan_op = GraphOp::ScanLinks {
                from: Some(THING_BOOT_ROOT),
                to: None,
                kind: Some(THING_BOOT_STATE_KIND)
            };
            if let Ok(GraphReply::Links(list)) = g.call_op(&scan_op, &mut buf) {
                for (_, target, _) in list {
                    boot_state_id = Some(target);
                    c.write_str("COMPOSITOR: Late-found BootState!\n");
                }
            }
        }

        if let Some(fb_id) = fb_thing_id {
            let op = GraphOp::GetThing { id: fb_id };
            match g.call_op(&op, &mut buf) {
                Ok(GraphReply::TypedValue(tb)) => {
                    if let Ok(fb) = postcard::from_bytes::<DisplayFramebufferBody>(&tb.bytes) {
                        let ptr = fb.address as *mut u32;
                        let pitch = fb.pitch as u32;
                        let fb_width = fb.width as u32;
                        let fb_height = fb.height as u32;

                        if cursor_x < 0 { cursor_x = 0; }
                        if cursor_y < 0 { cursor_y = 0; }
                        if cursor_x >= fb_width as i32 { cursor_x = fb_width as i32 - 1; }
                        if cursor_y >= fb_height as i32 { cursor_y = fb_height as i32 - 1; }

                        // 1. Boot State Processing
                        if let Some(bsid) = boot_state_id {
                             let op = GraphOp::GetThing { id: bsid };
                             if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                                 if let Ok(state) = postcard::from_bytes::<BootStateBody>(&tb.bytes) {
                                     if state.step > last_boot_step {
                                         boot_scene.add_milestone(&state.message);
                                         last_boot_step = state.step;
                                     }
                                 }
                             }
                        }

                        // 2. Window Logic (Detect Desktop)
                        // ...
                        let mut has_windows = false;
                        let mut window_ids = [ThingId(0); 16];
                        let mut count = 0;

                        let scan_op = GraphOp::ScanLinks {
                            from: Some(THING_BOOT_ROOT),
                            to: None,
                            kind: Some(THING_WINDOW_KIND)
                        };

                        if let Ok(GraphReply::Links(list)) = g.call_op(&scan_op, &mut buf) {
                             for (_, target, _) in list {
                                 if count < window_ids.len() {
                                     window_ids[count] = target;
                                     count += 1;
                                 }
                             }
                             if count > 0 { has_windows = true; }
                        }

                        // 3. Update Boot Scene
                        if has_windows {
                             boot_scene.desktop_ready = true;
                        }
                        boot_scene.update();
                        
                        // 4. Rendering Strategy
                        let should_draw_desktop = !boot_scene.is_active || boot_scene.global_alpha < 1.0;
                         
                        if should_draw_desktop {
                            render::primitives::fill_rect(
                                ptr, pitch, fb_width, fb_height,
                                0, 0, fb.width as i32, fb.height as i32,
                                0xFF2E80D1,
                                None
                            );
                            
                             for i in 0..count {
                                 let wid = window_ids[i];
                                 let get_w = GraphOp::GetThing { id: wid };
                                 if let Ok(GraphReply::TypedValue(w_tb)) = g.call_op(&get_w, &mut buf) {
                                     if let Ok(window) = postcard::from_bytes::<thing_models::schema::window::WindowBody>(&w_tb.bytes) {
                                         draw_window(ptr, pitch, fb_width, fb_height, &window);
                                     }
                                 }
                             }
                        }
                        
                        // Boot Scene Overlay
                        if boot_scene.is_active {
                             boot_scene.render(ptr, pitch, fb_width, fb_height);
                        }

                        if let Some(ref frames) = cursor_frames {
                            let bmp = &frames[cursor_frame_idx].bitmap;
                            render::draw_bitmap(ptr, pitch, fb_width, fb_height, cursor_x, cursor_y, bmp);
                        } else if let Some(ref bmp) = cursor_bitmap {
                            render::draw_bitmap(ptr, pitch, fb_width, fb_height, cursor_x, cursor_y, bmp);
                        } else {
                            render::primitives::fill_rect(ptr, pitch, fb_width, fb_height, cursor_x, cursor_y, 10, 10, 0xFFFFFFFF, None);
                        }
                    }
                }
                _ => {}
            }
        }
        
        for _ in 0..16000 { unsafe { core::arch::asm!("nop"); } }
    }
}

mod bitmap_parser;
mod cursor_parser;

fn draw_window(fb_ptr: *mut u32, pitch: u32, fb_w: u32, fb_h: u32, window: &thing_models::schema::window::WindowBody) {
    let x = window.x;
    let y = window.y;
    let w = window.width as i32;
    let h = window.height as i32;
    
    render::primitives::fill_rect(fb_ptr, pitch, fb_w, fb_h, x - 2, y - 22, w + 4, h + 24, 0xFFCCCCCC, None);
    render::primitives::fill_rect(fb_ptr, pitch, fb_w, fb_h, x, y - 20, w, 20, 0xFF000088, None);
    render::text::draw_text(fb_ptr, pitch, fb_w, fb_h, x + 4, y - 18, window.title.as_str(), 0xFFFFFFFF);
    render::primitives::fill_rect(fb_ptr, pitch, fb_w, fb_h, x, y, w, h, 0xFF000000, None);

    if window.content.contains('\n') {
        let mut row = 0;
        for line in window.content.split('\n') {
            render::text::draw_text(fb_ptr, pitch, fb_w, fb_h, x + 4, y + 4 + (row * 10), line, 0xFFFFFFFF);
            row += 1;
        }
    } else {
        let content_len = window.content.len() as u32;
        let text_size = layout::Size { width: content_len * 8, height: 8 };
        let bounds = layout::Rect::new(x, y, window.width, window.height);
        let pos = layout::center_text(bounds, text_size);
        render::text::draw_text(fb_ptr, pitch, fb_w, fb_h, pos.x, pos.y, window.content.as_str(), 0xFFFFFFFF);
    }
}

mod render;
mod layout;
mod boot_scene;
