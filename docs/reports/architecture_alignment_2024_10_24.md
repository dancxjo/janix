# Architecture Alignment 2024-10-24

## Summary
The `abi` and `thing_models` crates have been refactored to align with architectural invariants.
The `abi` crate is now a strict `no_std` wire-definition library with no allocator dependencies.
`thing_models` is now the authoritative source for system ontology, including `PropValue`, `PropKey`, `PropType`, and high-level `Thing` definitions.

## Changes

### `abi` Crate
- Removed `extern crate alloc`.
- Removed `mod prop_value` and moved it to `thing_models`.
- Removed `alloc` usage in favor of `WireProp` and `sys*` definitions.
- Deprecated `ThingPropData` and `ThingGetSyscallResult`.
- Removed `syscall_numbers.rs` and centralized syscall numbers in `syscalls.rs`.

### `thing_models` Crate
- Added `props.rs` (migrated form `abi`).
- Exports `PropKey`, `PropType`, `PropValue`.
- Updated imports to use `abi::syscall_defs` where appropriate.

### Downstream Updates
- `thing_os`, `kernel`, `boot`, and `user` crates have been updated to import property types from `thing_models` instead of `abi`.
- Syscall usage in drivers has been updated to use `abi::syscalls`.

## Verification
- Compilation verified via `make check`.
- Unit tests verified via `make test`.
- System boot verified via `make run`.
