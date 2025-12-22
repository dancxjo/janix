use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let fonts_dir = Path::new(&manifest_dir)
        .join("..")
        .join("assets")
        .join("fonts");
    let unifont_hex = fonts_dir.join("unifont.hex");

    if !fonts_dir.exists() {
        fs::create_dir(&fonts_dir).unwrap();
    }

    if !unifont_hex.exists() {
        println!("cargo:warning=Downloading Unifont to {:?}", unifont_hex);
        let url =
            "https://mirrors.kernel.org/gnu/unifont/unifont-16.0.02/unifont_all-16.0.02.hex.gz";
        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (compatible; ThingOS-Build/1.0)")
            .build()
            .unwrap();
        let resp = client.get(url).send().expect("Failed to download Unifont");

        if !resp.status().is_success() {
            panic!("Failed to download Unifont: status {}", resp.status());
        }

        let mut decoder = flate2::read::GzDecoder::new(resp);
        let mut content = String::new();
        std::io::Read::read_to_string(&mut decoder, &mut content)
            .expect("Failed to decompress Unifont");

        fs::write(&unifont_hex, content).expect("Failed to write Unifont file");
    }

    println!("cargo:rerun-if-changed={}", unifont_hex.display());
    println!("cargo:rerun-if-changed=build.rs");

    generate_font_source(&unifont_hex);
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
