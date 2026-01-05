//! Text rendering for Bloom compositor.
//!
//! Uses embedded Unifont (bitmap font) with glyph caching for speed.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use spin::Once;
use crate::painter::Painter;

// Embed unifont.hex (8MB bitmap font covering most Unicode)
static UNIFONT_HEX: &[u8] = include_bytes!("../../../assets/fonts/unifont.hex");

const UNIFONT_GLYPH_HEIGHT: usize = 16;
const UNIFONT_GLYPH_MAX_BYTES: usize = 32;

/// A parsed unifont glyph (8 or 16 pixels wide, 16 pixels tall)
#[derive(Clone, Copy)]
struct GlyphBuffer {
    bytes: [u8; UNIFONT_GLYPH_MAX_BYTES],
    bytes_used: usize,
    bytes_per_row: usize,
}

impl GlyphBuffer {
    const fn new() -> Self {
        Self {
            bytes: [0; UNIFONT_GLYPH_MAX_BYTES],
            bytes_used: 0,
            bytes_per_row: 0,
        }
    }

    fn width(&self) -> usize {
        self.bytes_per_row * 8
    }
}

/// Global glyph cache - parsed once, used forever
struct GlyphCache {
    glyphs: UnsafeCell<BTreeMap<char, GlyphBuffer>>,
}

unsafe impl Sync for GlyphCache {}

static GLYPH_CACHE: GlyphCache = GlyphCache {
    glyphs: UnsafeCell::new(BTreeMap::new()),
};

static CACHE_INIT: Once<()> = Once::new();

fn parse_hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn parse_hex_pair(hi: u8, lo: u8) -> Option<u8> {
    Some(parse_hex_value(hi)? << 4 | parse_hex_value(lo)?)
}

fn parse_codepoint_hex(hex: &[u8]) -> Option<u32> {
    let mut value = 0u32;
    for &digit in hex {
        value = value.checked_mul(16)? + parse_hex_value(digit)? as u32;
    }
    Some(value)
}

fn parse_glyph_hex(glyph_hex: &[u8]) -> Option<GlyphBuffer> {
    if glyph_hex.len() % 2 != 0 {
        return None;
    }

    let glyph_bytes = glyph_hex.len() / 2;
    if glyph_bytes == 0
        || glyph_bytes > UNIFONT_GLYPH_MAX_BYTES
        || glyph_bytes % UNIFONT_GLYPH_HEIGHT != 0
    {
        return None;
    }

    let mut glyph = GlyphBuffer::new();
    for idx in 0..glyph_bytes {
        let byte = parse_hex_pair(glyph_hex[idx * 2], glyph_hex[idx * 2 + 1])?;
        glyph.bytes[idx] = byte;
    }
    glyph.bytes_used = glyph_bytes;
    glyph.bytes_per_row = glyph_bytes / UNIFONT_GLYPH_HEIGHT;
    Some(glyph)
}

/// Ensure glyphs for requested characters are in cache
fn ensure_glyphs_cached(text: &str) {
    let cache = unsafe { &mut *GLYPH_CACHE.glyphs.get() };
    
    // Find which characters we need to load
    let mut needed: BTreeSet<char> = BTreeSet::new();
    for ch in text.chars() {
        if !cache.contains_key(&ch) {
            needed.insert(ch);
        }
    }
    
    // Always need fallbacks
    if !cache.contains_key(&'\u{FFFD}') {
        needed.insert('\u{FFFD}');
    }
    if !cache.contains_key(&'?') {
        needed.insert('?');
    }
    
    if needed.is_empty() {
        return;
    }

    // Parse the hex file for needed glyphs
    let mut remaining = needed.len();
    let mut line_start = 0;

    while line_start < UNIFONT_HEX.len() && remaining > 0 {
        let mut line_end = line_start;
        while line_end < UNIFONT_HEX.len() && UNIFONT_HEX[line_end] != b'\n' {
            line_end += 1;
        }
        let line = &UNIFONT_HEX[line_start..line_end];
        line_start = line_end.saturating_add(1);

        let Some(colon) = line.iter().position(|&b| b == b':') else {
            continue;
        };

        let Some(codepoint) = parse_codepoint_hex(&line[..colon]) else {
            continue;
        };

        let Some(ch) = core::char::from_u32(codepoint) else {
            continue;
        };

        if !needed.contains(&ch) || cache.contains_key(&ch) {
            continue;
        }

        if let Some(glyph) = parse_glyph_hex(&line[colon + 1..]) {
            cache.insert(ch, glyph);
            remaining = remaining.saturating_sub(1);
        }
    }
}

fn get_cached_glyph(ch: char) -> Option<GlyphBuffer> {
    let cache = unsafe { &*GLYPH_CACHE.glyphs.get() };
    cache.get(&ch)
        .or_else(|| cache.get(&'\u{FFFD}'))
        .or_else(|| cache.get(&'?'))
        .copied()
}

/// Pre-warm the cache with common ASCII characters
pub fn ensure_font_loaded() {
    CACHE_INIT.call_once(|| {
        // Pre-cache ASCII printable range for fast first render
        let ascii: alloc::string::String = (32u8..127u8).map(|b| b as char).collect();
        ensure_glyphs_cached(&ascii);
        thing_std::log_info("BLOOM: unifont ASCII glyphs cached");
    });
}

/// Draw text using the Painter API with embedded Unifont.
pub fn draw_text_on_painter(
    painter: &mut dyn Painter,
    x: i32,
    y: i32,
    text: &str,
    color: u32,
    _size_px: f32, // Size is fixed at 16px for unifont
) {
    // Ensure glyphs are cached
    ensure_glyphs_cached(text);

    let color_a = ((color >> 24) & 0xFF) as u8;
    let color_r = ((color >> 16) & 0xFF) as u8;
    let color_g = ((color >> 8) & 0xFF) as u8;
    let color_b = (color & 0xFF) as u8;

    let mut cursor_x = x;
    let glyph_spacing = 1i32;

    for ch in text.chars() {
        if ch == '\n' {
            break;
        }

        let Some(glyph) = get_cached_glyph(ch) else {
            continue;
        };

        let glyph_w = glyph.width() as i32;
        let glyph_h = UNIFONT_GLYPH_HEIGHT as i32;

        // Build pixel buffer for this glyph
        let mut pixels: Vec<u32> = Vec::with_capacity((glyph_w * glyph_h) as usize);
        
        let mut row_offset = 0;
        for _row in 0..UNIFONT_GLYPH_HEIGHT {
            for byte_idx in 0..glyph.bytes_per_row {
                let byte = glyph.bytes[row_offset + byte_idx];
                for bit in 0..8 {
                    if (byte & (0x80 >> bit)) != 0 {
                        let pixel = ((color_a as u32) << 24)
                            | ((color_r as u32) << 16)
                            | ((color_g as u32) << 8)
                            | (color_b as u32);
                        pixels.push(pixel);
                    } else {
                        pixels.push(0);
                    }
                }
            }
            row_offset += glyph.bytes_per_row;
        }

        painter.blit_rgba_alpha(cursor_x, y, &pixels, glyph_w as u32, glyph_h as u32);
        cursor_x += glyph_w + glyph_spacing;
    }
}

/// Measure the width of text (unifont is fixed 8 or 16px wide per char).
pub fn measure_text_width(text: &str, _size_px: f32) -> i32 {
    ensure_glyphs_cached(text);
    
    let mut width = 0i32;
    let glyph_spacing = 1i32;
    
    for ch in text.chars() {
        if ch == '\n' {
            break;
        }
        if let Some(glyph) = get_cached_glyph(ch) {
            width += glyph.width() as i32 + glyph_spacing;
        }
    }
    width.saturating_sub(glyph_spacing)
}

/// Legacy draw_text function using raw buffer.
pub fn draw_text(
    buffer: *mut u32,
    fb_width: u32,
    fb_height: u32,
    x: i32,
    y: i32,
    text: &str,
    color: u32,
    _size_px: f32,
) {
    ensure_glyphs_cached(text);

    let color_r = ((color >> 16) & 0xFF) as u32;
    let color_g = ((color >> 8) & 0xFF) as u32;
    let color_b = (color & 0xFF) as u32;

    let mut cursor_x = x;
    let glyph_spacing = 1i32;

    for ch in text.chars() {
        if ch == '\n' {
            break;
        }

        let Some(glyph) = get_cached_glyph(ch) else {
            continue;
        };

        let glyph_w = glyph.width() as i32;

        let mut row_offset = 0;
        for row in 0..UNIFONT_GLYPH_HEIGHT {
            let draw_y = y + row as i32;
            if draw_y < 0 || draw_y >= fb_height as i32 {
                row_offset += glyph.bytes_per_row;
                continue;
            }

            for byte_idx in 0..glyph.bytes_per_row {
                let byte = glyph.bytes[row_offset + byte_idx];
                if byte == 0 {
                    continue;
                }

                for bit in 0..8 {
                    if (byte & (0x80 >> bit)) != 0 {
                        let draw_x = cursor_x + (byte_idx * 8 + bit) as i32;
                        if draw_x < 0 || draw_x >= fb_width as i32 {
                            continue;
                        }

                        let idx = draw_y as usize * fb_width as usize + draw_x as usize;
                        unsafe {
                            let pixel = (0xFF << 24) | (color_r << 16) | (color_g << 8) | color_b;
                            *buffer.add(idx) = pixel;
                        }
                    }
                }
            }
            row_offset += glyph.bytes_per_row;
        }

        cursor_x += glyph_w + glyph_spacing;
    }
}
