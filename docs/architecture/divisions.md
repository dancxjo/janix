# Architecture Divisions

To maintain a clean and sustainable architecture, ThingOS follows strict package boundaries. These rules prevent policy leakage into plumbing and ensuring clear separation of concerns.

## 1. /arch/<arch> (Hardware Plumbing)

**Purpose**: Hardware bring-up, entering the kernel, and satisfying the bridge traits (`CpuBridge` / `FullMachineBridge`).

**Allowed**:
- CPU/board bring-up (GDT, IDT, paging, interrupts).
- Extracting boot facts (memory map, module list) into generic structs.
- Wiring `Kernel<BridgeImpl>`.
- Minimal glue for the bridge traits.

**Forbidden**:
- **Policy**: "Load these modules first", "Spawn compositor".
- **Module Classification**: Heuristics for "Driver" vs "App".
- **Graph Schema**: Deciding what a "node" means.
- **Bootloader specifics**: Storing Limine types beyond the extraction phase.
- **Driver Implementations**: No drivers in arch (framebuffer driver goes in `/drivers`).

**Bridge Location**:
- The bridge crate implementation MUST live at `arch/<arch>/bridge`.

## 2. /crates/kernel (Core OS Semantics)

**Purpose**: The OS logic, scheduler, syscalls, and graph store.

**Allowed**:
- Graph store, syscalls, encoding/decoding.
- Machine registry, endpoint dispatch.
- Scheduler, memory management (generic).

**Forbidden**:
- **Bootloader specifics**: No `limine` imports.
- **Arch-specifics**: No direct register manipulation (use the bridge traits).
- **Hardcoded Startup**: No "Start compositor" logic.

## 3. /drivers/* (Capability Modules)

**Purpose**: Drivers that provide capabilities to the OS.

**Allowed**:
- Implementing `DriverDescriptor`, `thingos_driver_init`.
- Using `DriverContext`.

**Forbidden**:
- **Kernel Internals**: No importing `kernel::*` (except necessary traits via `abi` or `thing_std` wrappers).
- **Orchestration**: Drivers don't decide what starts next.

## 4. /user/apps/* (Policy & Userland)

**Purpose**: The actual behavior of the system.

**Allowed**:
- **Loaded Policy**: Discover modules, start drivers, spawn apps.
- UI, applications.

**Forbidden**:
- **Boot Protocol**: No parsing raw bootloader structures.
- **Physical Addresses**: No assumptions about physical memory.

## Enforcement

These rules are enforced by `cargo xtask boundary-check`.
