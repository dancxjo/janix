# Schema Authority Ledger

## Core Schemata (Kernel-Known)

Defined in `thing_models`, registered by `kernel` at boot.

| Kind (Source) | Description (Source) | Registration |
|---|---|---|
| `PhysFrame::KIND` | `PhysFrame::DESCRIPTION` | `thing_models/src/lib.rs` |
| `FramePool::KIND` | `FramePool::DESCRIPTION` | `thing_models/src/lib.rs` |
| `AddressSpace::KIND` | `AddressSpace::DESCRIPTION` | `thing_models/src/lib.rs` |
| `VirtRegion::KIND` | `VirtRegion::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Process::KIND` | `Process::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Thread::KIND` | `Thread::DESCRIPTION` | `thing_models/src/lib.rs` |
| `ThreadInfo::KIND` | `ThreadInfo::DESCRIPTION` | `thing_models/src/lib.rs` |
| `CpuCore::KIND` | `CpuCore::DESCRIPTION` | `thing_models/src/lib.rs` |
| `SleepEvent::KIND` | `SleepEvent::DESCRIPTION` | `thing_models/src/lib.rs` |
| `BootProfile::KIND` | `BootProfile::DESCRIPTION` | `thing_models/src/lib.rs` |
| `BootProgram::KIND` | `BootProgram::DESCRIPTION` | `thing_models/src/lib.rs` |
| `ProgramImage::KIND` | `ProgramImage::DESCRIPTION` | `thing_models/src/lib.rs` |
| `FontModule::KIND` | `FontModule::DESCRIPTION` | `thing_models/src/lib.rs` |
| `TimeSource::KIND` | `TimeSource::DESCRIPTION` | `thing_models/src/lib.rs` |
| `graph_kinds::KIND_IO_PORT_REGION` | `IoPortRegion::DESCRIPTION` | `thing_models/src/lib.rs` |
| `graph_kinds::KIND_IO_PORT_OP` | `IoPortOp::DESCRIPTION` | `thing_models/src/lib.rs` |
| `graph_kinds::KIND_INTERRUPT_EVENT` | `InterruptEvent::DESCRIPTION` | `thing_models/src/lib.rs` |
| `graph_kinds::KIND_INTERRUPT_REQUEST` | `InterruptRequest::DESCRIPTION` | `thing_models/src/lib.rs` |
| `graph_kinds::KIND_ALARM_REQUEST` | `AlarmRequest::DESCRIPTION` | `thing_models/src/lib.rs` |
| `graph_kinds::KIND_ALARM_EVENT` | `AlarmEvent::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Display::KIND` | `Display::DESCRIPTION` | `thing_models/src/lib.rs` |
| `SharedBuffer::KIND` | `SharedBuffer::DESCRIPTION` | `thing_models/src/lib.rs` |
| `DisplayFramebuffer::KIND` | `DisplayFramebuffer::DESCRIPTION` | `thing_models/src/lib.rs` |
| `DisplayFrame::KIND` | `DisplayFrame::DESCRIPTION` | `thing_models/src/lib.rs` |
| `DisplayPresentRequest::KIND` | `DisplayPresentRequest::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Mode::KIND` | `Mode::DESCRIPTION` | `thing_models/src/lib.rs` |
| `ModeSwitchEvent::KIND` | `ModeSwitchEvent::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Place::KIND` | `Place::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Window::KIND` | `Window::DESCRIPTION` | `thing_models/src/lib.rs` |
| `Surface::KIND` | `Surface::DESCRIPTION` | `thing_models/src/lib.rs` |

## Package Schemata

- **Registration:** Scoped to the calling process/package via `SYSCALL_SCHEMA_REGISTER_PACKAGE`.
- **Storage:** Stored in the kernel's Graph Schema Store.
- **Conflicts:** Returns `SchemaRegistryOutcome::Conflict` or `AlreadyRegisteredSame` if a schema with the same Kind but different definition exists.
- **Fingerprinting:** SchemaGet returns a 64-bit fingerprint (currently placeholder 0 in `kernel/src/lib.rs`).
