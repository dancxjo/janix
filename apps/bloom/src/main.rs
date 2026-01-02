#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    log_info("BLOOM: PANIC!");
    loop {}
}

const _WIDTH: usize = 1024; // Default/Fallback
const _HEIGHT: usize = 768; // Default/Fallback
const _BPP: usize = 4;

#[allow(dead_code)]
struct Surface {
    width: u32,
    height: u32,
    stride: u32,
    ptr: *mut u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(syscall_ptr: u64) -> ! {
    thing_std::init(syscall_ptr);
    log_info("BLOOM: Starting Compositor...");

    // 1. Find Display Surface
    let mut attempt = 0;
    let _surface_id = loop {
        if let Some(id) = thing_find("surface.display0") {
            break id;
        }
        attempt += 1;
        if attempt > 10 {
            log_info("BLOOM: Waiting for surface.display0...");
            attempt = 0;
        }
        // sleep/yield
        for _ in 0..100000 { core::hint::spin_loop(); }
    };
    log_info("BLOOM: Found surface.display0!");

    // 2. Get Attributes (Size, Stride, Buffer)
    // We need to query relationships from surface.display0
    // Helpers:
    //   --[backs]--> bytespace.framebuffer0
    //   --[size]--> rect
    //   --[stride]--> value
    
    let _rels = [abi::ids::ThingId(0); 32];

    // Shortcut for VS: Find "bytespace.framebuffer0" directly.
    let fb_bs_id = thing_find("bytespace.framebuffer0").expect("BLOOM: No FB Bytespace");
    log_info("BLOOM: Found bytespace.framebuffer0");
    
    // Map it.
    let fb_vaddr = 0x8000_0000;
    let fb_size = 8 * 1024 * 1024; 
    let res = space_map(fb_bs_id, fb_vaddr, 0, fb_size);
    if res.is_err() {
        log_info("BLOOM: Failed to map FB!");
        loop {}
    }
    let fb_ptr = fb_vaddr as *mut u32;

    // 3. Main Loop
    let mut frame = 0;
    loop {
        // Clear / Background
        // Blue-ish background
        let color: u32 = if frame % 60 < 30 { 0xFF0000AA } else { 0xFF000088 };
        
        unsafe {
            for i in 0..(1024*768) {
                *fb_ptr.add(i) = color;
            }
        }
        
        // Find Windows
        if let Some(_win) = thing_find("window.clock") {
             // Find target surface?
             // Hardcode: "surface.clock"
             if let Some(surf) = thing_find("surface.clock") {
                 // Map it? We need to map it once.
                 // We don't have persistent map management in this loop yet.
                 // Map it at 0x9000_0000?
                 // Note: re-mapping every frame is bad, but fine for smoke test if idempotent/fast enough?
                 // Default map allows overlaps? No.
                 // We'll map it once if not mapped.
                 // Logic: check 'mapped_surfaces' map.
                 // Simplification: Try to map. If EEXIST, ignore.
                 let surf_vaddr = 0x9000_0000;
                 let _ = space_map(surf, surf_vaddr, 0, 320*200*4);
                 
                 // Blit (Fixed pos 100,100)
                 let src = surf_vaddr as *const u32;
                 let dst = fb_ptr;
                 let wx = 100;
                 let wy = 100;
                 let ww = 320;
                 let wh = 200;
                 
                 unsafe {
                     for y in 0..wh {
                         for x in 0..ww {
                             let pixel = *src.add(y*ww + x);
                             let dst_idx = (wy + y) * 1024 + (wx + x);
                             if dst_idx < 1024*768 {
                                 *dst.add(dst_idx) = pixel;
                             }
                         }
                     }
                 }
             }
        }

        frame += 1;
        // Yield
        // Helper:
        // for _ in 0..100000 {}
        // sched_yield();
    }
}
