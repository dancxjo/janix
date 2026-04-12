# Rustc on ThingOS: Current Status (April 2026)

The effort to build a stage-1 `rustc` cross-compiled for `x86_64-unknown-thingos`
is **in progress**. The vendored LLVM tree now carries a ThingOS-specific CMake
classification so LLVM no longer falls back to the Generic support headers when
building for a ThingOS host target.

## LLVM Host Detection Fix

The vendored Rust/LLVM sources now treat `thingos` as Unix-like during LLVM's
CMake platform setup:

* `src/bootstrap/src/core/build_steps/llvm.rs` now sets
  `CMAKE_SYSTEM_NAME=ThingOS` for `thingos` targets.
* `src/llvm-project/llvm/cmake/modules/HandleLLVMOptions.cmake` maps
  `ThingOS` to `LLVM_ON_UNIX=1`.
* `src/llvm-project/llvm/cmake/config-ix.cmake` treats `ThingOS` as a known
  Unix-like platform for the support-library configuration probes.

This addresses the earlier LLVMSupport failures where the Generic fallback
omitted Unix-only definitions such as `EnvPathSeparator` and
`sys::fs::file_status::getSize()`.

## Current State

The remaining end-to-end blocker has not been revalidated in this pass. Older
notes below about `rustc_driver` reflect the historical state of the effort and
should be treated as background, not as a freshly confirmed current failure.

### Completed Work (already landed)

| Location | Commit | Description |
|---|---|---|
| Outer repo (`trunk`) | `229ceee0` | `build: relax rust bootstrap warnings for thingos` |
| Rust fork (`thingos-patched`) | `125fe06c` | PAL/module fixes, restricted-std enablement, bootstrap proc-macro probe fix |

The following also land with the PR that introduced this status update:

* `targets/x86_64-unknown-thingos.json` – `"dynamic-linking": false` added so
  that every Rust tool chain (cargo, bootstrap, the compiler itself) knows this
  target cannot produce or load shared libraries.
* `xtask/src/rustc_thingos.rs` – generated `config.toml` now sets
  `[rust] rpath = false` and declares
  `[target.x86_64-unknown-thingos] sanitizers = false` and `profiler = false`,
  disabling runtime components that require dynamic libraries.

### Active Blocker: `rustc_driver` dylib

The Rust bootstrap unconditionally builds `rustc_driver` with
`--crate-type dylib`.  Compiling that crate type for a target with
`dynamic-linking = false` is rejected by the compiler, halting the stage-1
`compiler/rustc` build.

**Verification steps** (with the fork checked out):

```bash
# succeeds:
python3 vendor/rust/x.py build --stage 1 library/std \
    --host x86_64-unknown-thingos --target x86_64-unknown-thingos

# proceeds past previous failures, then stops at rustc_driver dylib step:
python3 vendor/rust/x.py build --stage 1 compiler/rustc \
    --host x86_64-unknown-thingos --target x86_64-unknown-thingos
```

## Static-Linking Strategy

The intended fix is to build `rustc_driver` as a **static rlib** and link it
directly into the `rustc` binary.  This produces a fully self-contained
`rustc` ELF suitable for a system without a dynamic linker.

### Required change in the Rust fork (`vendor/rust`, branch `thingos-patched`)

The key file is:

```
src/bootstrap/src/core/build_steps/compile.rs
```

In the function(s) that construct the `rustc_driver` cargo invocation the
bootstrap should check whether the *host* target supports dynamic linking and,
if not, switch to `--crate-type rlib` only, while also ensuring the main
`compiler/rustc` Cargo.toml dependency on `rustc_driver` resolves to that
rlib.  A minimal patch would be:

```rust
// Before calling cargo for rustc_driver, check host capability:
let use_dylib = builder.config.rust_rpath
    && !target.triple.contains("thingos");

if use_dylib {
    cargo.arg("--crate-type").arg("dylib,rlib");
} else {
    cargo.arg("--crate-type").arg("rlib");
}
```

The exact insertion point varies with the bootstrap version; search for
`rustc_driver` and `crate-type` in that file to locate it.

Alternatively (and more robustly), read the target spec's
`supports_dylib()` / `dynamic_linking` field that is already
surfaced by `rustc_target::spec::Target`.

### Outer-repo side already done

* `"dynamic-linking": false` in `targets/x86_64-unknown-thingos.json`
  makes the limitation machine-readable and prevents cargo from
  accidentally attempting a dylib build for user code.
* `[rust] rpath = false` in the generated `config.toml` keeps the
  bootstrap aligned with static-only linking.

## How to Trigger the Build

```bash
# Full attempt (will block at rustc_driver until fork patch lands):
cargo xtask rustc-thingos

# Skip rustc-thingos (e.g. in CI where build time is constrained):
SKIP_RUSTC_THINGOS=1 just iso
```

## Requirements for Remaining Work

1. **Fork patch** – implement the static-link fallback for `rustc_driver` in
   `src/bootstrap/src/core/build_steps/compile.rs` as described above.
2. **Verify stage-1 compiler/rustc** – ensure a statically-linked `rustc`
   binary is produced under
   `build/x86_64-unknown-linux-gnu/stage1/bin/rustc`.
3. **End-to-end smoke test** – boot ThingOS, copy the binary in from the ISO,
   run a trivial `rustc hello.rs` on the device.

---
*Document last updated: April 12, 2026*
*Status: In progress – fork patch needed for `rustc_driver` static link*
