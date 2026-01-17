// use crate::logging;

pub struct FrameLoop {
    frame: u64,
    target_ns: u64,
    frame_start_ns: u64,
}

impl FrameLoop {
    pub fn new(target_fps: u32) -> Self {
        let target_ns = if target_fps > 0 { 1_000_000_000 / target_fps as u64 } else { 16_666_666 };
        crate::log!("running (fps_target={})", target_fps);
        Self {
            frame: 0,
            target_ns,
            frame_start_ns: stem::monotonic_ns(),
        }
    }

    pub fn next(&mut self) -> u64 {
        self.frame_start_ns = stem::monotonic_ns();
        let f = self.frame;
        self.frame = self.frame.wrapping_add(1);
        f
    }

    pub fn sleep(&self) {
        let now = stem::monotonic_ns();
        let elapsed = now.saturating_sub(self.frame_start_ns);
        if elapsed < self.target_ns {
            let remaining_ns = self.target_ns - elapsed;
            stem::syscall::sleep_ns(remaining_ns);
        } else {
            // We missed the target, yield to other threads
            stem::yield_now();
        }
    }

    pub fn heartbeat(&mut self, _cursor_x: i32, _cursor_y: i32) {
        if self.frame > 0 && self.frame % 60 == 0 {
             // Optional: log drift here if needed
        }
    }
}
