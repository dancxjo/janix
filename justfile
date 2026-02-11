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

# Run smoke tests (quick boot validation)
smoke:
    cargo xtask bdd --arch x86_64 --tags @smoke

# --- Vendored Rust Standard Library ---

# The commit hash of rust-lang/rust matching our nightly toolchain
rust_commit := "9e79395f92bff6a8f536430e42a4beae69f60ff8"

# Fetch (shallow clone) the Rust source tree into vendor/rust/
fetch-rust:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -d vendor/rust/.git ]; then
        echo "vendor/rust already exists, skipping clone."
        echo "  To re-fetch, run: just rust-reset  (or rm -rf vendor/rust)"
    else
        echo "==> Shallow-cloning rust-lang/rust at {{rust_commit}}..."
        git clone --depth 1 --filter=blob:none --no-checkout \
            https://github.com/rust-lang/rust.git vendor/rust
        cd vendor/rust
        git fetch --depth 1 origin {{rust_commit}}
        git checkout {{rust_commit}}
        echo "==> Rust source ready at vendor/rust/"
    fi
    # Ensure library submodules needed for std are initialized
    if [ ! -f vendor/rust/library/backtrace/Cargo.toml ]; then
        echo "==> Initializing library/backtrace submodule..."
        cd vendor/rust && git submodule update --init --depth 1 library/backtrace
    fi
    echo "  library/std/src/lib.rs exists: $(test -f vendor/rust/library/std/src/lib.rs && echo yes || echo no)"

# Save local modifications in vendor/rust/ as patches
rust-save-patches:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p patches/rust
    cd vendor/rust
    # Stage new (untracked) files so they appear in git diff
    git add -N .
    if git diff --quiet && git diff --cached --quiet; then
        echo "No changes to save."
        exit 0
    fi
    git diff > ../../patches/rust/thingos-pal.patch
    echo "==> Saved patch to patches/rust/thingos-pal.patch"
    echo "  $(wc -l < ../../patches/rust/thingos-pal.patch) lines"
    echo "  Files changed: $(grep -c '^diff' ../../patches/rust/thingos-pal.patch)"

# Hard-reset vendor/rust/ to the pinned commit (discards local changes)
rust-reset:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d vendor/rust/.git ]; then
        echo "vendor/rust does not exist. Run: just fetch-rust"
        exit 1
    fi
    cd vendor/rust
    # Un-stage any intent-to-add files (from git add -N)
    git reset HEAD -- . 2>/dev/null || true
    git checkout -- .
    git clean -fd
    echo "==> vendor/rust reset to {{rust_commit}}"

# Apply saved patches from patches/rust/ to vendor/rust/
rust-apply-patches:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d vendor/rust/.git ]; then
        echo "vendor/rust does not exist. Run: just fetch-rust"
        exit 1
    fi
    if [ ! -f patches/rust/thingos-pal.patch ]; then
        echo "No patches found in patches/rust/. Nothing to apply."
        exit 0
    fi
    cd vendor/rust
    git apply ../../patches/rust/thingos-pal.patch
    echo "==> Applied patches/rust/thingos-pal.patch"

# Build a userspace app with vendored std (experimental)
rust-build-std app="hello_std" arch=karch:
    #!/usr/bin/env bash
    set -euo pipefail
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    # Point build-std at our vendored (patchable) Rust library source
    export __CARGO_TESTS_ONLY_SRC_ROOT="$(pwd)/vendor/rust/library"
    echo "Building {{app}} for $TARGET_ARCH with vendored Rust source..."
    echo "  std source: $__CARGO_TESTS_ONLY_SRC_ROOT"
    RUSTFLAGS="-Awarnings" cargo +nightly \
        -Z build-std=core,alloc,std,panic_abort \
        -Z build-std-features=compiler-builtins-mem \
        -Z json-target-spec \
        build --target "$TARGET_JSON" -p {{app}}
