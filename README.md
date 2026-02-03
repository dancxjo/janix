# Thing-OS

A graph-based operating system kernel built with Rust and the Limine bootloader.
<img width="1920" height="1080" alt="Screenshot" src="https://github.com/user-attachments/assets/e6e879ea-389a-4757-86dd-85c2cdb6190c" />

## Architecture Overview

ThingOS follows a unique architecture centered around a **System Graph**. Unlike traditional OSes where state is scattered across files, sysfs, and memory structures, ThingOS consolidates everything into a directed property graph.

### The Boot Lineage

The system initializes in the following sequence:

`Firmware → Bootloader → Bran → Kernel → Sprout → Root → Bloom → Leaves`

1.  **[Bran](./docs/components/bran.md)** (Boot Runtime Abstraction Node): Normalizes bootloader (Limine) quirks and hardware differences, presenting a clean interface to the kernel.
2.  **[Kernel](./docs/components/kernel.md)**: The core OS logic. Host of the **[Root](./docs/components/root.md)** service.
3.  **[Root](./docs/components/root.md)**: The in-memory graph database service. The "brain" of the OS.
4.  **[Sprout](./docs/components/sprout.md)**: The userspace init process. Orchestrates the startup of system services.
5.  **[Stem](./docs/components/stem.md)**: The userspace library ("libc") for interacting with the Kernel and Root.
6.  **[Bristle](./docs/components/bristle.md)**: The input aggregation service (drivers → events).
7.  **[Bloom](./docs/components/bloom.md)**: The Compositor and Window Manager.
8.  **[Blossom](./docs/components/blossom.md)**: The UI Paint Service (Vector Graphics).
9.  **Leaves**: User applications.

### Key Components

*   **[Bud](./docs/components/bud.md)**: Early-boot display and diagnostics.
*   **[Ingestd](./docs/components/ingestd.md)**: Asset watcher service (hot-loading resources).

## Quick Start

```bash
# Build and run
just run

# Run BDD tests
just behave
```

## Prerequisites

- Rust (with nightly toolchain)
- [just](https://github.com/casey/just) command runner
- `xorriso` for ISO building
- `qemu-system-x86_64` for running

## Documentation

*   **[Concepts](./docs/concepts/)**: Deep dives into specific subsystems (scheduling, UI architecture, etc.).
*   **[Components](./docs/components/)**: Detailed documentation for each system component.
*   **[Behavior Reports](./docs/behavior/)**: BDD test results.

## Commands

| Command | Description |
|---------|-------------|
| `just run` | Build and run in QEMU |
| `just iso` | Build bootable ISO |
| `just behave` | Run BDD tests |
| `just behave --feature simple-boot` | Run specific feature |
| `just clean` | Clean build artifacts |
| `just clear-behavior` | Clear all BDD reports |

### Architecture Targets

Set `KARCH` environment variable to target different architectures:

```bash
KARCH=aarch64 just run
KARCH=riscv64 just behave
```

Supported: `x86_64` (default), `aarch64`, `riscv64`, `loongarch64`

## Project Structure

```
├── bran/           # Boot Runtime Abstraction Node
├── kernel/         # Core Kernel & Root Service
├── bud/            # Boot Display Library
├── stem/           # Userspace System Library
├── userspace/      # Applications & Services
│   ├── sprout/     # Init Process
│   ├── bloom/      # Compositor
│   ├── blossom/    # Paint Service
│   └── bristle/    # Input Service
├── tools/bdd/      # BDD test framework
├── xtask/          # Build automation
└── docs/           # Documentation
```

## License

Apache 2.0
