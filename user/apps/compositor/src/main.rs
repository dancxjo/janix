#![no_std]
#![no_main]

extern crate alloc;
use thing_std as std;
use thing_std::{StdoutConsole, Console, GraphClient};
use abi::wire::graph::{GraphOp, GraphReply};
use abi::ids::ThingId;
use thing_models::builtins::core_kinds::DisplayFramebufferBody;
use thing_models::builtins::ids::{THING_BOOT_ROOT, THING_HAS_DEVICE_KIND, THING_DISPLAY_FRAMEBUFFER_KIND};

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 1024 * 1024); }
    std::init();

    let c = StdoutConsole;
    let _ = c.write_str("COMPOSITOR: Starting...\n");

    let g = GraphClient::new();
    let mut buf = [0u8; 4096];
    let mut fb_thing_id: Option<ThingId> = None;

    // 1. Locate Framebuffer
    loop {
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
                             break;
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

        let _ = std::time::sleep_ms(&g, 1000);
    }
}

mod render;

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
