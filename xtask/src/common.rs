//! Common utilities shared across xtask modules.

use std::path::PathBuf;

/// Get the project root directory (parent of xtask).
pub fn project_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).parent().unwrap().to_path_buf()
}

/// Map architecture to Rust target triple.
pub fn rust_target(arch: &str) -> &'static str {
    match arch {
        "riscv64" => "riscv64gc-unknown-none-elf",
        "x86_64" => "x86_64-unknown-none",
        "aarch64" => "aarch64-unknown-none",
        "loongarch64" => "loongarch64-unknown-none",
        _ => panic!("Unsupported architecture: {}", arch),
    }
}

/// Map Cargo profile to output subdirectory.
pub fn profile_subdir(profile: &str) -> &str {
    if profile == "dev" { "debug" } else { profile }
}

/// Generate image name for architecture.
pub fn image_name(arch: &str) -> String {
    format!("thing-os-{}", arch)
}

/// Result type alias for xtask operations.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
