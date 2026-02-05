# Thing-OS

> **A graph-native operating system where everything is a Thing.**

<img width="1920" height="1080" alt="Screenshot" src="https://github.com/user-attachments/assets/e6e879ea-389a-4757-86dd-85c2cdb6190c" />

Thing-OS is an experimental, Rust-based operating system built on a radical idea: **the entire system state is a live, queryable graph**. Hardware, processes, files, windows, network sockets—everything is a first-class node in a directed property graph, discoverable and manipulable through a unified API.

---

## ✨ Features

| Feature | Status |
|---------|--------|
| **Symmetric Multi-Processing (SMP)** | ✅ Full 6-core support with per-CPU runqueues |
| **GPU Acceleration** | ✅ Virgl 3D via VirtIO-GPU |
| **TCP/IP Networking** | ✅ DHCP, TCP, UDP via smoltcp |
| **Compositor & Window Manager** | ✅ GPU-accelerated with damage tracking |
| **System Graph** | ✅ Live in-memory graph database |
| **Graph Query Language (GQL)** | ✅ OpenGQL subset with REST API |
| **Hot Asset Loading** | ✅ Wallpapers, cursors, fonts on-the-fly |
| **Storage: AHCI & ISO9660** | ✅ SATA disks, CD-ROM, boot modules |
| **Audio** | 🚧 VirtIO-Sound (in progress) |
| **WebAssembly Driver Host** | 🚧 Experimental |
| **Multi-Architecture** | 🚧 x86_64 stable; aarch64, riscv64, loongarch64 WIP |

---

## 📐 Architecture

Thing-OS follows a unique architecture centered around the **System Graph**. Unlike traditional OSes where state is scattered across `/proc`, `/sys`, registries, and opaque kernel structures, Thing-OS consolidates *everything* into a single, coherent graph.

```
┌─────────────────────────────────────────────────────────────────────┐
│                         System Graph (Root)                        │
│   ┌───────┐  ┌──────────┐  ┌─────────┐  ┌────────┐  ┌───────────┐  │
│   │dev.Cpu│──│svc.Sprout│──│task.Task│──│mem.Range│──│fs.File   │  │
│   │  ×6   │  │          │  │  (100s) │  │         │  │          │  │
│   └───────┘  └──────────┘  └─────────┘  └────────┘  └───────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### The Boot Lineage

```
Firmware → Limine → Bran → Kernel → Root → Sprout → Bloom → Leaves
```

| Stage | Component | Role |
|-------|-----------|------|
| -1 | **Limine** | Bootloader (provides framebuffer, memory map, modules) |
| 0 | **Bran** | Boot Runtime Abstraction Node — normalizes hardware |
| 1 | **Kernel** | Core OS: scheduling, memory, syscalls |
| 1 | **Root** | In-kernel graph database — the "brain" of the OS |
| 2 | **Sprout** | Userspace init — service orchestration |
| 3 | **Bloom** | GPU compositor & window manager |
| 3+ | **Leaves** | User applications |

---

## 📦 Crate Map

This workspace contains **60+ crates** organized into logical layers:

### Core System

| Crate | Description |
|-------|-------------|
| `kernel` | Core kernel: scheduler (SMP), memory manager, syscalls, Root graph service |
| `bran` | Boot runtime: Limine integration, APIC, SMP trampoline, paging |
| `abi` | Wire-compatible ABI shared by kernel and userspace |
| `abi-macros` | Procedural macros for ABI definitions |
| `stem` | Userspace runtime library ("libc") — syscalls, allocator, threading |
| `stem-macros` | `#[stem::main]` entry point macro |
| `bulb` | **B**oot **U**p **L**iveness **B**ehavior — early-boot display with Wasm plugin support |

### Graphics & Display

| Crate | Description |
|-------|-------------|
| `bloom` | GPU-accelerated compositor, window manager, damage tracking |
| `blossom` | Vector graphics painter (Béziers, paths, text) |
| `photosynthesis` | Native graph visualization (Cytoscape-style) |
| `display_virtio_gpu` | VirtIO-GPU display driver with Virgl 3D |
| `display_bootfb` | Simple bootloader framebuffer driver |
| `display_fake` | Null display for headless testing |
| `virtio_gpu` | VirtIO-GPU protocol implementation |
| `fontd` | Font rasterization daemon |
| `flytrap` | Asset watcher — hot-loads wallpapers, cursors, fonts |

### Input

| Crate | Description |
|-------|-------------|
| `bristle` | Input event aggregation (keyboard + mouse → unified events) |
| `ps2_kbd` | PS/2 keyboard driver |
| `ps2_mouse` | PS/2 mouse driver |

### Networking

| Crate | Description |
|-------|-------------|
| `virtio_netd` | VirtIO-Net hardware driver |
| `netd` | TCP/IP stack (smoltcp): DHCP, sockets, packet routing |
| `fetchd` | HTTP client daemon |
| `nectar` | mDNS/DNS-SD service publisher |
| `anther` | REST API & admin console for the System Graph |
| `phloem` | GQL (Graph Query Language) engine |

### Storage

| Crate | Description |
|-------|-------------|
| `ahci_disk` | AHCI/SATA disk driver with ISO9660 reader |
| `ata_disk` | Legacy ATA PIO driver |
| `disk_probe` | Disk/partition discovery |
| `iso9660` / `iso9660d` | ISO9660 filesystem library & daemon |
| `iso_reader` / `iso_cat` | Boot-time ISO utilities |

### Audio

| Crate | Description |
|-------|-------------|
| `virtio_sound` | VirtIO-Sound driver (WIP) |
| `beeper` | System beep/chime generator |

### System Services

| Crate | Description |
|-------|-------------|
| `sprout` | Init daemon — service supervision, pipeline orchestration |
| `cambium` | Property propagation daemon (reactive bindings) |
| `clock` | Real-time clock service |
| `rtc_cmos` | CMOS RTC driver |

### Shared Libraries

| Crate | Description |
|-------|-------------|
| `libs/fb_common` | Framebuffer primitives (pitch, stride, blitting) |
| `libs/llm` | LLM integration interface |
| `libs/llm_stub` | Stub implementation for testing |
| `userspace/virtio` | Shared VirtIO primitives (virtqueues, feature negotiation) |

### Development & Testing

| Crate | Description |
|-------|-------------|
| `xtask` | Build automation (ISO generation, elf bundling) |
| `tools/bdd` | BDD test harness (Gherkin-style) |
| `tools/pciids` | PCI ID database generator |
| `tools/display_proto_tests` | Display protocol conformance tests |
| Benchmarks | `root_batch_bench`, `bench_blit`, `scheduler_fairness` |
| Demos | `echo`, `hogger`, `tick_printer`, `irqdump`, `drawlist_demo`, `virgl_demo` |

### Experimental

| Crate | Description |
|-------|-------------|
| `driver_wasm_host` | WebAssembly driver runtime |
| `proto_driver_wasm` | Example WASM driver |
| `described` / `sysdescribe` | Self-describing system introspection |

---

## 🚀 Quick Start

### Prerequisites

- **Rust** (nightly toolchain — see `rust-toolchain.toml`)
- **[just](https://github.com/casey/just)** command runner
- **xorriso** for ISO building
- **qemu-system-x86_64** for virtualization

### Build & Run

```bash
# Build and run in QEMU (6-core SMP, VirtIO-GPU, VirtIO-Net)
just run

# Build bootable ISO
just iso

# Run BDD test suite
just behave

# Target a different architecture
KARCH=aarch64 just run
```

### Available Commands

| Command | Description |
|---------|-------------|
| `just run` | Build & run in QEMU with 6 CPUs, GPU, networking |
| `just iso` | Generate bootable ISO image |
| `just behave` | Run full BDD test suite |
| `just behave --feature <name>` | Run specific feature test |
| `just clean` | Clean all build artifacts |
| `just die` | Kill any running QEMU instance |

---

## 🌳 Project Structure

```
thing-os/
├── abi/                    # Wire ABI (kernel ↔ userspace)
├── abi-macros/             # Procedural macros for ABI
├── bran/                   # Boot runtime (per-arch)
│   └── src/arch/{x86_64,aarch64,riscv64,loongarch64}/
├── bulb/                   # Boot Up Liveness Behavior (early-boot display + Wasm)
├── kernel/                 # Core kernel + Root graph service
│   └── src/{memory,task,syscall,root}/
├── stem/                   # Userspace "libc"
├── stem-macros/            # Entry point macros
├── libs/                   # Shared libraries
│   ├── fb_common/          # Framebuffer utilities
│   └── llm*/               # LLM integration
├── userspace/              # ~50 userspace applications
│   ├── sprout/             # Init daemon
│   ├── bloom/              # Compositor
│   ├── blossom/            # Paint service
│   ├── netd/               # TCP/IP stack
│   ├── anther/             # Graph API server
│   └── .../                # Drivers, services, demos
├── tools/                  # Development tooling
│   ├── bdd/                # BDD test framework
│   └── pciids/             # PCI database
├── xtask/                  # Build automation
├── assets/                 # Wallpapers, cursors, fonts
├── docs/                   # Documentation
│   ├── components/         # Per-component docs
│   └── concepts/           # Deep-dive topics
└── targets/                # Custom Rust target specs
```

---

## 📚 Documentation

- **[Components](./docs/components/)** — Detailed docs for each system component
- **[Concepts](./docs/concepts/)** — Deep dives: scheduling, the graph model, UI architecture
- **[Behavior Reports](./docs/behavior/)** — BDD test results

---

## 🛠️ Architecture Targets

| Architecture | Status |
|--------------|--------|
| `x86_64` | ✅ Stable (default) |
| `aarch64` | 🚧 In progress |
| `riscv64` | 🚧 In progress |
| `loongarch64` | 🚧 In progress |

Set `KARCH` to target:
```bash
KARCH=aarch64 just run
```

---

## 🧬 Design Philosophy

1. **Everything is a Thing** — Hardware, tasks, sockets, windows, files: all are nodes in the System Graph.
2. **Queryable State** — Inspect and manipulate the OS via GQL queries or REST API.
3. **Reactive by Default** — Changes propagate automatically via watch subscriptions.
4. **SMP from Day One** — Multi-core support is fundamental, not bolted on.
5. **Userspace Drivers** — VirtIO drivers run in userspace for isolation.

---

## 📄 License

[Apache 2.0](./LICENSE)
