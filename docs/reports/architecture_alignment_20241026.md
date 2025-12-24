# Periodic Architecture Alignment Review (2024-10-26)

## 1. Executive Summary

The repository currently exhibits a **dangerous dichotomy**: the static architecture (type definitions, module boundaries, schema authority) is disciplined and aligned with ThingOS philosophy, but the runtime implementation (syscall dispatch, pointer handling) is critically fragile.

While the "ABI Is a Postcard" invariant is respected in *definition* (plain enums, flat structs), it is flagrantly violated in *implementation* via dangerous `transmute` casts that invite Undefined Behavior. More alarmingly, the kernel currently trusts user-provided pointers blindly, lacking any validation that they reside in user memory. This allows any user process to read or corrupt kernel memory, violating the "Kernel Correctness" invariant and negating all security boundaries.

The system is structurally sound but security-critical implementation details have been neglected.

## 2. Invariant-by-Invariant Assessment

| Invariant | Status | Judgment |
| :--- | :--- | :--- |
| **1) Kernel Correctness** | 🔴 **Red** | Critical failure. The kernel performs no validation on user-provided pointers (`user_slice`), allowing arbitrary read/write of kernel memory by userland. |
| **2) ABI Is a Postcard** | 🟡 **Yellow** | Definitions are excellent (flat, explicit), but the implementation uses `unsafe { transmute }` to convert integers to enums, bypassing Rust's safety guarantees and risking UB. |
| **3) Clear Authority** | 🟢 **Green** | Boundaries are well-respected. `thing_models` owns the ontology, `kernel` enforces it, and `thing_os` wraps it ergonomically. |
| **4) Schema Centralization** | 🟢 **Green** | `kernel::graph::schema` correctly enforces fingerprinting, and `thing_models` serves as the single source of truth for core types. |
| **5) No Undefined Behavior** | 🔴 **Red** | `transmute` is used as a convenience tool in syscall dispatch. Invalid user input (e.g., a bad enum integer) causes immediate UB in the kernel. |
| **6) Drift Is the Enemy** | 🟢 **Green** | No significant drift found. `PixelFormat` and other types are defined once in `abi` and correctly consumed/re-exported. |

## 3. Concrete Findings

### The Good ✅
*   **Ontology Authority:** `thing_models/src/lib.rs` (`kernel_core_schemas`) provides a clear, single source of truth for the system's core vocabulary, which the kernel correctly ingests.
*   **ABI Definitons:** `abi/src/wire/` definitions are exemplary. They are `repr(C)`, data-only, and use explicit sizing (e.g., `u64` instead of `usize`). `WirePropValue` correctly manages the boundary between raw data and high-level types.
*   **Schema Enforcement:** `kernel/src/graph/schema.rs` implements robust fingerprinting (`SchemaRegistryOutcome::Conflict`) to prevent schema drift or corruption.

### The Bad ⚠️
*   **Drift/Re-exports:** `thing_models` re-exports `SchedThreadInfo` from `abi`. While this avoids duplication (good), it blurs the line between "wire format" and "domain model". Ideally, `thing_models` should define the domain type and `abi` the wire type, with explicit conversion.
*   **Fragile MMIO:** `kernel/src/hal_impl.rs` contains extensive `unsafe` blocks for MMIO. While necessary for a kernel, the sheer volume without higher-level safe abstractions makes it a regression risk.

### The Ugly 🪳
*   **Unchecked User Pointers (Critical Security Hole):**
    *   **File:** `arch/src/x86_64/syscall.rs`
    *   **Code:** `unsafe fn user_slice<'a, T>(ptr: u64, len: u64) -> &'a [T]` simply calls `slice::from_raw_parts` without checking if `ptr` falls within the user's address space (`0x0000_0000_4000_0000` to `USER_HEAP_END`).
    *   **Consequence:** A user process can pass a kernel address (e.g., `0xFF...`) to `SYSCALL_DEV_READ` or `SYSCALL_THING_GET`, allowing it to overwrite kernel code/data or exfiltrate secrets.

*   **UB via Transmute:**
    *   **File:** `arch/src/x86_64/syscall.rs`
    *   **Code:** `let format = unsafe { core::mem::transmute($a3 as u8) };` (in `SYSCALL_CREATE_SHARED_BUFFER`).
    *   **Code:** `let policy = unsafe { core::mem::transmute($a2 as u32) };` (in `SYSCALL_THING_REST`).
    *   **Consequence:** If the user passes an integer that does not correspond to a valid enum variant (e.g., `PixelFormat` has variants 0 and 1, user passes 255), this is instant Undefined Behavior. The compiler assumes the value is valid and may optimize incorrectly, leading to crashes or vulnerabilities.

## 4. Top 3 Risks

1.  **Arbitrary Kernel Memory Access:** The lack of pointer validation in `syscall.rs` is a "game over" vulnerability. Any malicious or buggy program can crash or hijack the kernel.
2.  **Enum Undefined Behavior:** The use of `transmute` for `PixelFormat`, `RestPolicy`, and `DeviceKind` means the kernel's behavior is undefined for invalid inputs. This violates the "Deterministic Errors" philosophy.
3.  **Silent Data Corruption:** Because `user_slice` trusts the length provided by the user, a large length could cause the kernel to read/write beyond allocated pages, potentially causing page faults in kernel mode or silent corruption of adjacent data.

## 5. Top 3 Strengths to Protect

1.  **The Graph Ontology:** The centralized definition in `thing_models` is a powerful architectural asset that keeps the system coherent. Do not fragment this.
2.  **Flat ABI:** The disciplined refusal to use Rust-specific features (traits, generics) in `abi` ensures the boundary remains clean and portable.
3.  **Schema Fingerprinting:** The kernel's refusal to register conflicting schemas protects the integrity of the data graph.

## 6. Suggested Follow-Up Tasks

1.  **Implement `UserSlice` Validation:** Immediately modify `user_slice` (and friends) in `arch` to strictly validate that the pointer range `[ptr, ptr + len)` falls entirely within the user address space.
2.  **Sanitize Enums:** Remove `transmute` in syscall dispatchers. Use `TryFrom` or `match` to convert integers to enums, returning `SysError::INVALID_ARG` on failure.
