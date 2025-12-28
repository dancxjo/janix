use anyhow::{ensure, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use image::{Rgba, RgbaImage};

#[derive(clap::Args, Debug)]
pub struct FetchArgs {
    /// Only fetch specific assets
    #[arg(long, value_enum)]
    pub only: Option<FetchTarget>,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum FetchTarget {
    Limine,
    Ovmf,
    Fonts,
    Icons,
    Cursors,
}

pub fn run(args: FetchArgs) -> Result<()> {
    let root = project_root();
    let vendor = root.join("vendor");
    let assets = root.join("assets");

    fs::create_dir_all(&vendor)?;
    fs::create_dir_all(&assets)?;

    let targets = match args.only {
        Some(t) => vec![t],
        None => vec![
            FetchTarget::Limine,
            FetchTarget::Ovmf,
            FetchTarget::Fonts,
            FetchTarget::Icons,
            FetchTarget::Cursors,
        ],
    };

    for target in targets {
        match target {
            FetchTarget::Limine => fetch_limine(&vendor)?,
            FetchTarget::Ovmf => fetch_ovmf(&vendor)?,
            FetchTarget::Fonts => fetch_fonts(&assets)?,
            FetchTarget::Icons => fetch_icons(&assets)?,
            FetchTarget::Cursors => fetch_cursors(&assets)?,
        }
    }

    Ok(())
}

fn fetch_limine(vendor: &Path) -> Result<()> {
    println!("==> Fetching Limine...");
    require_tool("git")?;

    let limine_dir = vendor.join("limine");
    if limine_dir.join(".git").exists() {
        println!("    Removing existing limine repo...");
        fs::remove_dir_all(&limine_dir)?;
    }

    run_cmd(
        Command::new("git")
            .arg("clone")
            .arg("--branch=v9.x-binary")
            .arg("--depth=1")
            .arg("https://github.com/limine-bootloader/limine.git")
            .arg(&limine_dir),
    )?;

    // Verify
    let required = [
        "limine-bios.sys",
        "limine-bios-cd.bin",
        "limine-uefi-cd.bin",
        "BOOTX64.EFI",
        "BOOTAA64.EFI",
    ];
    for f in required {
        let p = limine_dir.join(f);
        if !p.exists() {
            ensure!(p.exists(), "Missing Limine artifact: {}", f);
        }
    }
    println!("    Limine fetched.");
    Ok(())
}

fn fetch_ovmf(vendor: &Path) -> Result<()> {
    println!("==> Fetching OVMF...");
    require_tool("curl")?;

    let ovmf_dir = vendor.join("ovmf");
    fs::create_dir_all(&ovmf_dir)?;

    let base_url = "https://github.com/rust-osdev/ovmf-prebuilt/releases/download/v2025.10.09";

    let files = [
        ("ovmf-code-x86_64.fd", "ovmf-x86_64-code.fd"),
        ("ovmf-vars-x86_64.fd", "ovmf-x86_64-vars.fd"),
        ("ovmf-code-aarch64.fd", "ovmf-aarch64-code.fd"),
        ("ovmf-vars-aarch64.fd", "ovmf-aarch64-vars.fd"),
    ];

    for (dest_name, src_name) in files {
        let dest = ovmf_dir.join(dest_name);
        if dest.exists() {
            continue;
        }
        let url = format!("{}/{}", base_url, src_name);
        println!("    Downloading {}...", dest_name);

        if download_file(&url, &dest).is_err() {
            eprintln!(
                "    [WARNING] Failed to download {}. Creating placeholder.",
                dest_name
            );
            let mut data = vec![0u8; 4 * 1024 * 1024];
            let msg = b"PLACEHOLDER: Replace with real OVMF firmware";
            data[..msg.len()].copy_from_slice(msg);
            fs::write(&dest, &data)?;
        }
    }

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
            "https://ftp.gnu.org/gnu/unifont/unifont-15.1.05/unifont-15.1.05.hex.gz",
            "https://ftp.gnu.org/gnu/unifont/unifont-15.1.04/unifont-15.1.04.hex.gz",
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
    if temp_dir.exists() { fs::remove_dir_all(&temp_dir)?; }
    fs::create_dir_all(&temp_dir)?;

    let tango_tar = temp_dir.join("tango.tar.gz");
    println!("    Downloading Tango...");
    download_file(
        "http://tango.freedesktop.org/releases/tango-icon-theme-0.8.90.tar.gz",
        &tango_tar
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
                .current_dir(&temp_dir)
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
    println!("==> Generatng Cursor...");
    let cursors_dir = assets.join("cursors");
    fs::create_dir_all(&cursors_dir)?;

    let cursor_path = cursors_dir.join("cursor.png");
    if cursor_path.exists() {
        println!("    Cursor already exists.");
        return Ok(());
    }

    let img = generate_arrow_cursor(32);
    img.save(&cursor_path)?;
    println!("    Generated cursor.png");
    Ok(())
}

fn generate_arrow_cursor(size: u32) -> RgbaImage {
    let mut img = RgbaImage::new(size, size);

    // Simple Arrow
    // 0,0
    // | \
    // |  \
    // |   \
    // |    \
    // |     \
    // |      \
    // v       \
    //

    for y in 0..size {
        for x in 0..size {
            // Check if point is inside triangle (0,0), (0, 24), (16, 16)
            // Or simpler: x < y/2 (roughly)

            // Draw Outline (Black) and Fill (White)
            let is_inside = x < (size - y/2) && x < (y/2 + 5); // Just random math? No.

            // Let's use specific coordinates
            // Top: 0,0
            // Bottom Left: 0, 22
            // Right: 16, 16 (Not really)
            // Cursor is usually (0,0) -> (0, 20) -> (12, 12) -> ...

            // Let's draw pixel by pixel logic or simple loop
            // Standard pointer:
            // x=0, y=0 to 20
            // x=y (diagonal) from 0,0 to 12,12
            // bottom edge from 0,20 to 12,12? No.

            // Let's just draw a white square for now to be safe, or a simple cross.
            // Or a real arrow.

            let mut color = Rgba([0, 0, 0, 0]);

            // Bounding box for arrow
            if x < 18 && y < 24 {
                // Main body
                // x=0 is left edge
                // y = x*1.5 + something?

                // Let's use a simple heuristic
                // If x == 0 && y < 20: Black
                // If x == y && x < 14: Black
                // If y == 14 && x > 5 && x < 14: Black (stem)

                // Better:
                // Inside: x > 1 && y > 1 && x < y && y < 20 - x/2

                // I'll draw a simple white triangle with black border.

                let slope = x as f32 / y as f32; // 0 at x=0

                // 3 vertices: (0,0), (0, 22), (15, 15)
                // Inside test
                // v1=(0,0), v2=(0,22), v3=(15,15)
                // P=(x,y)

                // Check edge 2-3: (15-0)*(y-22) - (15-22)*(x-0) = 15(y-22) + 7x
                // 15y - 330 + 7x <= 0  => 7x + 15y <= 330

                // Check edge 3-1: (0-15)*(y-15) - (0-15)*(x-15) = -15(y-15) + 15(x-15)
                // -15y + 225 + 15x - 225 >= 0 => 15x >= 15y => x >= y (Wait, x <= y for the other side)

                // Actually: x <= y (approx)

                // Let's just enable pixels
                if x == 0 && y < 22 {
                     color = Rgba([0, 0, 0, 255]); // Left edge
                } else if x == y && x < 16 {
                     color = Rgba([0, 0, 0, 255]); // Diagonal
                } else if y == 22 && x < 6 {
                     color = Rgba([0, 0, 0, 255]); // Bottom
                } else if 7*x + 15*y == 330 {
                     // color = Rgba([0, 0, 0, 255]);
                }

                // Fill
                if x > 0 && x < y && (7*x + 15*y < 320) {
                    color = Rgba([255, 255, 255, 255]);
                }

                // Border override
                if x == 0 && y < 22 { color = Rgba([0,0,0,255]); }
                else if (x as i32 - y as i32).abs() <= 1 && x < 16 { color = Rgba([0,0,0,255]); }
                else if (7*x + 15*y > 310) && (7*x + 15*y < 340) && x < 16 && y > 10 { color = Rgba([0,0,0,255]); }
            }

            img.put_pixel(x, y, color);
        }
    }
    img
}

fn require_tool(tool: &str) -> Result<()> {
    if Command::new("which").arg(tool).output().is_err() {
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
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
