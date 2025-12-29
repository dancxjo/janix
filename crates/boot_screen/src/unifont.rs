const FONT_DATA: &[u8] = include_bytes!("../../../assets/fonts/unifont.hex");

static mut ASCII_CACHE: [Option<[u8; 16]>; 128] = [None; 128];
static mut ASCII_CACHE_VALID: [bool; 128] = [false; 128];

pub fn get_glyph(ch: char) -> Option<[u8; 16]> {
    let cp = ch as u32;
    if cp < 128 {
        unsafe {
            if ASCII_CACHE_VALID[cp as usize] {
                return ASCII_CACHE[cp as usize];
            }
            let g = get_glyph_slow(ch);
            ASCII_CACHE[cp as usize] = g;
            ASCII_CACHE_VALID[cp as usize] = true;
            return g;
        }
    }
    get_glyph_slow(ch)
}

fn get_glyph_slow(ch: char) -> Option<[u8; 16]> {
    let target = ch as u32;
    let data = FONT_DATA;

    let mut left = 0;
    let mut right = data.len();

    while left < right {
        let mid = left + (right - left) / 2;

        // Align to start of line (scan backwards)
        let mut line_start = mid;
        while line_start > 0 && data[line_start - 1] != b'\n' {
            line_start -= 1;
        }

        if line_start >= right {
            break;
        }

        let line = &data[line_start..];

        // Parse CodePoint:
        let mut colon_idx = 0;
        while colon_idx < 8 && colon_idx < line.len() && line[colon_idx] != b':' {
            colon_idx += 1;
        }

        if colon_idx >= line.len() || line[colon_idx] != b':' {
            // Should not happen on valid lines
            break;
        }

        let cp_bytes = &line[..colon_idx];
        let cp = parse_hex_bytes(cp_bytes)?;

        if cp == target {
            // Found
            let hex_start = colon_idx + 1;
            let mut hex_end = hex_start;
            while hex_end < line.len() && line[hex_end] != b'\n' {
                hex_end += 1;
            }

            let hex_len = hex_end - hex_start;
            if hex_len == 32 {
                return parse_bitmap_32(&line[hex_start..hex_end]);
            } else {
                // Wide glyph or unknown format
                return None;
            }
        } else if cp < target {
            // Target is after this line.
            // Move left to start of next line.
            let mut next_line = line_start + hex_end_offset(line);
            // Ensure we advance
            if next_line == line_start {
                next_line += 1;
            } // Should imply \n was processed
            left = next_line;
        } else {
            // Target is before this line.
            right = line_start;
        }
    }

    None
}

fn hex_end_offset(line: &[u8]) -> usize {
    let mut i = 0;
    while i < line.len() && line[i] != b'\n' {
        i += 1;
    }
    if i < line.len() {
        i + 1
    } else {
        i
    }
}

fn parse_hex_bytes(bytes: &[u8]) -> Option<u32> {
    let mut v = 0u32;
    for &b in bytes {
        let d = match b {
            b'0'..=b'9' => b - b'0',
            b'A'..=b'F' => b - b'A' + 10,
            b'a'..=b'f' => b - b'a' + 10,
            _ => return None,
        };
        v = (v << 4) | (d as u32);
    }
    Some(v)
}

fn parse_bitmap_32(bytes: &[u8]) -> Option<[u8; 16]> {
    let mut out = [0u8; 16];
    if bytes.len() < 32 {
        return None;
    }

    for i in 0..16 {
        let hi_c = bytes[i * 2];
        let lo_c = bytes[i * 2 + 1];

        let hi = match hi_c {
            b'0'..=b'9' => hi_c - b'0',
            b'A'..=b'F' => hi_c - b'A' + 10,
            b'a'..=b'f' => hi_c - b'a' + 10,
            _ => return None,
        };
        let lo = match lo_c {
            b'0'..=b'9' => lo_c - b'0',
            b'A'..=b'F' => lo_c - b'A' + 10,
            b'a'..=b'f' => lo_c - b'a' + 10,
            _ => return None,
        };
        out[i] = (hi << 4) | lo;
    }
    Some(out)
}
