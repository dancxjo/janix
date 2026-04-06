//! Cursor State - Pure logical cursor state.
//!
//! This module only manages the logical state of the cursor:
//! - Position (x, y)
//! - Button states
//!
//! Cursor rasterization and caching is handled by the `cursor_rasterizer` module.
//! The compositor blends the cached cursor snapshot at the final stage.

use crate::geometry::Rect;

/// Pure logical cursor state.
///
/// This struct only contains the logical state of the cursor position
/// and button presses. It does NOT:
/// - Hold cursor assets
/// - Own cursor image data
/// - Generate any drawing commands
///
/// This separation ensures that cursor position changes do not trigger
/// any rasterization or window damage.
pub struct CursorState {
    /// Cursor X position on screen
    pub x: i32,
    /// Cursor Y position on screen
    pub y: i32,
    /// Bitmask of pressed buttons (bit 0 = left, bit 1 = right, bit 2 = middle, etc.)
    buttons: u32,
}

impl CursorState {
    /// Create a new cursor state at the given position.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, buttons: 0 }
    }

    /// Apply a relative movement delta, clamping to screen bounds.
    pub fn apply_move(&mut self, dx: i16, dy: i16, w: i32, h: i32) {
        let nx = self.x + dx as i32;
        let ny = self.y + dy as i32;
        self.set_position_clamped(nx, ny, w, h);
    }

    /// Set an absolute cursor position, clamping to screen bounds.
    pub fn set_position_clamped(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let mut nx = x;
        let mut ny = y;
        if nx < 0 {
            nx = 0;
        }
        if ny < 0 {
            ny = 0;
        }
        if nx >= w {
            nx = w.saturating_sub(1);
        }
        if ny >= h {
            ny = h.saturating_sub(1);
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

    /// Get the current button state bitmask.
    pub fn buttons(&self) -> u32 {
        self.buttons
    }

    /// Replace the current button bitmask.
    pub fn set_buttons(&mut self, buttons: u32) {
        self.buttons = buttons;
    }

    /// Get the current position as a tuple.
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Check if any button is pressed.
    pub fn any_button_pressed(&self) -> bool {
        self.buttons != 0
    }

    /// Check if a specific button is pressed.
    pub fn is_button_pressed(&self, button: u8) -> bool {
        if button < 32 {
            self.buttons & (1u32 << button) != 0
        } else {
            false
        }
    }

    /// Compute the cursor bounding box for damage tracking.
    ///
    /// This returns a fixed-size bounding box based on typical cursor sizes.
    /// The actual cursor dimensions come from the CursorRasterizer.
    ///
    /// Note: This is used for cursor damage tracking when the cursor moves.
    /// The size is an estimate; the actual cursor snapshot may be different.
    pub fn bbox(&self) -> Rect {
        // Estimate: typical cursor is 32x32 + 3px shadow padding
        const CURSOR_SIZE: i32 = 35;
        Rect::new(self.x - 16, self.y - 16, CURSOR_SIZE, CURSOR_SIZE)
    }

    /// Compute cursor bounding box with known cursor dimensions.
    pub fn bbox_with_size(&self, hotspot_x: i32, hotspot_y: i32, width: i32, height: i32) -> Rect {
        let x = self.x - hotspot_x;
        let y = self.y - hotspot_y;
        Rect::new(x, y, width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_move_clamps_to_bounds() {
        let mut cursor = CursorState::new(50, 50);

        // Move left past boundary
        cursor.apply_move(-100, 0, 100, 100);
        assert_eq!(cursor.x, 0);

        // Move right past boundary
        cursor.apply_move(200, 0, 100, 100);
        assert_eq!(cursor.x, 99);
    }

    #[test]
    fn cursor_buttons_work() {
        let mut cursor = CursorState::new(0, 0);

        assert!(!cursor.any_button_pressed());

        cursor.button_down(0);
        assert!(cursor.is_button_pressed(0));
        assert!(cursor.any_button_pressed());

        cursor.button_up(0);
        assert!(!cursor.is_button_pressed(0));
        assert!(!cursor.any_button_pressed());
    }
}
