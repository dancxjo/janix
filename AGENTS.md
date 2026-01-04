# Agent Operational Briefing

**Read this first.**

You are an agent working on ThingOS. This repository is not a standard Rust project. It is an ontological experiment with strict rules. Your primary goal is to **maintain conceptual integrity**, even at the cost of immediate convenience.

## 1. The Prime Directive

**The Graph is the Truth.**

*   Do not add hidden state (static muts, side-channel arrays).
*   Do not create magical global managers.
*   If it exists, it must be a **Thing** in the graph.
*   If it relates to something else, it must have a **Link**.

## 2. Navigation Overview

Map of the territory:

```
crates/
├── abi/          # The System Contract (Wire types, ThingIDs)
├── bran/         # Boot scaffolding (The "Loader") - Disappears after boot
├── kernel/       # The Graph Host (Scheduling, Memory, Capability)
├── models/       # Schema Definitions (The "Seed" data structures)
├── sprout/       # PID 1 (The first graph tenant)
├── bloom/        # Compositor (The visualizer)
└── thing_std/    # Userland library (No libc, just GraphOps)

machines/         # Machine descriptions (Hardware facts)
xtask/            # Build system & tooling (Rust-based make)
```

## 3. Operational Protocols (How to work)

### Building & Running
Use `just`. Do not use `cargo run` directly unless you know exactly why.

*   **Build everything**: `just build`
*   **Run x86_64**: `just run-x86_64` (Graphic) or `just run-headless x86_64`
*   **Run AArch64**: `just run-aarch64`
*   **Reset Environment**: `just die` (Kills stuck QEMU/GDB instances)

### Testing
*   **Unit & BDD Tests**: `just test` (Runs all architectures)
*   **Architecture specific**: `just test-x86_64`

### Debugging
*   **GDB Attach**:
    1.  `just run x86_64` (Listens on :1234)
    2.  `gdb target/x86_64/debug/kernel`
    3.  `target remote :1234`
*   **Stuck/Frozen?**: Use `just die` to clean up orphaned QEMU processes.

## 4. File Editing Tool Workaround

> [!CAUTION]
> **The `replace_file_content` and `multi_replace_file_content` tools corrupt files** when attempting to replace multi-line blocks. They duplicate the top of the file instead of replacing the target block.

**Mandatory Workarounds:**

1.  **Avoid Multi-line Targets**: Never try to match large blocks of code spanning multiple lines.
2.  **Use Single-Line Anchors**: Target a single, unique line to anchor your replacement.
3.  **Rewrite Entire Files**: For larger changes, use `write_to_file` with `Overwrite: true` to safely replace the entire file contents.

When in doubt, **rewrite the whole file**. It is safer than risking corruption.

## 5. Architectural Constraints (The Strict Rules)

Refer to `ARCHITECTURE.md` and `GROWTH_MODEL.md` for deep context.

1.  **Phase Separation**:
    *   **BRAN** code must not persist.
    *   **Kernel** must not render pixels (that's Bloom's job).
    *   **Seed** must not contain code. (It's just an ISO image.)

2.  **Platform vs Machine**:
    *   `Platform`: CPU stuff (Registers, Page Tables) -> `arch/` (in kernel)
    *   `Machine`: Hardware stuff (UART, PCI) -> `machines/`

3.  **No shortcuts**:
    *   Do not carry POSIX assumptions. There is no `fork()`. There are no files (only ByteSpaces).
    *   Do not import `std` in kernel or drivers. Use `core` and `alloc`.

## 6. What To Do When You Are Stuck

1.  **Check the Graph**: Can you model the problem as a missing Thing?
2.  **Check the Phase**: Are you trying to do a Bloom task in Sprout?
3.  **Check the Logs**: Serial logs are the primary debug output.
4.  **Reset**: `just die` is your friend if the terminal gets weird.

## 7. Self-Correction Checklist

Before you commit code:
*   [ ] Did I add hidden global state? (If yes -> **Refactor to Graph**)
*   [ ] Did I hardcode a machine assumption in the kernel? (If yes -> **Move to Machine**)
*   [ ] Did I confuse a Platform (CPU) with a Machine (Board)?
*   [ ] Did I break `just test`?

## 8. Knowledge Base

*   **Philosophy**: `MANIFESTO.md`
*   **Structure**: `ARCHITECTURE.md`
*   **Lifecycle**: `GROWTH_MODEL.md`

**Go forth and make the graph truthful.**
