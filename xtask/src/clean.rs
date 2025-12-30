use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn run() -> Result<()> {
    let root = project_root();

    let targets = [
        root.join("assets/fonts"),
        root.join("assets/icons"),
        root.join("assets/cursors"),
        root.join("vendor/limine"),
        root.join("vendor/ovmf"),
        root.join("target"),
        root.join("user/target"),
        root.join("disk.img"),
    ];

    for path in targets {
        remove_path(&path)?;
    }

    Ok(())
}

fn remove_path(path: &Path) -> Result<()> {
    if !path.exists() {
        println!("Skipping {}, not present.", path.display());
        return Ok(());
    }

    if path.is_dir() {
        println!("Removing directory {}...", path.display());
        fs::remove_dir_all(path)
            .with_context(|| format!("Failed to remove directory {:?}", path))?;
    } else {
        println!("Removing file {}...", path.display());
        fs::remove_file(path).with_context(|| format!("Failed to remove file {:?}", path))?;
    }

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
