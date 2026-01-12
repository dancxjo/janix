extern crate alloc;

use alloc::vec::Vec;

use bloom::cursor::ani::load_ani;
use bloom::cursor::cur::load_cur;
use bloom::cursor::{CursorAsset, CursorFrame};

pub struct CursorTheme {
    pub asset: CursorAsset,
}

impl CursorTheme {
    pub fn placeholder() -> Self {
        let size = 16u32;
        let mut pixels = Vec::with_capacity((size * size) as usize);
        for y in 0..size {
            for x in 0..size {
                let on = x == y || x == (size - 1 - y) || x == size / 2 || y == size / 2;
                if on {
                    pixels.push(0xFFFF_FFFF);
                } else {
                    pixels.push(0);
                }
            }
        }
        let frame = CursorFrame::new(pixels, size, size, 0, 0);
        Self {
            asset: CursorAsset::static_cursor(frame),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if let Some(asset) = load_ani(bytes) {
            return Some(Self { asset });
        }
        if let Some(frame) = load_cur(bytes) {
            return Some(Self {
                asset: CursorAsset::static_cursor(frame),
            });
        }
        None
    }

    pub fn frame_at_ms(&self, elapsed_ms: u64) -> Option<&CursorFrame> {
        if self.asset.frames.is_empty() {
            return None;
        }
        if !self.asset.is_animated {
            return self.asset.frames.first();
        }

        let step_count = if self.asset.sequence.is_empty() {
            self.asset.frames.len()
        } else {
            self.asset.sequence.len()
        };
        if step_count == 0 {
            return None;
        }

        let mut total_ms: u32 = 0;
        for step in 0..step_count {
            total_ms = total_ms.saturating_add(self.delay_ms(step));
        }
        if total_ms == 0 {
            return self.asset.frames.first();
        }

        let mut cursor_ms = (elapsed_ms as u32) % total_ms;
        for step in 0..step_count {
            let delay = self.delay_ms(step);
            if cursor_ms < delay {
                let frame_idx = if self.asset.sequence.is_empty() {
                    step
                } else {
                    self.asset.sequence[step] as usize
                };
                return self.asset.frames.get(frame_idx);
            }
            cursor_ms = cursor_ms.saturating_sub(delay);
        }

        self.asset.frames.first()
    }

    fn delay_ms(&self, step: usize) -> u32 {
        if self.asset.delays_ms.is_empty() {
            60
        } else {
            let idx = step % self.asset.delays_ms.len();
            self.asset.delays_ms[idx].max(1)
        }
    }
}
