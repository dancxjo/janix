
const FONT_DATA: &str = include_str!("../../../assets/fonts/unifont.hex");

pub fn get_glyph(ch: char) -> Option<[u8; 16]> {
    let target = ch as u32;
    let mut rest = FONT_DATA;

    while !rest.is_empty() {
        // Parse 4-digit hex at start
        if rest.len() < 5 { break; } // XXXX:

        // Safety check for format
        let bytes = rest.as_bytes();
        if bytes.len() > 4 && bytes[4] == b':' {
            if let Some(val) = parse_hex_4(bytes) {
                if val == target {
                    // Match! Parse bitmap.
                    // The bitmap follows ':'.
                    // Width check? "Duospaced".
                    // 8x16 = 32 hex chars.
                    // 16x16 = 64 hex chars.
                    // We only support 8x16 (32 hex chars) for this boot screen logic?
                    // Or we can try to render wide?
                    // BootScreen assumes 8x16. If we get 64 hex chars, we might fail or clip.
                    // For now, assume we want [u8; 16].
                    // If the line is 32 chars long, we parse.
                    // Check length to newline.
                    let end_of_line = rest.find('\n').unwrap_or(rest.len());
                    let hex_len = end_of_line - 5; // skip XXXX:
                    if hex_len == 32 {
                        return parse_bitmap_32(&rest[5..5+32]);
                    } else {
                        // Ignore wide glyphs or wrong format
                        return None;
                    }
                } else if val > target {
                    // Sorted file: target not found
                    return None;
                }
            }
        }

        // Next line
        if let Some(idx) = rest.find('\n') {
             rest = &rest[idx+1..];
        } else {
             break;
        }
    }
    None
}

fn parse_hex_4(bytes: &[u8]) -> Option<u32> {
    let mut v = 0u32;
    for i in 0..4 {
        let b = bytes[i];
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

fn parse_bitmap_32(hex: &str) -> Option<[u8; 16]> {
    let mut out = [0u8; 16];
    let bytes = hex.as_bytes();
    if bytes.len() < 32 { return None; }

    for i in 0..16 {
        let hi_c = bytes[i*2];
        let lo_c = bytes[i*2+1];

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
