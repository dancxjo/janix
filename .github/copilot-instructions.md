# GitHub Copilot Instructions for ThingOS

## Project Overview

ThingOS is a microkernel operating system written in Rust using the Limine bootloader. It features a graph database subsystem at its core and is designed to run on multiple architectures (x86_64, aarch64, riscv64, loongarch64).

## Repository Structure

This is a Cargo workspace with the following crates:

### Core Components (no_std)
- **boot/** - Limine bootloader entry point and kernel initialization
- **kernel_core/** - Core kernel functionality including graph database subsystem, transaction management, and kernel logging
- **abi/** - Shared ABI definitions with type definitions (ProcessId, TransactionId, NodeId) and KernelRequest/KernelResponse enums
- **userland_rt/** - Userland runtime with Sys trait for system calls and HostedSys stub implementation for testing

### Userland Components (std)
- **userland_std/** - Userland standard library with `println()`, `graph_query()`, and transaction management functions
- **host_harness/** - Host-side testing harness that displays simulated kernel logs
- **user_app_hello/** - Example userland application demonstrating userland_std APIs

## Build System

### Dependencies
- **GNU Make** (`gmake` on non-GNU systems) - Required for all make commands
- **Rust Toolchain** - Nightly channel with architecture-specific targets (see rust-toolchain.toml)
- **xorriso** - Required for building ISO images with `make all`
- **sgdisk** and **mtools** - Required for building HDD/USB images with `make all-hdd`
- **qemu** - Required for running targets (`make run`, `make run-hdd`, etc.)

### Build Commands

#### Workspace Components (non-kernel)
```bash
# Build and run host harness
cargo run -p host_harness

# Build and run example userland application
cargo run -p user_app_hello

# Build all workspace members except boot
cargo build --workspace --exclude boot
```

#### Kernel and Images
```bash
# Build kernel only
make kernel

# Build bootable ISO
make all

# Build bootable HDD/USB image
make all-hdd

# Build and run in QEMU
make run        # ISO with UEFI
make run-hdd    # HDD with UEFI
make run-bios   # ISO with BIOS
```

### Architecture Support

The `KARCH` make variable controls the target architecture:
- Default: `x86_64`
- Other options: `aarch64`, `riscv64`, `loongarch64`
- Example: `make KARCH=aarch64 all`

Note: Additional architectures need to be enabled in boot/rust-toolchain.toml

## Coding Standards

### Rust Edition
- All crates use Rust edition 2024
- Use nightly Rust features as needed for OS development

### no_std vs std
- **no_std crates**: boot, kernel_core, abi, userland_rt
- **std crates**: userland_std, host_harness, user_app_hello

Always maintain the no_std compatibility for kernel and low-level crates.

### Naming Conventions
- Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use descriptive names that reflect the microkernel and graph database architecture
- Type IDs use the pattern: `ProcessId`, `TransactionId`, `NodeId`

### Code Organization
- Keep ABI definitions in the `abi` crate to be shared between kernel and userland
- Separate kernel logic (kernel_core) from boot logic (boot)
- Keep userland runtime (userland_rt) independent of the standard library (userland_std)

### Host/Kernel Parity
- **Sync Requirement**: The `host_harness` (via `HostedSys`) and the kernel (via `KernelSys`) must always stay in sync.
- **Capability Parity**: Anything the kernel can do, the host harness must be able to do, and vice versa.
- **Verification**: When adding a new syscall or kernel feature, ensure it is implemented and tested in both environments.

## Testing Strategy

### Workspace Testing
```bash
# Test all workspace members (excluding boot which is no_std kernel code)
cargo test --workspace --exclude boot
```

### Host Harness Testing
The host_harness crate provides a way to test kernel interactions in a hosted environment without needing to build and run the full kernel.

## Common Tasks

### Adding a New Userland Application
1. Create a new crate in the workspace root
2. Add it to `Cargo.toml` workspace members
3. Depend on `userland_std` for standard APIs
4. Follow the pattern in `user_app_hello`

### Modifying Kernel APIs
1. Update types/enums in `abi` crate
2. Implement changes in `kernel_core`
3. Update `userland_std` to use new APIs
4. Update documentation in userland_std

### Adding Architecture Support
1. Add target to `rust-toolchain.toml`
2. Add target to `boot/rust-toolchain.toml`
3. Test build with `KARCH=<new-arch> make all`

## Important Notes

- The kernel uses the Limine bootloader protocol (v9.x-binary branch)
- Graph database is a core feature - consider it when making kernel changes
- Transaction management is integral to the kernel design
- Always test changes with both cargo commands (for userland) and make commands (for kernel)
- QEMU targets support both UEFI and BIOS boot modes

## Debugging

### QEMU Options
- Default memory: 2GB (override with `QEMUFLAGS="-m 4G"`)
- Graphics: QEMU uses appropriate graphics for each architecture (ramfb for ARM/RISC-V)
- OVMF firmware downloaded automatically for UEFI testing

### Common Issues
- If kernel build fails, ensure rust-toolchain.toml has the correct targets enabled
- If ISO/HDD creation fails, verify xorriso/sgdisk/mtools are installed
- If QEMU fails to start, check that OVMF firmware files downloaded correctly

## File Patterns to Ignore

When making changes, avoid committing:
- `iso_root/` - Temporary directory for ISO creation
- `*.iso`, `*.hdd` - Generated images
- `limine/` - Downloaded bootloader
- `ovmf/` - Downloaded UEFI firmware
- Standard Rust artifacts: `target/`, `Cargo.lock` (for applications, not libraries)
