use thing_os::prelude::*;
use thing_os::register_schema_for;

use crate::config::FRAME_INTERVAL_NS;
use crate::graph::{find_framebuffer_setup, collect_all_windows};
use crate::model::Compositor;
use crate::schemas::{Surface, RenderTree, FrameRenderIntent, FramebufferInfo, Buffer, PresentIntent, IntentState};
use abi::{MapFlags, KernelRequest, KernelResponse, ThingId}; 
use alloc::boxed::Box;
use alloc::vec::Vec;
use thing_os::syscalls::syscall;

fn map_buffer(buffer_id: ThingId) -> Option<(*mut u8, usize)> {
     match syscall(KernelRequest::MapSharedBuffer {
        buffer_id,
        flags: MapFlags::READ.union(MapFlags::WRITE).union(MapFlags::USER),
    }) {
        KernelResponse::SharedBufferMapped { vaddr, size } => Some((vaddr as *mut u8, size as usize)),
        _ => None,
    }
}

pub fn main() -> ! {
    println!(crate::l10n!("compositor: initium (modus securitatis graphidis)"));

    // Register schemas
    let _ = register_schema_for::<Surface>();
    let _ = register_schema_for::<RenderTree>();
    let _ = register_schema_for::<FrameRenderIntent>();
    let _ = register_schema_for::<FramebufferInfo>();
    let _ = register_schema_for::<Buffer>();
    let _ = register_schema_for::<PresentIntent>();

    let (fb_info, buffer, display) = loop {
        match find_framebuffer_setup() {
            Some(x) => break x,
            None => {
                thing_os::time::sleep(Duration::from_millis(100));
            }
        }
    };

    println!(crate::l10n!("compositor: inventa indicia tabulae imaginum {}x{}"), fb_info.width, fb_info.height);

    let (ptr, _size) = map_buffer(buffer.shared_buffer_id).expect(crate::l10n!("Non potui tabulam imaginum describere"));
    println!(crate::l10n!("compositor: tabula imaginum depicta"));

    let mut compositor = Compositor::new(fb_info, ptr, buffer.index, display.id);

    loop {
        draw_test_pattern(&mut compositor);
        
        // Submit PresentIntent
        let intent = PresentIntent {
            id: ThingId(0), // ignored on create
            buffer_index: compositor.buffer_index,
            state: IntentState::Pending,
            error_code: None,
            error_message: None,
        };
        
        println!(crate::l10n!("compositor: mittens consilium praesentandi"));
        if let Some(intent_id) = create_thing(&intent) {
            // Wait for it to be Done
            // In a real loop we wouldn't block, but for "First Light" logical verification:
            let mut retries = 0;
            loop {
                if let Some(updated) = load_thing::<PresentIntent>(intent_id) {
                    if updated.state == IntentState::Done {
                       println!(crate::l10n!("compositor: consilium perfectum"));
                       break;
                    } else if updated.state == IntentState::Error {
                       println!(crate::l10n!("compositor: error in consilio"));
                       break;
                    }
                }
                thing_os::time::sleep(Duration::from_millis(1));
                retries += 1;
                if retries > 1000 {
                     // Timeout
                     println!(crate::l10n!("compositor: mora consilii"));
                     break;
                }
            }
        } else {
            println!(crate::l10n!("compositor: non potui consilium creare"));
        }

        thing_os::time::sleep(Duration::from_nanos(FRAME_INTERVAL_NS));
    }
}

fn draw_test_pattern(compositor: &mut Compositor) {
    let width = compositor.fb_info.width as usize;
    let height = compositor.fb_info.height as usize;
    let stride_bytes = compositor.fb_info.stride as usize;
    let frame = compositor.frame_counter;
    
    compositor.frame_counter = compositor.frame_counter.wrapping_add(1);

    // Dynamic color
    let r = (frame % 255) as u8;
    let g = ((frame / 2) % 255) as u8;
    let b = ((frame / 4) % 255) as u8;
    let color: u32 = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    
    // Fill
    // Assuming 32bpp for now
    let ptr = compositor.fb_ptr;
    
    unsafe {
        for y in 0..height {
            let row = ptr.add(y * stride_bytes) as *mut u32;
            for x in 0..width {            
                *row.add(x) = color;
            }
        }
    }
}
