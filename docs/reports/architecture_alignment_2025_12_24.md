# Periodic Architecture Alignment Review

**Date:** 2025-12-24
**Agent:** Jules

## Executive Summary

The repository currently exhibits a mix of strong adherence to core philosophies (Graph Correctness, Thing Ontology) and significant architectural drift in the ABI and Safety layers. While `thing_models` firmly holds the system ontology, the `abi` crate is suffering from an identity crisis: it defines a high-level `KernelRequest` enum that mimics a wire format but is actually an internal kernel abstraction, while the true wire format (registers) remains implicit in `arch` macros.

Most critically, **safety invariants are being actively violated in `thing_os`**, where `unsafe { transmute }` is used to convert syscall integers into enums without validation. This is a ticking time bomb for Undefined Behavior.

The system is not in immediate danger of collapse, but the "ABI Is a Postcard" invariant is currently a lie, and the "No UB" invariant is being ignored in userland syscall wrappers.

## Invariant-by-Invariant Assessment

### 1. Kernel Correctness Over Features
**Status:** 🟢 **Green**
The kernel implementation (`kernel/src`) prioritizes correctness. The scheduler, memory management, and graph logic are rigorous. The use of `ThingId` and explicit ownership is strong. The kernel does not bend to userland whims; it enforces schema and permissions.

### 2. ABI Is a Postcard
**Status:** 🔴 **Red**
The philosophy demands a "data-only, flat, explicit" ABI.
*   **Violation:** `abi` defines `KernelRequest`, a Rust enum that acts as a "fat" interface. It is *not* the wire format. The wire format is registers (`u64`).
*   **Confusion:** `thing_os` constructs `KernelRequest` only to immediately destructure it into raw syscalls. `arch` receives raw syscalls and sometimes reconstructs `KernelRequest`. This double-conversion obscures the actual boundary.
*   **Pollution:** `abi` depends on `alloc` (`Vec`, `String`). While this is pragmatic for `thing_models`, it violates the strict "Postcard" definition for the wire layer.

### 3. Clear Authority Boundaries
**Status:** 🟡 **Yellow**
*   **Good:** `thing_models` is the clear source of truth for the ontology. `kernel` enforces it.
*   **Risk:** Userland apps (like `compositor`) are defining their own Schemas (e.g., `pkg.compositor.FrameRenderIntent`) in ad-hoc ways. While namespaced (`pkg.*`), the lack of a centralized userland schema registry (or `thing_models` equivalent for user-types) invites fragmentation.
*   **Good:** `arch` correctly acts as the translation layer between the raw wire and the kernel proper.

### 4. Schema Authority Is Centralized
**Status:** 🟢 **Green**
The kernel's `graph/schema.rs` enforces schema registration and fingerprinting. It correctly rejects conflicts. `thing_models` provides the "Standard Library" of schemas.

### 5. No Undefined Behavior as a Design Tool
**Status:** 🔴 **Red**
*   **Critical Violation:** `thing_os/src/syscalls.rs` uses `unsafe { core::mem::transmute }` to cast syscall return values (integers) to enums (e.g., `PixelFormat`, `SchemaRegistryOutcome`).
*   **Why it's bad:** If the kernel (or a malicious replacement) returns an invalid integer, the user process enters immediate UB.
*   **Fix:** Use `TryFrom` or explicit `match` with a default error variant.

### 6. Drift Is the Enemy
**Status:** 🟡 **Yellow**
*   **Drift:** The `KernelRequest` enum in `abi` implies a single dispatch path, but `arch` implements a "fast path" for some syscalls (like `SYSCALL_LOG`) that bypasses `KernelRequest` reconstruction. This creates two sources of truth for how a syscall is handled.

## Concrete Findings

### The Ugly 🪳 (Safety & Invariants)
*   **`thing_os/src/syscalls.rs`**: Multiple instances of `unsafe { core::mem::transmute(ret as u8) }`. This is textbook UB risk.
    *   `KernelRequest::SchemaRegisterPackage`: `transmute` for `SchemaRegistryOutcome`.
    *   `KernelRequest::CreateSharedBuffer`: `transmute` for `PixelFormat`.
*   **`thing_os/src/panic.rs`**: `unsafe` blocks used liberally.

### The Bad ⚠️ (Architecture & Confusion)
*   **`abi/src/requests.rs`**: Defines `KernelRequest`, which is a "Fake ABI". It suggests the kernel receives this enum, but it actually receives registers. It's a shared abstraction that misleads developers about the cost and shape of the boundary.
*   **`arch/src/x86_64/syscall.rs`**: Inconsistent handling.
    *   `SYSCALL_LOG`: Direct call to `kernel::log`.
    *   `SYSCALL_SPAWN_PROGRAM`: Rehydrates `KernelRequest::SpawnProgram` then calls `kernel::handle_request`.
    *   This inconsistency makes it hard to audit the full surface area.

### The Good ✅
*   **`abi/src/syscalls.rs`**: The `for_each_syscall!` macro is a brilliant way to enforce a single source of truth for syscall numbers across the entire codebase.
*   **`kernel/src/graph`**: The graph implementation is disciplined. Explicit `ThingId` usage, proper transactions, and schema enforcement.
*   **`thing_models`**: A clean, centralized ontology that keeps the kernel "boring" by moving domain definitions out.

## Top 3 Risks if Unaddressed

1.  **Userland Instability (UB):** A simple kernel change (adding an enum variant or returning an error code) could silently cause userland processes to misbehave or crash with cryptic memory errors due to `transmute`.
2.  **ABI Ossification:** The "Fake ABI" (`KernelRequest`) makes it harder to change the actual wire format because developers are coding against the high-level enum, not the primitives. Optimizing syscalls becomes harder.
3.  **Schema Fragmentation:** Without a clearer pattern for userland schemas, we will end up with `pkg.compositor`, `pkg.launcher`, `pkg.shell` all defining incompatible versions of "Window" or "Intent".

## Top 3 Strengths to Protect

1.  **The Graph Database Core:** The decision to make the kernel a graph database of `Thing`s is working. It provides a unified, inspectable state model.
2.  **Macro-Driven Syscalls:** The `for_each_syscall!` macro prevents number collisions and makes adding syscalls a "fill in the blank" exercise.
3.  **Strict Ontology separation:** Keeping `thing_models` separate from `kernel` prevents the kernel from becoming a monolith of business logic.

## Suggested Follow-Up Tasks

1.  **Safety Sweep:** Immediately replace all `transmute` in `thing_os/src/syscalls.rs` with `match` or `TryFrom` (returning `Result`).
2.  **Clarify ABI:** Rename `KernelRequest` to `SyscallShape` or similar, or enforce its usage everywhere in `arch` to remove the "Dual Path" drift. Alternatively, delete it and make `thing_os` and `arch` rely solely on the `for_each_syscall!` signatures.
3.  **Standardize User Schemas:** Create a `thing_user_models` or similar pattern for shared userland concepts, or document a strict "Package Registry" protocol.
