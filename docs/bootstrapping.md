# ThingOS Hosted Compiler: Bootstrap Notes

This document describes the strategy and current status for building a
stage-1 `rustc` that runs natively on `x86_64-unknown-thingos` (a
"ThingOS-hosted" compiler).

## Overview

A ThingOS-hosted rustc enables on-device compilation — users can compile
Rust programs directly on a running ThingOS instance.  The compiler is
cross-compiled from a Linux/x86_64 build host using the vendored Rust fork
at `vendor/rust/` (branch `thingos-patched`).

## Build Flow

```
just fetch-rust          # clone / sync vendor/rust submodule
cargo xtask rustc-thingos  # generate config.toml, run x.py, cache binary
just iso                 # ISO includes /bin/rustc if SKIP_RUSTC_THINGOS≠1
```

Inside `cargo xtask rustc-thingos` (`xtask/src/rustc_thingos.rs`):

1. Generates `vendor/rust/config.toml` via `bootstrap_config()`.
2. Sets `RUST_TARGET_PATH=targets/` so `x.py` can find the custom JSON spec.
3. Runs `python3 vendor/rust/x.py build --stage 1 compiler/rustc`.
4. Copies the resulting binary to `target/rustc-thingos/rustc`.
5. Writes a cache-key file so subsequent runs with no relevant changes are
   no-ops.

## Target Capabilities (`targets/x86_64-unknown-thingos.json`)

ThingOS uses a static-only execution model: no dynamic linker, no shared
libraries at runtime.  The target spec reflects this:

| Field | Value | Effect |
|---|---|---|
| `"relocation-model"` | `"static"` | All code is position-dependent |
| `"position-independent-executables"` | `false` | No PIE / PIC |
| `"dynamic-linking"` | `false` | Dylib / proc-macro crate types disallowed |
| `"panic-strategy"` | `"abort"` | No unwinding runtime needed |

`"dynamic-linking": false` is the machine-readable signal to both cargo and
the Rust bootstrap that this target cannot build or load shared libraries.
It prevents cargo from accidentally attempting `--crate-type dylib` for user
code, and is the prerequisite for the bootstrap fix described below.

## Static-Linking Strategy for `rustc_driver`

### Problem

The Rust bootstrap builds `rustc_driver` as a `dylib` (shared library) so
that the `rustc` binary can dynamically load it at runtime.  On ThingOS,
dynamic loading is not available: the build fails at this step.

### Solution

Build `rustc_driver` as an `rlib` (static archive) and link it **directly
into the `rustc` binary**.  The resulting binary is larger but fully
self-contained — no `.so` files required.

### Changes already in the outer repo

* `"dynamic-linking": false` in `targets/x86_64-unknown-thingos.json`.
* `[rust] rpath = false` in the generated bootstrap `config.toml` (no rpath
  embedding needed for a static binary).
* `[target.x86_64-unknown-thingos] sanitizers = false  profiler = false` –
  these instrumentations ship as shared-library runtimes and are therefore
  not applicable.

### Required fork patch (`vendor/rust`, branch `thingos-patched`)

**File**: `src/bootstrap/src/core/build_steps/compile.rs`

The bootstrap currently passes `--crate-type dylib` when building
`rustc_driver`.  The patch should consult `target.supports_dylib()` (which
reads `dynamic_linking` from the target spec) and fall back to `--crate-type
rlib` for targets where dynamic linking is unavailable.

Pseudocode for the change:

```rust
// Locate the cargo invocation for rustc_driver and wrap the crate-type
// selection behind a capabilities check:

let crate_types = if builder.target_config_for(target)
    .and_then(|t| t.dynamic_linking)
    .unwrap_or(true)
{
    "dylib,rlib"
} else {
    // Target has no dynamic linker (e.g. x86_64-unknown-thingos).
    // Build a static rlib only; the rustc binary will link it in.
    "rlib"
};
cargo.env("RUSTC_BOOTSTRAP_CRATE_TYPES_RUSTC_DRIVER", crate_types);
```

The precise mechanism differs between bootstrap versions.  The simplest
approach for the current fork is to look for:

```rust
// In compile.rs – the Rustc build step – search for "rustc_driver"
// and "dylib".  Wrap the dylib path with:
if !target.triple.contains("thingos") {
    // existing dylib build
} else {
    // rlib-only path
}
```

Once the fork patch is committed to `thingos-patched`, the build should
proceed all the way to a stage-1 `rustc` ELF at:

```
vendor/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
```

## Caching

The xtask caches the built binary and the `rustlib/` directory under
`target/rustc-thingos/`.  The cache is keyed on:

* Content of `targets/x86_64-unknown-thingos.json`
* Content of `rust-toolchain.toml`
* Git HEAD of `vendor/rust/`

Any change to these files invalidates the cache and triggers a rebuild.
Set `SKIP_RUSTC_THINGOS=1` to bypass the build entirely.

## ISO Staging

When the binary exists, `stage_rustc_for_iso()` copies it into the ISO at:

* `/bin/rustc` – the compiler binary
* `/usr/lib/rustlib/` – standard-library rlibs for the thingos target

These paths mirror the standard rustup/toolchain layout so that on-device
`rustc` invocations work without additional configuration.

## Compatibility with `just fetch-rust`

`just fetch-rust` always resets `vendor/rust/` to the submodule-pinned
commit on `thingos-patched`.  The generated `config.toml` is written into
`vendor/rust/` at build time and is not tracked by the submodule.  No
manual step is needed after a `just fetch-rust` to restore the config.

## Current Status

See `docs/status/rustc_build.md` for the up-to-date build status.
