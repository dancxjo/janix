# Schema Authority Ledger

## Core Schemata (Kernel-Known)

Defined in `thing_models`, registered by `kernel` at boot.

| Kind (Source) | Description (Source) | Registration |
|---|---|---|
| `PhysFrame::KIND` | `PhysFrame::DESCRIPTION` | `thing_models/src/lib.rs` |
| `FramePool::KIND` | `FramePool::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Process::KIND` | `Process::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Thread::KIND` | `Thread::DESCRIPTION` | `thing_models/src/lib.rs` |
| `CpuCore::KIND` | `CpuCore::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Display::KIND` | `Display::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Mode::KIND` | `Mode::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Place::KIND` | `Place::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Window::KIND` | `Window::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Surface::KIND` | `Surface::DESCRIPTION` | `thing_models/src/lib.rs` |

## Package Schemata

- **Registration:** Scoped to the calling process/package via `SYSCALL_SCHEMA_REGISTER_PACKAGE`.
- **Storage:** Stored in the kernel's Graph Schema Store.
- **Conflicts:** Returns `SchemaRegistryOutcome::Conflict` or `AlreadyRegisteredSame` if a schema with the same Kind but different definition exists.
- **Fingerprinting:** SchemaGet returns a 64-bit fingerprint (currently placeholder 0 in `kernel/src/lib.rs`).
