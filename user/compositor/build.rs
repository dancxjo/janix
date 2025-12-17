use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

// Name, URL, HotspotX, HotspotY
const CURSOR_SOURCES: &[(&str, &str, i32, i32)] = &[
    (
        "ARROW",
        "https://raw.githubusercontent.com/catppuccin/cursors/main/src/svgs/default.svg",
        0, 0
    ),
    (
        "MOVE",
        "https://raw.githubusercontent.com/catppuccin/cursors/main/src/svgs/all-scroll.svg",
        64, 64
    ),
    (
        "RESIZE_NS",
        "https://raw.githubusercontent.com/catppuccin/cursors/main/src/svgs/size_ver.svg",
        64, 64
    ),
    (
        "RESIZE_EW",
        "https://raw.githubusercontent.com/catppuccin/cursors/main/src/svgs/size_hor.svg",
        64, 64
    ),
    (
        "RESIZE_NWSE",
        "https://raw.githubusercontent.com/catppuccin/cursors/main/src/svgs/size_bdiag.svg",
        64, 64
    ),
    (
        "RESIZE_NESW",
        "https://raw.githubusercontent.com/catppuccin/cursors/main/src/svgs/size_fdiag.svg",
        64, 64
    ),
];

const FONT_SOURCES: &[(&str, &str)] = &[
    (
        "NOTO_SANS_REGULAR",
        "https://github.com/notofonts/noto-fonts/raw/refs/heads/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf",
    ),
    (
        "NOTO_SANS_SYMBOLS",
        "https://github.com/notofonts/noto-fonts/raw/refs/heads/main/hinted/ttf/NotoSansSymbols/NotoSansSymbols-Regular.ttf",
    ),
    (
        "NOTO_SANS_SYMBOLS2",
        "https://github.com/notofonts/noto-fonts/raw/refs/heads/main/hinted/ttf/NotoSansSymbols2/NotoSansSymbols2-Regular.ttf",
    ),
    (
        "HACK_REGULAR",
        "https://github.com/source-foundry/Hack/raw/master/build/ttf/Hack-Regular.ttf",
    ),
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_WORKSPACE_DIR");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    std::fs::create_dir_all(&out_dir).expect("failed to create OUT_DIR");
    let shared_fonts_dir = workspace_root().join("assets").join("fonts");
    fs::create_dir_all(&shared_fonts_dir).expect("failed to create shared fonts dir");

    // --- Fonts ---
    for (name, url) in FONT_SOURCES {
        let filename = format!("{name}.ttf");
        let dest = shared_fonts_dir.join(&filename);
        if !dest.exists() {
            fetch_url(url, &dest)
                .unwrap_or_else(|e| panic!("failed to download {name} from {url}: {e}"));
        }
    }

    let fonts_rs = out_dir.join("fonts_includes.rs");
    let mut file = File::create(&fonts_rs).expect("failed to create fonts_includes.rs");
    for (name, _) in FONT_SOURCES {
        let path = shared_fonts_dir.join(format!("{name}.ttf"));
        writeln!(
            file,
            "pub static {}: &[u8] = include_bytes!({:?});",
            name, path
        )
        .expect("failed to write fonts_includes.rs");
    }

    // --- Cursors ---
    let cursors_rs = out_dir.join("cursor_assets.rs");
    let mut cursor_file = File::create(&cursors_rs).expect("failed to create cursor_assets.rs");

    for (name, url, hot_x, hot_y) in CURSOR_SOURCES {
        let filename = format!("{}.svg", name.to_lowercase());
        let dest = out_dir.join(&filename);
        if !dest.exists() {
            fetch_url(url, &dest)
                .unwrap_or_else(|e| panic!("failed to download {name} from {url}: {e}"));
        }

        let (width, height, rgba_data) = render_svg(&dest)
             .unwrap_or_else(|e| panic!("failed to render cursor {name}: {e}"));

        // Write the data to a binary file to include it cleanly
        let bin_filename = format!("{}_data.bin", name.to_lowercase());
        let bin_path = out_dir.join(&bin_filename);
        let mut bin_file = File::create(&bin_path).expect("failed to create cursor bin file");
        
        // Write raw u32s (LE)
        for pixel in rgba_data {
            bin_file.write_all(&pixel.to_le_bytes()).expect("failed to write pixel");
        }

        let mod_name = name.to_lowercase();
        let mod_name = if mod_name == "move" { "r#move".to_string() } else { mod_name };

        writeln!(
            cursor_file,
            "pub mod {} {{",
            mod_name
        ).unwrap();
        writeln!(cursor_file, "    pub const WIDTH: u32 = {};", width).unwrap();
        writeln!(cursor_file, "    pub const HEIGHT: u32 = {};", height).unwrap();
        writeln!(cursor_file, "    pub const HOTSPOT_X: i32 = {};", hot_x).unwrap();
        writeln!(cursor_file, "    pub const HOTSPOT_Y: i32 = {};", hot_y).unwrap();
        writeln!(cursor_file, "    pub static DATA: &[u8] = include_bytes!({:?});", bin_path).unwrap();
        writeln!(cursor_file, "}}").unwrap();
    }
}

fn fetch_url(url: &str, dest: &Path) -> io::Result<()> {
    for attempt in 0..3 {
        let status = Command::new("curl")
            .args(["-L", url, "-o"])
            .arg(dest)
            .status()?;
        if status.success() {
            return Ok(());
        }
        eprintln!("curl failed fetching {url} (attempt {attempt}), retrying...");
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!("curl failed fetching {url}"),
    ))
}

fn workspace_root() -> PathBuf {
    if let Ok(path) = env::var("CARGO_WORKSPACE_DIR") {
        return PathBuf::from(path);
    }

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .expect("failed to derive workspace root from manifest dir")
}



fn render_svg(path: &Path) -> io::Result<(u32, u32, Vec<u32>)> {
    let mut svg_data = fs::read_to_string(path)?;
    
    // Replace placeholder colors with "Cloud" theme
    // #00FF00 -> Border (Dark Charocal/Black)
    // #FF0000 -> Fill (Cloud Color from clouds.bmp: #D2DCE0)
    svg_data = svg_data.replace("#00FF00", "#1e1e2e"); // Catppuccin Base
    svg_data = svg_data.replace("#FF0000", "#D2DCE0"); // Cloud color
    
    // Also handle shorthand if present (though checking default.svg showed full hex)
    // Just in case:
    svg_data = svg_data.replace("fill:#0f0", "fill:#1e1e2e");
    svg_data = svg_data.replace("fill:#f00", "fill:#D2DCE0");

    let opt = resvg::usvg::Options::default();
    let rtree = resvg::usvg::Tree::from_str(&svg_data, &opt)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("usvg parse error: {}", e)))?;

    let width = 128; // Target width
    let height = 128; // Target height

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to create pixmap"))?;

    let size = rtree.size().to_int_size();
    let sx = width as f32 / size.width() as f32;
    let sy = height as f32 / size.height() as f32;
    let scale = sx.min(sy); // Keep aspect ratio
    
    let fit_transform = tiny_skia::Transform::from_scale(scale, scale);
    
    resvg::render(&rtree, fit_transform, &mut pixmap.as_mut());

    // Convert to ARGB u32 for compositor
    let pixels: Vec<u32> = pixmap
        .pixels()
        .iter()
        .map(|p| {
             let (r, g, b, a) = (p.red(), p.green(), p.blue(), p.alpha());
             // tiny-skia uses premultiplied alpha already
             // We need to pack as 0xAARRGGBB
             
             ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        })
        .collect();

    Ok((width, height, pixels))
}
