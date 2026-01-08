//! Present Loop - Continuous Frame Pump
//!
//! This module implements the presentation thread that copies frames from
//! Bloom's backbuffer to the hardware framebuffer. 
//!
//! ## Architecture
//! The present thread maps the display bytespace itself (proving that the
//! shared address space works - all threads in a process share page tables).
//! It then polls a dirty flag and copies the entire frame when dirty.

use thing_std::*;
use abi::ids::ThingId;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub const BLOOM_TIMEFRAME_MS: u64 = 16;

/// Shared state between main thread and present thread.
/// Main thread writes to frame buffer and sets dirty=true.
/// Present thread copies when dirty and clears the flag.
#[repr(C)]
pub struct PresentState {
    /// Pointer to Bloom's frame buffer (source for copy)
    pub frame_ptr: *const u32,
    /// Pointer to hardware framebuffer (destination for copy)
    /// Mapped by present thread using shared address space
    pub hw_fb_ptr: AtomicU32, // Upper bits
    pub hw_fb_ptr_lo: AtomicU32, // Lower bits (combined into 64-bit pointer)
    /// Display width in pixels
    pub width: u32,
    /// Display height in pixels  
    pub height: u32,
    /// Total pixel count (width * height)
    pub pixel_count: u32,
    /// Set by main thread when frame buffer has new content
    pub dirty: AtomicBool,
    /// Counter for telemetry - frames successfully presented
    pub frames_presented: AtomicU32,
    /// Flag indicating if present thread is alive
    pub alive: AtomicBool,
}

// Safety: PresentState is designed for cross-thread sharing via atomics
unsafe impl Sync for PresentState {}
unsafe impl Send for PresentState {}

impl PresentState {
    pub const fn new() -> Self {
        Self {
            frame_ptr: core::ptr::null(),
            hw_fb_ptr: AtomicU32::new(0),
            hw_fb_ptr_lo: AtomicU32::new(0),
            width: 0,
            height: 0,
            pixel_count: 0,
            dirty: AtomicBool::new(false),
            frames_presented: AtomicU32::new(0),
            alive: AtomicBool::new(false),
        }
    }
    
    /// Initialize with frame buffer info (NOT hw_fb_ptr - that's mapped by present thread)
    pub fn init(&mut self, frame_ptr: *const u32, width: u32, height: u32) {
        self.frame_ptr = frame_ptr;
        self.width = width;
        self.height = height;
        self.pixel_count = width * height;
        self.dirty.store(true, Ordering::Release); // Initial frame is dirty
    }
    
    /// Set hw_fb_ptr atomically (called by present thread after mapping)
    pub fn set_hw_fb_ptr(&self, ptr: *mut u32) {
        let addr = ptr as u64;
        self.hw_fb_ptr.store((addr >> 32) as u32, Ordering::Release);
        self.hw_fb_ptr_lo.store(addr as u32, Ordering::Release);
    }
    
    /// Get hw_fb_ptr atomically
    pub fn get_hw_fb_ptr(&self) -> *mut u32 {
        let hi = self.hw_fb_ptr.load(Ordering::Acquire) as u64;
        let lo = self.hw_fb_ptr_lo.load(Ordering::Acquire) as u64;
        ((hi << 32) | lo) as *mut u32
    }
    
    /// Mark the frame as dirty (main thread calls this after writing to frame buffer)
    pub fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::Release);
    }
    
    /// Check if present thread is alive
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::Acquire)
    }
}

/// Configuration passed to present thread at spawn time
#[derive(Clone, Copy, Debug)]
pub struct PresentConfig {
    /// Pointer to shared PresentState
    pub state: *const PresentState,
    /// Display bytespace ID to map
    pub display_bs_id: u128,
    /// Virtual address to map display at
    pub fb_vaddr: u64,
    /// Size of framebuffer in bytes
    pub fb_bytes: u64,
}

/// Present thread entry point.
/// Maps display bytespace itself (proving shared address space), then
/// continuously copies frames from backbuffer to hardware framebuffer.
pub extern "C" fn present_loop_entry(arg: u64) -> ! {
    log_info("[PRESENT] 1: entered");
    
    let config_ptr = arg as *const PresentConfig;
    let config = unsafe { *config_ptr };
    let state = unsafe { &*config.state };
    
    log_info("[PRESENT] 2: config read");
    
    // Signal we're alive
    state.alive.store(true, Ordering::Release);
    state.frames_presented.store(1, Ordering::Release);
    
    log_info("[PRESENT] 3: mapping display bytespace");
    
    // Map display bytespace in THIS thread - use raw syscall to check status
    let display_bs = ThingId(config.display_bs_id);
    let map_result = unsafe {
        thing_std::syscall(
            abi::syscall::nr::SYS_SPACE_MAP,
            display_bs.low(),
            display_bs.high(),
            config.fb_vaddr,
            0, // offset
            config.fb_bytes,
            0,
        )
    };
    
    if map_result.status != 0 {
        log_info(&alloc::format!(
            "[PRESENT] FATAL: space_map failed! status={} bs={:#x} vaddr={:#x}",
            map_result.status, config.display_bs_id, config.fb_vaddr
        ));
        // Fall into idle loop - don't try to write to unmapped memory!
        loop {
            state.frames_presented.fetch_add(1, Ordering::Relaxed);
            thing_std::time::sleep_ms(BLOOM_TIMEFRAME_MS);
        }
    }
    
    let hw_fb_ptr = config.fb_vaddr as *mut u32;
    
    log_info(&alloc::format!(
        "[PRESENT] 4: mapped display at {:#x} (bs={:#x})",
        config.fb_vaddr, config.display_bs_id
    ));
    
    // Store in shared state so main thread can see we're ready
    state.set_hw_fb_ptr(hw_fb_ptr);
    state.frames_presented.store(2, Ordering::Release);
    
    log_info(&alloc::format!(
        "[PRESENT] 5: ready - hw_fb={:#x}, frame={:#x}, {}x{}",
        hw_fb_ptr as usize,
        state.frame_ptr as usize,
        state.width, state.height
    ));
    
    let mut frame_count: u32 = 2;
    
    loop {
        // Check if main thread marked frame as dirty
        if state.dirty.swap(false, Ordering::AcqRel) {
            // Copy entire frame from backbuffer to hardware framebuffer
            if !state.frame_ptr.is_null() && !hw_fb_ptr.is_null() {
                let count = state.pixel_count as usize;
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        state.frame_ptr,
                        hw_fb_ptr,
                        count,
                    );
                }
                
                frame_count = frame_count.wrapping_add(1);
                state.frames_presented.store(frame_count, Ordering::Relaxed);
                
                // Log periodically (every ~1 second at 60fps)
                if frame_count % 60 == 0 {
                    log_info(&alloc::format!("[PRESENT] frame n={}", frame_count));
                }
            }
        }

        thing_std::time::sleep_ms(BLOOM_TIMEFRAME_MS);
    }
}

