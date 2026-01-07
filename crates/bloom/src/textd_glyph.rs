//! textd glyph cache integration for Bloom
//!
//! Reads GlyphCache Things from textd, maps atlas bytespaces,
//! and provides glyph blitting with A8 coverage masks.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use abi::ids::{SymbolId, ThingId};
use thing_std::graph::{symbol_intern, thing_find, thing_get_body, relationships_from};
use thing_std::watch::watch_create;
use abi::types::WatchKind;
use abi::ids::WatchId;
use thing_std::memory::space_map;
use models::{FontFace, GlyphCache, Thing};

/// Glyph index entry format (matches textd's GlyphIndexEntry)
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct GlyphIndexEntry {
    pub atlas_offset: u32,
    pub width: u16,
    pub height: u16,
    pub advance_width: i16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub _pad: u16,
}

impl GlyphIndexEntry {
    /// Size in bytes of one entry
    pub const SIZE: usize = core::mem::size_of::<Self>();
    
    /// Parse from a byte slice at offset
    pub fn from_bytes(data: &[u8], index: usize) -> Option<Self> {
        let offset = index * Self::SIZE;
        if offset + Self::SIZE > data.len() {
            return None;
        }
        let slice = &data[offset..offset + Self::SIZE];
        Some(Self {
            atlas_offset: u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]),
            width: u16::from_le_bytes([slice[4], slice[5]]),
            height: u16::from_le_bytes([slice[6], slice[7]]),
            advance_width: i16::from_le_bytes([slice[8], slice[9]]),
            bearing_x: i16::from_le_bytes([slice[10], slice[11]]),
            bearing_y: i16::from_le_bytes([slice[12], slice[13]]),
            _pad: u16::from_le_bytes([slice[14], slice[15]]),
        })
    }
}

/// Mapped glyph cache with atlas and index data
pub struct MappedGlyphCache {
    pub face_id: ThingId,
    pub px_size: u16,
    pub ascent: i16,
    pub descent: i16,
    atlas_ptr: *const u8,
    atlas_len: usize,
    index_ptr: *const u8,
    index_len: usize,
}

// SAFETY: The pointers are to mapped memory that outlives the cache
unsafe impl Send for MappedGlyphCache {}
unsafe impl Sync for MappedGlyphCache {}

impl MappedGlyphCache {
    /// Get glyph entry for a character
    pub fn get_glyph(&self, ch: char) -> Option<GlyphIndexEntry> {
        let codepoint = ch as usize;
        if codepoint >= 128 {
            return None; // Only ASCII cached for now
        }
        let index_data = unsafe { 
            core::slice::from_raw_parts(self.index_ptr, self.index_len) 
        };
        GlyphIndexEntry::from_bytes(index_data, codepoint)
    }
    
    /// Get atlas bitmap slice for a glyph
    pub fn get_glyph_bitmap(&self, entry: &GlyphIndexEntry) -> Option<&[u8]> {
        let start = entry.atlas_offset as usize;
        let len = (entry.width as usize) * (entry.height as usize);
        if start + len > self.atlas_len {
            return None;
        }
        let atlas_data = unsafe { core::slice::from_raw_parts(self.atlas_ptr, self.atlas_len) };
        Some(&atlas_data[start..start + len])
    }
}

/// Cache of mapped glyph caches by (font_name, px_size)
pub struct TextdGlyphCaches {
    /// (face_id, px_size) -> MappedGlyphCache
    caches: BTreeMap<(u64, u16), MappedGlyphCache>,
    /// Default font face ID
    default_face: Option<ThingId>,
    /// Next mapping virtual address
    next_map_addr: u64,
    /// Whether textd is available
    pub textd_available: bool,
    /// Watch ID for fonts graph (0 = not registered)
    fonts_watch_id: Option<WatchId>,
    /// Fonts graph Thing ID
    fonts_graph: Option<ThingId>,
}

impl TextdGlyphCaches {
    pub const fn new() -> Self {
        Self {
            caches: BTreeMap::new(),
            default_face: None,
            next_map_addr: 0x6800_0000,
            textd_available: false,
            fonts_watch_id: None,
            fonts_graph: None,
        }
    }
    
    /// Register a watch on graph.text.fonts to get notified when fonts become available
    pub fn register_fonts_watch(&mut self) -> bool {
        if self.fonts_watch_id.is_some() {
            return true; // Already registered
        }
        
        // Find or wait for graph.text.fonts
        let fonts_graph = match thing_find("graph.text.fonts") {
            Some(g) => g,
            None => return false,
        };
        
        self.fonts_graph = Some(fonts_graph);
        
        // Register watch
        match watch_create(WatchKind::GraphMembership, fonts_graph) {
            Ok(watch_id) => {
                self.fonts_watch_id = Some(watch_id);
                thing_std::log_info(&alloc::format!(
                    "BLOOM: Registered fonts watch id={}", watch_id.0
                ));
                true
            }
            Err(_) => false,
        }
    }
    
    /// Called when the fonts watch triggers - re-check for available fonts
    pub fn on_fonts_watch_triggered(&mut self) {
        // Force re-check even if already marked as available (new fonts may have been added)
        self.textd_available = false;
        self.check_textd();
        if self.textd_available {
            thing_std::log_info("BLOOM: textd fonts now available!");
        }
    }
    
    /// Get the fonts watch ID (for matching against wake reasons)
    pub fn fonts_watch_id(&self) -> Option<u64> {
        self.fonts_watch_id.map(|w| w.0)
    }
    
    /// Check if textd has published any fonts
    pub fn check_textd(&mut self) {
        if self.textd_available {
            return; // Already confirmed
        }
        
        // Look for graph.text.fonts
        let fonts_graph = match thing_find("graph.text.fonts") {
            Some(g) => g,
            None => return,
        };
        
        // Look for any FontFamily children
        let pred_contains = symbol_intern("predicate.contains");
        let mut buf = [abi::types::RelationshipRef {
            id: ThingId(0),
            kind: SymbolId(0),
            target: ThingId(0),
        }; 8];
        
        if let Ok((n, _)) = relationships_from(fonts_graph, 0, &mut buf) {
            for i in 0..(n as usize) {
                if buf[i].kind == pred_contains {
                    if let Some((body, _)) = thing_get_body(buf[i].target) {
                        if FontFace::decode_full(&body).is_ok() {
                            self.textd_available = true;
                            if self.default_face.is_none() {
                                self.default_face = Some(buf[i].target);
                            }
                            return;
                        }
                    }
                }
            }
        }
    }
    
    /// Get or create a mapped glyph cache for a font at a size
    pub fn get_cache(&mut self, face_id: ThingId, px_size: u16) -> Option<&MappedGlyphCache> {
        let key = (face_id.low(), px_size);
        
        if self.caches.contains_key(&key) {
            return self.caches.get(&key);
        }
        
        // Try to find and map the GlyphCache from graph
        let cache_name = alloc::format!("glyphcache.{}.{}", face_id.low(), px_size);
        let cache_id = thing_find(&cache_name)?;
        
        let (body, _) = thing_get_body(cache_id)?;
        let cache_thing = GlyphCache::decode_full(&body).ok()?;
        
        // Map atlas bytespace
        let atlas_addr = self.next_map_addr;
        self.next_map_addr += 0x10000;
        let atlas_mapped = space_map(cache_thing.atlas_bytespace, atlas_addr, 0, 0x10000);
        if atlas_mapped == 0 {
            return None;
        }
        
        // Map index bytespace
        let index_addr = self.next_map_addr;
        self.next_map_addr += 0x10000;
        let index_mapped = space_map(cache_thing.index_bytespace, index_addr, 0, 0x10000);
        if index_mapped == 0 {
            return None;
        }
        
        let mapped_cache = MappedGlyphCache {
            face_id,
            px_size,
            ascent: cache_thing.ascent,
            descent: cache_thing.descent,
            atlas_ptr: atlas_mapped as *const u8,
            atlas_len: 0x10000,
            index_ptr: index_mapped as *const u8,
            index_len: 128 * GlyphIndexEntry::SIZE, // 128 ASCII entries
        };
        
        self.caches.insert(key, mapped_cache);
        self.caches.get(&key)
    }
    
    /// Get default font face
    pub fn default_face(&self) -> Option<ThingId> {
        self.default_face
    }
}

/// Blend an A8 coverage value with a color onto a destination pixel
#[inline]
pub fn blend_a8_pixel(dst: u32, coverage: u8, color: u32) -> u32 {
    if coverage == 0 {
        return dst;
    }
    if coverage == 255 {
        return color;
    }
    
    let alpha = coverage as u32;
    let inv_alpha = 255 - alpha;
    
    let dst_r = (dst >> 16) & 0xFF;
    let dst_g = (dst >> 8) & 0xFF;
    let dst_b = dst & 0xFF;
    
    let src_r = (color >> 16) & 0xFF;
    let src_g = (color >> 8) & 0xFF;
    let src_b = color & 0xFF;
    
    let r = (src_r * alpha + dst_r * inv_alpha) / 255;
    let g = (src_g * alpha + dst_g * inv_alpha) / 255;
    let b = (src_b * alpha + dst_b * inv_alpha) / 255;
    
    0xFF000000 | (r << 16) | (g << 8) | b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_glyph_index_entry_size() {
        assert_eq!(GlyphIndexEntry::SIZE, 16);
    }
    
    #[test]
    fn test_glyph_index_entry_from_bytes() {
        let data = [
            0x10, 0x00, 0x00, 0x00,  // atlas_offset = 0x10
            0x08, 0x00,              // width = 8
            0x10, 0x00,              // height = 16
            0x09, 0x00,              // advance_width = 9
            0x01, 0x00,              // bearing_x = 1
            0x0E, 0x00,              // bearing_y = 14
            0x00, 0x00,              // _pad
        ];
        
        let entry = GlyphIndexEntry::from_bytes(&data, 0).unwrap();
        assert_eq!(entry.atlas_offset, 0x10);
        assert_eq!(entry.width, 8);
        assert_eq!(entry.height, 16);
        assert_eq!(entry.advance_width, 9);
        assert_eq!(entry.bearing_x, 1);
        assert_eq!(entry.bearing_y, 14);
    }
    
    #[test]
    fn test_blend_a8_full_coverage() {
        let dst = 0xFF000000;
        let color = 0xFFFF0000;
        assert_eq!(blend_a8_pixel(dst, 255, color), color);
    }
    
    #[test]
    fn test_blend_a8_zero_coverage() {
        let dst = 0xFF00FF00;
        let color = 0xFFFF0000;
        assert_eq!(blend_a8_pixel(dst, 0, color), dst);
    }
    
    #[test]
    fn test_blend_a8_half_coverage() {
        let dst = 0xFF000000;  // black
        let color = 0xFFFF0000; // red
        let result = blend_a8_pixel(dst, 128, color);
        
        // Should be approximately half red
        let r = (result >> 16) & 0xFF;
        assert!(r > 100 && r < 150, "Expected ~127, got {}", r);
    }
}
