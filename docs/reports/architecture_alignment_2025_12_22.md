# Architecture Alignment Review — 2025-12-22

## 1. Executive Summary

The ThingOS repository demonstrates strong discipline in its core ABI and Ontology layers, successfully adhering to the "ABI Is a Postcard" and "Centralized Schema" invariants. The separation between `abi` (wire) and `thing_models` (logic) is clean.

However, significant "Drift" and "Undefined Behavior" risks have accumulated in the userland and driver layers. `thing_os` is beginning to shadow `thing_models` with duplicate struct definitions, and the USB driver contains flagrant safety violations (fake `'static` lifetimes). The kernel itself remains "boring" and correct, but the ecosystem around it is showing signs of undisciplined growth. Immediate intervention is required to prevent `thing_os` from becoming a second source of truth.

## 2. Invariant-by-Invariant Assessment

| Invariant | Status | Assessment |
| :--- | :---: | :--- |
| **1) Kernel Correctness** | 🟡 **Yellow** | Core kernel is solid, but `thing_os` (standard library) forces memory leaks (`Box::leak`) on clients for simple syscalls. |
| **2) ABI Is a Postcard** | ✅ **Green** | Excellent. `WirePropValue` and `UserSlice` are explicit, flat, and data-only. No complex Rust types crossing the boundary. |
| **3) Clear Authority** | 🟡 **Yellow** | Generally good, but `thing_os` is redefining `thing_models` structs (`DisplayThing` vs `Display`), confusing the authority on what a "Display" is. |
| **4) Centralized Schema** | ✅ **Green** | `thing_models` is the clear ontology owner. Schema registration is correctly implemented. |
| **5) No Undefined Behavior** | 🔴 **Red** | Critical violation in `user/drivers/usb`. `unsafe { transmute }` is being used to forge `'static` lifetimes, risking memory corruption. |
| **6) Drift Is the Enemy** | ⚠️ **Bad** | Duplicate definitions of `SchedThreadInfo` and system structs (`Display`, `Process`, etc.) exist between `abi`, `thing_models`, and `thing_os`. |

## 3. Concrete Findings

### 🔴 The Ugly: Safety Violations & Leaks

*   **`user/drivers/usb/src/xhci.rs`**: **Fake Static Lifetimes.**
    *   Code uses `unsafe { core::mem::transmute(mmio) }` to cast a local reference to `&'static`. This is text-book Undefined Behavior and a ticking time bomb.
*   **`thing_os/src/lib.rs`**: **Intentional Memory Leak.**
    *   `create_thread` calls `Box::leak` on the thread name string before passing it to the kernel. Since `UserSlice` is just a pointer passed to a blocking syscall, this leak is wholly unnecessary and wasteful.

### ⚠️ The Bad: Architectural Drift

*   **`thing_os/src/lib.rs`**: **Shadow Structs.**
    *   Defines `DisplayThing`, `CpuCoreThing`, `SharedBufferThing` which are near-duplicates of `thing_models::Display`, `thing_models::CpuCore`, etc. This violates "Single Source of Truth". `thing_os` should re-export or use `thing_models` types.
*   **`abi/src/lib.rs` vs `thing_models/src/lib.rs`**: **Duplicate Types.**
    *   `SchedThreadInfo` and `SchemaRegistryOutcome` are defined identically in both crates. `thing_models` depends on `abi` and should use the `abi` definitions.
*   **`thing_os/src/syscalls.rs`**: **Bypassing `KernelRequest`.**
    *   `sys_pci_read_config` and `sys_dev_open/read` use raw syscall numbers and ad-hoc structs, bypassing the central `KernelRequest` enum. This fractures the ABI surface.

### ✅ The Good: Strengths to Protect

*   **`abi/src/wire/graph.rs`**: `WirePropValue` is a perfect example of a "Postcard" ABI—flat, explicit, and layout-guaranteed.
*   **`thing_models`**: correctly centralizes the system ontology. The `kernel_core_schemas` function provides a canonical registry of truth.

## 4. Top 3 Risks if Unaddressed

1.  **Memory Corruption in Drivers**: The fake `'static` pattern in `xhci.rs` will eventually cause a use-after-free kernel panic or silent corruption when the driver model evolves.
2.  **Ecosystem Split**: If `thing_os` continues to redefine `thing_models` structs, user apps will have to choose which "Display" they are talking about, leading to conversion hell and type incompatibilities.
3.  **Resource Exhaustion**: The `create_thread` leak in `thing_os` means long-running services that spawn threads will slowly consume all memory, violating "Kernel Correctness".

## 5. Top 3 Strengths to Protect

1.  **The "Postcard" Wire Format**: Do not introduce Serde or complex serialization. The manual `WireProp` packing is robust and debuggable.
2.  **Graph Ontology Authority**: Keep `thing_models` as the only place where `KIND_*` and schema structs are defined.
3.  **Explicit Syscall Macros**: The `for_each_syscall!` macro ensures `abi` and the kernel are always in sync regarding syscall numbers.

## 6. Suggested Follow-Up Tasks

1.  **Purge Fake Statics**: Refactor `user/drivers/usb` to pass context properly (e.g., via a handle or closure) instead of transmuting to static.
2.  **Unify Models**: Remove `DisplayThing`, `CpuCoreThing`, etc. from `thing_os` and implement `Thing` for `thing_models` structs if missing (or just use them).
3.  **Deduplicate ABI Types**: Remove `SchedThreadInfo` and `SchemaRegistryOutcome` from `thing_models` and use the `abi` versions.
4.  **Fix Leaks**: Rewrite `thing_os::create_thread` to use `UserSlice::from_slice(name.as_bytes())` without `Box::leak`.
