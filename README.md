# ThingOS

**ThingOS** is an experimental microkernel operating system written in Rust, built around one central idea:

-> **The kernel *is* a graph.**

Everything meaningful in the system—processes, resources, windows, transactions, modes—should eventually be represented as things and links in a single transactional graph that forms the heart of the OS.

ThingOS boots via the **Limine** bootloader and is structured as a modern Rust **Cargo workspace** with a strict separation between kernel logic, boot code, shared ABI, and userland libraries.

This repository currently provides a minimal working skeleton of that system: a booting kernel, an in-kernel graph database, a graph-driven scheduler, and several userland applications (compositor, input drivers) that run through the kernel and userland runtime stack.

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
  * `thing_os` gives friendly, std-like APIs to userland programs
  * `thing_models` defines the system ontology (shared Types and Schemas)

* **Comfortable userland experience**
  User programs should feel “normallish”—like writing small Rust CLI utilities—while still interacting with the kernel via the ABI.

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
│   ├── graph.rs        # Graph storage + queries
│   ├── sched.rs        # Graph-driven scheduler
│   └── log.rs          # Kernel log buffer
│
├── abi/                # Shared ABI types (no_std)
│   ├── lib.rs          # ThingId, UserSlice, etc.
│   ├── requests.rs     # KernelRequest, KernelResponse
│   └── syscalls.rs     # Syscall definitions
│
├── thing_models/       # System ontology (std/no_std)
│   └── lib.rs          # Shared PropKeys, Kinds, Predicates
│
├── thing_os/       # Userland library (std)
│   └── lib.rs          # println(), graph functions, syscall wrappers
│
└── user/               # User applications compiled to ELF modules
    ├── init/           # PID 1
    ├── clock/          # Demo app
    ├── compositor/     # Display server & Window manager
    ├── drivers/        # Userland drivers (framebuffer, ps2, etc.)
    └── ...
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

The hosted harness is retired; building and running now centers on the real kernel image.

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

### Run the kernel image via QEMU

Use `make run` (defaults to `KARCH=x86_64`) or `make run-<arch>` to launch the ISO with the QEMU watcher.

For the RISC-V target you can run `make run-riscv64` or `make run-hdd-riscv64`.

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
3. `kernel::init()` brings up logging, graph, transactions, scheduler
4. `kernel::boot_sequence()` creates initial kernel graph things
5. `init_machine` spawns userland processes (drivers, init)
6. Scheduler takes over

### ABI

Userland communicates with the kernel via `KernelRequest` and `KernelResponse` packets sent over specific syscalls.

Common operations include:

* `ThingCreate { kind, props }`
* `ThingGet { id }` / `ThingUpdate { id, props }`
* `AddLink { src, pred, dst }`
* `Log { message }`

The ABI is strictly data-driven (Postcard invariant), using flat buffers and explicit types.

### Userland runtime

`thing_os` provides a friendly, standard-library-like environment for userland programs. It handles:

* Heap allocation (GlobalAllocator)
* Panic handling (logging to kernel)
* Syscall wrapping
* Graph API helpers (`create_thing`, `get_thing`, etc.)

---

# 🚧 Current Status

ThingOS currently **boots successfully via Limine** on x86_64, AArch64, RISC-V, and LoongArch64.

It features:
* **Real In-Kernel Graph**: Things, Links, and Schemas are implemented.
* **Graph-Driven Scheduler**: Threads and CPU cores are Things; scheduling is based on graph links.
* **Userland**: Multiple processes (Compositor, Input Drivers, Demo Apps) running in separate address spaces.
* **Display**: Framebuffer support with software compositing.
* **Input**: PS/2 Keyboard/Mouse and USB HID support (partial).

Next steps include:
* Improved IPC mechanism.
* More advanced drivers (Networking, Storage).
* Robust error handling and resource reclamation.

---

# 🤝 Contributing

We welcome improvements, experiments, and structural refinements.

Principles for contributions:

* Maintain clean separation between boot, kernel, ABI, and userland.
* Keep kernel pure `no_std`.
* Keep ABI small, stable, and data-only.
* Avoid over-engineering until necessary—grow organically.
* Prefer small, composable changes over monolithic refactors.
* Document invariants for any unsafe code.

---

# 📜 License

ThingOS is released under the MIT license unless noted otherwise.
