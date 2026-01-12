extern crate alloc;

use bloom::{DrawCmd, DrawList};
use bloom::cursor::CursorFrame;

use crate::asset::wallpaper::WallpaperSurface;
use crate::scene::SceneState;
use crate::surface::{PixelFormat, Surface};

pub struct Compositor;

impl Compositor {
    pub fn compose(
        &self,
        scene: &SceneState,
        surface: &mut Surface<'_>,
        cursor_frame: Option<&CursorFrame>,
        drawlist: Option<&DrawList>,
    ) {
        if surface.buf.len() < surface.required_len() {
            return;
        }

        tile_wallpaper(surface, &scene.assets.wallpaper);

        if let Some(list) = drawlist {
            render_drawlist(surface, list);
        }

        if let Some(frame) = cursor_frame {
            draw_cursor(surface, scene.cursor_pos, frame);
        }
    }
}

fn tile_wallpaper(surface: &mut Surface<'_>, wallpaper: &WallpaperSurface) {
    let max_width = surface.stride_bytes / 4;
    let width = surface.width.min(max_width);
    let height = surface.height;
    if wallpaper.width == 0 || wallpaper.height == 0 {
        return;
    }
    let wp_w = wallpaper.width as usize;
    let wp_h = wallpaper.height as usize;

    for y in 0..height {
        let src_y = y % wp_h;
        let row = y * surface.stride_bytes;
        for x in 0..width {
            let src_x = x % wp_w;
            let pixel = wallpaper.pixels[src_y * wp_w + src_x];
            let out = match surface.format {
                PixelFormat::Xrgb8888 => pixel & 0x00FF_FFFF,
                PixelFormat::Argb8888 => 0xFF00_0000 | (pixel & 0x00FF_FFFF),
            };
            let idx = row + x * 4;
            surface.buf[idx..idx + 4].copy_from_slice(&out.to_le_bytes());
        }
    }
}

fn draw_cursor(surface: &mut Surface<'_>, pos: (i32, i32), frame: &CursorFrame) {
    let (hot_x, hot_y) = (frame.hotspot_x, frame.hotspot_y);
    let base_x = pos.0 - hot_x;
    let base_y = pos.1 - hot_y;

    draw_cursor_layer(
        surface,
        base_x + frame.shadow_offset_x,
        base_y + frame.shadow_offset_y,
        frame.width,
        frame.height,
        &frame.shadow_pixels,
    );
    draw_cursor_layer(
        surface,
        base_x,
        base_y,
        frame.width,
        frame.height,
        &frame.pixels,
    );
}

fn draw_cursor_layer(
    surface: &mut Surface<'_>,
    origin_x: i32,
    origin_y: i32,
    width: u32,
    height: u32,
    pixels: &[u32],
) {
    let surf_w = surface.width as i32;
    let surf_h = surface.height as i32;
    let w = width as i32;
    let h = height as i32;

    let x0 = origin_x.max(0);
    let y0 = origin_y.max(0);
    let x1 = (origin_x + w).min(surf_w);
    let y1 = (origin_y + h).min(surf_h);

    if x0 >= x1 || y0 >= y1 {
        return;
    }

    for y in y0..y1 {
        let src_y = (y - origin_y) as usize;
        let row = y as usize * surface.stride_bytes;
        for x in x0..x1 {
            let src_x = (x - origin_x) as usize;
            let src_pixel = pixels[src_y * width as usize + src_x];
            if (src_pixel >> 24) == 0 {
                continue;
            }
            let idx = row + x as usize * 4;
            let dst = u32::from_le_bytes([
                surface.buf[idx],
                surface.buf[idx + 1],
                surface.buf[idx + 2],
                surface.buf[idx + 3],
            ]);
            let blended = blend_premultiplied(dst, src_pixel, surface.format);
            surface.buf[idx..idx + 4].copy_from_slice(&blended.to_le_bytes());
        }
    }
}

fn blend_premultiplied(dst: u32, src: u32, format: PixelFormat) -> u32 {
    let src_a = (src >> 24) & 0xFF;
    if src_a == 255 {
        return match format {
            PixelFormat::Xrgb8888 => src & 0x00FF_FFFF,
            PixelFormat::Argb8888 => src,
        };
    }
    if src_a == 0 {
        return dst;
    }
    let inv_a = 255 - src_a;
    let dst_r = (dst >> 16) & 0xFF;
    let dst_g = (dst >> 8) & 0xFF;
    let dst_b = dst & 0xFF;
    let src_r = (src >> 16) & 0xFF;
    let src_g = (src >> 8) & 0xFF;
    let src_b = src & 0xFF;

    let out_r = src_r + (dst_r * inv_a) / 255;
    let out_g = src_g + (dst_g * inv_a) / 255;
    let out_b = src_b + (dst_b * inv_a) / 255;

    match format {
        PixelFormat::Xrgb8888 => (out_r << 16) | (out_g << 8) | out_b,
        PixelFormat::Argb8888 => 0xFF00_0000 | (out_r << 16) | (out_g << 8) | out_b,
    }
}

fn render_drawlist(surface: &mut Surface<'_>, list: &DrawList) {
    let max_width = surface.stride_bytes / 4;
    let width = surface.width.min(max_width);
    let height = surface.height;

    match surface.format {
        PixelFormat::Xrgb8888 => render_drawlist_xrgb(surface, list, width, height),
        PixelFormat::Argb8888 => render_drawlist_argb(surface, list, width, height),
    }
}

fn render_drawlist_xrgb(surface: &mut Surface<'_>, list: &DrawList, width: usize, height: usize) {
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

fn render_drawlist_argb(surface: &mut Surface<'_>, list: &DrawList, width: usize, height: usize) {
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
