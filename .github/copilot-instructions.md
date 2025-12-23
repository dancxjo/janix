# GitHub Copilot Instructions for ThingOS

## Project Overview

ThingOS is a microkernel operating system written in Rust using the Limine bootloader. It features a graph database subsystem at its core and is designed to run on multiple architectures (x86_64, aarch64, riscv64, loongarch64).

## Repository Structure

This is a Cargo workspace with the following crates:

### Core Components (no_std)
- **boot/** - Limine bootloader entry point and kernel initialization
- **kernel/** - Core kernel functionality including graph database subsystem, transaction management, and kernel logging
- **abi/** - Shared ABI definitions with type definitions (ProcessId, TransactionId, NodeId) and KernelRequest/KernelResponse enums
- **thing_models/** - Shared system ontology (Kinds, Properties, Predicates) used by both kernel and userland.

### Userland Components (std)
- **thing_os/** - Userland standard library with `println()`, `graph_query()`, and transaction management functions
- **user/** - Userland applications and drivers (compositor, drivers, init, etc.)

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
# Build all workspace members except boot (which is no_std kernel)
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
- **no_std crates**: boot, kernel, abi, thing_models
- **std crates**: thing_os, user/*

Always maintain the no_std compatibility for kernel and low-level crates.

### Naming Conventions
- Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use descriptive names that reflect the microkernel and graph database architecture
- Type IDs use the pattern: `ProcessId`, `TransactionId`, `NodeId`

### Code Organization
- Keep ABI definitions in the `abi` crate to be shared between kernel and userland
- Separate kernel logic (kernel) from boot logic (boot)
- Keep userland runtime (`thing_os`) independent of the kernel implementation details.

### Host/Kernel Parity
- **Sync Requirement**: Tests should run on both host (unit tests) and target (smoke tests) where possible.
- **Verification**: When adding a new syscall or kernel feature, ensure it is implemented and tested in both environments if possible (using mocks in `thing_os` for host tests).

## Testing Strategy

### Workspace Testing
```bash
# Test all workspace members (excluding boot which is no_std kernel code and user apps which are target-specific)
make test
```

### Smoke Tests
Smoke tests run the full OS in QEMU and verify boot milestones.
```bash
make smoke
```

## Common Tasks

### Adding a New Userland Application
1. Create a new crate in the `user/` directory
2. Add it to `Cargo.toml` workspace members
3. Depend on `thing_os` for standard APIs
4. Follow the pattern in `user/init` or `user/clock`

### Modifying Kernel APIs
1. Update types/enums in `abi` crate
2. Implement changes in `kernel`
3. Update `thing_os` to use new APIs
4. Update documentation in `docs/` and `thing_os`

### Adding Architecture Support
1. Add target to `rust-toolchain.toml`
2. Add target to `boot/rust-toolchain.toml`
3. Test build with `KARCH=<new-arch> make all`

## Important Notes

- The kernel uses the Limine bootloader protocol (v9.x-binary branch)
- Graph database is a core feature - consider it when making kernel changes
- Transaction management is integral to the kernel design
- Reminder: keep ABI types data-only (avoid Rust trait APIs or `&'static str` fields in request/response structs).
- Always test changes with both cargo commands (for userland/unit tests) and make commands (for kernel/smoke tests)
- QEMU targets support both UEFI and BIOS boot modes

## Debugging

### Automated Crash Analysis
Use the `make debug-<arch>` command to automatically analyze crashes (Triple Faults, Panics). This tool runs QEMU with logging, captures the crash, and maps the instruction pointer to source code.

```bash
make debug-x86_64
make debug-aarch64
make debug-riscv64
make debug-loongarch64
```

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
