# Architecture Alignment Review: ThingOS Kernel
**Date:** 2024-10-24
**Auditor:** Jules

## 1. Executive Summary

The ThingOS repository shows a mix of disciplined progress and persistent architectural danger. Since the previous review (Oct 18, 2024), the kernel has improved its internal robustness (reducing `unwrap()` usage by ~40%), and the schema authority in `thing_models` remains pristine.

However, the **ABI layer remains a critical liability**. The "Postcard" invariant is actively violated: the `abi` crate is still heavily polluted with Rust-specific `alloc` types (`String`, `Vec`), acting more like a shared library than a strict wire definition. Furthermore, **drift has not been addressed**: duplicate syscall number files and competing wire formats persist, creating a confused boundary between kernel and userland.

Immediate action is required to flatten the ABI and purge the drift, or the system risks ossifying around these incorrect boundaries.

## 2. Invariant-by-Invariant Assessment

### 1) Kernel Correctness Over Features
**Status:** 🟢 **Green**
The kernel continues to prioritize correctness. A significant effort to reduce panics is visible (`unwrap()` count dropped from 64 to 38). No new "flashy" features have compromised stability.

### 2) ABI Is a Postcard
**Status:** 🔴 **Red** (Unchanged)
The `abi` crate is still failing this invariant. It exports `PropKey` as `String` and `PropValue` with `Vec<u8>`. `KernelRequest` uses `UserSlice` (good) but is co-located with high-level Rust enums. The presence of `extern crate alloc` in `abi/src/lib.rs` is a smoking gun. The ABI is not a flat postcard; it's a Rust crate.

### 3) Clear Authority Boundaries
**Status:** 🟡 **Yellow** (Unchanged)
The boundary remains blurred. `thing_models` is correct (ontology), but `abi` is doing too much work for `thing_os`. `thing_os` should be the one defining the ergonomic Rust wrapper types, not `abi`. The kernel is currently importing types that look like they belong in a high-level SDK.

### 4) Schema Authority Is Centralized
**Status:** 🟢 **Green**
`thing_models` and `kernel/src/graph/schema.rs` are strongly aligned. The registration logic properly checks fingerprints (`schema.rs:49`) and enforces strict typing (`schema.rs:116`). This subsystem is a model citizen.

### 5) No Undefined Behavior as a Design Tool
**Status:** 🟢 **Green**
No regressions found. The codebase avoids `transmute` hacks. `unsafe` remains contained.

### 6) Drift Is the Enemy
**Status:** 🔴 **Red** (Unchanged)
The "Drift" findings from the previous report were **not addressed**.
- `abi/src/syscall_numbers.rs` still exists and conflicts with `abi/src/syscalls.rs`.
- Two wire formats (`WireProp` vs `ThingPropData`) still exist side-by-side.

## 3. Concrete Findings

### ABI Leaks (The Ugly)
*   **File:** `abi/src/lib.rs` & `abi/src/prop_value.rs`
    *   `extern crate alloc;` is still present.
    *   `PropKey` is a type alias for `String`.
    *   `PropValue::Blob` holds a `Vec<u8>`.
    *   **Violation:** The kernel and userland share heap-allocating Rust types via the ABI crate. This makes the ABI dependent on the specific Rust allocator and `std`-like behavior.

### Persistent Drift (The Bad)
*   **File:** `abi/src/syscall_numbers.rs`
    *   Defines `SYS_RESIDENT_ALLOC = 0x60` (96).
*   **File:** `abi/src/syscalls.rs`
    *   Defines `SYSCALL_RESIDENT_ALLOC = 26`.
    *   **Impact:** Confusing source of truth. `syscall_numbers.rs` appears to be dead code that was never deleted.

### Dual Wire Formats
*   **File:** `abi/src/lib.rs` vs `abi/src/wire/graph.rs`
    *   `ThingPropData` (fixed 128-byte buffers) is still defined in `lib.rs`.
    *   `WireProp` (pointer/len blobs) is defined in `wire/graph.rs`.
    *   **Impact:** The system hasn't decided how to talk about properties. This increases complexity for any tool trying to inspect the graph.

### Improved Error Handling (The Good)
*   **Metric:** `kernel/src`
    *   `unwrap()` calls reduced to 38 (down from 64).
    *   This shows active maintenance and alignment with the "Correctness" invariant.

## 4. Top 3 Risks if Unaddressed

1.  **ABI Lock-in:** If we don't remove `String`/`Vec` from `abi` now, every future driver will depend on them. Changing it later will require a "flag day" rewrite of the entire ecosystem.
2.  **Syscall Collision:** Leaving `syscall_numbers.rs` is asking for a bug where a developer adds a syscall to the wrong file and debugging why it doesn't dispatch.
3.  **Authority Confusion:** The kernel depends on `thing_models` (Good) but also effectively depends on "Userland Rust" via the polluted `abi` crate. This creates a circular mental model where the kernel knows about high-level user types.

## 5. Top 3 Strengths to Protect

1.  **The Schema Registry:** `kernel/src/graph/schema.rs` is robust, centralized, and correct. Do not break this.
2.  **Syscall Macro System:** `abi/src/syscalls.rs` using macros to generate dispatch tables is excellent. It guarantees the kernel and userland agree on numbers (ignoring the `syscall_numbers.rs` file).
3.  **Memory Safety:** Despite the complexity of a graph OS, the kernel resists the temptation to use `unsafe` for performance in the graph layer.

## 6. Suggested Follow-Up Tasks

1.  **Delete `abi/src/syscall_numbers.rs`**: This is a trivial win to stop the bleeding.
2.  **Split `abi`**: Move `PropValue`, `PropKey`, `KernelRequest` (the Rust enums) into `thing_os` or a `thing_api` crate. Strip `abi` down to just `syscalls.rs`, `WireProp`, and `repr(C)` structs.
3.  **Decide on Wire Format**: Deprecate `ThingPropData`. Update `SYSCALL_THING_GET` to use the `WireProp` format (or a similar batch-blob format) to unify the read/write paths.
