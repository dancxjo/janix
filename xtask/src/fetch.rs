use anyhow::{ensure, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
}

pub fn run(args: FetchArgs) -> Result<()> {
    let root = project_root();
    let vendor = root.join("vendor");
    let assets = root.join("assets");

    fs::create_dir_all(&vendor)?;
    fs::create_dir_all(&assets)?;

    let targets = match args.only {
        Some(t) => vec![t],
        None => vec![FetchTarget::Limine, FetchTarget::Ovmf, FetchTarget::Fonts],
    };

    for target in targets {
        match target {
            FetchTarget::Limine => fetch_limine(&vendor)?,
            FetchTarget::Ovmf => fetch_ovmf(&vendor)?,
            FetchTarget::Fonts => fetch_fonts(&assets)?,
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

    // git clone --branch=v9.x-binary --depth=1 https://github.com/limine-bootloader/limine.git vendor/limine
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
        // Sometimes binary branch has them in root, or subdirs. v9.x-binary usually has them in root.
        if !p.exists() {
            // Check if they are maybe inside? v9.x-binary puts them at top level usually.
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

        // Try download, if fail, create placeholder
        if download_file(&url, &dest).is_err() {
            eprintln!(
                "    [WARNING] Failed to download {}. Creating placeholder.",
                dest_name
            );
            // QEMU requires pflash images to be 4KB aligned.
            // We create a 4MB zeroed file with a warning header.
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

        // Move ttf (Hack-v3.003-ttf/ttf/Hack-Regular.ttf usually)
        // We'll check recursively or specific paths
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
            // Fallback: finding it
            // For now, if not found, we warn.
            eprintln!("    [WARNING] Could not locate Hack-Regular.ttf after unzip.");
        }

        // Cleanup
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
                eprintln!("    [WARNING] Failed to download {}: {}. Creating placeholder.", name, e);
                 fs::write(&dest, b"PLACEHOLDER FONT")?;
            }
        }
    }

    Ok(())
}

fn require_tool(tool: &str) -> Result<()> {
    if Command::new("which").arg(tool).output().is_err() {
        anyhow::bail!("Missing required tool: {}", tool);
    }
    Ok(())
}

fn download_file(url: &str, dest: &Path) -> Result<()> {
    // curl -fL -o dest url
    // -f: fail on HTTP error (404, etc)
    // -L: follow redirects
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
