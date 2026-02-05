//! Cursor State - Pure logical cursor state.
//!
//! This module manages the logical state of the cursor:
//! - Position (x, y)
//! - Button states
//! - Visibility

/// Pure logical cursor state.
pub struct CursorState {
    /// Cursor X position on screen
    pub x: i32,
    /// Cursor Y position on screen
    pub y: i32,
    /// Screen width for clamping
    screen_w: i32,
    /// Screen height for clamping
    screen_h: i32,
    /// Bitmask of pressed buttons (bit 0 = left, bit 1 = right, bit 2 = middle)
    buttons: u32,
    /// Whether cursor is visible
    pub visible: bool,
}

impl CursorState {
    /// Create a new cursor state at center of screen.
    pub fn new(screen_w: i32, screen_h: i32) -> Self {
        Self {
            x: screen_w / 2,
            y: screen_h / 2,
            screen_w,
            screen_h,
            buttons: 0,
            visible: true,
        }
    }

    /// Apply a relative movement delta, clamping to screen bounds.
    pub fn apply_move(&mut self, dx: i16, dy: i16) {
        let mut nx = self.x + dx as i32;
        let mut ny = self.y + dy as i32;
        
        if nx < 0 {
            nx = 0;
        }
        if ny < 0 {
            ny = 0;
        }
        if nx >= self.screen_w {
            nx = self.screen_w.saturating_sub(1);
        }
        if ny >= self.screen_h {
            ny = self.screen_h.saturating_sub(1);
        }
        
        self.x = nx;
        self.y = ny;
    }

    /// Record a button press.
    pub fn button_down(&mut self, button: u8) {
        if button < 32 {
            self.buttons |= 1u32 << button;
        }
    }

    /// Record a button release.
    pub fn button_up(&mut self, button: u8) {
        if button < 32 {
            self.buttons &= !(1u32 << button);
        }
    }

    /// Get the current position as a tuple.
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Check if position changed since last check.
    pub fn moved_since(&self, prev_x: i32, prev_y: i32) -> bool {
        self.x != prev_x || self.y != prev_y
    }
}
