use thing_std::*;
use core::sync::atomic::Ordering;

pub const BLOOM_TIMEFRAME_MS: u64 = 16;

#[derive(Clone, Copy, Debug)]
pub struct PresentConfig {
    pub fb_vaddr: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

pub extern "C" fn present_loop_entry(arg: u64) -> ! {
    log_info("BLOOM PRESENT: thread started");

    let config_ptr = arg as *const PresentConfig;
    let config = unsafe { *config_ptr };
    
    // Safety: The config is on the stack of the main thread, BUT we just copied it. 
    // Wait, the pointer passed is to a struct that might drop if it was on the stack?
    // In `app.rs`, we need to make sure we pass a pointer to something that lives long enough or is copied.
    // Ideally we pass the struct by value if it fits in u64, but it doesn't (4 fields).
    // So we pass a pointer. The spawner must ensure the data is valid until we read it.
    // OR we allocate it on the heap and leak it/box it.
    
    log_info(&alloc::format!("BLOOM PRESENT: {}x{} stride={}", config.width, config.height, config.stride));
    
    let fb_len = (config.height as usize).checked_mul(config.stride as usize).expect("fb size overflow");
    if fb_len > isize::MAX as usize {
        log_info("BLOOM FATAL: fb too large for slice");
        loop { sched_yield(); }
    }
    if (config.fb_vaddr as usize) % 4 != 0 {
        log_info("BLOOM FATAL: fb unaligned");
        loop { sched_yield(); }
    }
    
    let fb_slice = match crate::pixels::pixels_u32_mut(
        config.fb_vaddr as *mut u8,
        fb_len * 4,
        "present_loop::fb"
    ) {
        Some(s) => s,
        None => {
            log_info("BLOOM FATAL: cannot create fb slice (alignment)");
            loop { sched_yield(); }
        }
    };

    let mut frame_count: u64 = 0;
    
    loop {
        let start_time = monotonic_now();
        
        // Color cycling removed as requested.
        // The present loop now just maintains cadence until we have real content to present.
        
        frame_count += 1;
        
        if frame_count % 60 == 0 {
            log_info(&alloc::format!("BLOOM PRESENT: frame {}", frame_count));
        }

        let elapsed = (monotonic_now() - start_time) / 1_000_000;
        let sleep_ms = if elapsed < BLOOM_TIMEFRAME_MS {
            BLOOM_TIMEFRAME_MS - elapsed
        } else {
            0
        };
        
        if sleep_ms > 0 {
            thing_std::time::sleep_ms(sleep_ms);
        } else {
            sched_yield();
        }
    }
}
