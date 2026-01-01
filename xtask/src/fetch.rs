use anyhow::{ensure, Context, Result};
use image::{Rgba, RgbaImage};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn fetch() -> Result<()> {
    let root = project_root();
    let assets = root.join("assets");
    
    // Ensure assets dir exists
    if !assets.exists() {
        fs::create_dir_all(&assets)?;
    }

    fetch_fonts(&assets)?;
    fetch_icons(&assets)?;
    fetch_cursors(&assets)?;

    Ok(())
}

fn fetch_fonts(assets: &Path) -> Result<()> {
    println!("==> Fetching Fonts...");
    require_tool("curl")?;
    require_tool("unzip")?;
    require_tool("gunzip")?;

    let fonts_dir = assets.join("fonts");
    fs::create_dir_all(&fonts_dir)?;

    // Hack
    let hack_dest = fonts_dir.join("Hack-Regular.ttf");
    if !hack_dest.exists() {
        println!("    Fetching Hack-Regular.ttf...");
        println!("    Downloading Hack-v3.003-ttf.zip");
        let zip_path = fonts_dir.join("temp_hack.zip");
        download_file(
            "https://github.com/source-foundry/Hack/releases/download/v3.003/Hack-v3.003-ttf.zip",
            &zip_path,
        )?;

        run_cmd(
            Command::new("unzip")
                .arg("-o")
                .arg(&zip_path)
                .current_dir(&fonts_dir),
        )?;

        let candidates = [
            fonts_dir.join("ttf/Hack-Regular.ttf"),
            fonts_dir.join("Hack-Regular.ttf"),
        ];

        let mut found = false;
        for c in candidates {
            if c.exists() {
                if c != hack_dest {
                    fs::rename(&c, &hack_dest)?;
                }
                found = true;
                break;
            }
        }

        if !found {
            eprintln!("    [WARNING] Could not locate Hack-Regular.ttf after unzip.");
        }

        let _ = fs::remove_file(&zip_path);
        let _ = fs::remove_dir_all(fonts_dir.join("ttf"));
    }

    // Unifont
    let unifont_dest = fonts_dir.join("unifont.hex");
    if !unifont_dest.exists() {
        println!("    Fetching unifont.hex...");
        let gz_path = fonts_dir.join("unifont.hex.gz");

        let mut downloaded = false;
        let urls = [
            "https://ftp.gnu.org/gnu/unifont/unifont-17.0.03/unifont_all-17.0.03.hex.gz",
            "https://ftp.gnu.org/gnu/unifont/unifont-16.0.01/unifont-16.0.01.hex.gz",
            "https://ftp.gnu.org/gnu/unifont/unifont-15.1.05/unifont-15.1.05.hex.gz",
        ];

        for url in urls {
            if download_file(url, &gz_path).is_ok() {
                downloaded = true;
                break;
            }
        }

        if downloaded {
            if run_cmd(Command::new("gunzip").arg("-k").arg("-f").arg(&gz_path)).is_ok() {
                let _ = fs::remove_file(&gz_path);
            } else {
                eprintln!("    [WARNING] Gunzip failed.");
            }
        } else {
            eprintln!("    [WARNING] Failed to download Unifont. Creating placeholder.");
            fs::write(&unifont_dest, "PLACEHOLDER: Replace with real unifont.hex")?;
        }
    }

    // Noto Fonts
    let noto_fonts = [
        ("NotoSans-Regular.ttf", "https://raw.githubusercontent.com/notofonts/noto-fonts/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf"),
        ("NotoSerif-Regular.ttf", "https://github.com/notofonts/noto-fonts/raw/HEAD/hinted/ttf/NotoSerif/NotoSerif-Regular.ttf"),
        ("NotoSansSymbol-Regular.ttf", "https://github.com/notofonts/noto-fonts/raw/HEAD/hinted/ttf/NotoSansSymbols/NotoSansSymbols-Regular.ttf"),
        ("NotoSansSymbol2-Regular.ttf", "https://github.com/notofonts/noto-fonts/raw/HEAD/hinted/ttf/NotoSansSymbols2/NotoSansSymbols2-Regular.ttf"),
    ];

    for (name, url) in noto_fonts {
        let dest = fonts_dir.join(name);
        if !dest.exists() {
            println!("    Downloading {}...", name);
            if let Err(e) = download_file(url, &dest) {
                eprintln!(
                    "    [WARNING] Failed to download {}: {}. Creating placeholder.",
                    name, e
                );
                fs::write(&dest, b"PLACEHOLDER FONT")?;
            }
        }
    }

    Ok(())
}

fn fetch_icons(assets: &Path) -> Result<()> {
    println!("==> Fetching Icons (Tango)...");
    require_tool("curl")?;
    require_tool("tar")?;

    let icons_dir = assets.join("icons");
    fs::create_dir_all(&icons_dir)?;

    // Check if we already have them (heuristic)
    if icons_dir.join("folder.png").exists() {
        println!("    Tango icons already exist.");
        return Ok(());
    }

    let root = project_root();
    let temp_dir = root.join("target/temp_icons");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    let tango_tar = temp_dir.join("tango.tar.gz");
    println!("    Downloading Tango...");
    download_file(
        "http://tango.freedesktop.org/releases/tango-icon-theme-0.8.90.tar.gz",
        &tango_tar,
    )?;

    println!("    Extracting specific icons...");
    let targets = [
        "tango-icon-theme-0.8.90/32x32/places/folder.png",
        "tango-icon-theme-0.8.90/32x32/mimetypes/text-x-generic.png",
        "tango-icon-theme-0.8.90/32x32/apps/utilities-terminal.png",
    ];

    for t in targets {
        run_cmd(
            Command::new("tar")
                .arg("-xzf")
                .arg(&tango_tar)
                .arg(t)
                .current_dir(&temp_dir),
        )?;

        let name = Path::new(t).file_name().unwrap();
        let src = temp_dir.join(t);
        let dst = icons_dir.join(name);
        if src.exists() {
            fs::rename(&src, &dst)?;
            println!("    Installed {}", name.to_string_lossy());
        }
    }

    let _ = fs::remove_dir_all(&temp_dir);
    Ok(())
}

fn fetch_cursors(assets: &Path) -> Result<()> {
    println!("==> Fetching Cursors...");
    let cursors_dir = assets.join("cursors");
    fs::create_dir_all(&cursors_dir)?;

    // 1. Plain Cursors (Public Domain)
    let plain_dir = cursors_dir.join("plain");
    if !plain_dir.exists() {
        println!("    Fetching Plain Cursors...");
        require_tool("curl")?;
        require_tool("unzip")?;

        let url = "https://www.rw-designer.com/cursor-downloadset/plain.zip";
        let zip_path = cursors_dir.join("plain.zip");

        if download_file(url, &zip_path).is_ok() {
            println!("    Extracting Plain Cursors...");
            fs::create_dir_all(&plain_dir)?;
            run_cmd(
                Command::new("unzip")
                    .arg("-o") // Overwrite
                    .arg(&zip_path)
                    .arg("-d") // Extract to directory
                    .arg(&plain_dir),
            )?;
            let _ = fs::remove_file(&zip_path);
        } else {
            eprintln!("    [WARNING] Failed to download Plain Cursors.");
        }
    }

    // 2. Fallback Cursor (cursor.bmp)
    let cursor_path = cursors_dir.join("cursor.bmp");
    if cursor_path.exists() {
        println!("    Fallback cursor.bmp already exists.");
    } else {
        println!("    Generating fallback cursor.bmp...");
        let img = generate_arrow_cursor(32);
        img.save(&cursor_path)?;
    }

    Ok(())
}

fn generate_arrow_cursor(size: u32) -> RgbaImage {
    let mut img = RgbaImage::new(size, size);
    for y in 0..size {
        for x in 0..size {
            let mut color = Rgba([0, 0, 0, 0]);
            if x < 18 && y < 24 {
                if x == 0 && y < 22 {
                    color = Rgba([0, 0, 0, 255]); // Left edge
                } else if x == y && x < 16 {
                    color = Rgba([0, 0, 0, 255]); // Diagonal
                } else if y == 22 && x < 6 {
                    color = Rgba([0, 0, 0, 255]); // Bottom
                }
                
                // Fill
                if x > 0 && x < y && (7 * x + 15 * y < 320) {
                    color = Rgba([255, 255, 255, 255]);
                }

                // Border override
                if x == 0 && y < 22 {
                    color = Rgba([0, 0, 0, 255]);
                } else if (x as i32 - y as i32).abs() <= 1 && x < 16 {
                    color = Rgba([0, 0, 0, 255]);
                } else if (7 * x + 15 * y > 310) && (7 * x + 15 * y < 340) && x < 16 && y > 10 {
                    color = Rgba([0, 0, 0, 255]);
                }
            }
            img.put_pixel(x, y, color);
        }
    }
    img
}

fn require_tool(tool: &str) -> Result<()> {
    if Command::new("which").arg(tool).output().is_err() {
        // bail! is cleaner if anyhow::bail
        anyhow::bail!("Missing required tool: {}", tool);
    }
    Ok(())
}

fn download_file(url: &str, dest: &Path) -> Result<()> {
    run_cmd(
        Command::new("curl")
            .arg("-f")
            .arg("-L")
            .arg("-o")
            .arg(dest)
            .arg(url),
    )
}

fn run_cmd(cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("Failed to run {:?}", cmd))?;
    ensure!(status.success(), "Command failed: {:?}", cmd);
    Ok(())
}

fn project_root() -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .to_path_buf()
}
