#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};
use abi::ids::ThingId;
use abi::wire::graph::{GraphOp, GraphReply};
use models::builtins::core_kinds::DisplayFramebufferBody;

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { thing_std::rt::init_heap(heap_start as usize, 4 * 1024 * 1024); }
    let g = GraphClient::new();
    let c = StdoutConsole;
    c.write_str("FB_SMOKE: Starting...\n");
    
    let mut buf = [0u8; 4096];
    let mut fb_thing_id: Option<ThingId> = None;
    
    // 1. Locate DisplayFramebuffer
    loop {
        if fb_thing_id.is_none() {
            // Find Thing by Kind
             c.write_str("FB_SMOKE: Scanning for Framebuffer...\n");
             // We can use a debug scan for ONE thing of usage.
             // Root -> HasDevice -> Framebuffer
             // Or just search all nodes? No, use links.
             let op = GraphOp::ScanLinks {
                 from: Some(models::builtins::ids::THING_BOOT_ROOT),
                 to: None,
                 kind: Some(models::builtins::ids::THING_HAS_DEVICE_KIND)
             };
             if let Ok(GraphReply::Links(list)) = g.call_op(&op, &mut buf) {
                 for (_, target, _) in list {
                     // Check Kind
                     let get_op = GraphOp::GetThing { id: abi::ids::ThingId(target.0) };
                     if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&get_op, &mut buf) {
                         // Check type_id
                         if tb.type_id.0 == models::builtins::ids::THING_DISPLAY_FRAMEBUFFER_KIND.0 as u128 {
                             fb_thing_id = Some(target);
                             c.write_str("FB_SMOKE: Found DisplayFramebuffer!\n");
                             break;
                         }
                     }
                 }
             }
        }
        
        if let Some(id) = fb_thing_id {
            // Get Props
            let op = GraphOp::GetThing { id: abi::ids::ThingId(id.0) };
            if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&op, &mut buf) {
                if let Ok(fb) = postcard::from_bytes::<DisplayFramebufferBody>(&tb.bytes) {
                     c.write_str("FB_SMOKE: Drawing to ");
                     // print address
                     // fb.address
                     
                     // Draw Gradient
                     draw_pattern(&fb);
                     
                     thing_std::time::sleep_ms(&g, 1000);
                } else {
                    c.write_str("FB_SMOKE: Postcard decode failed\n");
                }
            }
        }
        
        thing_std::time::sleep_ms(&g, 500);
    }
}

fn draw_pattern(fb: &DisplayFramebufferBody) {
    let ptr = fb.address as *mut u32;
    let width = fb.width as usize;
    let height = fb.height as usize;
    let pitch = fb.pitch as usize;
    
    // Safety: Userland has mapped this address. Pointer access is unsafe.
    if ptr.is_null() { return; }
    
    // Simple verification pattern
    // Color varies by X and Y
    unsafe {
        for y in 0..height {
            let row_ptr = (ptr as *mut u8).add(y * pitch) as *mut u32;
            for x in 0..width {
                let r = (x % 255) as u32;
                let g = (y % 255) as u32;
                let b = ((x+y) % 255) as u32;
                
                let color = 0xFF000000 | (r << 16) | (g << 8) | b;
                *row_ptr.add(x) = color;
            }
        }
    }
}
