#![no_std]

pub mod logging;
pub mod time;

pub trait BootRuntime {
    fn putchar(&self, c: u8);
    fn halt(&self) -> !;

    /// Returns a strictly monotonic tick count since boot.
    ///
    /// This value must never decrease. If the underlying hardware timer
    /// wraps or is reset, the implementation must clamp the return value
    /// to the last largest value seen.
    fn mono_ticks(&self) -> u64 {
        0
    }

    /// Returns the frequency of the monotonic tick counter in Hz.
    ///
    /// Returns 0 if no monotonic timer is available.
    fn mono_freq_hz(&self) -> u64 {
        0
    }
    /// Per-CPU enable/config of SIMD/FPU. Idempotent.
    fn simd_init_cpu(&self) {}

    /// (size, align) required for saving one task's SIMD state.
    /// 
    /// Default implementation returns (0, 1) for no SIMD support.
    fn simd_state_layout(&self) -> (usize, usize) {
        (0, 1)
    }

    /// Save current CPU SIMD state into `dst` (must be size/align from layout).
    /// 
    /// # Safety
    /// `dst` must be valid for writes of size `layout.size` and aligned to `layout.align`.
    unsafe fn simd_save(&self, _dst: *mut u8) {}

    /// Restore CPU SIMD state from `src`.
    /// 
    /// # Safety
    /// `src` must be valid for reads of size `layout.size` and aligned to `layout.align`.
    unsafe fn simd_restore(&self, _src: *const u8) {}
}

pub fn start(runtime: &'static dyn BootRuntime) -> ! {
    unsafe {
        logging::init(runtime);
    }

    kinfo!("System booted");

    kinfo!("System halted");
    runtime.halt();
}
pub mod simd;
pub mod task;
