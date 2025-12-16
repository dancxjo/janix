use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Path to things-os/fonts relative to user/geographer
    let fonts_dir = Path::new(&manifest_dir).join("../../fonts");
    let unifont_hex = fonts_dir.join("unifont.hex");

    println!("cargo:rerun-if-changed={}", unifont_hex.display());
    println!("cargo:rerun-if-changed=build.rs");

    if !unifont_hex.exists() {
        // If the font doesn't exist, we can't generate it. 
        // We assume the kernel build has already downloaded it or will download it.
        // However, since we are in userland, we might build independently.
        // For now, we will just panic with a helpful message if it's missing, 
        // relying on the top-level build to handle dependencies or the user to run kernel build first.
        println!("cargo:warning=Unifont not found at {:?}. Please run kernel build first to download it.", unifont_hex);
    } else {
        generate_font_source(&unifont_hex);
    }
}

fn generate_font_source(unifont_path: &Path) {
    let content = fs::read_to_string(unifont_path).expect("Failed to read unifont.hex");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("unifont.rs");
    let mut out_file = fs::File::create(&dest_path).expect("Failed to create unifont.rs");

    const GLYPH_WIDTH: u32 = 8;
    const GLYPH_HEIGHT: u32 = 16;
    let start_char = 0x20;
    let end_char = 0x7E;
    let count = end_char - start_char + 1;

    writeln!(out_file, "pub const GLYPH_WIDTH: u32 = {};", GLYPH_WIDTH).unwrap();
    writeln!(out_file, "pub const GLYPH_HEIGHT: u32 = {};", GLYPH_HEIGHT).unwrap();
    writeln!(
        out_file,
        "pub static GLYPHS: [[u8; GLYPH_HEIGHT as usize]; {}] = [",
        count
    )
    .unwrap();

    let mut glyphs = std::collections::HashMap::new();

    for line in content.lines() {
        if let Some((code_str, hex_str)) = line.split_once(':') {
            if let Ok(code) = u32::from_str_radix(code_str, 16) {
                if code >= start_char && code <= end_char {
                    if hex_str.len() == 32 {
                        let mut bitmap = [0u8; 16];
                        for i in 0..16 {
                            let byte_str = &hex_str[i * 2..i * 2 + 2];
                            if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                                bitmap[i] = byte;
                            }
                        }
                        glyphs.insert(code, bitmap);
                    }
                }
            }
        }
    }

    for code in start_char..=end_char {
        let bitmap = glyphs.get(&code).copied().unwrap_or([0u8; 16]);
        write!(out_file, "    [").unwrap();
        for byte in bitmap {
            write!(out_file, "0x{:02X}, ", byte).unwrap();
        }
        writeln!(out_file, "], // U+{:04X}", code).unwrap();
    }

    writeln!(out_file, "];").unwrap();

    writeln!(
        out_file,
        r#"
#[inline]
pub fn lookup_glyph(ch: char) -> Option<&'static [u8; GLYPH_HEIGHT as usize]> {{
    let c = ch as u32;
    if (0x20..=0x7E).contains(&c) {{
        Some(&GLYPHS[(c - 0x20) as usize])
    }} else {{
        None
    }}
}}
"#
    )
    .unwrap();
}
