//! Build and package a stage-1 `rustc` cross-compiler for `x86_64-unknown-thingos`.
//!
//! Two artifacts are produced from a single `x.py build --stage 1` invocation:
//!
//! 1. **Linux-hosted cross-compiler** (`target/rustc-thingos/rustc`): runs on
//!    `x86_64-unknown-linux-gnu`, targets `x86_64-unknown-thingos`.  Used on
//!    the developer machine (via the `rustc-wrapper` shim) when building
//!    ThingOS userspace programs.
//!
//! 2. **ThingOS-native compiler** (`target/rustc-thingos/thingos-rustc`): a
//!    stage-1 `rustc` binary cross-compiled *for* `x86_64-unknown-thingos` so
//!    it runs natively inside a ThingOS instance.  This is the binary that
//!    `stage_rustc_for_iso()` stages into the boot ISO at:
//!    * `bin/rustc`              – the compiler binary
//!    * `lib/rustlib/`           – the sysroot (core/alloc/std rlibs)
//!
//! # Sysroot layout on ThingOS
//! When `rustc` runs from `/bin/rustc`, Rust's sysroot-detection logic strips
//! the trailing `bin/` component and derives sysroot = `/`.  It therefore
//! looks for rustlib at `/lib/rustlib/`.  The ISO staging mirrors this layout.
//!
//! # Environment variables
//! * `SKIP_RUSTC_THINGOS=1` – skip the build entirely.
//!
//! # Caching
//! The build is considered up-to-date when both of these conditions hold:
//! 1. `target/rustc-thingos/rustc` **and** `target/rustc-thingos/thingos-rustc` exist.
//! 2. `target/rustc-thingos/.cache-key` contains a hash derived from the
//!    content of `targets/x86_64-unknown-thingos.json`, `rust-toolchain.toml`,
//!    and the git revision of `vendor/rust/`. Changing any of these
//!    invalidates the cache.

use crate::common::Result;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use xshell::{Shell, cmd};

/// Where the cached Linux-hosted rustc binary lives (relative to the project root).
const RUSTC_BINARY: &str = "target/rustc-thingos/rustc";
/// Wrapper that injects sysroot and shared-library paths for the cached Linux-hosted rustc.
const RUSTC_WRAPPER: &str = "target/rustc-thingos/rustc-wrapper";
/// Where the cache-key file lives.
const CACHE_KEY_FILE: &str = "target/rustc-thingos/.cache-key";
/// Output directory for all rustc-thingos build artifacts.
const OUTPUT_DIR: &str = "target/rustc-thingos";
/// Cached rustlib directory containing host and ThingOS target libraries for the Linux-hosted compiler.
const RUSTLIB_CACHE_DIR: &str = "target/rustc-thingos/lib/rustlib";
/// Cached dynamic libraries needed by the Linux-hosted rustc binary.
const LIB_CACHE_DIR: &str = "target/rustc-thingos/lib";
/// Cached ThingOS-native rustc binary (runs on ThingOS, not Linux).
const THINGOS_RUSTC_BINARY: &str = "target/rustc-thingos/thingos-rustc";
/// Cached ThingOS sysroot (lib/rustlib tree) that the ThingOS-native rustc expects.
const THINGOS_RUSTLIB_CACHE_DIR: &str = "target/rustc-thingos/thingos-rustlib";

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
    if !Path::new(THINGOS_RUSTC_BINARY).exists() {
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

/// Locate the stage-1 `rustc` binary that was cross-compiled *for*
/// `x86_64-unknown-thingos` (the Canadian-cross artifact).
///
/// x.py places the cargo build artifact for each additional host under:
///   `build/<build-triple>/stage1-rustc/<host-triple>/release/rustc-main`
///
/// If it has been "installed" by x.py into the staged tree, it can also appear
/// under `build/<host-triple>/stage1/bin/rustc`.  Both paths are searched.
fn locate_thingos_rustc(cwd: &Path, rust_src: &Path) -> Option<PathBuf> {
    let root_build = xpy_build_root(cwd);
    let candidates = [
        root_build
            .join("stage1-rustc/x86_64-unknown-thingos/release/rustc-main"),
        // Installed stage-1 tree for the ThingOS host.
        cwd.join("build/x86_64-unknown-thingos/stage1/bin/rustc"),
        rust_src
            .join("build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-thingos/release/rustc-main"),
        rust_src.join("build/x86_64-unknown-thingos/stage1/bin/rustc"),
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
            let keep = matches!(path.extension().and_then(|ext| ext.to_str()), Some("so"))
                || path
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

/// Cache the ThingOS-native rustc binary and its sysroot (the rlibs it ships
/// with) so `stage_rustc_for_iso` can later copy them into the boot ISO.
///
/// The ThingOS sysroot layout mirrors what `locate_thingos_rustc` locates
/// plus the ThingOS target rlibs from `stage1-std`.  We do *not* copy Linux
/// shared libraries here – the ThingOS binary is statically linked.
fn cache_thingos_rustc_tree(sh: &Shell, cwd: &Path, thingos_rustc: &Path) -> Result<()> {
    let build_root = xpy_build_root(cwd);
    let cached_dir = cwd.join(THINGOS_RUSTLIB_CACHE_DIR);

    sh.remove_path(&cached_dir)?;
    sh.create_dir(&cached_dir)?;

    // Cache the ThingOS-native compiler binary itself.
    sh.copy_file(thingos_rustc, cwd.join(THINGOS_RUSTC_BINARY))?;
    #[cfg(unix)]
    {
        let binary_path = cwd.join(THINGOS_RUSTC_BINARY);
        let mut perms = std::fs::metadata(&binary_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&binary_path, perms)?;
    }

    // The ThingOS rustc's sysroot at runtime will be `/` (since the binary
    // sits at `/bin/rustc`).  Rustc therefore looks for rustlib at
    // `/lib/rustlib/`.  We cache the ThingOS target rlibs under the same
    // sub-path so the ISO staging step can mirror it verbatim.
    let thingos_rlibs_dst = cached_dir.join("x86_64-unknown-thingos/lib");
    sh.create_dir(thingos_rlibs_dst.parent().unwrap())?;
    sh.create_dir(&thingos_rlibs_dst)?;

    let thingos_std = build_root.join("stage1-std/x86_64-unknown-thingos/release/deps");
    if thingos_std.exists() {
        for entry in std::fs::read_dir(&thingos_std)? {
            let entry = entry?;
            let path = entry.path();
            let keep = matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("rlib") | Some("rmeta")
            );
            if keep {
                let file_name = path.file_name().unwrap();
                std::fs::copy(&path, thingos_rlibs_dst.join(file_name))?;
            }
        }
    } else {
        println!(
            "rustc-thingos: warning: ThingOS std rlibs not found at {:?}; \
             the ThingOS-native rustc will ship without prebuilt libraries",
            thingos_std
        );
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Bootstrap configuration
// ---------------------------------------------------------------------------

/// Generate a `config.toml` for `x.py` that builds a stage-1 `rustc`
/// that both runs on linux-gnu *and* runs natively on `x86_64-unknown-thingos`.
///
/// Bootstrap terminology:
///   `build`  = the machine running the build (always linux-gnu here)
///   `host`   = the machine(s) the resulting compiler will *run* on
///   `target` = the machine(s) the resulting compiler can produce code for
///
/// Including `x86_64-unknown-thingos` in the `host` list triggers a
/// Canadian-cross build: x.py uses the linux-gnu stage-1 compiler (built
/// using CI LLVM) to cross-compile a second stage-1 `rustc` binary that runs
/// *on* ThingOS.  No LLVM recompilation is required because the linker and
/// LLVM libraries are already present from the CI download; the ThingOS rustc
/// binary is statically linked (no shared libs, matching ThingOS's
/// `"dynamic-linking": false`).
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
# Include x86_64-unknown-thingos as a host so x.py produces a stage-1 rustc
# binary that runs natively on ThingOS (Canadian-cross build).
host  = ["x86_64-unknown-linux-gnu", "x86_64-unknown-thingos"]
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

/// Build a Linux-hosted and ThingOS-native stage-1 `rustc` and cache both
/// results under `target/rustc-thingos/`.
///
/// The bootstrap now includes `x86_64-unknown-thingos` in the `host` list,
/// so a single `x.py build --stage 1` run produces two compiler binaries:
///
/// * `target/rustc-thingos/rustc`         – runs on Linux (developer use)
/// * `target/rustc-thingos/thingos-rustc` – runs on ThingOS (ISO use)
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

    // Allow explicit opt-out for faster dev loops.
    if std::env::var("SKIP_RUSTC_THINGOS").as_deref() == Ok("1") {
        return Ok(None);
    }

    // Fast path: cache is still valid.
    if is_cache_valid() {
        println!("rustc-thingos: cache hit, reusing {}", RUSTC_BINARY);
        return Ok(Some(PathBuf::from(RUSTC_BINARY)));
    }

    println!("rustc-thingos: building stage-1 rustc (Linux-hosted + ThingOS-native) …");

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

    // Run the bootstrap.  We build both `library` and `compiler/rustc` so that
    // the stage-1 sysroot has prebuilt core/alloc/std for x86_64-unknown-linux-gnu
    // (needed when the stage-1 rustc is used as RUSTC for cargo builds — cargo
    // runs build scripts on the host and they need host std in the sysroot).
    // Because x86_64-unknown-thingos is now also in `host`, x.py will also
    // produce a stage-1 rustc cross-compiled to run on ThingOS.
    cmd!(
        sh,
        "python3 {rust_src_str}/x.py build --stage 1 library compiler/rustc"
    )
    .env("RUST_TARGET_PATH", target_dir_str)
    .run()?;

    // Locate and cache the Linux-hosted stage-1 compiler.
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

    // Locate and cache the ThingOS-native stage-1 compiler.
    match locate_thingos_rustc(&cwd, &rust_src) {
        Some(thingos_binary) => {
            cache_thingos_rustc_tree(sh, &cwd, &thingos_binary)?;
            println!(
                "rustc-thingos: ThingOS-native binary cached at {}",
                THINGOS_RUSTC_BINARY
            );
        }
        None => {
            println!(
                "rustc-thingos: warning: ThingOS-native rustc binary not found after bootstrap; \
                 ISO will not include a native compiler. \
                 Searched paths: build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-thingos/..."
            );
        }
    }

    write_cache_key()?;
    println!("rustc-thingos: Linux-hosted binary cached at {}", RUSTC_BINARY);
    Ok(Some(PathBuf::from(RUSTC_BINARY)))
}

/// Stage the ThingOS-native stage-1 `rustc` binary and its sysroot into the
/// boot ISO image so programs can be compiled natively on a running ThingOS
/// instance.
///
/// ## ISO layout
/// ```text
/// bin/rustc                                        ← compiler binary
/// lib/rustlib/x86_64-unknown-thingos/lib/*.rlib    ← prebuilt std rlibs
/// ```
///
/// With `rustc` at `/bin/rustc`, Rust's sysroot-detection code strips the
/// trailing `bin/` component and derives **sysroot = `/`**.  Rustlib is
/// therefore found at `/lib/rustlib/` — exactly what we stage here.
///
/// Users invoke the compiler on ThingOS with no extra flags:
/// ```
/// rustc --edition 2021 hello.rs
/// ```
pub fn stage_rustc_for_iso(sh: &Shell, iso_root: &Path) -> Result<()> {
    if std::env::var("SKIP_RUSTC_THINGOS").as_deref() == Ok("1") {
        return Ok(());
    }

    let thingos_rustc = Path::new(THINGOS_RUSTC_BINARY);
    if !thingos_rustc.exists() {
        println!(
            "rustc-thingos: ThingOS-native rustc not in cache ({}); skipping ISO staging.",
            THINGOS_RUSTC_BINARY
        );
        return Ok(());
    }

    // ── 1. Copy the compiler binary to bin/rustc ──────────────────────────
    let iso_bin = iso_root.join("bin");
    sh.create_dir(&iso_bin)?;
    let iso_rustc = iso_bin.join("rustc");
    sh.copy_file(thingos_rustc, &iso_rustc)?;
    #[cfg(unix)]
    {
        let mut perms = std::fs::metadata(&iso_rustc)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&iso_rustc, perms)?;
    }
    println!("rustc-thingos: staged {}", iso_rustc.display());

    // ── 2. Copy sysroot rlibs to lib/rustlib/ ─────────────────────────────
    let thingos_rustlib_src = Path::new(THINGOS_RUSTLIB_CACHE_DIR);
    if thingos_rustlib_src.exists() {
        let iso_rustlib = iso_root.join("lib/rustlib");
        sh.create_dir(iso_rustlib.parent().unwrap())?;
        sh.create_dir(&iso_rustlib)?;
        let src = thingos_rustlib_src.to_str().unwrap();
        let dst = iso_rustlib.to_str().unwrap();
        cmd!(sh, "cp -r {src}/. {dst}").run()?;
        println!("rustc-thingos: staged sysroot to {}", iso_rustlib.display());
    } else {
        println!(
            "rustc-thingos: warning: ThingOS sysroot cache not found at {}; \
             staged rustc will lack prebuilt rlibs",
            THINGOS_RUSTLIB_CACHE_DIR
        );
    }

    Ok(())
}

