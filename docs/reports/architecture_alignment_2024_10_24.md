# Architecture Alignment Review: ThingOS Kernel
**Date:** 2024-10-24
**Auditor:** Jules

## 1. Executive Summary

The ThingOS repository has been brought back into alignment with its core invariants. Since the previous review (Oct 18, 2024), critical ABI pollution and drift have been aggressively remediated.

The `abi` crate is now a strict wire-definition library ("Postcard"), stripped of `alloc` dependencies and high-level Rust enums. The authority for system ontology (`PropValue`, `PropKey`) has been correctly moved to `thing_models`. Duplicate syscall definitions have been eliminated. The system is now safer, flatter, and more honest about its boundaries.

## 2. Invariant-by-Invariant Assessment

### 1) Kernel Correctness Over Features
**Status:** 🟢 **Green**
The kernel continues to prioritize correctness. Panic sites (`unwrap()`) are down to ~38. No new features have compromised stability.

### 2) ABI Is a Postcard
**Status:** 🟢 **Green** (Remediated)
Major improvement.
-   **Fixed:** `abi` no longer depends on `alloc` (in `lib.rs`).
-   **Fixed:** `PropValue` (which uses `Vec`/`String`) moved to `thing_models`.
-   **Fixed:** `PropKey` and `PropType` moved to `thing_models`.
-   **Fixed:** `ThingPropData` (legacy wire format) is now `#[deprecated]`, favoring `WireProp`.
The ABI is now properly focused on `repr(C)` structs and syscall constants.

### 3) Clear Authority Boundaries
**Status:** 🟢 **Green** (Remediated)
The boundary between `abi` and `thing_models` is now crisp.
-   `abi`: Defines bytes, syscall numbers, and handles.
-   `thing_models`: Defines the ontology (`PropValue` enum, `Kind` strings).
-   `thing_os`: Provides the userland runtime (and re-exports `thing_models` types).
Consumers like `kernel` and `boot` now import high-level types from `thing_models`, respecting the "Ontology Authority" invariant.

### 4) Schema Authority Is Centralized
**Status:** 🟢 **Green**
`thing_models` and `kernel/src/graph/schema.rs` remain strongly aligned. Registration logic is robust.

### 5) No Undefined Behavior as a Design Tool
**Status:** 🟢 **Green**
No regressions.

### 6) Drift Is the Enemy
**Status:** 🟢 **Green** (Remediated)
-   **Fixed:** `abi/src/syscall_numbers.rs` (the duplicate source of truth) has been **deleted**.
-   **Fixed:** Userland drivers (`ps2_keyboard`, `ps2_mouse`) now use the authoritative `abi::syscalls` constants.
-   **Mitigated:** `ThingPropData` is marked deprecated to prevent future usage drift.

## 3. Concrete Findings (Remediation Report)

### 1. Syscall Drift Eliminated
*   **Action:** Deleted `abi/src/syscall_numbers.rs`.
*   **Result:** `abi/src/syscalls.rs` is now the Single Source of Truth for syscall numbers.

### 2. ABI Pollution Purged
*   **Action:** Moved `PropValue`, `PropKey`, `PropType` to `thing_models/src/props.rs`.
*   **Action:** Removed `extern crate alloc` from `abi/src/lib.rs`.
*   **Result:** The `abi` crate is now `no_std` and allocator-agnostic, suitable for raw wire usage.

### 3. Wire Format Consolidation
*   **Action:** Added `#[deprecated]` to `ThingPropData` in `abi/src/lib.rs`.
*   **Result:** Future code is steered towards the blob-based `WireProp` format, reducing protocol confusion.

## 4. Top 3 Strengths to Protect

1.  **The New Clean ABI:** Do not re-add `alloc` or complex Enums to `abi`. Keep it flat.
2.  **Schema Centralization:** `thing_models` is the pristine home for system types. Keep it that way.
3.  **Syscall Macro System:** The `for_each_syscall!` macro is the gold standard for maintaining the syscall table.

## 5. Suggested Follow-Up Tasks

1.  **Finish Wire Format Migration:** Update `SYSCALL_THING_GET` implementation in the kernel to use `WireProp` (blob format) instead of the deprecated `ThingPropData`.
2.  **Strict Enforce ABI:** Add a CI check to ensure `abi` does not depend on `alloc` or `std`.
