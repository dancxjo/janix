# Syscall Dispatch Invariant

ThingOS enforces a strict invariant for syscall dispatching:

> **Every syscall defined in the ABI must have a corresponding dispatcher arm in the architecture-specific kernel code.**

## Enforcement Mechanism

1.  **Single Source of Truth:** Syscalls are defined in `abi/src/syscalls.rs` using the `for_each_syscall!` macro.
2.  **Generated Dispatch:** The `arch` crate uses `abi::syscalls::for_each_syscall!` to generate the dispatch match arms.
    *   This ensures that if a syscall is added to the ABI macro, the `arch` crate **must** implement a handler (or a stub) for it, or it will fail to compile (if the macro rules are not comprehensive).
    *   Our `dispatch_syscall!` macro includes a fallback stub for unimplemented syscalls, returning `u64::MAX`.
3.  **No Drift:** Since both the constants and the dispatch logic are derived from the same macro invocation, it is impossible for them to drift. `ABI_SYSCALL_NUMBERS` and `DISPATCHED_SYSCALL_NUMBERS` arrays are also generated to verify alignment.

## Adding a New Syscall

To add a new syscall:
1.  Add it to the `for_each_syscall!` macro in `abi/src/syscalls.rs`.
2.  (Optional) Add a specific handler in `arch/src/x86_64/syscall.rs` inside the `dispatch_syscall!` macro. If you don't, it will automatically use the default stub (returning `u64::MAX`).
3.  Run `tools/generate_contracts.py` to update documentation.
