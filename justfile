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
iso arch=karch: ensure-rust-patched
    cargo xtask iso --env {{arch}} --profile {{rust_profile}}

# Build HDD image
hdd arch=karch:
    cargo xtask hdd --env {{arch}} --profile {{rust_profile}}

# Run with QEMU (UEFI mode)
# Examples: just run, just run aarch64, just run -i, just run x86_64 -i
run *args: ensure-rust-patched
    #!/usr/bin/env bash
    set -e
    ARCH="{{karch}}"
    # We use a helper to split args into a proper array
    ARGS_ARRAY=({{args}})
    if [[ "${ARGS_ARRAY[0]}" != "" && "${ARGS_ARRAY[0]}" != -* ]]; then
        ARCH="${ARGS_ARRAY[0]}"
        # Shift the array
        ARGS_ARRAY=("${ARGS_ARRAY[@]:1}")
    fi
    RUSTFLAGS="-Awarnings" cargo xtask run --env "$ARCH" --profile "{{rust_profile}}" "${ARGS_ARRAY[@]}" --qemu-flags "{{qemuflags}}"

# Start HTTPS proxy for guest (runs on port 8081)
# Guest accesses via: http://10.0.2.2:8081/?url=https://example.com/
proxy port="8081":
    cargo xtask guest-proxy --port {{port}}

# Start HTTPS reverse proxy for host browser access to guest anther
# Wraps guest's HTTP server (port 8888) with self-signed HTTPS
# Browser accesses: https://localhost:8443/
https-proxy port="8443" target="8888":
    cargo xtask https-proxy --port {{port}} --target {{target}}

# Run QEMU with proxy (starts proxy in background, then QEMU)
run-with-proxy arch=karch port="8081":
    #!/usr/bin/env bash
    echo "Starting HTTPS proxy on port {{port}}..."
    cargo xtask guest-proxy --port {{port}} &
    PROXY_PID=$!
    trap "kill $PROXY_PID 2>/dev/null" EXIT
    echo "Proxy PID: $PROXY_PID"
    RUSTFLAGS="-Awarnings" cargo xtask run --env {{arch}} --profile {{rust_profile}} --qemu-flags "{{qemuflags}}"

# Run HDD with QEMU
run-hdd *args:
    #!/usr/bin/env bash
    set -e
    ARCH="{{karch}}"
    # If first arg doesn't start with -, treat as arch
    if [[ "$1" != "" && "$1" != -* ]]; then
        ARCH="$1"
        shift
    fi
    cargo xtask run-hdd --env "$ARCH" --profile "{{rust_profile}}" "$@" --qemu-flags "{{qemuflags}}"

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

# Clean build artifacts plus fetched/vendor state
clean:
    cargo xtask clean

# Compatibility alias for clean
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

# Kill all running QEMU instances
die:
    cargo xtask kill

# Build sprout user app
sprout arch=karch: fetch-rust
    #!/usr/bin/env bash
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    export __CARGO_TESTS_ONLY_SRC_ROOT="$(pwd)/vendor/rust/library"
    echo "Building sprout for $TARGET_ARCH using $TARGET_JSON..."
    RUSTFLAGS="-Awarnings" cargo -Z build-std=core,alloc,std,panic_abort -Z build-std-features=compiler-builtins-mem -Z json-target-spec build --target "$TARGET_JSON" -p sprout

# Build rtc_cmos user app
rtc_cmos arch=karch: fetch-rust
    #!/usr/bin/env bash
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    export __CARGO_TESTS_ONLY_SRC_ROOT="$(pwd)/vendor/rust/library"
    echo "Building rtc_cmos for $TARGET_ARCH using $TARGET_JSON..."
    RUSTFLAGS="-Awarnings" cargo -Z build-std=core,alloc,std,panic_abort -Z build-std-features=compiler-builtins-mem -Z json-target-spec build --target "$TARGET_JSON" -p rtc_cmos

# Build clock user app
clock arch=karch: fetch-rust
    #!/usr/bin/env bash
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    export __CARGO_TESTS_ONLY_SRC_ROOT="$(pwd)/vendor/rust/library"
    echo "Building clock for $TARGET_ARCH using $TARGET_JSON..."
    RUSTFLAGS="-Awarnings" cargo -Z build-std=core,alloc,std,panic_abort -Z build-std-features=compiler-builtins-mem -Z json-target-spec build --target "$TARGET_JSON" -p clock

# Build bristle user app
bristle arch=karch: fetch-rust
    #!/usr/bin/env bash
    TARGET_ARCH="{{arch}}"
    if [ "$TARGET_ARCH" == "riscv64" ]; then
        TARGET_JSON="targets/riscv64gc-unknown-thingos.json"
    else
        TARGET_JSON="targets/${TARGET_ARCH}-unknown-thingos.json"
    fi
    export __CARGO_TESTS_ONLY_SRC_ROOT="$(pwd)/vendor/rust/library"
    echo "Building bristle for $TARGET_ARCH using $TARGET_JSON..."
    RUSTFLAGS="-Awarnings" cargo -Z build-std=core,alloc,std,panic_abort -Z build-std-features=compiler-builtins-mem -Z json-target-spec build --target "$TARGET_JSON" -p bristle

# Build stage-1 rustc cross-compiled to run on x86_64-unknown-thingos.
# Caches the result under target/rustc-thingos/; a second run with no
# relevant changes is a no-op.  Set SKIP_RUSTC_THINGOS=1 to skip entirely.
rustc-thingos: fetch-rust
    cargo xtask rustc-thingos

# Fetch vendor assets (Limine, OVMF, Fonts, Icons, Cursors)
fetch:
    cargo run -p xtask --features svg-cursors -- fetch

# Run all unit tests (host-testable crates only)
test *args:
    cargo test \
        -p abi \
        -p abi-macros \
        -p pciids \
        -p xtask \
        -p stem \
        -p stem-macros \
        -p bulb \
        -p llm \
        -p llm_stub \
        -p fb_common \
        {{args}}

# Check everything (compilation + UI split)
check: check-ui-split ensure-rust-patched
    export __CARGO_TESTS_ONLY_SRC_ROOT="$(pwd)/vendor/rust/library"
    cargo -Z build-std=core,alloc,std,panic_abort -Z build-std-features=compiler-builtins-mem -Z json-target-spec check --target targets/x86_64-unknown-thingos.json -p sprout

# Run smoke tests (quick boot validation)
smoke:
    cargo xtask bdd --arch x86_64 --tags @smoke

# --- Vendored Rust Standard Library ---

# The commit hash of rust-lang/rust matching our nightly toolchain
rust_commit := "main"

# Ensure the Rust stdlib patches are applied (idempotent).
# Uses a hash of all patches to detect changes.
ensure-rust-patched: fetch-rust
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d vendor/rust/.git ]; then
        echo "ERROR: vendor/rust/ not found after fetch-rust. Run: just fetch-rust" >&2
        exit 1
    fi
    shopt -s nullglob
    patches=( patches/rust/*.patch )
    if [ ${#patches[@]} -eq 0 ]; then
        echo "No patches found in patches/rust/. Nothing to apply."
        exit 0
    fi
    # Compute current hash of all patches
    # Sort by filename so numbered patches apply in order
    IFS=$'\n' sorted=($(printf '%s\n' "${patches[@]}" | sort))
    current_hash=$(sha256sum "${sorted[@]}" | sha256sum | cut -d' ' -f1)

    stored_hash=""
    if [ -f vendor/rust/.patches_hash ]; then
        stored_hash=$(cat vendor/rust/.patches_hash)
    fi

    if [ "$current_hash" != "$stored_hash" ]; then
        echo "==> Rust patches changed or missing. Re-applying..."
        just rust-reset
        cd vendor/rust
        for p in "${sorted[@]}"; do
            echo "  Applying $p..."
            git apply "../../$p" || { echo "ERROR: Failed to apply $p" >&2; exit 1; }
        done
        echo "$current_hash" > .patches_hash
        echo "==> Applied ${#sorted[@]} patch file(s) from patches/rust/"
    else
        echo "==> Rust patches already up-to-date."
    fi

# Fetch (shallow clone) the Rust source tree into vendor/rust/
fetch-rust:
    #!/usr/bin/env bash
    set -euo pipefail
    ROOT_DIR="$(pwd)"
    if [ -d vendor/rust/.git ]; then
        echo "vendor/rust already exists, skipping clone."
        echo "  To re-fetch, run: just rust-reset  (or rm -rf vendor/rust)"
    else
        echo "==> Shallow-cloning rust-lang/rust at {{rust_commit}}..."
        git clone --depth 1 --filter=blob:none --no-checkout \
            https://github.com/dancxjo/rust-thingos.git vendor/rust
        cd vendor/rust
        git fetch --depth 1 origin {{rust_commit}}
        git checkout {{rust_commit}}
        echo "==> Rust source ready at vendor/rust/"
        cd "$ROOT_DIR"
    fi
    # Ensure library submodules needed for std are initialized
    if [ ! -f vendor/rust/library/backtrace/Cargo.toml ]; then
        echo "==> Initializing library/backtrace submodule..."
        cd vendor/rust && git submodule update --init --depth 1 library/backtrace
        cd "$ROOT_DIR"
    fi
    if [ ! -d vendor/rust/src/llvm-project/llvm ]; then
        echo "==> Initializing src/llvm-project submodule..."
        cd vendor/rust && git submodule update --init --depth 1 src/llvm-project
        cd "$ROOT_DIR"
    fi
    echo "  library/std/src/lib.rs exists: $(test -f vendor/rust/library/std/src/lib.rs && echo yes || echo no)"


# Save local modifications in vendor/rust/ as patches
# Save local vendor/rust modifications as a patch.
# Usage: just rust-save-patches [name]
# If [name] is given (e.g. "30-fs"), saves to patches/rust/30-fs.patch.
# Otherwise saves to patches/rust/thingos-pal.patch (legacy default).
rust-save-patches name="thingos-pal":
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
    OUTFILE="../../patches/rust/{{name}}.patch"
    git diff > "$OUTFILE"
    echo "==> Saved patch to patches/rust/{{name}}.patch"
    echo "  $(wc -l < "$OUTFILE") lines"
    echo "  Files changed: $(grep -c '^diff' "$OUTFILE")"

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

# Apply all patches in patches/rust/*.patch to vendor/rust/ in sorted order.
rust-apply-patches:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -d vendor/rust/.git ]; then
        echo "vendor/rust does not exist. Run: just fetch-rust"
        exit 1
    fi
    shopt -s nullglob
    patches=( patches/rust/*.patch )
    if [ ${#patches[@]} -eq 0 ]; then
        echo "No patches found in patches/rust/. Nothing to apply."
        exit 0
    fi
    # Sort by filename so numbered patches apply in order
    IFS=$'\n' sorted=($(printf '%s\n' "${patches[@]}" | sort))
    cd vendor/rust
    for p in "${sorted[@]}"; do
        echo "==> Applying $p ..."
        git apply "../../$p"
    done
    echo "==> All patches applied (${#sorted[@]} files)"

