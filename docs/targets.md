# ThingOS App Targets

This document defines the supported compilation targets for ThingOS applications.

| Triple | Arch | ABI/ELF | Entry Symbol | Link Script | PIE Policy | Notes |
|--------|------|---------|--------------|-------------|------------|-------|
| `x86_64-thingos-unknown` | x86_64 | SysV / ELF64 | `_start` | Default or Custom | **Avoid PIE** (Static) | Red Zone disabled? Check rustflags. |
| `aarch64-thingos-unknown` | aarch64 | AAPCS64 / ELF64 | `_start` | Default | Avoid PIE | |
| `riscv64-thingos-unknown` | riscv64gc | lp64d / ELF64 | `_start` | Default | Avoid PIE | Base: riscv64gc-unknown-none-elf |
| `loongarch64-thingos-unknown` | loongarch64 | lp64d / ELF64 | `_start` | Default | Avoid PIE | Base: loongarch64-unknown-none |

## Build Policy

- **Relocation Model**: `static` is preferred to produce strictly `ET_EXEC` ELFs.
- **Linker Details**:
  - We currently rely on the default linker layouts provided by `rust-lld` for bare metal, barring specific overrides.
  - No dynamic linker is used.
