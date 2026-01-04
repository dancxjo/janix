use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn run() -> Result<()> {
    let root = project_root();

    // Cargo clean
    println!("==> Cargo cleaning...");
    Command::new("cargo")
        .arg("clean")
        .current_dir(&root)
        .status()?;

    // Remove artifacts
    let paths = [
        "target", // Workspace target
        "vendor/limine",
        "vendor/ovmf",
        // "assets", // Maybe keep assets valid? fetch.rs downloads safely. Gitignore handles them.
        // GNUmakefile distclean removes limine ovmf.
        "iso_root_x86_64", // Legacy
    ];

    for p in paths {
        let path = root.join(p);
        if path.exists() {
            println!("    Removing {}", p);
            let _ = fs::remove_dir_all(&path);
        }
    }

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
