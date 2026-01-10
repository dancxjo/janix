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
    cargo xtask run --env {{arch}} --profile {{rust_profile}} --qemu-flags "{{qemuflags}}"

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
#   just behave --tag @smoke
behave *args:
    cargo xtask bdd {{args}}

# Alias for behave
bdd *args:
    @just behave {{args}}

# Clear all BDD behavior reports (preserves .feature files and top-level README)
clear-behavior:
    rm -rf docs/behavior/x86_64 docs/behavior/aarch64 docs/behavior/riscv64 docs/behavior/loongarch64
