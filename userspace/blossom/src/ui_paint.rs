//! Blossom paint boundary.
//!
//! This module owns rasterization and composition primitives so Bloom never
//! grows paint logic again.

use abi::schema::ui_snapshot;
use stem::thing::ThingId;

use crate::compose::blit;
use crate::model::TextRunModel;
use crate::raster_svg::raster_placeholder;
use crate::raster_text::{draw_text, TextStyle};
use crate::surface::MappedSurface;

pub fn raster_svg(surface: &mut MappedSurface, asset_id: ThingId) {
    crate::raster_svg::raster_svg(surface, asset_id);
}

pub fn raster_text_runs(surface: &mut MappedSurface, runs: &[TextRunModel], width: u32, height: u32) {
    for run in runs {
        let style = TextStyle {
            color: run.color,
            size_px: run.size_px,
        };
        let scale = (style.size_px / 8).max(1) as i32;
        let mut x = run.x;
        let mut y = run.y;
        let text_width = (run.text.len() as i32) * (6 * scale);
        let text_height = 7 * scale;
        if run.center_x {
            x = (width as i32 - text_width) / 2;
        }
        if run.center_y {
            y = (height as i32 - text_height) / 2;
        }
        draw_text(surface, &run.text, x, y, &style);
    }
}

pub fn compose(
    src: &MappedSurface,
    dst: &mut MappedSurface,
    sx: i32,
    sy: i32,
    sw: i32,
    sh: i32,
    dx: i32,
    dy: i32,
) {
    blit(src, dst, sx, sy, sw, sh, dx, dy);
}

pub fn snapshot_format() -> u64 {
    ui_snapshot::PIXEL_FORMAT_RGBA8888
}
