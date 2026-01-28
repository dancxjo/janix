//! Cursor Rasterizer - Owns cursor snapshot caching and pre-composition.
//!
//! This module is the single authority for cursor image processing.
//! It pre-composites the cursor with shadow layers once per asset change,
//! and the compositor blends this cached snapshot at the final stage.
//!
//! # Architecture
//!
//! - **Input** produces logical cursor state only (position, buttons)
//! - **CursorRasterizer** owns cursor image caching and pre-composites cursor + shadow once
//! - **Compositor** blends pre-cached cursor snapshot at final stage, outside window damage

extern crate alloc;

use alloc::sync::Arc;
use alloc::vec;

use crate::asset::{CursorAsset, Image};
use crate::frame::AssetGeneration;

/// Cached cursor snapshot with pre-composited shadow.
///
/// This snapshot is created once when the cursor asset changes,
/// and reused for all subsequent frames until the asset changes again.
#[derive(Clone, Debug)]
pub struct CursorSnapshot {
    /// Pre-composited cursor image (main + shadow layers)
    pub image: Image,
    /// Hotspot X offset from top-left of the snapshot
    pub hotspot_x: i32,
    /// Hotspot Y offset from top-left of the snapshot
    pub hotspot_y: i32,
    /// Generation when this snapshot was created
    pub gen: AssetGeneration,
}

/// Owns cursor rasterization and caching.
///
/// The rasterizer holds a cached snapshot that is only regenerated
/// when the source asset's generation changes.
pub struct CursorRasterizer {
    /// Current cached snapshot (None = not rasterized yet)
    snapshot: Option<CursorSnapshot>,
    /// Generation of source asset used to create snapshot
    source_gen: AssetGeneration,
}

impl CursorRasterizer {
    /// Create a new cursor rasterizer with empty cache.
    pub fn new() -> Self {
        Self {
            snapshot: None,
            source_gen: AssetGeneration::ZERO,
        }
    }

    /// Update cache if asset changed; return current snapshot.
    ///
    /// This method is idempotent: calling it multiple times with the same
    /// asset generation returns the cached snapshot without re-rasterizing.
    pub fn get_snapshot(&mut self, asset: &CursorAsset) -> Option<&CursorSnapshot> {
        let asset_gen = asset.generation();

        // Only rasterize if asset changed or no snapshot exists
        if self.source_gen != asset_gen || self.snapshot.is_none() {
            stem::info!(
                "[cursor_rasterizer] rasterizing cursor snapshot gen={}",
                asset_gen.0
            );
            self.snapshot = Some(self.rasterize_cursor(asset));
            self.source_gen = asset_gen;
        }

        self.snapshot.as_ref()
    }

    /// Check if we have a cached snapshot without triggering rasterization.
    pub fn has_snapshot(&self) -> bool {
        self.snapshot.is_some()
    }

    /// Pre-composite cursor with shadow layers into single image.
    ///
    /// Creates a "Windows 2000 style" cursor with 3-layer soft shadow:
    /// - Layer 1: Offset (1,1), alpha 48 (~19%)
    /// - Layer 2: Offset (2,2), alpha 48 (~19%)
    /// - Layer 3: Offset (3,3), alpha 24 (~9%)
    /// - Main: Offset (0,0), full alpha
    fn rasterize_cursor(&self, asset: &CursorAsset) -> CursorSnapshot {
        let frame = match asset {
            CursorAsset::Static(f) => f,
            CursorAsset::Animated { frames } => {
                // For animated cursors, use first frame for now
                // TODO: Animation support via frame timing
                frames.first().expect("animated cursor has no frames")
            }
        };

        // Allocate buffer with padding for shadow offsets (3px on right and bottom)
        let padding = 3;
        let w = frame.image.width as i32 + padding;
        let h = frame.image.height as i32 + padding;
        let mut pixels = vec![0u32; (w * h) as usize];

        // Composite shadow layers (Windows 2000 style)
        Self::blit_with_alpha(&mut pixels, w, &frame.image, 1, 1, 48); // ~19%
        Self::blit_with_alpha(&mut pixels, w, &frame.image, 2, 2, 48); // ~19%
        Self::blit_with_alpha(&mut pixels, w, &frame.image, 3, 3, 24); // ~9%

        // Main cursor (no alpha modulation)
        Self::blit_with_alpha(&mut pixels, w, &frame.image, 0, 0, 255);

        CursorSnapshot {
            image: Image {
                width: w as u32,
                height: h as u32,
                pixels: Arc::from(pixels.into_boxed_slice()),
                gen: asset.generation(),
            },
            hotspot_x: frame.hotspot_x as i32,
            hotspot_y: frame.hotspot_y as i32,
            gen: asset.generation(),
        }
    }

    /// Alpha-blend a source image into a pixel buffer with constant alpha modulation.
    ///
    /// Uses SrcOver blending: out = src * src_alpha + dst * (1 - src_alpha)
    fn blit_with_alpha(
        dst: &mut [u32],
        dst_stride: i32,
        src: &Image,
        ox: i32,
        oy: i32,
        const_alpha: u8,
    ) {
        for sy in 0..src.height as i32 {
            for sx in 0..src.width as i32 {
                let dx = ox + sx;
                let dy = oy + sy;

                if dx < 0 || dy < 0 || dx >= dst_stride || dy * dst_stride + dx >= dst.len() as i32
                {
                    continue;
                }

                let src_px = src.pixels[(sy as usize) * (src.width as usize) + (sx as usize)];
                let sa = ((src_px >> 24) & 0xFF) as u32;

                if sa == 0 {
                    continue;
                }

                // Modulate source alpha by const_alpha
                let modulated_a = (sa * const_alpha as u32) / 255;
                if modulated_a == 0 {
                    continue;
                }

                let dst_idx = (dy * dst_stride + dx) as usize;
                let dst_px = dst[dst_idx];

                // Extract components
                let sr = ((src_px >> 16) & 0xFF) as u8;
                let sg = ((src_px >> 8) & 0xFF) as u8;
                let sb = (src_px & 0xFF) as u8;

                let da = ((dst_px >> 24) & 0xFF) as u8;
                let dr = ((dst_px >> 16) & 0xFF) as u8;
                let dg = ((dst_px >> 8) & 0xFF) as u8;
                let db = (dst_px & 0xFF) as u8;

                // SrcOver blending
                let ma = modulated_a as u8;
                let out_a = ma as u32 + ((da as u32 * (255 - ma as u32)) >> 8);
                let out_r = Self::blend_ch(sr, dr, ma);
                let out_g = Self::blend_ch(sg, dg, ma);
                let out_b = Self::blend_ch(sb, db, ma);

                dst[dst_idx] =
                    (out_a << 24) | ((out_r as u32) << 16) | ((out_g as u32) << 8) | (out_b as u32);
            }
        }
    }

    /// Blend a single channel using SrcOver: s * sa + d * (1 - sa)
    #[inline(always)]
    fn blend_ch(s: u8, d: u8, sa: u8) -> u8 {
        if sa == 255 {
            return s;
        }
        if sa == 0 {
            return d;
        }
        (Self::scale_ch(s, sa) + Self::scale_ch(d, 255 - sa)) as u8
    }

    /// Scale a channel by alpha: (c * a) / 255
    #[inline(always)]
    fn scale_ch(c: u8, a: u8) -> u32 {
        ((c as u32 * a as u32) + 128) / 255
    }
}

impl Default for CursorRasterizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset::CursorFrame;
    use alloc::sync::Arc;

    fn make_test_asset() -> CursorAsset {
        let pixels: Vec<u32> = (0..16 * 16).map(|_| 0xFFFFFFFF).collect();
        CursorAsset::Static(CursorFrame {
            image: Image {
                width: 16,
                height: 16,
                pixels: Arc::from(pixels.into_boxed_slice()),
                gen: AssetGeneration(1),
            },
            delay_ms: 0,
            hotspot_x: 0,
            hotspot_y: 0,
        })
    }

    #[test]
    fn cursor_snapshot_cached_across_calls() {
        let asset = make_test_asset();
        let mut rasterizer = CursorRasterizer::new();

        // First call creates snapshot
        let snap1 = rasterizer.get_snapshot(&asset).unwrap();
        let gen1 = snap1.gen;

        // Second call returns cached snapshot (same generation)
        let snap2 = rasterizer.get_snapshot(&asset).unwrap();
        let gen2 = snap2.gen;

        assert_eq!(gen1, gen2, "snapshot should be reused");
    }

    #[test]
    fn cursor_snapshot_includes_shadow_padding() {
        let asset = make_test_asset();
        let mut rasterizer = CursorRasterizer::new();

        let snap = rasterizer.get_snapshot(&asset).unwrap();

        // Snapshot should be 3px larger than original (for shadow)
        assert_eq!(snap.image.width, 16 + 3);
        assert_eq!(snap.image.height, 16 + 3);
    }
}
