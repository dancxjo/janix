//! Build and package a stage-1 `rustc` cross-compiler for `x86_64-unknown-thingos`
//! (runs on linux-gnu, targets ThingOS).
//!
//! # Environment variables
//! * `BUILD_RUSTC=1` – enable the build (off by default).
//!
//! # Caching
//! The build is considered up-to-date when both of these conditions hold:
//! 1. `target/rustc-thingos/rustc` exists.
//! 2. `target/rustc-thingos/.cache-key` contains a hash derived from the
//!    content of `targets/x86_64-unknown-thingos.json`, `rust-toolchain.toml`,
//!    and the git revision of `vendor/rust/`. Changing any of these
//!    invalidates the cache.
//!
//! The current bootstrap produces a Linux-hosted stage-1 compiler plus a
//! ThingOS target sysroot. That compiler is cached for local development but
//! is not yet staged into the ISO because it is not runnable on ThingOS.

use crate::common::Result;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use xshell::{Shell, cmd};

/// Where the cached rustc binary lives (relative to the project root).
const RUSTC_BINARY: &str = "target/rustc-thingos/rustc";
/// Wrapper that injects sysroot and shared-library paths for the cached rustc.
const RUSTC_WRAPPER: &str = "target/rustc-thingos/rustc-wrapper";
/// Where the cache-key file lives.
const CACHE_KEY_FILE: &str = "target/rustc-thingos/.cache-key";
/// Output directory for all rustc-thingos build artifacts.
const OUTPUT_DIR: &str = "target/rustc-thingos";
/// Cached rustlib directory containing host and ThingOS target libraries.
const RUSTLIB_CACHE_DIR: &str = "target/rustc-thingos/lib/rustlib";
/// Cached dynamic libraries needed by the Linux-hosted rustc binary.
const LIB_CACHE_DIR: &str = "target/rustc-thingos/lib";

// ---------------------------------------------------------------------------
// Cache-key helpers
// ---------------------------------------------------------------------------

fn compute_cache_key() -> String {
    let mut hasher = DefaultHasher::new();
    for path in &["targets/x86_64-unknown-thingos.json", "rust-toolchain.toml"] {
        if let Ok(content) = std::fs::read_to_string(path) {
            content.hash(&mut hasher);
        }
    }

    // Hash the current git revision of vendor/rust/ to invalidate the cache
    // when the fork is updated.
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir("vendor/rust")
        .output()
    {
        if output.status.success() {
            output.stdout.hash(&mut hasher);
        }
    }

    format!("{:016x}", hasher.finish())
}

fn is_cache_valid() -> bool {
    if !Path::new(RUSTC_BINARY).exists() {
        return false;
    }
    match std::fs::read_to_string(CACHE_KEY_FILE) {
        Ok(stored) => stored.trim() == compute_cache_key(),
        Err(_) => false,
    }
}

fn write_cache_key() -> std::io::Result<()> {
    std::fs::write(CACHE_KEY_FILE, compute_cache_key())
}

fn write_rustc_wrapper(cwd: &Path) -> Result<()> {
    let wrapper_path = cwd.join(RUSTC_WRAPPER);
    let compiler_path = cwd.join(RUSTC_BINARY);
    let sysroot = cwd.join("target/rustc-thingos");
    let libdir = cwd.join(LIB_CACHE_DIR);

    let script = format!(
        "#!/usr/bin/env bash\nset -euo pipefail\nexport LD_LIBRARY_PATH=\"{libdir}:${{LD_LIBRARY_PATH:-}}\"\nexec \"{compiler}\" --sysroot \"{sysroot}\" \"$@\"\n",
        libdir = libdir.display(),
        compiler = compiler_path.display(),
        sysroot = sysroot.display(),
    );

    std::fs::write(&wrapper_path, script)?;
    #[cfg(unix)]
    {
        let mut perms = std::fs::metadata(&wrapper_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&wrapper_path, perms)?;
    }
    Ok(())
}

fn xpy_build_root(cwd: &Path) -> PathBuf {
    cwd.join("build/x86_64-unknown-linux-gnu")
}

fn locate_stage1_rustc(cwd: &Path, rust_src: &Path) -> Option<PathBuf> {
    let root_build = xpy_build_root(cwd);
    let candidates = [
        root_build.join("stage1/bin/rustc"),
        root_build.join("stage1-rustc/x86_64-unknown-linux-gnu/release/rustc-main"),
        rust_src.join("build/x86_64-unknown-linux-gnu/stage1/bin/rustc"),
        rust_src.join(
            "build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-thingos/release/rustc",
        ),
    ];

    candidates.into_iter().find(|path| path.exists())
}

fn cache_rustlib_tree(sh: &Shell, cwd: &Path) -> Result<()> {
    let build_root = xpy_build_root(cwd);
    let cached_rustlib = cwd.join(RUSTLIB_CACHE_DIR);
    let cached_lib = cwd.join(LIB_CACHE_DIR);

    sh.remove_path(&cached_rustlib)?;
    sh.create_dir(&cached_rustlib)?;
    sh.create_dir(&cached_lib)?;

    // Stage the host sysroot that the cached rustc itself needs.
    let host_rustlib = build_root.join("stage1/lib/rustlib");
    if host_rustlib.exists() {
        let host_src = host_rustlib.to_str().unwrap();
        let host_dst = cached_rustlib.to_str().unwrap();
        cmd!(sh, "cp -r {host_src}/. {host_dst}").run()?;
    }

    // Preserve the dynamic libraries the Linux-hosted compiler needs at
    // runtime, notably libLLVM.so from the stage1 sysroot.
    let host_lib = build_root.join("stage1/lib");
    if host_lib.exists() {
        for entry in std::fs::read_dir(&host_lib)? {
            let entry = entry?;
            let path = entry.path();
            let keep = matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("so")
            ) || path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.contains(".so."));
            if keep {
                let file_name = path.file_name().unwrap();
                std::fs::copy(&path, cached_lib.join(file_name))?;
            }
        }
    }

    // x.py leaves the ThingOS target libraries under stage1-std cargo output.
    // Mirror the standard rustlib layout so the cached compiler can target
    // ThingOS without a custom sysroot path.
    let thingos_lib = build_root.join("stage1-std/x86_64-unknown-thingos/release/deps");
    if thingos_lib.exists() {
        let thingos_dst = cached_rustlib.join("x86_64-unknown-thingos/lib");
        sh.create_dir(thingos_dst.parent().unwrap())?;
        sh.create_dir(&thingos_dst)?;
        for entry in std::fs::read_dir(&thingos_lib)? {
            let entry = entry?;
            let path = entry.path();
            let keep = matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("rlib") | Some("rmeta")
            );
            if keep {
                let file_name = path.file_name().unwrap();
                std::fs::copy(&path, thingos_dst.join(file_name))?;
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Bootstrap configuration
// ---------------------------------------------------------------------------

/// Generate a `config.toml` for `x.py` that builds a stage-1 `rustc`
/// cross-compiler (runs on linux-gnu) that targets `x86_64-unknown-thingos`.
///
/// Bootstrap terminology:
///   `build`  = the machine running the build (always linux-gnu here)
///   `host`   = the machine(s) the resulting compiler will *run* on
///   `target` = the machine(s) the resulting compiler can produce code for
///
/// By keeping `host = ["x86_64-unknown-linux-gnu"]` we reuse the prebuilt
/// CI LLVM (linux-gnu) — no LLVM recompilation from source required.  The
/// resulting stage-1 `rustc` is a linux-gnu binary that can cross-compile
/// Rust code targeting `x86_64-unknown-thingos`.
///
/// `local-rebuild = true` allows the stage-0 (downloaded CI) compiler to
/// build the ThingOS standard library directly, without first building a
/// stage-1 linux-gnu compiler.  Our `StdLink::run()` fork patch ensures the
/// built ThingOS sysroot is copied into the stage0-sysroot so stage-1
/// compiler artifacts can find `core`/`alloc`/`std` for ThingOS.
///
/// Static-linking strategy for rustc_driver
/// -----------------------------------------
/// `x86_64-unknown-thingos` has `"dynamic-linking": false` in its JSON
/// target spec.  The fork patch in
///   compiler/rustc_driver/Cargo.toml  (adds "rlib" alongside "dylib")
///   src/bootstrap/src/core/build_steps/compile.rs
/// ensures `rustc_driver` is built as an rlib (static archive) for no-dylib
/// targets, linked statically into the `rustc` binary.
fn bootstrap_config() -> String {
    r#"change-id = "ignore"
# Auto-generated by `cargo xtask rustc-thingos` – do not edit by hand.
[build]
build = "x86_64-unknown-linux-gnu"
host  = ["x86_64-unknown-linux-gnu"]
target = ["x86_64-unknown-linux-gnu", "x86_64-unknown-thingos"]
local-rebuild = true
docs = false
compiler-docs = false

[rust]
optimize = true
debug-assertions = false
codegen-units = 1
lto = "off"
deny-warnings = false
# ThingOS uses static linking only; disable rpath so the produced compiler
# binary does not embed paths to shared libraries it cannot use.
rpath = false

# Declare per-target capabilities to the bootstrap.  The ThingOS target does
# not have a dynamic linker, so sanitizers and profiler instrumentation (which
# typically require shared-library runtimes) are also disabled.
[target.x86_64-unknown-thingos]
sanitizers = false
profiler = false

[llvm]
download-ci-llvm = true
"#
    .to_string()
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Build a Linux-hosted stage-1 `rustc` cross-compiler targeting
/// `x86_64-unknown-thingos` and cache the result under `target/rustc-thingos/`.
///
/// Returns `Ok(None)` when the step is intentionally skipped (either via the
/// `SKIP_RUSTC_THINGOS=1` env-var or because the build is not yet supported
/// for the requested architecture).
///
/// The `arch` parameter guards the build: only `"x86_64"` is supported for
/// now; all other architectures silently return `Ok(None)`.
pub fn build_rustc_thingos(sh: &Shell, arch: &str) -> Result<Option<PathBuf>> {
    // Only x86_64 is supported at the moment.
    if arch != "x86_64" {
        return Ok(None);
    }

    // Honour the opt-in env-var.
    if std::env::var("BUILD_RUSTC").as_deref() != Ok("1") {
        return Ok(None);
    }

    // Fast path: cache is still valid.
    if is_cache_valid() {
        println!("rustc-thingos: cache hit, reusing {}", RUSTC_BINARY);
        return Ok(Some(PathBuf::from(RUSTC_BINARY)));
    }

    println!("rustc-thingos: building stage-1 rustc for x86_64-unknown-thingos …");

    // The Rust source tree is required.
    if !Path::new("vendor/rust/.git").exists() {
        return Err(
            "vendor/rust not found – run `just fetch-rust` first to clone the Rust source tree."
                .into(),
        );
    }

    std::fs::create_dir_all(OUTPUT_DIR)?;

    let cwd = std::env::current_dir()?;
    let rust_src = cwd.join("vendor/rust");
    let rust_src_str = rust_src.to_str().unwrap();

    // Write our bootstrap config.toml into the vendored source tree.
    std::fs::write(rust_src.join("config.toml"), bootstrap_config())?;

    // RUST_TARGET_PATH tells x.py where to find custom JSON target specs.
    let target_dir = cwd.join("targets");
    let target_dir_str = target_dir.to_str().unwrap();

    // Run the bootstrap.  Stage-1 is sufficient to produce a usable compiler;
    // a full stage-2 bootstrap can be added later.
    // Run the bootstrap.  We build both `library` and `compiler/rustc` so that
    // the stage-1 sysroot has prebuilt core/alloc/std for x86_64-unknown-linux-gnu
    // (needed when the stage-1 rustc is used as RUSTC for cargo builds — cargo
    // runs build scripts on the host and they need host std in the sysroot).
    cmd!(
        sh,
        "python3 {rust_src_str}/x.py build --stage 1 library compiler/rustc"
    )
    .env("RUST_TARGET_PATH", target_dir_str)
    .run()?;

    // Recent bootstrap layouts place the resulting binary under the repository
    // root's `build/` tree as `stage1-rustc/.../release/rustc-main` rather than
    // under `vendor/rust/build/.../stage1/bin/rustc`. Accept both layouts.
    let stage1_rustc = locate_stage1_rustc(&cwd, &rust_src).ok_or_else(|| {
        let root_build = xpy_build_root(&cwd);
        format!(
            "stage-1 rustc binary not found after bootstrap.\n\
                 Looked for common paths under:\n  {:?}\n  {:?}",
            root_build,
            rust_src.join("build/x86_64-unknown-linux-gnu")
        )
    })?;

    sh.copy_file(&stage1_rustc, RUSTC_BINARY)?;
    cache_rustlib_tree(sh, &cwd)?;
    write_rustc_wrapper(&cwd)?;

    write_cache_key()?;
    println!("rustc-thingos: binary cached at {}", RUSTC_BINARY);
    Ok(Some(PathBuf::from(RUSTC_BINARY)))
}

/// Staging a cached compiler into the ISO is currently disabled.
///
/// The bootstrap configuration used here produces a Linux-hosted cross-compiler
/// plus a ThingOS target sysroot. That artifact is useful on the developer
/// machine but not yet runnable inside the ThingOS image.
pub fn stage_rustc_for_iso(sh: &Shell, iso_root: &Path) -> Result<()> {
    let _ = (sh, iso_root);
    if std::env::var("BUILD_RUSTC").as_deref() != Ok("1") {
        return Ok(());
    }
    println!(
        "rustc-thingos: cached compiler is a Linux-hosted cross-compiler; skipping ISO staging."
    );
    Ok(())
}
