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

# Build everything (ISO)
iso:
    cargo xtask iso --env {{karch}} --profile {{rust_profile}}

# Build HDD image
hdd:
    cargo xtask hdd --env {{karch}} --profile {{rust_profile}}

# Run with QEMU (UEFI mode)
run:
    cargo xtask run --env {{karch}} --profile {{rust_profile}} --qemu-flags "{{qemuflags}}"

# Run HDD with QEMU
run-hdd:
    cargo xtask run-hdd --env {{karch}} --profile {{rust_profile}} --qemu-flags "{{qemuflags}}"

# Run with BIOS (x86_64 only)
run-bios:
    cargo xtask run-bios --qemu-flags "{{qemuflags}}"

# Build the kernel
kernel:
    cargo xtask build --env {{karch}} --profile {{rust_profile}}

# Clone and build limine bootloader
limine:
    cargo xtask limine

# Download OVMF firmware
ovmf:
    cargo xtask ovmf --env {{karch}}

# Clean build artifacts
clean:
    cargo xtask clean

# Clean everything including downloaded dependencies
distclean:
    cargo xtask distclean
