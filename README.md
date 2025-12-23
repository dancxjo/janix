# ThingOS

**ThingOS** is an experimental microkernel operating system written in Rust, built around one central idea:

-> **The kernel *is* a graph.**

Everything meaningful in the system—processes, resources, windows, transactions, modes—should eventually be represented as things and links in a single transactional graph that forms the heart of the OS.

ThingOS boots via the **Limine** bootloader and is structured as a modern Rust **Cargo workspace** with a strict separation between kernel logic, boot code, shared ABI, and userland libraries.

This repository currently provides a minimal working skeleton of that system: a booting kernel, a small in-kernel graph stub, logging, transaction stubs, a userland runtime, and a simple “hello” application that runs through the kernel and userland runtime stack.

---

## ✨ Project Goals

* **Graph-centric kernel**
  All kernel state is encoded as graph things/links. Kernel operations are graph transactions.

* **Transactional updates**
  Mutations occur through an atomic transaction API exposed via a small ABI.

* **Separation of concerns**

  * `boot` handles hardware + Limine
  * `kernel` holds pure no_std kernel logic
  * `abi` defines shared types
  * `runtime` exposes a syscall-like trait
  * `thing_os` gives friendly, std-like APIs to userland programs

* **Comfortable userland experience**
  User programs should feel “normallish”—like writing small Rust CLI user—while still interacting with the kernel via the ABI.

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
│   └── src/main.rs     # kmain() → initializes kernel
│
├── kernel/        # Pure kernel logic (no_std)
│   ├── graph.rs        # Minimal thing storage + queries
│   ├── transaction.rs  # Transaction ID + stub commit
│   └── log.rs          # Fixed-size kernel log buffer
│
├── abi/                # Shared ABI types (no_std)
│   └── lib.rs          # KernelRequest, KernelResponse, NodeId, etc.
│
├── runtime/        # no_std runtime / syscall interface
│   └── lib.rs          # Sys trait + KernelSys implementation
│
├── thing_os/       # std-like userland library (std)
│   └── lib.rs          # println(), graph_query(), transaction helpers
│
└── user/               # User applications compiled to ELF modules
    ├── init/
    ├── clock/
    └── compositor/
```

---

# 📚 Documentation

The detailed documentation is located in the `docs/` directory:

* **Architecture & Design**
  * [Hardware Design (Graph Actualization)](docs/hardware_design.md) - How the kernel delegates hardware control to the graph.
  * [Graph Scheduling](docs/graph_scheduling.md) - How the scheduler uses the graph as the source of truth.
  * [Input Pipeline](docs/input_pipeline.md) - Flow of input events from hardware to userland.

* **Contracts & Interfaces**
  * [Userland Process Contract](docs/userland_process_contract.md) - Lifecycle and environment of user processes.
  * [Thing Descriptions](docs/thing_descriptions.md) - How to use and query Thing descriptions.
  * [ABI Surface](docs/contracts/abi_surface.md)
  * [Syscalls](docs/contracts/syscalls.md)
  * [Schema Authority](docs/contracts/schema_authority.md)
  * [Invariants](docs/contracts/invariant.md)

* **Reports**
  * [Architecture Alignment (Dec 2025)](docs/reports/architecture_alignment_2025_12_20.md)
  * [No-nos Compliance Report](docs/contracts/nonos_report.txt)

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

For the RISC-V target you can run `make run-riscv64` or `make run-hdd-riscv64`; those targets use `qemu-system-riscv64 -cpu rv64` and rely on the `riscv64gc-unknown-none-elf` toolchain target that is now installed automatically (the RISC-V artifacts show up as `template-riscv64.iso` / `template-riscv64.hdd`).

### Debugging Crashes

If the OS crashes (e.g., Triple Fault, Panic), you can use the automated debug tool to analyze the crash log and pinpoint the source code location:

```bash
make debug-x86_64
# or
make debug-aarch64
make debug-riscv64
make debug-loongarch64
```

This command will:
1. Run the OS in QEMU with debug logging enabled.
2. Wait for the crash.
3. Automatically parse the log, find the crash address, and display the corresponding source code and disassembly.

---

# 🧠 Architectural Overview

### Kernel lifetime

1. Limine loads `boot/kernel`
2. `kmain()` asserts Limine revision → initializes `kernel`
3. `kernel::init()` brings up logging, graph, transactions
4. `kernel::boot_sequence()` creates initial kernel graph things
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

`runtime` defines a `Sys` trait that abstracts the syscall interface. On native kernels it exposes `KernelSys`, which forwards to `kernel`.

`thing_os` provides friendly wrapper functions so programs can write:

```rust
thing_os::println("Hello!");
let value = thing_os::graph_query(NodeId(3));
```

---

# 🚧 Current Status

ThingOS currently **boots successfully via Limine**, initializes a minimal kernel core, writes some pixels to the framebuffer, and logs messages into a kernel-side circular buffer.

Userland applications run through the kernel's syscall ABI with the `thing_os` helpers, matching the native execution path.

Next steps include:

* Real graph implementation (links, attributes, schemas)
* Real transactions that mutate the graph
* Process model & scheduler
* Memory map represented as graph things
* Device drivers as graph-attached components
* System call mechanism for actual in-kernel userland

---

# 🤝 Contributing

We welcome improvements, experiments, and structural refinements.

Principles for contributions:

* Maintain clean separation between boot, kernel, ABI, and userland.
* Keep kernel pure `no_std`.
* Keep ABI small and stable.
* Avoid over-engineering until necessary—grow organically.
* Prefer small, composable changes over monolithic refactors.
* Document invariants for any unsafe code.

---

# 📜 License

ThingOS is released under the MIT license unless noted otherwise.
