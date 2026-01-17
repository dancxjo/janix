
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Guess {
    pub mime: &'static str,
    pub extension: &'static str,
    pub confidence: u16,
    pub bytes_needed: usize,
}

pub fn sniff(data: &[u8]) -> Option<Guess> {
    let kind = infer::get(data);
    
    if let Some(k) = kind {
        return Some(Guess {
            mime: k.mime_type(),
            extension: k.extension(),
            confidence: 1000, 
            bytes_needed: 0,
        });
    }

    // Fallback for Fonts (if infer doesn't catch them with default-features=false)
    if data.len() >= 4 {
        // TrueType: 0x00010000
        if data[0] == 0x00 && data[1] == 0x01 && data[2] == 0x00 && data[3] == 0x00 {
            return Some(Guess { mime: "font/ttf", extension: "ttf", confidence: 900, bytes_needed: 0 });
        }
        // OpenType: OTTO
        if data[0] == 0x4F && data[1] == 0x54 && data[2] == 0x54 && data[3] == 0x4F {
             return Some(Guess { mime: "font/otf", extension: "otf", confidence: 900, bytes_needed: 0 });
        }
        // Collection: ttcf
        if data[0] == 0x74 && data[1] == 0x74 && data[2] == 0x63 && data[3] == 0x66 {
             return Some(Guess { mime: "font/collection", extension: "ttc", confidence: 900, bytes_needed: 0 });
        }
    }
    
    None
}
