use thing_std::*;
use abi::ids::ThingId;

pub const BLOOM_TIMEFRAME_MS: u64 = 16;

#[derive(Clone, Copy, Debug)]
pub struct PresentConfig {
    pub width: u32,
    pub height: u32,
}

pub extern "C" fn present_loop_entry(arg: u64) -> ! {
    log_info("BLOOM PRESENT: thread alive (cadence only)");

    let config_ptr = arg as *const PresentConfig;
    let config = unsafe { *config_ptr };
    
    log_info(&alloc::format!("BLOOM PRESENT: {}x{}", config.width, config.height));
    
    // This thread just maintains frame cadence - actual presents happen in main loop
    // Threads cannot share bytespace mappings, so we can't access hw framebuffer here
    let mut frame_count: u64 = 0;
    
    loop {
        frame_count += 1;
        
        if frame_count % 300 == 0 {
            log_info(&alloc::format!("BLOOM PRESENT: frame {}", frame_count));
        }

        thing_std::time::sleep_ms(BLOOM_TIMEFRAME_MS);
    }
}

