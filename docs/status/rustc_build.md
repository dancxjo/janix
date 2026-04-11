# Rustc on ThingOS: Current Status (April 2026)

The effort to build a stage-1 `rustc` cross-compiled for `x86_64-unknown-thingos` is currently **on hold** due to upstream LLVM compilation issues.

## Current State

As of April 11, 2026, the `rustc-thingos` target in `xtask` is configured to build a stage-1 compiler. While the Rust bootstrap and host tooling downloads succeed, the build fails during the LLVM compilation phase.

### Technical Blocker: LLVM Target Incompatibility

The primary blocker is that the custom target triple `x86_64-unknown-thingos` is not recognized by the vendored LLVM build system as a standard POSIX or recognized generic target. This leads to the following C++ compilation errors in `LLVMSupport`:

- `error: 'EnvPathSeparator' was not declared in this scope`
- `error: 'class llvm::sys::fs::file_status' has no member named 'getSize'`

These errors occur because LLVM's platform-specific headers fall back to a "Generic" configuration that is missing critical definitions required by the Rest compiler's support library.

## How to Trigger the Build

The build is currently **disabled by default** in the main `just run` and `just iso` workflows. To attempt a build anyway, use the dedicated xtask command:

```bash
cargo xtask rustc-thingos
```

Or re-enable the commented-out calls in `xtask/src/main.rs`.

## Requirements for Future Work

To resolve the current blockers, the following areas need investigation:

1. **LLVM Shims**: Add proper platform shims to the LLVM source tree (under `vendor/rust/src/llvm-project/`) specifically for ThingOS.
2. **Target Specification**: Refine the custom JSON target spec to better align with what LLVM expects for a Unix-like generic target.
3. **Cross-Compilation Toolchain**: Ensure a proper C++ standard library is available for the "Generic" target during the cross-compilation of LLVM itself.

---
*Document created: April 11, 2026*
*Status: Blocked / Experimental*
