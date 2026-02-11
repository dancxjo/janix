//! Text rendering using SIMD-accelerated glyph compositing.
//!
//! This module bridges fontd's atlas format with stem's glyph rendering primitives.

use abi::font_protocol::GlyphPlacement as FontdPlacement;
use alloc::vec::Vec;
use stem::simd::text::{GlyphPlacement, GlyphRun, PositionedGlyph, Rect, PHASE_COUNT};

/// Convert a fontd GlyphPlacement to our internal format.
///
/// For v0, we use a simple approach: all phases point to the same atlas rect.
/// This means we don't get true subpixel rendering yet, but the infrastructure
/// is in place. Future versions can generate phase-shifted masks in the atlas.
pub fn convert_placement(fp: &FontdPlacement) -> GlyphPlacement {
    let rect = Rect::new(fp.x as i32, fp.y as i32, fp.w as i32, fp.h as i32);

    GlyphPlacement {
        glyph_id: fp.glyph_id,
        // For now, all phases use the same rect
        // TODO: Generate phase-shifted variants in atlas
        phase_rects: [rect; PHASE_COUNT],
        bearing_x: fp.bearing_x,
        bearing_y: fp.bearing_y,
        advance: fp.advance,
    }
}

/// Convert a list of fontd placements to internal format.
pub fn convert_placements(fps: &[FontdPlacement]) -> Vec<GlyphPlacement> {
    fps.iter().map(convert_placement).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_placement() {
        let fp = FontdPlacement {
            glyph_id: 42,
            x: 10,
            y: 20,
            w: 8,
            h: 12,
            bearing_x: 1,
            bearing_y: 10,
            advance: 9,
        };

        let p = convert_placement(&fp);

        assert_eq!(p.glyph_id, 42);
        assert_eq!(p.bearing_x, 1);
        assert_eq!(p.bearing_y, 10);
        assert_eq!(p.advance, 9);

        // All phases should have the same rect for v0
        for phase in 0..PHASE_COUNT {
            let r = p.phase_rects[phase];
            assert_eq!(r.x, 10);
            assert_eq!(r.y, 20);
            assert_eq!(r.w, 8);
            assert_eq!(r.h, 12);
        }
    }
}
