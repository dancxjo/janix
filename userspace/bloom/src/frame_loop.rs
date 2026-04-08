//! Frame loop using unified Instant timebase.
//!
//! All timing derives from `stem::time::now()` which returns an `Instant`.

use stem::syscall::ChannelHandle;
use stem::time::{Duration, Instant};

pub struct FrameLoop {
    frame: u64,
    target_duration: Duration,
    frame_start: Instant,
}

impl FrameLoop {
    pub fn new(target_fps: u32) -> Self {
        let target_ns = if target_fps > 0 {
            1_000_000_000 / target_fps as u64
        } else {
            16_666_666 // ~60 FPS default
        };
        crate::log!("running (fps_target={})", target_fps);
        Self {
            frame: 0,
            target_duration: Duration::from_nanos(target_ns),
            frame_start: stem::time::now(),
        }
    }

    /// Start the next frame. Returns the frame number.
    pub fn next(&mut self) -> u64 {
        self.frame_start = stem::time::now();
        let f = self.frame;
        self.frame = self.frame.wrapping_add(1);
        f
    }

    /// Sleep until the target frame duration has elapsed.
    pub fn sleep(&self) {
        let now = stem::time::now();
        let elapsed = now.saturating_sub(self.frame_start);
        if elapsed.as_nanos() < self.target_duration.as_nanos() {
            let remaining = self.target_duration.saturating_sub(elapsed);
            stem::time::sleep(remaining);
        } else {
            // We missed the target, yield to other threads
            stem::yield_now();
        }
    }

    pub fn sleep_until_input(&self, input_handle: Option<ChannelHandle>) {
        let now = stem::time::now();
        let elapsed = now.saturating_sub(self.frame_start);

        if elapsed.as_nanos() >= self.target_duration.as_nanos() {
            stem::yield_now();
            return;
        }

        let remaining = self.target_duration.saturating_sub(elapsed);

        if let Some(handle) = input_handle {
            let mut ws = stem::wait_set::WaitSet::new();
            if ws.add_port_readable(handle as u64).is_ok() {
                let _ = ws.wait(Some(remaining));
                return;
            }
        }

        stem::time::sleep(remaining);
    }

    /// Periodic heartbeat for diagnostics.
    pub fn heartbeat(&mut self, _cursor_x: i32, _cursor_y: i32) {
        if self.frame > 0 && self.frame % 60 == 0 {
            // Optional: log timing drift here if needed
        }
    }

    /// Returns the current frame number.
    #[inline]
    pub fn frame_number(&self) -> u64 {
        self.frame
    }

    /// Returns the target duration per frame.
    #[inline]
    pub fn target_duration(&self) -> Duration {
        self.target_duration
    }
}
