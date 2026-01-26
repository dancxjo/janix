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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_subdir_maps_dev_to_debug() {
        assert_eq!(profile_subdir("dev"), "debug");
        assert_eq!(profile_subdir("release"), "release");
    }

    #[test]
    fn rust_target_maps_arches() {
        assert_eq!(rust_target("riscv64"), "riscv64gc-unknown-none-elf");
        assert_eq!(rust_target("x86_64"), "x86_64-unknown-none");
        assert_eq!(rust_target("aarch64"), "aarch64-unknown-none");
        assert_eq!(rust_target("loongarch64"), "loongarch64-unknown-none");
    }

    #[test]
    fn image_name_is_stable() {
        assert_eq!(image_name("x86_64"), "thing-os-x86_64");
    }

    #[test]
    fn project_root_ends_with_repo_name() {
        let root = project_root();
        // Check that the root directory contains the main Cargo.toml
        assert!(root.join("Cargo.toml").exists());
    }
}
