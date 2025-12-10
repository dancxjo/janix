use crate::config::CURSOR_COLOR;
use crate::render::primitives::fill_rect;

pub fn draw_cursor(buffer: *mut u32, stride: u32, fb_width: u32, fb_height: u32, cx: i32, cy: i32) {
    let size = 12;
    let x = cx.clamp(0, fb_width as i32 - 1);
    let y = cy.clamp(0, fb_height as i32 - 1);
    fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x - 1,
        y,
        size,
        2,
        CURSOR_COLOR,
    );
    fill_rect(
        buffer,
        stride,
        fb_width,
        fb_height,
        x,
        y - 1,
        2,
        size,
        CURSOR_COLOR,
    );
}
