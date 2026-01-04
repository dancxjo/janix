//! Cursor asset types and animation support
//!
//! Provides:
//! - `CursorFrame` - Single cursor image with hotspot and prerendered shadow
//! - `CursorAsset` - Complete cursor (static or animated)
//! - `CursorAnimator` - Stateful playback for animated cursors

extern crate alloc;
use alloc::vec::Vec;

pub mod ani;
pub mod cur;

/// Shadow configuration
const SHADOW_OFFSET_X: i32 = 2;
const SHADOW_OFFSET_Y: i32 = 3;
const SHADOW_BLUR_RADIUS: u32 = 3;
const SHADOW_OPACITY: u8 = 128;

/// A single cursor frame with premultiplied ARGB pixels and prerendered shadow
pub struct CursorFrame {
    /// Pixels in premultiplied ARGB format (0xAARRGGBB)
    pub pixels: Vec<u32>,
    /// Prerendered shadow pixels (same dimensions, offset applied at draw time)
    pub shadow_pixels: Vec<u32>,
    pub width: u32,
    pub height: u32,
    /// Hotspot X offset from top-left
    pub hotspot_x: i32,
    /// Hotspot Y offset from top-left
    pub hotspot_y: i32,
    /// Shadow offset X (positive = right)
    pub shadow_offset_x: i32,
    /// Shadow offset Y (positive = down)
    pub shadow_offset_y: i32,
}

impl CursorFrame {
    /// Create a new cursor frame and prerender its shadow
    pub fn new(pixels: Vec<u32>, width: u32, height: u32, hotspot_x: i32, hotspot_y: i32) -> Self {
        let shadow_pixels = generate_shadow(&pixels, width, height);
        Self {
            pixels,
            shadow_pixels,
            width,
            height,
            hotspot_x,
            hotspot_y,
            shadow_offset_x: SHADOW_OFFSET_X,
            shadow_offset_y: SHADOW_OFFSET_Y,
        }
    }
}

/// Generate a blurred shadow from cursor alpha channel
fn generate_shadow(pixels: &[u32], width: u32, height: u32) -> Vec<u32> {
    let w = width as usize;
    let h = height as usize;
    let mut shadow = alloc::vec![0u32; w * h];

    // Extract alpha channel and apply blur
    let radius = SHADOW_BLUR_RADIUS as i32;
    let kernel_size = (radius * 2 + 1) as usize;
    let divisor = (kernel_size * kernel_size) as u32;

    for y in 0..h {
        for x in 0..w {
            let mut alpha_sum: u32 = 0;
            let mut sample_count: u32 = 0;

            // Box blur: sample surrounding pixels
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let sx = x as i32 + dx;
                    let sy = y as i32 + dy;

                    if sx >= 0 && sx < w as i32 && sy >= 0 && sy < h as i32 {
                        let src_idx = sy as usize * w + sx as usize;
                        let src_alpha = (pixels[src_idx] >> 24) & 0xFF;
                        alpha_sum += src_alpha;
                        sample_count += 1;
                    }
                }
            }

            if sample_count > 0 {
                // Calculate blurred alpha, scale to shadow opacity
                let blurred_alpha = alpha_sum / sample_count;
                let shadow_alpha = ((blurred_alpha as u32 * SHADOW_OPACITY as u32) / 255) as u8;

                // Shadow is black with computed alpha (premultiplied, so all channels are 0)
                shadow[y * w + x] = (shadow_alpha as u32) << 24;
            }
        }
    }

    shadow
}

/// A cursor asset, either static or animated
pub struct CursorAsset {
    /// All frames (single for .cur, multiple for .ani)
    pub frames: Vec<CursorFrame>,
    /// Per-frame delay in milliseconds (empty for static cursors)
    pub delays_ms: Vec<u32>,
    /// Frame playback order (indices into frames). If empty, use 0..frames.len()
    pub sequence: Vec<u32>,
    /// Whether this cursor has animation
    pub is_animated: bool,
}

impl CursorAsset {
    /// Create a static cursor from a single frame
    pub fn static_cursor(frame: CursorFrame) -> Self {
        Self {
            frames: alloc::vec![frame],
            delays_ms: Vec::new(),
            sequence: Vec::new(),
            is_animated: false,
        }
    }

    /// Get the current frame for a static cursor
    pub fn frame(&self) -> Option<&CursorFrame> {
        self.frames.first()
    }
}

/// Animator for playing back animated cursors
pub struct CursorAnimator {
    /// The cursor asset being animated
    asset: CursorAsset,
    /// Current position in the sequence (or frame index if no sequence)
    current_step: usize,
    /// Tick value when next frame transition should occur
    next_deadline_ticks: u64,
    /// Ticks per millisecond for timing conversion
    ticks_per_ms: u64,
}

impl CursorAnimator {
    /// Create a new animator for the given cursor asset
    pub fn new(asset: CursorAsset, ticks_per_ms: u64) -> Self {
        Self {
            asset,
            current_step: 0,
            next_deadline_ticks: 0,
            ticks_per_ms: if ticks_per_ms == 0 { 1 } else { ticks_per_ms },
        }
    }

    /// Advance the animation based on current tick count
    pub fn advance(&mut self, now_ticks: u64) -> bool {
        if !self.asset.is_animated || self.asset.frames.is_empty() {
            return false;
        }
        if self.next_deadline_ticks == 0 {
            self.next_deadline_ticks = now_ticks + self.current_delay_ticks();
            return false;
        }
        if now_ticks < self.next_deadline_ticks {
            return false;
        }
        let mut changed = false;
        while now_ticks >= self.next_deadline_ticks {
            self.current_step = (self.current_step + 1) % self.step_count();
            self.next_deadline_ticks += self.current_delay_ticks();
            changed = true;
        }
        changed
    }

    /// Get the current frame to display
    pub fn current_frame(&self) -> Option<&CursorFrame> {
        let frame_idx = self.current_frame_index();
        self.asset.frames.get(frame_idx)
    }

    fn current_frame_index(&self) -> usize {
        if self.asset.sequence.is_empty() {
            self.current_step % self.asset.frames.len()
        } else {
            let seq_idx = self.current_step % self.asset.sequence.len();
            self.asset.sequence[seq_idx] as usize
        }
    }

    fn current_delay_ticks(&self) -> u64 {
        let delay_ms = if self.asset.delays_ms.is_empty() {
            100
        } else {
            let idx = self.current_step % self.asset.delays_ms.len();
            self.asset.delays_ms[idx] as u64
        };
        delay_ms * self.ticks_per_ms
    }

    fn step_count(&self) -> usize {
        if self.asset.sequence.is_empty() {
            self.asset.frames.len()
        } else {
            self.asset.sequence.len()
        }
    }

    pub fn is_animated(&self) -> bool {
        self.asset.is_animated
    }

    pub fn hotspot(&self) -> (i32, i32) {
        self.current_frame()
            .map(|f| (f.hotspot_x, f.hotspot_y))
            .unwrap_or((0, 0))
    }
}
