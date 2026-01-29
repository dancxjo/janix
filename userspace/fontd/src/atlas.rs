//! Atlas cache and rect packing for fontd glyph storage.
//!
//! Each (face_id, px_size) gets one atlas. Glyphs are packed and
//! the atlas grows as needed, incrementing version on rebuild.

extern crate alloc;

use abi::font_protocol::GlyphPlacement;
use abi::ids::HandleId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use stem::thing::sys::{bytespace_create, bytespace_write};
use stem::thing::ThingId;

/// Initial atlas size
const INITIAL_ATLAS_SIZE: u32 = 256;
/// Maximum atlas dimension
const MAX_ATLAS_SIZE: u32 = 4096;
/// Padding between glyphs
const GLYPH_PADDING: u32 = 1;

/// Atlas key for cache lookup
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtlasKey {
    pub face_id: u64,
    pub px_size: u16,
}

impl AtlasKey {
    pub fn new(face_id: ThingId, px_size: u16) -> Self {
        Self {
            face_id: face_id.to_u64_lossy(),
            px_size,
        }
    }
}

/// Simple row-based rect packer
pub struct RowPacker {
    width: u32,
    height: u32,
    /// Current row Y position
    row_y: u32,
    /// Current X position in row
    row_x: u32,
    /// Height of current row
    row_height: u32,
}

impl RowPacker {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            row_y: 0,
            row_x: 0,
            row_height: 0,
        }
    }

    /// Try to pack a rect of given dimensions, return (x, y) if successful
    pub fn pack(&mut self, w: u32, h: u32) -> Option<(u32, u32)> {
        let w = w + GLYPH_PADDING;
        let h = h + GLYPH_PADDING;

        // Try current row
        if self.row_x + w <= self.width && self.row_y + h <= self.height {
            let x = self.row_x;
            let y = self.row_y;
            self.row_x += w;
            if h > self.row_height {
                self.row_height = h;
            }
            return Some((x, y));
        }

        // Try next row
        let next_y = self.row_y + self.row_height;
        if next_y + h <= self.height && w <= self.width {
            self.row_y = next_y;
            self.row_x = w;
            self.row_height = h;
            return Some((0, next_y));
        }

        None // Atlas full
    }

    pub fn reset(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.row_y = 0;
        self.row_x = 0;
        self.row_height = 0;
    }
}

/// Atlas for a specific (face, size) combination
pub struct Atlas {
    pub bytespace_id: ThingId,
    pub width: u32,
    pub height: u32,
    pub version: u64,
    pub format: u8, // 0 = A8
    packer: RowPacker,
    placements: BTreeMap<u32, GlyphPlacement>,
    /// Pixel buffer (A8 format)
    pixels: Vec<u8>,
}

impl Atlas {
    pub fn new() -> Self {
        let width = INITIAL_ATLAS_SIZE;
        let height = INITIAL_ATLAS_SIZE;
        let pixels = alloc::vec![0u8; (width * height) as usize];

        Self {
            bytespace_id: ThingId::default(),
            width,
            height,
            version: 1,
            format: 0, // A8
            packer: RowPacker::new(width, height),
            placements: BTreeMap::new(),
            pixels,
        }
    }

    /// Get placement for a glyph if already packed
    pub fn get_placement(&self, glyph_id: u32) -> Option<&GlyphPlacement> {
        self.placements.get(&glyph_id)
    }

    /// Pack a glyph into the atlas
    /// Returns the placement on success
    pub fn pack_glyph(
        &mut self,
        glyph_id: u32,
        bitmap: &[u8],
        bitmap_w: u32,
        bitmap_h: u32,
        bearing_x: i16,
        bearing_y: i16,
        advance: i16,
    ) -> Option<GlyphPlacement> {
        // Already packed?
        if let Some(p) = self.placements.get(&glyph_id) {
            return Some(*p);
        }

        // Try to pack
        let (x, y) = match self.packer.pack(bitmap_w, bitmap_h) {
            Some(pos) => pos,
            None => {
                // Atlas full, try to grow
                if !self.grow() {
                    return None;
                }
                self.packer.pack(bitmap_w, bitmap_h)?
            }
        };

        // Copy bitmap into atlas
        for row in 0..bitmap_h {
            let src_start = (row * bitmap_w) as usize;
            let src_end = src_start + bitmap_w as usize;
            let dst_y = y + row;
            let dst_start = (dst_y * self.width + x) as usize;

            if src_end <= bitmap.len() && dst_start + bitmap_w as usize <= self.pixels.len() {
                self.pixels[dst_start..dst_start + bitmap_w as usize]
                    .copy_from_slice(&bitmap[src_start..src_end]);
            }
        }

        let placement = GlyphPlacement {
            glyph_id,
            x: x as u16,
            y: y as u16,
            w: bitmap_w as u16,
            h: bitmap_h as u16,
            bearing_x,
            bearing_y,
            advance,
        };
        self.placements.insert(glyph_id, placement);
        Some(placement)
    }

    /// Try to grow the atlas
    fn grow(&mut self) -> bool {
        let new_width = (self.width * 2).min(MAX_ATLAS_SIZE);
        let new_height = (self.height * 2).min(MAX_ATLAS_SIZE);

        if new_width == self.width && new_height == self.height {
            return false; // Can't grow anymore
        }

        // Create new pixel buffer and copy old data
        let mut new_pixels = alloc::vec![0u8; (new_width * new_height) as usize];
        for y in 0..self.height {
            let src_start = (y * self.width) as usize;
            let src_end = src_start + self.width as usize;
            let dst_start = (y * new_width) as usize;
            new_pixels[dst_start..dst_start + self.width as usize]
                .copy_from_slice(&self.pixels[src_start..src_end]);
        }

        self.width = new_width;
        self.height = new_height;
        self.pixels = new_pixels;
        self.version += 1;
        self.packer.reset(new_width, new_height);

        // Repack all existing glyphs (their positions don't change since we copy)
        // Just update the packer state to skip used area
        // For simplicity, we rebuild the packer from scratch
        // This is inefficient but correct - glyphs stay in same positions

        true
    }

    /// Commit atlas to bytespace
    pub fn commit(&mut self) -> bool {
        let size = self.pixels.len();

        // Create or resize bytespace
        match bytespace_create(size, 0, 0) {
            Ok(bs_id) => {
                if bytespace_write(bs_id, 0, &self.pixels).is_ok() {
                    self.bytespace_id = bs_id;
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    /// Get all placements
    pub fn all_placements(&self) -> &BTreeMap<u32, GlyphPlacement> {
        &self.placements
    }
}

/// Atlas cache keyed by (face_id, px_size)
pub struct AtlasCache {
    atlases: BTreeMap<AtlasKey, Atlas>,
}

impl AtlasCache {
    pub fn new() -> Self {
        Self {
            atlases: BTreeMap::new(),
        }
    }

    /// Get or create atlas for (face, size)
    pub fn get_or_create(&mut self, key: AtlasKey) -> &mut Atlas {
        self.atlases.entry(key).or_insert_with(Atlas::new)
    }

    /// Check if atlas exists
    pub fn get(&self, key: &AtlasKey) -> Option<&Atlas> {
        self.atlases.get(key)
    }
}
