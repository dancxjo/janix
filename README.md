# ThingOS

A microkernel operating system built in Rust using Limine bootloader, organized as a Cargo workspace.

## Architecture

This repository is structured as a Cargo workspace with the following crates:

- **boot/** - Limine bootloader entry point and kernel initialization
- **kernel_core/** - Core kernel functionality (no_std):
  - Graph database subsystem
  - Transaction management
  - Kernel logging
- **abi/** - Shared ABI definitions (no_std):
  - Type definitions (ProcessId, TransactionId, NodeId)
  - KernelRequest and KernelResponse enums
- **userland_rt/** - Userland runtime (no_std):
  - Sys trait for system calls
  - HostedSys stub implementation for testing
- **userland_std/** - Userland standard library (std):
  - `println()` for kernel logging
  - `graph_query()` for querying the graph database
  - Transaction creation and commit functions
- **host_harness/** - Host-side testing harness (std):
  - Displays simulated kernel logs
- **user_app_hello/** - Example userland application (std):
  - Demonstrates using userland_std APIs

## How to use this?

### Dependencies

Any `make` command depends on GNU make (`gmake`) and is expected to be run using it. This usually means using `make` on most GNU/Linux distros, or `gmake` on other non-GNU systems.

All `make all*` targets depend on Rust.

Additionally, building an ISO with `make all` requires `xorriso`, and building a HDD/USB image with `make all-hdd` requires `sgdisk` (usually from `gdisk` or `gptfdisk` packages) and `mtools`.

### Architectural targets

The `KARCH` make variable determines the target architecture to build the kernel and image for.

The default `KARCH` is `x86_64`. Other options include: `aarch64`, `riscv64`, and `loongarch64`.

Other architectures will need to be enabled in boot/rust-toolchain.toml

### Building individual components

To build and run the host harness:
```bash
cargo run -p host_harness
```

To build and run the example userland application:
```bash
cargo run -p user_app_hello
```

To build all workspace members (except boot):
```bash
cargo build --workspace --exclude boot
```

To build the boot kernel:
```bash
make kernel
```

### Makefile targets

Running `make all` will compile the kernel (from the `boot/` directory) and then generate a bootable ISO image.

Running `make all-hdd` will compile the kernel and then generate a raw image suitable to be flashed onto a USB stick or hard drive/SSD.

Running `make run` will build the kernel and a bootable ISO (equivalent to make all) and then run it using `qemu` (if installed).

Running `make run-hdd` will build the kernel and a raw HDD image (equivalent to make all-hdd) and then run it using `qemu` (if installed).

The `run-uefi` and `run-hdd-uefi` targets are equivalent to their non `-uefi` counterparts except that they boot `qemu` using a UEFI-compatible firmware.
