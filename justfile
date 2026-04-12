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
iso arch=karch: fetch-rust
    cargo xtask iso --env {{arch}} --profile {{rust_profile}}

# Build HDD image
hdd arch=karch:
    cargo xtask hdd --env {{arch}} --profile {{rust_profile}}

# Run with QEMU (UEFI mode)
# Examples: just run, just run aarch64, just run -i, just run x86_64 -i
run *args: fetch-rust
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

# Build and cache the current stage-1 Rust bootstrap output.
# Today this produces a Linux-hosted cross-compiler for x86_64-unknown-thingos
# plus a cached rustlib tree under target/rustc-thingos/. Set
# SKIP_RUSTC_THINGOS=1 to opt out.
rustc-thingos: fetch-rust rust-apply-patches
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
check: check-ui-split fetch-rust
    #!/usr/bin/env bash
    export __CARGO_TESTS_ONLY_SRC_ROOT="$(pwd)/vendor/rust/library"
    cargo -Z build-std=core,alloc,std,panic_abort -Z build-std-features=compiler-builtins-mem -Z json-target-spec check --target targets/x86_64-unknown-thingos.json -p sprout

# Run smoke tests (quick boot validation)
smoke:
    cargo xtask bdd --arch x86_64 --tags @smoke

# The Rust fork branch that carries the Thing-OS std/LLVM integration.
rust_branch := "thingos-patched"

# Fetch/update the Rust fork submodule into vendor/rust/
fetch-rust:
    #!/usr/bin/env bash
    set -euo pipefail

    if [ ! -f .gitmodules ]; then
        echo "Missing .gitmodules. vendor/rust must be configured as a submodule."
        exit 1
    fi

    if ! git config -f .gitmodules --get submodule.vendor/rust.path >/dev/null; then
        echo "Missing submodule.vendor/rust entry in .gitmodules."
        exit 1
    fi

    if ! git ls-files --stage -- vendor/rust | grep -q '^160000 '; then
        echo "vendor/rust is not recorded as a submodule gitlink (mode 160000)."
        echo "Remediation:"
        echo "  1) remove legacy checkout: rm -rf vendor/rust"
        echo "  2) re-add submodule metadata if needed: git submodule add -b {{rust_branch}} https://github.com/dancxjo/rust-thingos.git vendor/rust"
        echo "  3) run: just fetch-rust"
        exit 1
    fi

    if [ ! -e vendor/rust ]; then
        echo "==> Initializing vendor/rust submodule..."
    else
        echo "==> Syncing vendor/rust submodule metadata..."
    fi
    git submodule sync --recursive vendor/rust
    git submodule set-branch --branch {{rust_branch}} vendor/rust
    git submodule update --init --recursive vendor/rust

    if ! git -C vendor/rust ls-remote --exit-code --heads origin {{rust_branch}} >/dev/null 2>&1; then
        echo "Remote branch origin/{{rust_branch}} was not found in vendor/rust."
        exit 1
    fi

    git -C vendor/rust fetch origin {{rust_branch}}
    if git -C vendor/rust show-ref --verify --quiet refs/heads/{{rust_branch}}; then
        git -C vendor/rust switch {{rust_branch}}
    else
        git -C vendor/rust switch -c {{rust_branch}} --track origin/{{rust_branch}}
    fi
    git -C vendor/rust pull --ff-only origin {{rust_branch}}
    echo "==> Rust source ready at vendor/rust/ on branch {{rust_branch}}"

    # Ensure library submodules needed for std are initialized
    if [ ! -f vendor/rust/library/backtrace/Cargo.toml ]; then
        echo "==> Initializing library/backtrace submodule..."
        git -C vendor/rust submodule update --init --depth 1 library/backtrace
    fi
    if [ ! -d vendor/rust/src/llvm-project/llvm ]; then
        echo "==> Initializing src/llvm-project submodule..."
        git -C vendor/rust submodule update --init --depth 1 src/llvm-project
    fi
    echo "  library/std/src/lib.rs exists: $(test -f vendor/rust/library/std/src/lib.rs && echo yes || echo no)"

# Reapply local Rust fork snapshots into vendor/rust/ and its llvm-project
# submodule.
rust-apply-patches: fetch-rust
    #!/usr/bin/env bash
    set -euo pipefail
    shopt -s nullglob
    rust_patches=(patches/rust/vendor-rust/*.patch)
    llvm_patches=(patches/rust/llvm-project/*.patch)
    if [ ${#rust_patches[@]} -eq 0 ] && [ ${#llvm_patches[@]} -eq 0 ]; then
        echo "==> No Rust patch snapshots to apply"
        exit 0
    fi
    for patch in "${rust_patches[@]}"; do
        if git -C vendor/rust apply --reverse --check "../../${patch}" >/dev/null 2>&1; then
            echo "==> ${patch} already applied in vendor/rust/"
        else
            echo "==> Applying ${patch} to vendor/rust/"
            git -C vendor/rust apply --3way "../../${patch}"
        fi
    done
    for patch in "${llvm_patches[@]}"; do
        if git -C vendor/rust/src/llvm-project apply --reverse --check "../../../../${patch}" >/dev/null 2>&1; then
            echo "==> ${patch} already applied in vendor/rust/src/llvm-project/"
        else
            echo "==> Applying ${patch} to vendor/rust/src/llvm-project/"
            git -C vendor/rust/src/llvm-project apply --3way "../../../../${patch}"
        fi
    done
    echo "==> Rust patch snapshots applied"

# Hard-reset vendor/rust to thingos-patched and discard local changes.
rust-reset:
    #!/usr/bin/env bash
    set -euo pipefail

    if [ ! -f .gitmodules ] || ! git config -f .gitmodules --get submodule.vendor/rust.path >/dev/null; then
        echo "Missing vendor/rust submodule metadata."
        exit 1
    fi

    if ! git ls-files --stage -- vendor/rust | grep -q '^160000 '; then
        echo "vendor/rust is not a submodule gitlink."
        echo "Run: rm -rf vendor/rust && just fetch-rust"
        exit 1
    fi

    git submodule sync --recursive vendor/rust
    git submodule set-branch --branch {{rust_branch}} vendor/rust
    git submodule update --init --recursive --checkout vendor/rust
    git -C vendor/rust fetch origin {{rust_branch}}
    git -C vendor/rust switch {{rust_branch}} || git -C vendor/rust switch -c {{rust_branch}} --track origin/{{rust_branch}}
    git -C vendor/rust reset --hard origin/{{rust_branch}}
    git -C vendor/rust clean -fd
    echo "==> vendor/rust reset to origin/{{rust_branch}}"
