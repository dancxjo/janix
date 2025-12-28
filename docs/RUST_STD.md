# ThingOS Rust Std Port

This directory contains patches and scripts to build a custom Rust toolchain with `std` support for ThingOS.

## Setup

1.  Ensure you have `rustup` installed and `nightly-2025-12-27` toolchain available.
2.  Run the setup script:
    ```bash
    ./scripts/setup_thingos_toolchain.sh
    ```
    This will create a linked toolchain named `thingos`.

## Usage

To build a ThingOS application with `std`:

```bash
cargo +thingos build -Z build-std=std,panic_abort --target targets/x86_64-unknown-thingos.json --release
```

Note: You may need `#![feature(restricted_std)]` in your crate root if the target is not recognized as a standard tier 3 target yet.

## Patches

*   `patches/rust-thingos/`: Contains the source files and module replacements for `std`.
    *   `mod.rs`: PAL module entry point.
    *   `alloc.rs`: Global allocator (stub).
    *   `stdio.rs`: Standard I/O using `SYSCALL_LOG`.
    *   `thread.rs`: Threading stubs (yield, sleep).
    *   `time.rs`: Time implementation (Instant, SystemTime).
    *   `*_mod_replacement.rs`: Replacements for `std` internal modules (`sys/alloc`, `sys/stdio`, etc.) to hook up the ThingOS backend.
