#!/bin/bash
set -e

# This script sets up a custom Rust toolchain 'thingos' with patched std.
# Usage: ./scripts/setup_thingos_toolchain.sh

REPO_ROOT=$(pwd)
WORK_DIR="/tmp/rust-thingos-toolchain"
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR"

echo "Using nightly-2025-12-27..."
NIGHTLY_SYSROOT=$(rustc +nightly-2025-12-27 --print sysroot)
CUSTOM_SYSROOT="$WORK_DIR/custom-sysroot"
mkdir -p "$CUSTOM_SYSROOT"

# Copy bin and libexec
cp -r "$NIGHTLY_SYSROOT/bin" "$CUSTOM_SYSROOT/bin"
cp -r "$NIGHTLY_SYSROOT/libexec" "$CUSTOM_SYSROOT/libexec"

# Copy lib structure (shallow copy of lib, deep copy of rustlib except src)
mkdir -p "$CUSTOM_SYSROOT/lib/rustlib"
for f in "$NIGHTLY_SYSROOT/lib/"*; do
    if [ "$(basename "$f")" != "rustlib" ]; then
        ln -s "$f" "$CUSTOM_SYSROOT/lib/"
    fi
done

for f in "$NIGHTLY_SYSROOT/lib/rustlib/"*; do
    NAME=$(basename "$f")
    if [ "$NAME" != "src" ]; then
        ln -s "$f" "$CUSTOM_SYSROOT/lib/rustlib/"
    fi
done

# Copy source
mkdir -p "$CUSTOM_SYSROOT/lib/rustlib/src/rust"
SYS_RUST_SRC="$NIGHTLY_SYSROOT/lib/rustlib/src/rust"
echo "Copying rust-src..."
cp -r "$SYS_RUST_SRC/library" "$CUSTOM_SYSROOT/lib/rustlib/src/rust/"
cp "$SYS_RUST_SRC/Cargo.toml" "$CUSTOM_SYSROOT/lib/rustlib/src/rust/" 2>/dev/null || true
cp "$SYS_RUST_SRC/Cargo.lock" "$CUSTOM_SYSROOT/lib/rustlib/src/rust/" 2>/dev/null || true

# Apply patches
SRC_ROOT="$CUSTOM_SYSROOT/lib/rustlib/src/rust"
echo "Applying patches..."

DEST_THINGOS_PAL="$SRC_ROOT/library/std/src/sys/pal/thingos"
mkdir -p "$DEST_THINGOS_PAL"
cp patches/rust-thingos/mod.rs "$DEST_THINGOS_PAL/mod.rs"
cp patches/rust-thingos/env.rs "$DEST_THINGOS_PAL/env.rs"
cp patches/rust-thingos/thread_local_key.rs "$DEST_THINGOS_PAL/thread_local_key.rs"
# args is handled by sys/args/unsupported, so PAL doesn't need it.
# time is handled by sys/time, so PAL doesn't need it.

# alloc files
DEST_THINGOS_ALLOC="$SRC_ROOT/library/std/src/sys/alloc"
cp patches/rust-thingos/alloc.rs "$DEST_THINGOS_ALLOC/thingos.rs"

# stdio files
DEST_THINGOS_STDIO="$SRC_ROOT/library/std/src/sys/stdio"
cp patches/rust-thingos/stdio.rs "$DEST_THINGOS_STDIO/thingos.rs"

# thread files
DEST_THINGOS_THREAD="$SRC_ROOT/library/std/src/sys/thread"
cp patches/rust-thingos/thread.rs "$DEST_THINGOS_THREAD/thingos.rs"

# time files
DEST_THINGOS_TIME="$SRC_ROOT/library/std/src/sys/time"
mkdir -p "$DEST_THINGOS_TIME"
cp patches/rust-thingos/time.rs "$DEST_THINGOS_TIME/thingos.rs"

# Apply replacement mod.rs files
cp patches/rust-thingos/alloc_mod_replacement.rs "$SRC_ROOT/library/std/src/sys/alloc/mod.rs"
cp patches/rust-thingos/stdio_mod_replacement.rs "$SRC_ROOT/library/std/src/sys/stdio/mod.rs"
cp patches/rust-thingos/time_mod_replacement.rs "$SRC_ROOT/library/std/src/sys/time/mod.rs"
cp patches/rust-thingos/thread_mod_replacement.rs "$SRC_ROOT/library/std/src/sys/thread/mod.rs"
cp patches/rust-thingos/thread_local_mod_replacement.rs "$SRC_ROOT/library/std/src/sys/thread_local/mod.rs"
cp patches/rust-thingos/random_mod_replacement.rs "$SRC_ROOT/library/std/src/sys/random/mod.rs"

# Patch sys/pal/mod.rs
cd "$SRC_ROOT"
patch -p1 < "$REPO_ROOT/patches/rust-thingos/0001-add-thingos-pal.patch"

# Link toolchain
INSTALL_DIR="$HOME/.rustup/toolchains/thingos"
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing existing thingos toolchain..."
    rustup toolchain uninstall thingos
fi

echo "Moving toolchain to $INSTALL_DIR..."
mv "$CUSTOM_SYSROOT" "$INSTALL_DIR"

rustup toolchain link thingos "$INSTALL_DIR"

echo "Toolchain 'thingos' setup complete."
echo "Use 'cargo +thingos build -Z build-std=std,panic_abort --target x86_64-unknown-thingos.json'"
