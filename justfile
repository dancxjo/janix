# ThingOS Build System
# Uses xtask (Rust) for build automation

# Target architecture to build for. Default to x86_64.
karch := env_var_or_default("KARCH", "x86_64")

# Default user QEMU flags
qemuflags := env_var_or_default("QEMUFLAGS", "-m 2G")

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
    RUSTFLAGS="-Awarnings" cargo xtask run --env {{arch}} --profile {{rust_profile}} --qemu-flags "{{qemuflags}}"

# Start HTTPS proxy for guest (runs on port 8080)
# Guest accesses via: http://10.0.2.2:8080/?url=https://example.com/
proxy port="8080":
    python3 scripts/https_proxy.py {{port}}

# Run QEMU with proxy (starts proxy in background, then QEMU)
run-with-proxy arch=karch port="8080":
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

# Run smoke tests (quick boot validation)
smoke:
    cargo xtask bdd --arch x86_64 --tags @smoke
