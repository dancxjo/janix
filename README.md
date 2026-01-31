# Thing-OS

A graph-based operating system kernel built with Rust and the Limine bootloader.
<img width="1920" height="1080" alt="Screenshot" src="https://github.com/user-attachments/assets/e6e879ea-389a-4757-86dd-85c2cdb6190c" />


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

## BDD Test Reports

Test results are saved to `docs/behavior/` and can be viewed directly on GitHub:

📊 **[View Test Reports](./docs/behavior/)**

Reports are organized by architecture:
- [x86_64 Results](./docs/behavior/x86_64/) - x86_64 support
- [aarch64 Results](./docs/behavior/aarch64/) - ARM64 support
- [riscv64 Results](./docs/behavior/riscv64/) - RISC-V support
- [loongarch64 Results](./docs/behavior/loongarch64/) - LoongArch support

Each report includes:
- ✅/❌ Pass/fail status per feature, scenario, and step
- 📜 Serial console logs
- 📷 Screenshots (when available)
- Structured JSON results for tooling

### Running Tests

```bash
# Run all tests for default arch (x86_64)
just behave

# Run specific feature
just behave --feature simple-boot

# Run for different architecture
KARCH=aarch64 just behave

# Clear all reports (before clean run)
just clear-behavior
```

## Project Structure

```
├── bran/           # Kernel ("Boot Runtime Abstraction Node")
├── bloom/          # Compositor/window manager
├── apps/           # Userland applications
├── tools/bdd/      # BDD test framework
├── xtask/          # Build automation
└── docs/behavior/  # Test reports (generated)
```

## License

Apache 2.0
