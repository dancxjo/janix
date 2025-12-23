use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cmp::{max, min};

use crate::render::bitmap::Bitmap;

pub const CURSOR_SIZE: usize = 32; // Most of these are 32x32 now

mod assets {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/cursor_assets.rs"));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorKind {
    Arrow,
    Move,
    ResizeN,
    ResizeS,
    ResizeE,
    ResizeW,
    ResizeNE,
    ResizeNW,
    ResizeSE,
    ResizeSW,
}

#[derive(Clone, Debug)]
pub struct CursorIcon {
    pub bitmap: Arc<Bitmap>,
    pub hotspot: (i32, i32),
}

#[derive(Debug)]
pub struct CursorSprites {
    pub arrow: CursorIcon,
    pub move_icon: CursorIcon,
    pub resize_ns: CursorIcon,
    pub resize_ew: CursorIcon,
    pub resize_ne_sw: CursorIcon,
    pub resize_nw_se: CursorIcon,
}

impl CursorSprites {
    pub fn for_kind(&self, kind: CursorKind) -> &CursorIcon {
        match kind {
            CursorKind::Arrow => &self.arrow,
            CursorKind::Move => &self.move_icon,
            CursorKind::ResizeN | CursorKind::ResizeS => &self.resize_ns,
            CursorKind::ResizeE | CursorKind::ResizeW => &self.resize_ew,
            CursorKind::ResizeNE | CursorKind::ResizeSW => &self.resize_ne_sw,
            CursorKind::ResizeNW | CursorKind::ResizeSE => &self.resize_nw_se,
        }
    }
}

fn load_asset(data: &[u8], width: u32, height: u32, hot_x: i32, hot_y: i32) -> CursorIcon {
    // Data is raw LE u32s
    let pixels: Vec<u32> = data
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();

    CursorIcon {
        bitmap: Arc::new(Bitmap {
            width: width as usize,
            height: height as usize,
            data: pixels,
        }),
        hotspot: (hot_x, hot_y),
    }
}

pub fn build_cursor_sprites() -> CursorSprites {
    let arrow = load_asset(
        assets::arrow::DATA,
        assets::arrow::WIDTH,
        assets::arrow::HEIGHT,
        assets::arrow::HOTSPOT_X,
        assets::arrow::HOTSPOT_Y,
    );
    let move_icon = load_asset(
        assets::r#move::DATA,
        assets::r#move::WIDTH,
        assets::r#move::HEIGHT,
        assets::r#move::HOTSPOT_X,
        assets::r#move::HOTSPOT_Y,
    );
    let resize_ns = load_asset(
        assets::resize_ns::DATA,
        assets::resize_ns::WIDTH,
        assets::resize_ns::HEIGHT,
        assets::resize_ns::HOTSPOT_X,
        assets::resize_ns::HOTSPOT_Y,
    );
    let resize_ew = load_asset(
        assets::resize_ew::DATA,
        assets::resize_ew::WIDTH,
        assets::resize_ew::HEIGHT,
        assets::resize_ew::HOTSPOT_X,
        assets::resize_ew::HOTSPOT_Y,
    );
    let resize_nw_se = load_asset(
        assets::resize_nwse::DATA,
        assets::resize_nwse::WIDTH,
        assets::resize_nwse::HEIGHT,
        assets::resize_nwse::HOTSPOT_X,
        assets::resize_nwse::HOTSPOT_Y,
    );
    let resize_ne_sw = load_asset(
        assets::resize_nesw::DATA,
        assets::resize_nesw::WIDTH,
        assets::resize_nesw::HEIGHT,
        assets::resize_nesw::HOTSPOT_X,
        assets::resize_nesw::HOTSPOT_Y,
    );

    CursorSprites {
        arrow,
        move_icon,
        resize_ns,
        resize_ew,
        resize_ne_sw,
        resize_nw_se,
    }
}

pub fn raster_draw_cursor(
    buffer: *mut u32,
    stride_bytes: u32,
    fb_width: u32,
    fb_height: u32,
    origin: (i32, i32),
    sprite: &Bitmap,
    hotspot: (i32, i32),
) {
    let top_left_x = origin.0 - hotspot.0;
    let top_left_y = origin.1 - hotspot.1;

    let start_x = max(0, top_left_x);
    let start_y = max(0, top_left_y);
    let end_x = min(fb_width as i32, top_left_x + sprite.width as i32);
    let end_y = min(fb_height as i32, top_left_y + sprite.height as i32);

    if start_x >= end_x || start_y >= end_y {
        return;
    }

    let sprite_width = sprite.width;
    let sprite_data = &sprite.data;

    // Pre-calculate vertical steps to avoid repeated multiplication
    let start_sy = (start_y - top_left_y) as usize;
    let start_sx = (start_x - top_left_x) as usize;
    let width_to_draw = (end_x - start_x) as usize;

    for (row_idx, y) in (start_y..end_y).enumerate() {
        let sy = start_sy + row_idx;
        let sprite_offset = sy * sprite_width + start_sx;

        let sprite_row = &sprite_data[sprite_offset..sprite_offset + width_to_draw];

        unsafe {
            // Use byte-based stride arithmetic
            let dst_row_ptr = (buffer as *mut u8).add((y as usize) * (stride_bytes as usize)) as *mut u32;
            let dst_ptr = dst_row_ptr.add(start_x as usize);

            for (i, &px) in sprite_row.iter().enumerate() {
                let alpha = (px >> 24) & 0xFF;
                if alpha == 0 {
                    continue;
                }

                let dst = dst_ptr.add(i);

                if alpha == 0xFF {
                    *dst = px;
                } else {
                    let inv_a = 255 - alpha;
                    let dst_val = *dst;

                    let dst_r = (dst_val >> 16) & 0xFF;
                    let dst_g = (dst_val >> 8) & 0xFF;
                    let dst_b = dst_val & 0xFF;

                    // NOTE: src is premultiplied already
                    let src_r = (px >> 16) & 0xFF;
                    let src_g = (px >> 8) & 0xFF;
                    let src_b = px & 0xFF;

                    let r = src_r + (dst_r * inv_a) / 255;
                    let g = src_g + (dst_g * inv_a) / 255;
                    let b = src_b + (dst_b * inv_a) / 255;

                    *dst = 0xFF000000 | (r << 16) | (g << 8) | b;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn raster_draw_cursor_blends_correctly() {
        // White background
        let mut buffer = vec![0xFFFFFFFF; 100];
        let stride_bytes = 10 * 4; // 10 pixels stride
        let w = 10;
        let h = 10;

        // Simple 1x1 red sprite with 50% alpha
        // Alpha = 0x80 (128)
        // Color = Red (0xFF0000)
        // Premultiplied: R=128, G=0, B=0
        // Pixel = 0x80800000
        let sprite_data = vec![0x80800000];
        let sprite = Bitmap {
            width: 1,
            height: 1,
            data: sprite_data,
        };

        raster_draw_cursor(buffer.as_mut_ptr(), stride_bytes, w, h, (5, 5), &sprite, (0, 0));

        let idx = 5 * 10 + 5; // Stride is 10 pixels (40 bytes)
        let px = buffer[idx as usize];
        let r = (px >> 16) & 0xFF;
        let g = (px >> 8) & 0xFF;
        let b = px & 0xFF;

        assert_eq!(r, 255);
        assert!(g >= 126 && g <= 128);
        assert!(b >= 126 && b <= 128);
    }
}
