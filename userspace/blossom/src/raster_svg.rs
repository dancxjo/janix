//! Placeholder SVG rasterizer.
//!
//! The current implementation paints a deterministic placeholder based on the
//! asset bytespace id. This keeps progressive rendering behavior deterministic
//! while the full SVG pipeline is wired in.

use crate::surface::MappedSurface;

/// Fill the surface with a placeholder color derived from the asset id.
pub fn raster_placeholder(surface: &mut MappedSurface, asset_id: u64) {
    let color = color_from_id(asset_id);
    surface.clear(color);

    // Draw a simple "X" pattern to make missing SVGs obvious.
    let spec = surface.spec();
    let w = spec.width as i32;
    let h = spec.height as i32;
    let diag_len = w.min(h);
    for i in 0..diag_len {
        surface.put_px(i, i, 0xFFFFFFFF);
        surface.put_px(w - 1 - i, i, 0xFFFFFFFF);
    }
}

fn color_from_id(id: u64) -> u32 {
    let hash = id ^ (id >> 33) ^ (id << 11);
    let r = ((hash >> 16) & 0xFF) as u32;
    let g = ((hash >> 8) & 0xFF) as u32;
    let b = (hash & 0xFF) as u32;
    0xFF00_0000 | (r << 16) | (g << 8) | b
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::surface::{MappedSurface, SurfaceSpec};

    #[test]
    fn placeholder_varies_with_id() {
        let mut buf = [0u8; 16];
        let spec = SurfaceSpec { width: 2, height: 2, stride_bytes: 8 };
        let mut surface = unsafe { MappedSurface::from_parts(buf.as_mut_ptr(), buf.len(), spec) };
        raster_placeholder(&mut surface, 42);
        let c1 = surface.get_px(0, 0);
        raster_placeholder(&mut surface, 43);
        let c2 = surface.get_px(0, 0);
        assert_ne!(c1, c2);
    }
}
