use crate::logging;

pub struct FrameLoop {
    frame: u64,
    target_ms: u32,
    last_heartbeat: u64,
}

impl FrameLoop {
    pub fn new(target_fps: u32) -> Self {
        let target_ms = if target_fps > 0 { 1000 / target_fps } else { 16 };
        crate::log!("running (fps_target={})", target_fps);
        Self {
            frame: 0,
            target_ms,
            last_heartbeat: 0,
        }
    }

    pub fn next(&mut self) -> u64 {
        let f = self.frame;
        self.frame = self.frame.wrapping_add(1);
        f
    }

    pub fn sleep(&self) {
        stem::sleep_ms(self.target_ms as u64);
    }

    pub fn heartbeat(&mut self, cursor_x: i32, cursor_y: i32) {
        // Log once per second (approx 60 frames)
        // Using a simple modulo check on the frame counter
        if self.frame > 0 && self.frame % 60 == 0 {
             crate::log!("heartbeat frame={} cursor=({},{})", self.frame, cursor_x, cursor_y);
        }
    }
}
