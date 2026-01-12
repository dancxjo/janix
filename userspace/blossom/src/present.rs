use bloom::{DrawCmd, DrawList};

use crate::{PixelFormat, Surface};

/// Render a drawlist into the provided surface.
pub fn render(surface: &mut Surface<'_>, list: &DrawList) {
    if surface.buf.len() < surface.required_len() {
        return;
    }

    let max_width = surface.stride_bytes / 4;
    let width = surface.width.min(max_width);
    let height = surface.height;

    match surface.format {
        PixelFormat::Xrgb8888 => render_xrgb8888(surface, list, width, height),
        PixelFormat::Argb8888 => render_argb8888(surface, list, width, height),
    }
}

fn render_xrgb8888(surface: &mut Surface<'_>, list: &DrawList, width: usize, height: usize) {
    for cmd in &list.cmds {
        match *cmd {
            DrawCmd::Clear(rgba) => {
                let pixel = rgba & 0x00FF_FFFF;
                let bytes = pixel.to_le_bytes();
                for y in 0..height {
                    let row = y * surface.stride_bytes;
                    for x in 0..width {
                        let idx = row + x * 4;
                        surface.buf[idx..idx + 4].copy_from_slice(&bytes);
                    }
                }
            }
            DrawCmd::Rect { x, y, w, h, rgba } => {
                fill_rect(surface, width, height, x, y, w, h, rgba & 0x00FF_FFFF);
            }
        }
    }
}

fn render_argb8888(surface: &mut Surface<'_>, list: &DrawList, width: usize, height: usize) {
    for cmd in &list.cmds {
        match *cmd {
            DrawCmd::Clear(rgba) => {
                let bytes = rgba.to_le_bytes();
                for y in 0..height {
                    let row = y * surface.stride_bytes;
                    for x in 0..width {
                        let idx = row + x * 4;
                        surface.buf[idx..idx + 4].copy_from_slice(&bytes);
                    }
                }
            }
            DrawCmd::Rect { x, y, w, h, rgba } => {
                fill_rect(surface, width, height, x, y, w, h, rgba);
            }
        }
    }
}

fn fill_rect(
    surface: &mut Surface<'_>,
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    pixel: u32,
) {
    if w <= 0 || h <= 0 {
        return;
    }

    let x0 = x.max(0) as usize;
    let y0 = y.max(0) as usize;
    let x1 = (x + w).min(width as i32).max(0) as usize;
    let y1 = (y + h).min(height as i32).max(0) as usize;

    if x0 >= x1 || y0 >= y1 {
        return;
    }

    let bytes = pixel.to_le_bytes();
    for yy in y0..y1 {
        let row = yy * surface.stride_bytes;
        for xx in x0..x1 {
            let idx = row + xx * 4;
            surface.buf[idx..idx + 4].copy_from_slice(&bytes);
        }
    }
}
