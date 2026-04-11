# Thing-OS

> **If you cannot `cat` it, you cannot trust it.**

<img width="1920" height="1080" alt="Screenshot" src="https://github.com/user-attachments/assets/e6e879ea-389a-4757-86dd-85c2cdb6190c" />

Thing-OS is an experimental, Rust-based microkernel unix-y operating system. Hardware drivers, network stacks, window managers, and system services all expose their state and behavior as a mounted filesystem. If a service has structure, it mounts itself. If it has behavior, it exposes it through files.

The current architecture is called **janix** — *Just Another \*NIX*. The original vision was more ambitious: centralize the entire system state in a live, queryable **graph database** baked into the kernel. Every process, device, socket, and window would be a typed node; service discovery would be graph traversal; configuration would be property mutation. It was elegant on paper.

It was also a bad architecture. The graph became a bottleneck, a coupling point, and a source of subtle ordering bugs. Services that should have been independent ended up entangled through shared graph state. The "unified model" made everything harder to test, harder to reason about, and harder to compose.

So we're salvaging it. The graph is being demoted to a userspace concern — something `graphd` can manage if you want it — and the system is being re-plumbed around a VFS namespace, per-process mounts, and file I/O. Plan 9 figured this out in the 1980s. We're just catching up.

Hence: janix. Just another unix. Turns out that's enough.

---

## What works today

| Feature | Status |
|---------|--------|
| **Symmetric Multi-Processing (SMP)** | ✅ 6-core with per-CPU runqueues |
| **GPU Acceleration** | ✅ Virgl 3D via VirtIO-GPU |
| **TCP/IP Networking** | ✅ DHCP, TCP, UDP via smoltcp |
| **Compositor & Window Manager** | ✅ GPU-accelerated with damage tracking |
| **VFS Core** | ✅ Mount-based path routing in kernel |
| **Hot Asset Loading** | ✅ Wallpapers, cursors, fonts on-the-fly |
| **Storage: AHCI & ISO9660** | ✅ SATA disks, CD-ROM, boot modules |
| **Network VFS migration** | 🚧 `virtio_netd` → `/dev/net/`, `netd` → `/net/` |
| **Per-process namespaces** | 🚧 In progress |
| **Audio** | 🚧 VirtIO-Sound |
| **Multi-Architecture** | 🚧 x86_64 stable; aarch64, riscv64, loongarch64 WIP |

---

## Architecture

The kernel is intentionally small. It does five things:

1. **Schedule** tasks across CPUs
2. **Manage** virtual memory
3. **Route** VFS paths to the right mounted provider
4. **Deliver** interrupts and IPC
5. **Track** capabilities

Everything else — TCP/IP, NIC drivers, window management, fonts, storage — lives in userspace, mounted into the namespace.

### The namespace

```
/
├── dev/
│   ├── net/
│   │   └── virtio0/         ← mounted by virtio_netd
│   │       ├── rx, tx       ← raw Ethernet frames
│   │       ├── ctl, mac     ← control and metadata
│   │       └── events       ← pollable link events
│   └── fb0/                 ← mounted by display driver
├── net/                     ← mounted by netd
│   ├── interfaces/eth0/
│   ├── routes
│   └── tcp/<id>/{ctl,data,events}
├── proc/                    ← mounted by kernel
│   └── <pid>/{status,fd/,maps}
├── services/                ← mounted by userspace services
│   ├── net/                 ← network stack presence
│   └── display/             ← compositor presence
└── run/                     ← ephemeral state
```

### Boot sequence

```
Firmware → Limine → Bran → Kernel → Sprout → services mount themselves
```

| Stage | Component | Role |
|-------|-----------|------|
| -1 | **Limine** | Bootloader |
| 0 | **Bran** | Boot runtime — normalizes hardware across architectures |
| 1 | **Kernel** | Scheduling, memory, VFS core, IPC, capabilities |
| 2 | **Sprout** | Init — launches drivers and services in dependency order |
| 3 | **Drivers** | Mount `/dev/net/virtio0/`, `/dev/fb0/`, etc. |
| 3 | **Services** | Mount `/net/`, `/services/`, `/proc/` |
| 4 | **Bloom** | Compositor — binds to `/dev/fb0/`, `/services/display/` |
| 4+ | **Applications** | See the world through their own namespace |

### Inter-service communication

Services communicate by reading and writing files, not by calling each other's APIs:

- `netd` discovers the NIC by opening `/dev/net/virtio0/ctl`
- Applications open connections by reading `/net/tcp/new`
- Drivers signal events by writing to their `events` file
- Any program can inspect any service state with `cat`

---

## User interfaces

### Graphical (Bloom & Blossom)
**Bloom** is a GPU-accelerated compositor and window manager. **Blossom** handles vector graphics. They mount into the namespace and expose their state as files.

### REST & web (Anther)
**Anther** is a lightweight REST console. Useful for introspection and remote administration.

### Shell & telnet
Thing-OS runs a telnet server on port 2323. Connect from your host:
```
telnet localhost 2323
```

---

## Crate map

60+ crates organized into layers. The kernel is thin; nearly everything is a userspace service.

### Kernel & runtime

| Crate | Description |
|-------|-------------|
| `kernel` | Scheduler (SMP), memory, VFS path router, IPC, syscalls |
| `bran` | Boot runtime — Limine integration, APIC, SMP trampoline, paging |
| `abi` | Wire ABI shared between kernel and userspace |
| `abi-macros` | Procedural macros for ABI definitions |
| `stem` | Userspace runtime ("libc") — syscalls, allocator, threading, PAL |
| `stem-macros` | `#[stem::main]` entry point macro |
| `bulb` | Early-boot display with Wasm plugin support |

### Display & graphics

| Crate | Description |
|-------|-------------|
| `bloom` | GPU-accelerated compositor and window manager; mounts `/services/display/` |
| `blossom` | Vector graphics painter (Béziers, paths, text rendering) |
| `display_virtio_gpu` | VirtIO-GPU driver; mounts `/dev/fb0/` |
| `display_bootfb` | Bootloader framebuffer fallback |
| `display_fake` | Null display for headless testing |
| `fontd` | Font rasterization service |
| `flytrap` | Hot-loads wallpapers, cursors, fonts |

### Input

| Crate | Description |
|-------|-------------|
| `bristle` | Input aggregator — keyboard + mouse → unified event stream |
| `ps2_kbd` | PS/2 keyboard driver |
| `ps2_mouse` | PS/2 mouse driver |

### Networking

Services communicate over files, not ports. The migration to the janix network model is in progress.

| Crate | Description |
|-------|-------------|
| `virtio_netd` | VirtIO-Net driver; target: mounts `/dev/net/virtio0/` |
| `netd` | TCP/IP stack (smoltcp — DHCP, sockets); target: mounts `/net/` |
| `devd` | Device discovery and driver lifecycle manager *(planned)* |
| `fetchd` | HTTP client daemon |
| `nectar` | mDNS/DNS-SD service publisher |
| `anther` | Lightweight REST console |

### Storage

| Crate | Description |
|-------|-------------|
| `ahci_disk` | AHCI/SATA driver |
| `ata_disk` | Legacy ATA PIO driver |
| `disk_probe` | Disk/partition discovery |
| `iso9660` / `iso9660d` | ISO9660 library and daemon |
| `iso_reader` / `iso_cat` | Boot-time ISO utilities |

### Audio

| Crate | Description |
|-------|-------------|
| `virtio_sound` | VirtIO-Sound driver (WIP) |
| `beeper` | System beep generator |

### System services

| Crate | Description |
|-------|-------------|
| `sprout` | Init — launches services in dependency order |
| `clock` | Real-time clock service |
| `rtc_cmos` | CMOS RTC driver |

### Shared libraries

| Crate | Description |
|-------|-------------|
| `libs/fb_common` | Framebuffer primitives |
| `libs/llm` | LLM integration interface |
| `userspace/virtio` | Shared VirtIO primitives (virtqueues, feature negotiation) |

### Tooling & testing

| Crate | Description |
|-------|-------------|
| `xtask` | Build automation (ISO generation, ELF bundling) |
| `tools/bdd` | BDD test harness (Gherkin-style) |
| `tools/pciids` | PCI ID database generator |
| Benchmarks | `bench_blit`, `scheduler_fairness` |
| Demos | `echo`, `hogger`, `irqdump`, `drawlist_demo` |

---

## Quick start

### Prerequisites

- **Rust** (nightly — see `rust-toolchain.toml`)
- **[just](https://github.com/casey/just)**
- **xorriso** for ISO building
- **qemu-system-x86_64**
- **cmake** & **ninja-build** (required for `rustc-thingos` build)
- **zlib1g-dev** (required for LLVM build)
- **python3** and **git**

> [!NOTE]
> **Rustc on ThingOS**: The automatic build of `rustc` for ThingOS is currently disabled due to upstream LLVM target compatibility issues. See [docs/status/rustc_build.md](docs/status/rustc_build.md) for details.

### Build & run

```bash
# Build and run in QEMU (6-core SMP, VirtIO-GPU, VirtIO-Net)
just run

# Build a bootable ISO
just iso

# Run BDD test suite
just behave

# Target a different architecture
KARCH=aarch64 just run
```

### Available commands

| Command | Description |
|---------|-------------|
| `just run` | Build & run in QEMU |
| `just iso` | Generate bootable ISO |
| `just behave` | Full BDD test suite |
| `just clean` | Clean build artifacts |
| `just die` | Kill running QEMU |

---

## Project structure

```
thing-os/
├── abi/                    # Wire ABI (kernel ↔ userspace)
├── bran/                   # Boot runtime (per-arch)
│   └── src/arch/{x86_64,aarch64,riscv64,loongarch64}/
├── bulb/                   # Early-boot display
├── kernel/                 # Kernel: scheduler, memory, VFS, IPC
│   └── src/{memory,task,syscall,vfs}/
├── stem/                   # Userspace runtime + PAL
├── libs/                   # Shared libraries
├── userspace/              # All drivers and services (~50 crates)
│   ├── sprout/             # Init daemon
│   ├── bloom/              # Compositor (→ /services/display/)
│   ├── netd/               # TCP/IP stack (→ /net/)
│   ├── virtio_netd/        # VirtIO-Net driver (→ /dev/net/virtio0/)
│   ├── anther/             # REST console
│   └── .../
├── tools/                  # Dev tooling (BDD, pciids)
├── xtask/                  # Build automation
├── assets/                 # Wallpapers, fonts, cursors
├── docs/
│   ├── components/         # Per-component docs
│   ├── concepts/           # Deep dives
│   └── behavior/           # BDD test results
└── targets/                # Custom Rust target specs
```

---

## Documentation

- [Components](./docs/components/) — per-component design docs
- [Concepts](./docs/concepts/) — deep dives on scheduling, namespaces, VFS
- [Behavior Reports](./docs/behavior/) — BDD test results

---

## Architecture targets

| Architecture | Status |
|--------------|--------|
| `x86_64` | ✅ Stable |
| `aarch64` | 🚧 In progress |
| `riscv64` | 🚧 In progress |
| `loongarch64` | 🚧 In progress |

---

## Design principles

1. **Everything is a file.** If a service has structure, it mounts itself. If it has behavior, it exposes it through files.
2. **The kernel enforces authority; userspace provides everything else.** Drivers, TCP/IP, compositors — all live outside the kernel.
3. **Drivers speak hardware. Services speak protocols. Everything else is files.**
4. **Per-process namespaces.** Each process sees its own namespace. The same path can resolve differently in different processes.
5. **If you cannot `cat` it, you cannot trust it.** Service state is text in files, not opaque objects behind an API.
6. **SMP from day one.** Multi-core support is fundamental, not bolted on.

---

## License
