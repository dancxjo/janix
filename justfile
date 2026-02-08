# ThingOS Build System
# Uses xtask (Rust) for build automation

# Target architecture to build for. Default to x86_64.
karch := env_var_or_default("KARCH", "x86_64")

# Default user QEMU flags
qemuflags := env_var_or_default("QEMUFLAGS", "-m 2G -smp 6")

# Rust profile (dev/release)
rust_profile := env_var_or_default("RUST_PROFILE", "dev")

# Default target
default: iso

# Check UI split
check-ui-split:
    ./scripts/ci_check_ui_split.sh

# Audit platform boundary (verify no_std compliance)
audit-platform:
    python3 scripts/audit_platform_boundary.py

# Alias for iso
build arch=karch:
    @just iso {{arch}}

# Build everything (ISO) - optionally specify architecture
# Examples: just iso, just iso aarch64
iso arch=karch:
    cargo xtask iso --env {{arch}} --profile {{rust_profile}}

# Build HDD image
hdd arch=karch:
    cargo xtask hdd --env {{arch}} --profile {{rust_profile}}

# Run with QEMU (UEFI mode)
# Examples: just run, just run aarch64, just run riscv64
run arch=karch:
    RUSTFLAGS="-Awarnings" cargo xtask run --env {{arch}} --profile {{rust_profile}} --qemu-flags "{{qemuflags}} -device virtio-sound-pci,audiodev=snd0 -audiodev alsa,id=snd0"

# Start HTTPS proxy for guest (runs on port 8081)
# Guest accesses via: http://10.0.2.2:8081/?url=https://example.com/
proxy port="8081":
    python3 scripts/https_proxy.py {{port}}

# Start HTTPS reverse proxy for host browser access to guest anther
# Wraps guest's HTTP server (port 8888) with self-signed HTTPS
# Browser accesses: https://localhost:8443/
https-proxy port="8443" target="8888":
    cargo xtask https-proxy --port {{port}} --target {{target}}

# Run QEMU with proxy (starts proxy in background, then QEMU)
run-with-proxy arch=karch port="8081":
    #!/usr/bin/env bash
    echo "Starting HTTPS proxy on port {{port}}..."
    python3 scripts/https_proxy.py {{port}} &
    PROXY_PID=$!
    trap "kill $PROXY_PID 2>/dev/null" EXIT
    echo "Proxy PID: $PROXY_PID"
    RUSTFLAGS="-Awarnings" cargo xtask run --env {{arch}} --profile {{rust_profile}} --qemu-flags "{{qemuflags}}"

# Run HDD with QEMU
run-hdd arch=karch:
    cargo xtask run-hdd --env {{arch}} --profile {{rust_profile}} --qemu-flags "{{qemuflags}}"

# Run with BIOS (x86_64 only)
run-bios:
    cargo xtask run-bios --qemu-flags "{{qemuflags}}"

# Build the kernel
kernel arch=karch:
    cargo xtask build --env {{arch}} --profile {{rust_profile}}

# Clone and build limine bootloader
limine:
    cargo xtask limine

# Download OVMF firmware (all architectures by default)
ovmf:
    cargo xtask ovmf-all

# Clean build artifacts
clean:
    cargo xtask clean

# Clean everything including downloaded dependencies
distclean:
    cargo xtask distclean

# Run BDD tests (default: all architectures)
# Examples:
#   just behave                    # all architectures
#   just behave --arch x86_64      # single architecture
#   just behave --feature simple-boot
#   just behave --tags @smoke
behave *args:
    RUSTFLAGS="-Awarnings" cargo xtask bdd {{args}}

# Alias for behave
bdd *args:
    @just behave {{args}}

# Clear all BDD behavior reports (preserves .feature files and top-level README)
clear-behavior:
    rm -rf docs/behavior/x86_64 docs/behavior/aarch64 docs/behavior/riscv64 docs/behavior/loongarch64

# Run masked compositing microbenchmarks
bench-blit arch=karch:
    RUSTFLAGS="-Awarnings" cargo xtask run --env {{arch}} --profile {{rust_profile}} --qemu-flags "{{qemuflags}}" --userspace bench_blit

# CI smoke test for benchmarks (ensures they build)
bench-smoke:
    ./scripts/ci_bench_smoke.sh

# Kill all running QEMU instances
die:
    cargo xtask kill

# Build sprout user app (uses build-std for bare metal)
sprout arch=karch:
    #!/usr/bin/env bash
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    echo "Building sprout for $TARGET_ARCH using $TARGET_JSON..."
    cargo +nightly build -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem --target "$TARGET_JSON" -p sprout

# Build rtc_cmos user app
rtc_cmos arch=karch:
    #!/usr/bin/env bash
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    echo "Building rtc_cmos for $TARGET_ARCH using $TARGET_JSON..."
    cargo +nightly build -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem --target "$TARGET_JSON" -p rtc_cmos

# Build clock user app
clock arch=karch:
    #!/usr/bin/env bash
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    echo "Building clock for $TARGET_ARCH using $TARGET_JSON..."
    cargo +nightly build -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem --target "$TARGET_JSON" -p clock

# Build bristle user app
bristle arch=karch:
    #!/usr/bin/env bash
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    echo "Building bristle for $TARGET_ARCH using $TARGET_JSON..."
    cargo +nightly build -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem --target "$TARGET_JSON" -p bristle

# Fetch vendor assets (Limine, OVMF, Fonts, Icons, Cursors)
fetch:
    cargo run -p xtask --features svg-cursors -- fetch

# Run all unit tests (host-testable crates only)
test *args:
    cargo test -p abi -p pciids -p xtask {{args}}

# Check everything (compilation + UI split)
check: check-ui-split
    cargo +nightly check -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem --target targets/x86_64-unknown-thingos.json -p bloom -p blossom

# -----------------------------------------------------------------------------
# Rust std patch-queue workflow (Ramp A)
# -----------------------------------------------------------------------------

# Prepare a local rust-lang/rust checkout at the ThingOS std baseline.
# Example: just rust-std-setup ../rust
rust-std-setup rust_dir="../rust":
    #!/usr/bin/env bash
    set -euo pipefail
    BASELINE="286fbe5d84569c718f189122db9e68a16b50eeef"
    if [ ! -d "{{rust_dir}}/.git" ]; then
        git clone https://github.com/rust-lang/rust "{{rust_dir}}"
    fi
    git -C "{{rust_dir}}" fetch --all --tags
    if [ -n "$(git -C "{{rust_dir}}" status --porcelain)" ]; then
        echo "Rust checkout is dirty: {{rust_dir}}" >&2
        echo "Please commit/stash changes before running rust-std-setup." >&2
        exit 1
    fi
    git -C "{{rust_dir}}" checkout "$BASELINE"
    if git -C "{{rust_dir}}" rev-parse --verify thingos-std >/dev/null 2>&1; then
        git -C "{{rust_dir}}" checkout thingos-std
    else
        git -C "{{rust_dir}}" checkout -b thingos-std
    fi
    cp toolchains/rust/bootstrap.toml.example "{{rust_dir}}/bootstrap.toml"

# Apply ThingOS rust std patch queue to a rust checkout.
# Example: just rust-std-apply ../rust
rust-std-apply rust_dir="../rust":
    ./toolchains/rust/scripts/apply.sh "{{rust_dir}}"

# Build library/std for a ThingOS JSON target from patched rust checkout.
# Examples:
#   just rust-std-build
#   just rust-std-build ../rust aarch64
#   just rust-std-build ../rust riscv64
rust-std-build rust_dir="../rust" arch=karch:
    #!/usr/bin/env bash
    set -euo pipefail
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="$(pwd)/targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="$(pwd)/targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    (cd "{{rust_dir}}" && ./x.py build library/std --target "$TARGET_JSON")

# Build an opt-in ThingOS std demo without changing workspace userspace crates.
# By default this performs a compile/link proof and validates the ELF entrypoint.
# Optional `boot=1` injects it as `/boot/beeper` into the HDD image, and `run=1`
# then boots QEMU.
#
# Example:
#   just rust-std-demo
#   just rust-std-demo ../rust 1 1
rust-std-demo rust_dir="../rust" boot="0" run="0":
    #!/usr/bin/env bash
    set -euo pipefail
    ROOT="$(pwd)"
    TARGET_JSON="$ROOT/targets/x86_64-unknown-thingos.json"
    if [[ "{{rust_dir}}" = /* ]]; then
        RUST_DIR="{{rust_dir}}"
    else
        RUST_DIR="$ROOT/{{rust_dir}}"
    fi
    SYSROOT="$RUST_DIR/build/x86_64-unknown-linux-gnu/stage2"
    RUSTC="$SYSROOT/bin/rustc"
    DEMO_SRC="$ROOT/toolchains/rust/demo/std_demo.rs"
    DEMO_OUT="$ROOT/target/std-demo/beeper"
    HDD="$ROOT/thing-os-x86_64.hdd"

    just rust-std-build "$RUST_DIR" x86_64
    mkdir -p "$ROOT/target/std-demo"
    (cd "$ROOT" && "$RUSTC" -Zunstable-options --target "$TARGET_JSON" --sysroot "$SYSROOT" "$DEMO_SRC" -o "$DEMO_OUT")

    ENTRY="$(readelf -h "$DEMO_OUT" | awk '/Entry point address/ {print $4}')"
    if [ "$ENTRY" = "0x0" ]; then
        echo "std demo link proof failed: entrypoint is 0x0" >&2
        exit 1
    fi
    echo "std demo compile/link proof OK: $DEMO_OUT (entry $ENTRY)"

    if [ "{{boot}}" = "1" ]; then
        cargo xtask hdd --env x86_64 --profile {{rust_profile}}
        mcopy -o -i "$HDD"@@1M "$DEMO_OUT" ::/boot/beeper
        echo "Injected std demo as /boot/beeper"
        if [ "{{run}}" = "1" ]; then
            echo "Booting HDD image..."
            cargo xtask run-hdd --env x86_64 --profile {{rust_profile}} --qemu-flags "{{qemuflags}}"
        else
            echo "Skipping QEMU run (run={{run}})"
        fi
    fi

# Run smoke tests (quick boot validation)
smoke:
    cargo xtask bdd --arch x86_64 --tags @smoke
