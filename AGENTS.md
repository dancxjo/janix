# Thing-OS Agent Notes

This file is a quick map of the repository so agents (and humans) can orient fast.

## What this repo is
- Thing-OS is a Rust workspace that builds a graph-based OS kernel plus userland apps.
- Build/test automation lives in `xtask` and is surfaced via `just`.

## Common commands
- Build ISO: `just iso` (override arch with `KARCH=aarch64`, `riscv64`, `loongarch64`)
- Run QEMU: `just run`
- Run BDD tests: `just behave` (see `tools/bdd`)
- Clean: `just clean`

## Top-level layout (what's what)
- `abi/`: shared ABI types and syscalls between kernel/userspace.
- `bran/`: core kernel runtime (boot/runtime abstraction).
- `kernel/`: kernel crate and core kernel logic.
- `drivers/`: hardware driver crates.
- `userspace/`: user programs and demos (each subdir is a crate).
- `bloom/`, `blossom/`, `display/`: graphics/compositor-related crates.
- `stem/`, `stem-macros/`: internal libs and proc-macros.
- `targets/`: custom JSON target specs for bare metal builds.
- `tools/`: auxiliary tooling (BDD, pciids, etc).
- `xtask/`: build orchestration used by `just`.
- `docs/`: documentation and test reports (`docs/behavior/` is generated).
- `vendor/`: vendored dependencies (Limine, OVMF).

## Where to start when changing behavior
- Kernel interfaces: `abi/` and `kernel/`
- Syscall surface: `abi/src/syscall.rs`
- User apps: `userspace/`
- Build/config: `justfile`, `xtask/`, `targets/`

## Notes
- Workspace members are listed in `Cargo.toml`.
- `target/` is build output and can be ignored in reviews.
- Reminder: use the `apply_patch` tool directly for file edits (avoid running it via exec). 
- Reminder: hashing helpers must use the correct byte width for each integer type (u32/i32 = 4 bytes).
