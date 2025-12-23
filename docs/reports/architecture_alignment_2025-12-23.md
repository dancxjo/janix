# Architecture Alignment Review: 2025-12-23

## 1. Executive Summary

The repository remains generally aligned with the core ThingOS philosophy, maintaining strong discipline in `no_std` purity and explicit memory layouts for the ABI. The "Flat ABI" and "Kernel Correctness" invariants are largely respected.

However, a **critical architectural drift** has emerged in the property system (`PropType::Str` vs. `Symbol`). The current implementation of property conversion effectively renders string properties unusable from userland, creating a "dead zone" in the schema authority. Furthermore, the `KernelRequest` enum, intended as a comprehensive model of the kernel surface, is incomplete and bypassed by several core syscalls (e.g., `SYMBOL_INTERN`), leading to an incomplete ABI contract.

Immediate correction is required to unify the `Str`/`Symbol` handling and close the coverage gap in `KernelRequest` to prevent further divergence.

## 2. Invariant-by-Invariant Assessment

### 1) Kernel Correctness Over Features
**Status: GREEN**
*   **Justification:** The kernel code remains disciplined, with minimal usage of `unwrap()` and explicit error handling. Boot invariants are verified (`verify_boot_graph_invariants`), and the syscall handler correctly manages user context (`TrapFrame`). Feature flags are used appropriately to guard architecture-specific code.

### 2) ABI Is a Postcard
**Status: YELLOW**
*   **Justification:** The wire types (`WireProp`, `WirePropValue`, `UserSlice`) are exemplary: flat, data-only, and explicit (`#[repr(C)]`).
*   **Warning:** `abi/src/lib.rs` depends on `alloc` and `Vec`. While mostly used for internal helpers or `ThingId` logic, this blurs the line. Ideally, the ABI crate should be strictly `no_std` to guarantee zero allocation on the wire boundary.
*   **Violation:** `KernelRequest` is an incomplete abstraction. It is a Rust enum used by clients but ignored by the `SYSCALL_SYMBOL_INTERN` dispatch path, breaking the "single explicit surface" ideal.

### 3) Clear Authority Boundaries
**Status: YELLOW**
*   **Justification:** `thing_models` correctly serves as the ontology authority, and `kernel` acts as the enforcer.
*   **Violation:** The boundary between `thing_os` (user) and `kernel` is leaking implementation details regarding String/Symbol conversion. `thing_os` forces all strings to symbols, but `kernel` schema validation distinguishes them, leading to a situation where valid user input is rejected by valid kernel schemas.

### 4) Schema Authority Is Centralized
**Status: GREEN**
*   **Justification:** Schema registration is correctly centralized in `kernel/src/graph/schema.rs`, using idempotent fingerprinting. User/package schemas are properly scoped (though the "package" syscall exists).
*   **Note:** The kernel defaults to permissive mode for unknown kinds, which is a pragmatic choice but relies on discipline to avoid pollution.

### 5) No Undefined Behavior
**Status: GREEN**
*   **Justification:** Dangerous operations like `transmute` in `thing_os` are guarded by range checks. Syscall handlers verify pointers using `UserSlice` helpers. No obvious "UB as a design tool" was found.

### 6) Drift Is the Enemy
**Status: RED**
*   **Justification:** There is a confirmed logic conflict (drift) between `thing_models` (`PropType::Str`), `abi` (`WireValueTag::Str` is Symbol), and `kernel` (Expects `PropValue::Str` but receives `PropValue::Symbol`). This makes `PropType::Str` functionally broken for userland creation. Additionally, `SYSCALL_SYMBOL_INTERN` is missing from `KernelRequest`, causing drift between the "Model of the ABI" (`KernelRequest`) and the "Actual ABI" (syscall numbers).

## 3. Concrete Findings

### The Good ✅
*   **Flat Wire Types:** `abi/src/wire/*.rs` types are strictly `#[repr(C)]` with manual padding and `u64` fields, ensuring cross-compiler stability.
*   **Boot Verification:** `kernel/src/lib.rs:verify_boot_graph_invariants` ensures the initial graph state (CPU, Process, Thread) is correct before userland starts.
*   **TrapFrame Safety:** `arch/src/x86_64/syscall.rs` uses a comprehensive `TrapFrame` and `sysretq` logic, properly isolating user context.

### The Bad ⚠️
*   **PropType::Str Dead End:**
    *   File: `thing_os/src/lib.rs` (in `create_thing`) converts `PropValue::Str` -> `WirePropValue::sym`.
    *   File: `kernel/src/lib.rs` (in `handle_request`) converts `Tag 3` -> `PropValue::Symbol`.
    *   File: `kernel/src/graph/schema.rs` (in `validate_props`) fails `PropType::Str` if it sees `PropValue::Symbol`.
    *   **Result:** Any schema using `PropType::Str` cannot be instantiated from userland.
*   **KernelRequest Gaps:**
    *   File: `abi/src/requests.rs` defines `KernelRequest`.
    *   File: `abi/src/syscalls.rs` defines `SYSCALL_SYMBOL_INTERN`.
    *   File: `arch/src/x86_64/syscall.rs` handles `SYSCALL_SYMBOL_INTERN` manually, bypassing `KernelRequest`.
    *   **Result:** `KernelRequest` cannot be used to replay or mock the full system interaction.

### The Ugly 🪳
*   **Symbol/String Conflation:**
    *   `thing_models` has `Str` and `Symbol`.
    *   `abi` effectively has `Symbol` (as `Str` tag) and `Blob`.
    *   `thing_os` converts `PropValue::Symbol(id)` -> `Tag 3`. Kernel receives `Symbol(id)`.
    *   On read (`ThingGet`), Kernel sends `Tag 3`. `thing_os` resolves it and returns `PropValue::Str(string)`.
    *   **Violation:** Round-tripping `PropValue::Symbol` mutates it into `PropValue::Str`. This destroys the distinction between "cheap interned ID" and "resolved string" in the user API, forcing O(N) allocations on every read of a symbol property.

## 4. Top 3 Risks if Unaddressed

1.  **Broken String Properties:** Developers will be unable to use `String` fields in Schemas for user-creatable Things, forcing them to use `Blob` (raw bytes) or `Symbol` (interned limit), confusing the ontology.
2.  **Incomplete Mocking/Testing:** Tools relying on `KernelRequest` (like potential future record/replay debuggers or tests) will miss critical syscalls like Symbol Interning, leading to unreproducible bugs.
3.  **Schema Drift:** The disconnect between `PropType` definitions and `WireValueTag` capabilities will lead to "magic" properties that only work from inside the kernel (boot), creating a two-class system.

## 5. Top 3 Strengths to Protect

1.  **The "Postcard" Wire Format:** Keep `abi/src/wire` strict. Do not add convenience methods or `Vec` to these structs.
2.  **Graph Authority:** Continue forcing all state changes through `kernel::graph`. Do not add "fast paths" that bypass the graph store/events.
3.  **Deterministic Boot:** The `verify_boot_graph_invariants` check is excellent. Keep extending it.

## 6. Suggested Follow-Up Tasks

1.  **Fix PropType::Str:** Either merge `Str` and `Symbol` in `thing_models` (admitting everything is interned or a blob), OR update `kernel` to permit `PropValue::Symbol` to satisfy `PropType::Str` validation (treating them as compatible).
2.  **Complete KernelRequest:** Add `SymbolIntern` (and other missing calls) to `KernelRequest` and refactor `arch/src/*/syscall.rs` to decode *all* syscalls into this enum before dispatch (or at least ensure 1:1 mapping exist).
3.  **Purify ABI:** Move `alloc` dependency in `abi` behind a feature flag or remove it if possible, to strictly enforce the "data-only" invariant.
