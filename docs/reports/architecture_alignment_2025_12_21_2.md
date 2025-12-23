# Architecture Alignment Review: 2025-12-21

## Executive Summary

The ThingOS repository remains structurally sound and aligned with its core "Data-Oriented" and "No Drift" philosophies, but safety discipline is fraying at the edges. The macro-driven syscall system is a standout success, ensuring perfect alignment between ABI and Architecture. However, the kernel's implementation of these syscalls relies on fragile `transmute` casts and unchecked user pointer iterations that invite Undefined Behavior (UB) and potential Denial of Service (DoS). While the "Postcard ABI" invariant is largely respected, the presence of duplicate type definitions (`SchedThreadInfo`) and `static mut` logging buffers indicates creeping technical debt. Immediate attention is required to sanitize syscall inputs and secure the logging subsystem.

## Invariant-by-Invariant Assessment

### 1) Kernel Correctness Over Features
**Status: ⚠️ YELLOW**
*   **Justification:** The kernel core remains relatively small and focused. However, the `handle_request` function performs risky pointer arithmetic (`slice_ptr.add(i)`) on user-provided lengths without sufficient bounds checking or limits, potentially allowing a malicious user to trigger long loops or invalid memory accesses. The logging subsystem uses `static mut` without proper locking, which is a concurrency time-bomb.

### 2) ABI Is a Postcard
**Status: ⚠️ YELLOW**
*   **Justification:** The wire types (`WireProp`, `ThingPropData`) are excellent examples of POD (Plain Old Data) structs. However, the syscall implementation in `arch` violates the spirit of this invariant by using `transmute` to cast raw bytes directly into Rust enums (`PixelFormat`). This turns invalid user data into immediate Kernel UB.

### 3) Clear Authority Boundaries
**Status: ✅ GREEN**
*   **Justification:** `thing_models` correctly serves as the ontology authority, while `abi` defines the wire format. The separation is mostly clean. The `kernel` acts as the enforcer, registering schemas defined in `thing_models`.

### 4) Schema Authority Is Centralized
**Status: ✅ GREEN**
*   **Justification:** Schema registration via `KernelRequest::SchemaRegisterPackage` is robust, using fingerprinting to detect conflicts.
*   **Caveat:** The kernel currently permits "schemaless" Things (if no schema is found, validation passes). This weakens the authority model, allowing userland to pollute the graph with ad-hoc properties.

### 5) No Undefined Behavior as a Design Tool
**Status: 🔴 RED**
*   **Justification:** There are multiple instances of `transmute` being used as a shortcut for type conversion or to fabricate lifetimes.
    *   `arch/src/x86_64/syscall.rs`: Transmuting syscall args to `PixelFormat` or `ResidentPolicy`.
    *   `user/drivers/usb/src/xhci.rs`: Transmuting local references to `&'static`.
    *   `thing_os/src/syscalls.rs`: Transmuting return codes to enums.
    This creates a brittle system where a single bad integer from userland can cause undefined behavior in the kernel.

### 6) Drift Is the Enemy
**Status: ✅ GREEN**
*   **Justification:** The `for_each_syscall!` macro is a triumph. It forces `abi` and `arch` to stay in sync regarding syscall numbers.
*   **Caveat:** `SchedThreadInfo` is defined in both `abi` and `thing_models` with identical fields but distinct types. This is a minor drift that should be unified.

## Concrete Findings

*   **`arch/src/x86_64/syscall.rs`**:
    ```rust
    let format = unsafe { core::mem::transmute($a3 as u8) };
    ```
    This is UB if `$a3` is not a valid discriminant of `PixelFormat`.
*   **`arch/src/x86_64/syscall.rs`**:
    ```rust
    let policy = unsafe { core::mem::transmute($a2 as u32) };
    ```
    Same issue for `ResidentPolicy`.
*   **`kernel/src/lib.rs`**:
    ```rust
    unsafe { for i in 0..slice_len { let wire_prop = *slice_ptr.add(i as usize); ... } }
    ```
    Iterates `slice_len` times based on user input. A massive `len` could stall the kernel (DoS).
*   **`kernel/src/log.rs`**:
    Uses `static mut LOG_BUFFER` without a lock (only `unsafe`), relying on a comment that it's "single-threaded". Interrupts (e.g., keyboard, timer) calling `log()` will race and corrupt this buffer.
*   **`thing_models/src/lib.rs`**: Defines `struct SchedThreadInfo` which shadows `abi::SchedThreadInfo`.
*   **`user/drivers/usb/src/xhci.rs`**:
    ```rust
    let static_mmio: &'static dyn MmioMapper = unsafe { core::mem::transmute(mmio) };
    ```
    Fake static lifetime.

## Top 3 Risks

1.  **Kernel UB via Syscalls**: A compromised or buggy user program can crash the kernel or execute arbitrary code by passing invalid integers to syscalls that `transmute` them to enums.
2.  **Concurrency Corruption in Logging**: `log()` is not re-entrant. Calling it from an interrupt handler while the main thread is logging will corrupt the `LOG_BUFFER` indices, potentially causing infinite loops or memory overwrite.
3.  **Graph Pollution**: The "default allow" policy for unknown schemas permits disparate programs to invent conflicting ad-hoc ontologies, undermining the "Centralized Schema Authority" invariant.

## Top 3 Strengths to Protect

1.  **Macro-Enforced Syscall Tables**: The `for_each_syscall!` pattern in `abi/src/syscalls.rs` is the gold standard for preventing drift. It should never be removed.
2.  **POD Wire Types**: `WireProp` and `ThingPropData` are correctly designed as C-compatible, data-only structs, ensuring the ABI remains a "postcard".
3.  **Graph Centrality**: The architecture correctly treats the Graph as the single source of truth for system state, rather than scattering state across ad-hoc kernel structures.
