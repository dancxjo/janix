use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

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
    let shared_fonts_dir = workspace_root().join("target").join("compositor-fonts");
    fs::create_dir_all(&shared_fonts_dir).expect("failed to create shared fonts dir");

    for (name, url) in FONT_SOURCES {
        let filename = format!("{name}.ttf");
        let dest = out_dir.join(&filename);
        if !dest.exists() {
            fetch_url(url, &dest)
                .unwrap_or_else(|e| panic!("failed to download {name} from {url}: {e}"));
        }
        let shared_dest = shared_fonts_dir.join(&filename);
        copy_font_to_shared_dir(&dest, &shared_dest);
    }

    let fonts_rs = out_dir.join("fonts_includes.rs");
    let mut file = File::create(&fonts_rs).expect("failed to create fonts_includes.rs");
    for (name, _) in FONT_SOURCES {
        let path = out_dir.join(format!("{name}.ttf"));
        writeln!(
            file,
            "pub static {}: &[u8] = include_bytes!({:?});",
            name, path
        )
        .expect("failed to write fonts_includes.rs");
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

fn copy_font_to_shared_dir(src: &Path, dest: &Path) {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).expect("failed to create shared font parent dir");
    }
    fs::copy(src, dest).expect("failed to copy font into shared directory");
}
