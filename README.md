# ThingOS

**ThingOS** is an experimental microkernel operating system written in Rust, built around one central idea:

> **The kernel *is* a graph.**

Everything meaningful in the system—processes, resources, windows, transactions, modes—should eventually be represented as nodes and edges in a single transactional graph that forms the heart of the OS.

ThingOS boots via the **Limine** bootloader and is structured as a modern Rust **Cargo workspace** with a strict separation between kernel logic, boot code, shared ABI, and userland libraries.

This repository currently provides a minimal working skeleton of that system: a booting kernel, a small in-kernel graph stub, logging, transaction stubs, a userland runtime, and a simple “hello” application that runs through the kernel and userland runtime stack.

---

## ✨ Project Goals

* **Graph-centric kernel**
  All kernel state is encoded as graph nodes/edges. Kernel operations are graph transactions.

* **Transactional updates**
  Mutations occur through an atomic transaction API exposed via a small ABI.

* **Separation of concerns**

  * `boot` handles hardware + Limine
  * `kernel_core` holds pure no_std kernel logic
  * `abi` defines shared types
  * `userland_rt` exposes a syscall-like trait
  * `userland_std` gives friendly, std-like APIs to userland programs

* **Comfortable userland experience**
  User programs should feel “normallish”—like writing small Rust CLI apps—while still interacting with the kernel via the ABI.

* **Native-first kernel logic**
  Kernel logic is exercised on the real kernel builds (or QEMU) so the graph stays aligned with native behavior instead of a hosted shim.

---

# 📁 Repository Structure

This project is a Cargo workspace composed of several crates:

```
thing-os/
│
├── boot/               # Limine entrypoint + kernel binary (no_std)
│   ├── build.rs        # Linker setup
│   ├── linker-*.ld     # Linker scripts for supported arches
│   └── src/main.rs     # kmain() → initializes kernel_core
│
├── kernel_core/        # Pure kernel logic (no_std)
│   ├── graph.rs        # Minimal node storage + queries
│   ├── transaction.rs  # Transaction ID + stub commit
│   └── log.rs          # Fixed-size kernel log buffer
│
├── abi/                # Shared ABI types (no_std)
│   └── lib.rs          # KernelRequest, KernelResponse, NodeId, etc.
│
├── userland_rt/        # no_std runtime / syscall interface
│   └── lib.rs          # Sys trait + KernelSys implementation
│
├── userland_std/       # std-like userland library (std)
│   └── lib.rs          # println(), graph_query(), transaction helpers
│
└── apps/               # User applications compiled to ELF modules
    ├── hello/
    ├── heartbeat/
    ├── init/
    └── thread_dashboard/
```

---

# 🧵 Build and Run

The hosted harness is retired; building and running now centers on the real kernel image instead of a shim.

## Build the workspace (recommended first)

```bash
cargo build --workspace --exclude boot
```

This ensures the shared/runtime/userland crates are in a good state before producing boot artifacts.

## Build the bootable kernel image

ThingOS uses **GNUmakefiles** for the boot image builder.

### Build the kernel ELF for Limine:

```bash
make kernel
```

### Build a bootable ISO:

```bash
make all
```

### Build a raw HDD image (USB/VM):

```bash
make all-hdd
```

Output images appear at:

```
thing-os.iso
thing-os.hdd
```

You may boot these in QEMU, VirtualBox, or on real hardware with appropriate care.

### Run the kernel image via QEMU

Use `make run` (defaults to `KARCH=x86_64`) or `make run-<arch>` to launch the ISO with the QEMU watcher—it's the native kernel path now.

---

# 🧠 Architectural Overview

### Kernel lifetime

1. Limine loads `boot/kernel`
2. `kmain()` asserts Limine revision → initializes `kernel_core`
3. `kernel_core::init()` brings up logging, graph, transactions
4. `kernel_core::boot_sequence()` creates initial kernel graph nodes
5. Kernel halts in place (more work ahead!)

### ABI

Userland communicates with the kernel via:

```rust
KernelRequest → KernelResponse
```

Simple requests currently include:

* `GraphQuery { node_id }`
* `CreateTransaction`
* `CommitTransaction`
* `Log { message }`

This ABI will evolve into a richer transactional graph interface.

### Userland runtime

`userland_rt` defines a `Sys` trait that abstracts the syscall interface. On native kernels it exposes `KernelSys`, which forwards to `kernel_core`.

`userland_std` provides friendly wrapper functions so programs can write:

```rust
userland_std::println("Hello!");
let value = userland_std::graph_query(NodeId(3));
```

---

# 🚧 Current Status

ThingOS currently **boots successfully via Limine**, initializes a minimal kernel core, writes some pixels to the framebuffer, and logs messages into a kernel-side circular buffer.

Userland applications run through the kernel's syscall ABI with the `userland_std` helpers, matching the native execution path.

Next steps include:

* Real graph implementation (edges, attributes, schemas)
* Real transactions that mutate the graph
* Process model & scheduler
* Memory map represented as graph nodes
* Device drivers as graph-attached components
* System call mechanism for actual in-kernel userland

---

# 🤝 Contributing

We welcome improvements, experiments, and structural refinements.

Principles for contributions:

* Maintain clean separation between boot, kernel_core, ABI, and userland.
* Keep kernel_core pure `no_std`.
* Keep ABI small and stable.
* Avoid over-engineering until necessary—grow organically.
* Prefer small, composable changes over monolithic refactors.
* Document invariants for any unsafe code.

---

# 📜 License

ThingOS is released under the MIT license unless noted otherwise.
