# Architecture Alignment Review: ThingOS Kernel
**Date:** 2025-12-20
**Auditor:** Jules

## 1. Executive Summary

The ThingOS repository is in a **precarious** state of alignment. While the core invariants of "Kernel Correctness" and "Centralized Schema Authority" are holding well in the `thing_models` and `kernel` relationship, the **ABI layer is suffering from significant identity confusion and drift**.

The `abi` crate explicitly violates the "ABI is a Postcard" invariant by mixing raw wire types with high-level Rust abstractions (`String`, `Vec`, `&'static str`). This has led to dual definitions of syscall numbers and wire formats, creating a minefield for future development. The system is working, but the boundaries are blurred, risking architectural corruption if the ABI is not strictly flattened.

## 2. Invariant-by-Invariant Assessment

### 1) Kernel Correctness Over Features
**Status:** 🟢 **Green** (Mostly)
The kernel implementation focuses on "boring" correctness. The scheduler and graph store are simple. There is no evidence of chasing flashy features at the cost of stability. However, the presence of 64 `unwrap()` calls in `kernel/src` suggests a fragility that contradicts the "robustness" requirement.

### 2) ABI Is a Postcard
**Status:** 🔴 **Red**
The `abi` crate is polluted with Rust-specific abstractions. It exports `KernelRequest` and `PropValue` containing `alloc::string::String` and `&'static str`. The ABI should purely define memory layouts (`repr(C)` structs) and constants. Instead, it serves as a shared high-level library between kernel and userland, which is explicitly forbidden ("No Rust-only abstractions across ABI").

### 3) Clear Authority Boundaries
**Status:** 🟡 **Yellow**
`thing_models` correctly acts as the "kernel-known truth" for schemas. `kernel` properly enforces this by registering them. However, the boundary between `abi` and `thing_os` is wrong. `thing_os` (userland) should define the ergonomic Rust types (`KernelRequest` enum), but currently, `abi` defines them, forcing the kernel to essentially link against a "userland-ish" library.

### 4) Schema Authority Is Centralized
**Status:** 🟢 **Green**
`thing_models` is the single source of truth. `kernel/src/model.rs` imports and registers `kernel_core_schemas()`. This invariant is strongly upheld.

### 5) No Undefined Behavior as a Design Tool
**Status:** 🟢 **Green**
Zero instances of `mem::transmute` found in `kernel/src`. `unsafe` is used appropriately in `arch` and FFI boundaries. The kernel avoids UB shortcuts.

### 6) Drift Is the Enemy
**Status:** 🔴 **Red**
Significant drift detected in syscall numbers and wire formats.
- **Syscall Numbers:** `abi/src/syscalls.rs` (used by `arch`) vs `abi/src/syscall_numbers.rs` (unused/divergent).
- **Wire Formats:** `WireProp` (blob-based) vs `ThingPropData` (fixed-buffer). Two ways to do the same thing.

## 3. Concrete Findings

### ABI Pollution
*   **File:** `abi/src/lib.rs`
    *   `PropKey` defined as `alloc::string::String`.
    *   `PropValue` enum contains `Str(alloc::string::String)` and `Blob(alloc::vec::Vec<u8>)`.
    *   `KernelRequest` enum contains `&'static str` (e.g., `Log { message: &'static str }`).
    *   **Violation:** "ABI must be: data-only... No Rust-only abstractions... no &str".

### Syscall Number Drift
*   **File:** `abi/src/syscall_numbers.rs`
    *   Defines `SYS_RESIDENT_ALLOC = 0x60` (96).
*   **File:** `abi/src/syscalls.rs`
    *   Defines `SYSCALL_RESIDENT_ALLOC = 26`.
*   **File:** `arch/src/x86_64/syscall.rs`
    *   Uses `abi::syscalls::*`.
    *   **Violation:** "There must be a single source of truth for: syscall numbers". `syscall_numbers.rs` is dead/wrong code.

### Dual Wire Formats
*   **File:** `abi/src/wire/graph.rs`
    *   Defines `WireProp` using `WireValueTag` and `Blob` pointer/len. Used by `SYSCALL_THING_CREATE`.
*   **File:** `abi/src/lib.rs`
    *   Defines `ThingPropData` using fixed `[u8; 128]` buffers. Used by `SYSCALL_THING_GET`.
    *   **Violation:** Inconsistent wire protocol for properties.

### Fragile Error Handling
*   **File:** `kernel/src/graph/mod.rs` (and others)
    *   `store::things_slab().lock().as_ref().unwrap()`
    *   **Violation:** "Invalid user input must yield deterministic errors, never UB". While this is a boot invariant, widespread `unwrap()` makes the kernel prone to panics rather than graceful failure.

## 4. Top 3 Risks if Unaddressed

1.  **ABI Ossification with Rust Types:** If external drivers/apps bind to the current `abi` crate, breaking changes will be required to remove `String`/`Vec` later. We are leaking `alloc` dependencies into the wire definition.
2.  **Syscall Confusion:** A developer adding a new syscall might add it to `syscall_numbers.rs` and wonder why it doesn't work, or collide with an existing number in `syscalls.rs`.
3.  **Serialization Inconsistency:** Having two property wire formats (`WireProp` vs `ThingPropData`) complicates the compositor/inspector tools and increases the surface area for bugs.

## 5. Top 3 Strengths to Protect

1.  **Schema Centralization:** `thing_models` is a pristine example of Invariant 4. It separates ontology from implementation perfectly.
2.  **UB Discipline:** The kernel is remarkably free of "clever" `transmute` hacks. The `unsafe` blocks are largely contained in the `arch` layer where they belong.
3.  **Graph-First Design:** The kernel consistently uses the graph for state (scheduler, memory), adhering to the core philosophy.

## 6. Suggested Follow-Up Tasks

1.  **Purge `abi` of `alloc`:** Move `KernelRequest`, `PropValue`, and `Thing` trait to `thing_os` (or a new `thing_api` crate). Leave only `WireProp`, `WireThing`, and `SYSCALL_*` constants in `abi`.
2.  **Unify Syscall Numbers:** Delete `abi/src/syscall_numbers.rs` and ensure `abi/src/syscalls.rs` is the authoritative source.
3.  **Unify Wire Formats:** Deprecate `ThingPropData` (fixed buffer) and migrate `SYSCALL_THING_GET` to use a batched/blob approach similar to `WireProp` (or vice-versa, but pick one).
