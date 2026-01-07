#[cfg(test)]
mod tests {
    use crate::cursor_overlay::CursorOverlay;
    use crate::assets::cursor::CursorFrame;
    use crate::scene::Rect;
    use alloc::vec;

    fn create_mock_cursor(w: u32, h: u32, hx: i32, hy: i32) -> CursorFrame {
        let pixels = vec![0xFFFFFFFF; (w * h) as usize];
        CursorFrame::new(pixels, w, h, hx, hy)
    }

    #[test]
    fn test_cursor_overlay_bounds_correctness() {
        let screen_w = 100;
        let screen_h = 100;
        let mut overlay = CursorOverlay::new(screen_w, screen_h);

        let cursor_w = 10;
        let cursor_h = 10;
        let hotspot_x = 2;
        let hotspot_y = 2;
        let cursor = create_mock_cursor(cursor_w, cursor_h, hotspot_x, hotspot_y);

        let shadow_off_x = cursor.shadow_offset_x;
        let shadow_off_y = cursor.shadow_offset_y;

        // Mock buffers
        let buffer_size = (screen_w * screen_h) as usize;
        let scene_buffer = vec![0u32; buffer_size];
        let mut framebuffer = vec![0u32; buffer_size];

        // 1. Initial Present
        let x1 = 20;
        let y1 = 20;
        let dirty1 = overlay.present(&scene_buffer, &mut framebuffer, x1, y1, Some(&cursor));

        // Expected bounds 1:
        let sprite_x1 = x1 - hotspot_x;
        let sprite_y1 = y1 - hotspot_y;
        let shadow_x1 = sprite_x1 + shadow_off_x;
        let shadow_y1 = sprite_y1 + shadow_off_y;

        let min_x1 = sprite_x1.min(shadow_x1);
        let min_y1 = sprite_y1.min(shadow_y1);
        let max_x1 = (sprite_x1 + cursor_w as i32).max(shadow_x1 + cursor_w as i32);
        let max_y1 = (sprite_y1 + cursor_h as i32).max(shadow_y1 + cursor_h as i32);

        let expected_rect1 = Rect {
            x: min_x1,
            y: min_y1,
            w: (max_x1 - min_x1) as u32,
            h: (max_y1 - min_y1) as u32,
        };

        assert_eq!(dirty1, Some(expected_rect1), "First present should match bounds");

        // 2. Move Cursor
        let x2 = 30;
        let y2 = 30;
        let dirty2 = overlay.present(&scene_buffer, &mut framebuffer, x2, y2, Some(&cursor));

        // Expected bounds 2 (new position)
        let sprite_x2 = x2 - hotspot_x;
        let sprite_y2 = y2 - hotspot_y;
        let shadow_x2 = sprite_x2 + shadow_off_x;
        let shadow_y2 = sprite_y2 + shadow_off_y;

        let min_x2 = sprite_x2.min(shadow_x2);
        let min_y2 = sprite_y2.min(shadow_y2);
        let max_x2 = (sprite_x2 + cursor_w as i32).max(shadow_x2 + cursor_w as i32);
        let max_y2 = (sprite_y2 + cursor_h as i32).max(shadow_y2 + cursor_h as i32);

        let expected_rect2 = Rect {
            x: min_x2,
            y: min_y2,
            w: (max_x2 - min_x2) as u32,
            h: (max_y2 - min_y2) as u32,
        };

        // The returned dirty rect should be union(expected_rect1, expected_rect2)
        let expected_dirty = Rect::union(expected_rect1, expected_rect2);

        assert_eq!(dirty2, Some(expected_dirty), "Second present should be union of prev and new");
    }
}
