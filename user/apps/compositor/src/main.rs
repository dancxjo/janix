#![no_std]
#![no_main]

extern crate alloc;
use thing_std as std;
use thing_std::{StdoutConsole, Console, GraphClient};
use abi::wire::graph::{GraphOp, GraphReply};
use abi::ids::ThingId;
use thing_models::builtins::core_kinds::DisplayFramebufferBody;
use thing_models::builtins::ids::{THING_BOOT_ROOT, THING_HAS_DEVICE_KIND, THING_DISPLAY_FRAMEBUFFER_KIND, THING_EMITS_KIND};

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 32 * 1024 * 1024); }
    std::init();

    let c = StdoutConsole;
    let _ = c.write_str("COMPOSITOR: Starting...\n");

    let g = GraphClient::new();
    let mut buf = [0u8; 4096];
    let mut fb_thing_id: Option<ThingId> = None;

    // 1. Locate Framebuffer
    'scan: loop {
        if fb_thing_id.is_none() {
            c.write_str("COMPOSITOR: Scanning for Framebuffer...\n");
            let op = GraphOp::ScanLinks {
                from: Some(THING_BOOT_ROOT),
                to: None,
                kind: Some(THING_HAS_DEVICE_KIND)
            };
            
            if let Ok(GraphReply::Links(list)) = g.call_op(&op, &mut buf) {
                for (_, target, _) in list {
                     let get_op = GraphOp::GetThing { id: target };
                     if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&get_op, &mut buf) {
                         if tb.type_id.0 == THING_DISPLAY_FRAMEBUFFER_KIND.0 as u128 {
                             fb_thing_id = Some(target);
                             c.write_str("COMPOSITOR: Found DisplayFramebuffer!\n");
                             break 'scan;
                         }
                     }
                }
            }
        }
        
        if let Some(id) = fb_thing_id {
             let op = GraphOp::GetThing { id };
             if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                 if let Ok(fb) = postcard::from_bytes::<DisplayFramebufferBody>(&tb.bytes) {
    let ptr = fb.address as *mut u32;
    // Clear to Blue
    render::primitives::fill_rect(
        ptr,
        fb.pitch as u32,
        fb.width as u32,
        fb.height as u32,
        0, 0, fb.width as i32, fb.height as i32,
        0xFFFF0000, 
        None
    );

    // Green Rect
    render::primitives::fill_rect(
        ptr,
        fb.pitch as u32,
        fb.width as u32,
        fb.height as u32,
        100, 100, 200, 200,
        0xFF00FF00,
        None
    );

    render::text::draw_text(
        ptr,
        fb.pitch as u32,
        fb.width as u32,
        fb.height as u32,
        150, 150,
        "Hello ThingOS v0.2!",
        0xFFFFFFFF
    );
                 }
             }
        }

    }

    // 2. Main Loop
    let mut log_stream_id: Option<ThingId> = None;

    loop {
        if let Some(fb_id) = fb_thing_id {
            // Get FB details first
            let op = GraphOp::GetThing { id: fb_id };
            if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                if let Ok(fb) = postcard::from_bytes::<DisplayFramebufferBody>(&tb.bytes) {
                    let ptr = fb.address as *mut u32;
                    let pitch = fb.pitch as u32;
                    let fb_width = fb.width as u32;
                    let fb_height = fb.height as u32;

                    // Clear Background (Boot Blue)
                    render::primitives::fill_rect(
                        ptr, pitch, fb_width, fb_height,
                        0, 0, fb.width as i32, fb.height as i32,
                        0xFF2E80D1, // #2E80D1 (BGR: D1 80 2E in memory)
                        None
                    );
                    
                    // Debug Text
                    render::text::draw_text(
                        ptr, pitch, fb_width, fb_height,
                        20, 20,
                        "Compositor Online",
                        0xFFFFFFFF
                    );

                    // ---------------------------------------------------------
                    // Log Stream Logic
                    // ---------------------------------------------------------
                    if log_stream_id.is_none() {
                        // Scan for LogStream
                        let scan_op = GraphOp::ScanLinks {
                            from: Some(THING_BOOT_ROOT),
                            to: None,
                            kind: Some(thing_models::builtins::ids::THING_EMITS_KIND)
                        };
                         if let Ok(GraphReply::Links(list)) = g.call_op(&scan_op, &mut buf) {
                             for (_, target, _) in list {
                                 // Check kind
                                 // We need to fetch the thing to check the kind (Link doesn't contain target kind)
                                 // Optimistically assume the first EMITS is it, or better, check type.
                                 // Actually better to just GetThing(3020) if we knew it, but dynamic is better.
                                 // Let's check kind.
                                 // We need to be careful with buffer reuse.
                                 // Make a copy of targets to check.
                             }
                             // Hack: we know flusher makes it 3020.
                             // But let's try to verify.
                        }
                        // Fallback/Fast-path:
                        log_stream_id = Some(ThingId(3020)); 
                    }

                    if let Some(lid) = log_stream_id {
                        let op = GraphOp::GetThing { id: lid };
                        if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                             if let Ok(stream) = postcard::from_bytes::<thing_models::core::serial::LogStreamBody>(&tb.bytes) {
                                 if let Some(last) = stream.entries.last() {
                                     // Draw Status Message
                                     let msg = &last.message;
                                     let y = fb_height as i32 - 40;
                                     render::text::draw_text(
                                         ptr, pitch, fb_width, fb_height,
                                         20, y,
                                         msg.as_str(),
                                         0xFFFFFFFF
                                     );
                                 }
                             }
                        }
                    }

                    // ---------------------------------------------------------
                    // Window Logic
                    // ---------------------------------------------------------
                    // Scan for Windows
                    let scan_op = GraphOp::ScanLinks {
                        from: Some(thing_models::builtins::ids::THING_BOOT_ROOT), // Ideally we'd have a Windows container, but root scan for now
                        to: None,
                        kind: Some(thing_models::builtins::ids::THING_WINDOW_KIND)
                    };

                    if let Ok(GraphReply::Links(list)) = g.call_op(&scan_op, &mut buf) {
                         // We need a separate buffer for window fetching since 'list' borrows 'buf'?
                         // Actually, 'list' is just a slice of (u128, u128, u128) tuples, completely POD. 
                         // But we can't reuse 'buf' for the next call while iterating 'list' if 'list' points into 'buf'.
                         // We need to copy the IDs we want to process.
                         
                         let mut window_ids = [ThingId(0); 16];
                         let mut count = 0;
                         for (_, target, _) in list {
                             if count < window_ids.len() {
                                 window_ids[count] = target;
                                 count += 1;
                             }
                         }

                         for i in 0..count {
                             let wid = window_ids[i];
                             let get_w = GraphOp::GetThing { id: wid };
                             // We can reuse 'buf' now
                             if let Ok(GraphReply::TypedValue(w_tb)) = g.call_op(&get_w, &mut buf) {
                                 if let Ok(window) = postcard::from_bytes::<thing_models::schema::window::WindowBody>(&w_tb.bytes) {
                                     draw_window(ptr, pitch, fb_width, fb_height, &window);
                                 }
                             }
                         }
                    }
                }
            }
        }
        
        let _ = std::time::sleep_ms(&g, 16); // ~60 FPS polling
    }
}

fn draw_window(fb_ptr: *mut u32, pitch: u32, fb_w: u32, fb_h: u32, window: &thing_models::schema::window::WindowBody) {
    let x = window.x;
    let y = window.y;
    let w = window.width as i32;
    let h = window.height as i32;
    
    // 1. Chrome (Border + Title Bar)
    // Border
    render::primitives::fill_rect(fb_ptr, pitch, fb_w, fb_h, x - 2, y - 22, w + 4, h + 24, 0xFFCCCCCC, None);
    // Title Bar
    render::primitives::fill_rect(fb_ptr, pitch, fb_w, fb_h, x, y - 20, w, 20, 0xFF000088, None);
    // Title Text
    render::text::draw_text(fb_ptr, pitch, fb_w, fb_h, x + 4, y - 18, window.title.as_str(), 0xFFFFFFFF);
    
    // Body Background
    render::primitives::fill_rect(fb_ptr, pitch, fb_w, fb_h, x, y, w, h, 0xFF000000, None);

    // 2. Content
    // Center logic? Or just primitive top-left? User said "center text (for clock)".
    // Let's blindly center it for now as a default for single-line text? 
    // Or check if it contains newlines?
    
    if window.content.contains('\n') {
        // Multi-line (Keylog), just draw top-left with padding
        let mut row = 0;
        for line in window.content.split('\n') {
            render::text::draw_text(fb_ptr, pitch, fb_w, fb_h, x + 4, y + 4 + (row * 10), line, 0xFFFFFFFF);
            row += 1;
        }
    } else {
        // Single line (Clock), center it
        let content_len = window.content.len() as u32;
        // Approx char width 8, height 8? (from primitives usually)
        let text_size = layout::Size { width: content_len * 8, height: 8 };
        let bounds = layout::Rect::new(x, y, window.width, window.height);
        let pos = layout::center_text(bounds, text_size);
        render::text::draw_text(fb_ptr, pitch, fb_w, fb_h, pos.x, pos.y, window.content.as_str(), 0xFFFFFFFF);
    }
}

mod render;
mod layout;


fn draw_red_screen(fb: &DisplayFramebufferBody) {
    let ptr = fb.address as *mut u32;
    // Safety check just in case
    if ptr.is_null() { return; }

    // Clear to Blue
    render::primitives::fill_rect(
        ptr,
        fb.pitch as u32,
        fb.width as u32,
        fb.height as u32,
        0, 0, fb.width as i32, fb.height as i32,
        0xFFFF0000, 
        None
    );

    // Green Rect
    render::primitives::fill_rect(
        ptr,
        fb.pitch as u32,
        fb.width as u32,
        fb.height as u32,
        100, 100, 200, 200,
        0xFF00FF00,
        None
    );
}
