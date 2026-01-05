# ThingOS

ThingOS is an experimental operating system built around a single idea:

**Everything is a Thing, and the graph is the system.**

Rather than centering the OS on files, processes, or syscalls, ThingOS uses a typed, queryable graph as the primary model for all state:
hardware, memory, drivers, services, windows, input events, time, and user programs.

If you can describe it, you can put it in the graph.
If it’s in the graph, you can observe it, link to it, and act on it.

**Essential Reading:** 
* [ThingOS Manifesto](MANIFESTO.md) - The core philosophy and agent guidelines.
* [Architectural Manifesto](ARCHITECTURE.md) - The structural axes: Platform, Machine, and Driver.
* [Growth Model](GROWTH_MODEL.md) - The lifecycle phases: BRAN, Seed, Sprout, and Bloom.
* [Agent Operational Briefing](AGENTS.md) - **Read this if you are an AI agent.**

## Status

<!-- DOCGEN:STATUS:BEGIN -->
## Test Status

> _This section is auto-generated from BDD test results. Do not edit by hand._

### Boot contract and system bring-up

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| Bran hands off a boot contract to the Kernel | ✅ | ❌ | ❌ | ❌ |
| Sprout starts and publishes its presence in the graph | ❌ | ❌ | ❌ | ❌ |
| Sprout starts core services | ❌ | ❌ | ❌ | ❌ |
| The Kernel exposes a root graph and a devices graph | ❌ | ❌ | ❌ | ❌ |

### Bytespaces and address spaces

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| A process can create a bytespace and map it with permissions | ✅ | ❌ | ❌ | ❌ |
| Task stacks have explicit ownership and do not leak | ✅ | ❌ | ❌ | ❌ |
| Unmapping removes access | ✅ | ✅ | ✅ | ✅ |

### Capabilities and least privilege

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| A process cannot map memory without a mapping capability | ✅ | ✅ | ✅ | ✅ |
| A process cannot read raw input without an input capability | ✅ | ✅ | ✅ | ✅ |
| Framebuffer details are not exposed without explicit capability | ✅ | ✅ | ✅ | ✅ |
| Insecure escape hatches are feature-flagged | ✅ | ✅ | ✅ | ✅ |

### Demo app vertical slice

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| logview draws and receives input | ✅ | ✅ | ✅ | ✅ |
| logview exits without tearing down the world | ✅ | ✅ | ✅ | ✅ |
| logview logs through syscall and appears in the graph | ✅ | ✅ | ✅ | ✅ |
| logview starts as a user task with a heap | ✅ | ✅ | ✅ | ✅ |

### Graph REPL over Serial (OpenGQL)

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| Create a Thing using CREATE | ✅ | ⚪ | ⚪ | ⚪ |
| Create a node | ✅ | ⚪ | ⚪ | ⚪ |
| Create a relationship | ✅ | ⚪ | ⚪ | ⚪ |
| Create a relationship between Things | ✅ | ⚪ | ⚪ | ⚪ |
| Empty input does nothing | ✅ | ⚪ | ⚪ | ⚪ |
| Empty input is ignored | ✅ | ⚪ | ⚪ | ⚪ |
| Exit the REPL | ✅ | ⚪ | ⚪ | ⚪ |
| Invalid OpenGQL syntax | ✅ | ⚪ | ⚪ | ⚪ |
| Multi-line OpenGQL query | ✅ | ⚪ | ⚪ | ⚪ |
| Multi-line query input | ✅ | ⚪ | ⚪ | ⚪ |
| Query history is preserved | ✅ | ⚪ | ⚪ | ⚪ |
| Query history is recorded | ✅ | ⚪ | ⚪ | ⚪ |
| Query nodes using MATCH | ✅ | ⚪ | ⚪ | ⚪ |
| Query returns no results | ✅ | ⚪ | ⚪ | ⚪ |
| Query with no matching results | ✅ | ⚪ | ⚪ | ⚪ |
| REPL announces readiness | ✅ | ⚪ | ⚪ | ⚪ |
| REPL exit command | ✅ | ⚪ | ⚪ | ⚪ |
| Semantic error handling | ✅ | ⚪ | ⚪ | ⚪ |
| Simple MATCH query | ✅ | ⚪ | ⚪ | ⚪ |
| Syntax error handling | ✅ | ⚪ | ⚪ | ⚪ |
| Unknown REPL command | ✅ | ⚪ | ⚪ | ⚪ |
| Unknown command | ✅ | ⚪ | ⚪ | ⚪ |
| Valid syntax but invalid semantics | ✅ | ⚪ | ⚪ | ⚪ |

### Graph as the system

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| A task is linked to its owning process | ✅ | ❌ | ❌ | ✅ |
| Devices appear in graph.devices | ✅ | ❌ | ❌ | ✅ |
| Tasks are represented as Things | ✅ | ❌ | ❌ | ✅ |
| The graph is queryable at runtime | ✅ | ❌ | ❌ | ✅ |

### Input pipeline and focus model

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| Focus determines which surface receives TextEvent | ✅ | ❌ | ❌ | ❌ |
| Raw scancodes can be read by inputd with capability | ✅ | ❌ | ❌ | ❌ |
| inputd publishes KeyEvent Things to the graph | ✅ | ✅ | ✅ | ✅ |

### Keyboard input pipeline

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| A layout service can convert key events to text events | ❌ | ⚪ | ⚪ | ⚪ |
| Changing focus changes routing immediately | ❌ | ⚪ | ⚪ | ⚪ |
| Delivery can be traced | ❌ | ⚪ | ⚪ | ⚪ |
| Delivery is non-blocking | ❌ | ⚪ | ⚪ | ⚪ |
| Focus determines which window receives keyboard events | ❌ | ⚪ | ⚪ | ⚪ |
| Key activity produces raw key events | ❌ | ⚪ | ⚪ | ⚪ |
| Key events are deliverable to user programs | ❌ | ⚪ | ⚪ | ⚪ |
| Keyboard events are buffered during bursts | ❌ | ⚪ | ⚪ | ⚪ |
| Keyboard events are observable as Things | ❌ | ⚪ | ⚪ | ⚪ |
| Multiple consumers can observe raw key events | ❌ | ⚪ | ⚪ | ⚪ |
| No heavy work is required to capture key events | ❌ | ⚪ | ⚪ | ⚪ |
| Non-printing keys do not produce text | ❌ | ⚪ | ⚪ | ⚪ |
| The system exposes a keyboard-capable input device when available | ❌ | ⚪ | ⚪ | ⚪ |
| The system still boots without a keyboard | ❌ | ⚪ | ⚪ | ⚪ |

### Multi-architecture behavioral parity

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| Boot reaches kernel ready on each architecture | ✅ | ✅ | ✅ | ✅ |
| Missing devices are represented as absence, not failure | ✅ | ✅ | ✅ | ✅ |
| The syscall ABI returns a structured result on each architecture | ✅ | ✅ | ✅ | ✅ |

### Scheduler liveness and fairness

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| A blocked task wakes when its watch fires | ✅ | ✅ | ✅ | ✅ |
| A task may block on a deadline watch | ✅ | ✅ | ✅ | ✅ |
| Runnable tasks eventually run | ✅ | ✅ | ✅ | ✅ |
| The scheduler switches tasks over time | ✅ | ✅ | ✅ | ✅ |

### Surfaces and compositing

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| A surface is a Thing with ownership and bounds | ✅ | ❌ | ❌ | ✅ |
| Bloom can paint the screen a solid color | ✅ | ❌ | ❌ | ✅ |
| The display backend is swappable | ✅ | ❌ | ❌ | ✅ |

### Swappable display backends

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| Bloom can paint a solid color on any provider | ✅ | ✅ | ✅ | ✅ |
| Bloom cannot access display bytespace without capability | ✅ | ❌ | ❌ | ✅ |
| Limine framebuffer provider exposes a primary display bytespace | ✅ | ❌ | ❌ | ❌ |
| RAMFB provider exposes the same graph contract | ✅ | ✅ | ✅ | ❌ |

### Watches and event-driven waiting

| Scenario | x86_64 | aarch64 | riscv64 | loongarch64 |
|----------|--------|---------|---------|-------------|
| A device watch fires on an input event | ✅ | ✅ | ✅ | ✅ |
| A graph watch fires when a Thing is added to a graph | ✅ | ✅ | ✅ | ✅ |
| Waiting on "any watch" returns when one fires | ✅ | ✅ | ✅ | ✅ |

<!-- DOCGEN:STATUS:END -->

## Quick Start (x86_64, QEMU)

ThingOS is developed primarily on Linux using Rust nightly and QEMU.

### Prerequisites

You’ll need:

* Rust (nightly)
* `cargo`
* `qemu-system-x86_64`
* `llvm-tools-preview`
* `xorriso`
* `just`

On Debian/Ubuntu-like systems:

```sh
sudo apt install \
  qemu-system-x86 \
  xorriso \
  clang \
  lld \
  llvm \
  just
```

Install Rust nightly:

```sh
rustup toolchain install nightly
rustup default nightly
rustup component add llvm-tools-preview
```

### Build the System

From the repository root:

```sh
just iso x86_64
```

This will:
* build the kernel
* build userland programs and drivers
* assemble a bootable ISO using Limine

The ISO will appear under:
`target/iso/`

### Run in QEMU

```sh
just intel     # Run x86_64 (Intel/AMD)
just arm       # Run aarch64 (ARM64)
just risc      # Run riscv64 (RISC-V)
just loong     # Run loongarch64 (LoongArch)

# Or for headless (no graphics):
just run-headless x86_64
```

You should see:
* early boot logs over serial
* graph initialization
* drivers loading
* userland programs starting
* framebuffer output once the compositor comes up

If the screen is black but logs are scrolling, that usually means:
* the framebuffer driver is alive
* the compositor hasn’t claimed the screen yet

That’s normal during development.

### Debugging

To enable QEMU with GDB:

```sh
just run x86_64
```
(It listens on port 1234 by default for GDB)

Then in another terminal:

```sh
gdb target/x86_64/debug/kernel
```
(And connect with `target remote :1234`)

Serial logs are your best friend.
If something feels stuck, it usually is — on a lock, an interrupt, or a graph dependency.

### Supported Architectures

* ✅ x86_64 (primary)
* 🚧 aarch64 (in progress)
* 🚧 riscv64 (in progress)
* 🚧 loongarch64 (in progress)

### What to Try First

Good entry points for exploration:
* `crates/sprout` — the init process (simple userland program)
* `crates/bloom` — the compositor
* `crates/kernel/src/graph.rs` — the heart of the system
* `crates/abi` — the contract between kernel and userland

## What Makes ThingOS Different

### 1. The Graph Is the Kernel API

There is no traditional POSIX interface.

Instead, user programs and drivers communicate with the kernel by issuing graph operations:

* Create Things
* Link Things
* Update Things
* Observe changes

The kernel enforces safety, ownership, and scheduling — but the shape of the system lives in the graph.

Think of it as:

* a filesystem that understands relationships,
* an object model that spans kernel and userland,
* and an event system where state itself is observable.

### 2. Drivers Are Just Programs (With Privileges)

Drivers are not magical kernel modules.

They are:

* user programs,
* with explicit capabilities,
* that speak to hardware through a thin hardware bridge layer.

A keyboard driver, a framebuffer driver, and a clock driver all look structurally similar:
they observe Things, react to changes, and update the graph.

This keeps the kernel small and moves complexity outward where it belongs.

### 3. No Fake Abstractions

ThingOS avoids pretending things are simpler than they are.

There is:

* no fake “everything is a file” story,
* no hidden global state,
* no opaque IOCTL jungles.

If something exists, it exists as a Thing.
If something happens, it happens as a graph change.

## System Architecture (High Level)

```
┌─────────────────────────┐
│        User Apps        │
│  (clock, compositor…)   │
└───────────┬─────────────┘
            │ GraphOps
┌───────────▼─────────────┐
│        Kernel            │
│  - Scheduler             │
│  - Memory                │
│  - Graph Engine          │
│  - Capability Checks    │
└───────────┬─────────────┘
            │ Bridge Calls
┌───────────▼─────────────┐
│   Hardware Bridges       │
│ (x86_64, aarch64…)       │
└───────────┬─────────────┘
            │
        Real Hardware
```

The kernel owns:

* scheduling,
* memory,
* isolation,
* and the authoritative graph.

Everything else is layered on top.

## The Graph Model (In Brief)

* **Things**: Typed nodes with structured data.
* **Links**: Directed, typed relationships between Things.
* **Symbols**: Interned identifiers for stable naming across kernel and userland.
* **Observation**: Programs can watch the graph and react to changes instead of polling.

This makes the system naturally reactive:
windows redraw because the graph changed,
input flows because keys became Things,
time passes because the clock updates state.

## Userland

User programs are:

* `no_std`
* written in Rust
* linked as ELF binaries
* launched by the kernel loader

They interact with the system via a small ThingOS standard library, which:

* wraps graph operations,
* provides basic services (console output, time, sleep),
* avoids baking in policy.

There is no libc, and no POSIX compatibility layer by default.

**This is intentional.**

## Boot Process (Simplified)

1. **Bootloader (Limine)**
   * Sets up the environment and loads initial modules.
2. **Kernel Init**
   * Memory initialization
   * Hardware bridge setup
   * Graph initialization
   * Symbol table seeding
3. **Loader Program**
   * Loads drivers and services
   * Spawns initial user programs
   * Brings up the compositor and input stack
4. **The System Becomes Alive**
   * The graph starts changing
   * Programs observe and react
   * The UI appears as a side effect

## Current Status

ThingOS is actively evolving and not yet stable.

**What exists today:**

* A working kernel for x86_64, aarch64, riscv64, and loongarch64
* A functioning graph engine
* Userland programs
* Input drivers
* Framebuffer output
* A compositor in progress
* Real scheduling and isolation

**What does not exist yet:**

* Persistent storage
* Networking
* Security hardening
* ABI stability
* Documentation beyond this README

Breaking changes are expected.

## Why Build This?

Because existing operating systems:

* hide too much,
* lie about structure,
* and make introspection painful.

ThingOS is an attempt to build an OS that is:

* honest about its state,
* inspectable at runtime,
* composable instead of monolithic,
* and pleasant to reason about.

It is a research project, a playground, and a serious attempt — all at once.

## Non-Goals

ThingOS is not:

* Linux-compatible
* POSIX-compliant
* fast (yet)
* safe for real workloads
* intended for end users

It is intended for learning, experimentation, and rethinking assumptions.

## Contributing

This project is exploratory and opinionated.

If you’re interested in:

* operating systems,
* graph-based models,
* kernels without historical baggage,
* or simply strange ideas taken seriously,

then contributions, questions, and discussion are welcome.

Expect sharp edges.

## License

Apache License 2.0.
