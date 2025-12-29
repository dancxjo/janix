# ThingOS Architecture Boundaries

This document defines the strict boundaries between architecture-specific code, the generic kernel, and userland drivers.

## Goal

- **`/arch/*`**: Architecture-specific boot + low-level hardware glue.
- **`/crates/kernel`**: Architecture-neutral kernel logic and traits.
- **`user/drivers/*`**: Drivers live in userland (as apps/modules) unless unavoidably required for early boot.

## Rules

### `/arch/<arch>/`

**Allowed:**
- Boot entrypoints (`rust_main`, Limine handoff, early page tables)
- Early heap init + HHDM mapping needed to reach kernel entry
- Interrupt controller setup (IDT/GDT on x86_64; exception vectors / GIC basics on aarch64)
- Timer tick wiring (hooking the scheduler tick into the arch timer/irq)
- Very small arch shims that implement `kernel::bridge::HardwareBridge` or similar

**Not allowed:**
- Filesystem parsing, ISO9660, ELF parsing, module scanning logic
- "Drivers" (PS/2, AHCI, RTC policy, keyboard mapping, etc.)
- Graph seeding, symbol tables, app launching policy (except the minimal "jump to kernel/init" step)
- Any device model code that isn’t strictly required to bring the kernel up

### `/crates/kernel` (and kernel-core crates)

**Allowed:**
- Graph core, symbols, scheduling, syscalls, wire formats (ABI usage)
- Generic module loader interfaces and policy (but no direct device I/O)
- Traits for hardware access (bridge traits), with arch implementing them
- Kernel services that userland consumes (time source interface, logging, etc.)

**Not allowed:**
- Any direct MMIO/PIO reads/writes without going through a bridge abstraction
- Embedding platform device addresses, PCI scanning specifics, PS/2 controller pokes, etc.

### Userland Drivers (`user/apps/*` or `user/drivers/*` modules)

**Allowed:**
- AHCI/ATA driver as a user module that speaks to kernel via the driver syscall surface
- PS/2 keyboard driver (scancodes) + keymap service (unicode mapping) as user modules
- Framebuffer driver (if not required for early boot) and compositor, etc.
- Device enumeration and policy

**Not allowed:**
- Anything that needs to run before the kernel has a functioning syscall/driver surface
- Anything that requires being in ring0 without an explicit kernel-side driver ABI for it

## Component Placement

| Component | Placement | Reason |
| :--- | :--- | :--- |
| **ELF Loader** | `crates/kernel` (generic) + userland policy | Parsing is generic; arch just jumps to entry point. |
| **ISO9660 Reader** | Userland driver (preferred) | Filesystem logic shouldn't bloat the kernel; run as a service. |
| **IRQ Controller** | `/arch` | Strictly hardware-specific setup required for interrupts. |
| **RTC Access** | Userland driver | Kernel exposes generic time syscall; driver talks to hardware. |
| **PS/2 Controller** | Userland driver | Legacy I/O; can be managed via syscalls/ports from userland. |
| **PCI Scanning** | Userland driver | Bus enumeration logic is complex and policy-heavy. |
| **Scheduler** | `crates/kernel` | Core OS logic, agnostic of specific CPU details (mostly). |
| **Context Switch** | `/arch` | Assembly/register saving is inherently arch-specific. |

## Why?

**Prevent Fossilization**: Drivers in the kernel tend to get entangled with internal kernel APIs, making refactors hard.
**Per-Arch Snowflakes**: Logic duplicated across `arch/x86_64` and `arch/aarch64` wastes maintenance effort and diverges.
**Microkernel Principles**: Moving complexity to userland improves stability; a driver crash doesn't panic the kernel.
