# Boot Capability Grants System

## Overview

This document describes the boot capability grants system implemented in ThingOS to provide a centralized, predictable mechanism for granting capabilities to modules at boot time.

## Problem Statement

Previously, capability grants were scattered across the codebase:
- Manual injection in `boot.rs::inject_root_caps()` for Sprout
- Ad-hoc injection in syscalls like `surface.rs::sys_surface_create()`
- Module capabilities stored in Module bodies but not automatically applied

This made capability behavior unpredictable and hard to audit.

## Solution

### Boot Grants Table

A centralized registry (`boot_grants.rs`) that:
1. Maps module names to their default capabilities
2. Is populated during boot from Module bodies in the graph
3. Automatically applies grants when spawning tasks

### Creator Ownership Policy

A documented policy for object creation:
- When a task creates a Thing, it automatically receives `GraphRead` and `GraphWrite` capabilities scoped to that Thing
- Implemented via `grant_creator_ownership()` helper in `cap.rs`
- Applied consistently in creation syscalls (e.g., `surface_create`)

## Architecture

### Components

1. **`boot_grants.rs`**: Core registry module
   - `BootGrant` struct: Maps module ThingId + optional name → capabilities
   - `register_module_grants()`: Registers grants during seeding
   - `apply_boot_grants_by_name()`: Applies grants during spawn

2. **`seeding.rs`**: Module registration
   - Extracts capabilities from Module bodies
   - Registers grants for seeded services (bloom, inputd, etc.)

3. **`boot.rs`**: Sprout special case
   - Manually registers Sprout's grants before spawning
   - Sprout is PID 1 and spawned directly, not via service plan

4. **`proc/mod.rs`**: Grant application
   - `spawn_kernel_module()` applies boot grants by module path
   - Grants applied before task becomes runnable

5. **`syscall/cap.rs`**: Creator ownership
   - `grant_creator_ownership()`: Applies read/write caps on created Things
   - Replaces ad-hoc `inject_cap` calls

## Capability Grant Flow

### Boot Time

```
seed_service_plan()
  └─> create_module("bloom")
      ├─> default_caps_for("bloom") → [Log, MemManage, GraphCreate, ...]
      └─> register_module_grants(module_thing, "bloom", caps)

boot()
  └─> Manually register Sprout grants
      └─> register_module_grants(dummy_id, "sprout", caps)
```

### Task Spawn

```
spawn_module_by_name("bloom")
  └─> spawn_kernel_module(module_info)
      └─> Task::new(...)
          └─> apply_boot_grants_by_name(module.path, &mut task.caps)
              └─> Finds "bloom" in registry
                  └─> Extends task.caps with registered capabilities
```

### Object Creation

```
sys_surface_create(...)
  └─> surface_id = store::thing_create(KIND_SURFACE)
  └─> grant_creator_ownership(surface_id)
      └─> current_task.caps.push(GraphRead, Thing(surface_id))
      └─> current_task.caps.push(GraphWrite, Thing(surface_id))
```

## Default Capabilities

### Sprout (PID 1)
- Log, MemManage, GrantCaps
- GraphCreate, GraphLink, GraphUnlink, GraphRead, GraphWrite

### Bloom (Compositor)
- Log, MemManage
- GraphCreate, GraphLink, GraphUnlink, GraphRead, GraphWrite, GraphWatch

### InputD (Input Daemon)
- Log, MemManage, InputRead
- GraphCreate, GraphLink, GraphRead, GraphWrite

See `seeding.rs::default_caps_for()` for complete list.

## Testing

### Unit Tests

1. **`boot_grants::tests::test_register_and_apply`**
   - Verifies grant registration and application by ThingId

2. **`boot_grants::tests::test_apply_by_name`**
   - Verifies grant application by module name/path

3. **`syscall::host_tests::test_boot_grants_application`**
   - Integration test with test harness
   - Verifies grants are applied to fake tasks

4. **`syscall::host_tests::test_creator_ownership_policy`**
   - Verifies creator ownership caps are granted on Thing creation

All tests pass (17/17 in kernel unit tests).

## Migration Notes

### Removed
- `boot.rs::inject_root_caps()` - Replaced by boot grants
- `syscall/cap.rs::inject_cap()` - Replaced by `grant_creator_ownership()`

### Changed
- `sys_surface_create()`: Uses `grant_creator_ownership()` instead of direct injection
- Module capabilities are now automatically applied from Module bodies

## Future Enhancements

1. **Graph Reflection**: Store grants as Things in the graph for runtime inspection
2. **Dynamic Grants**: Support runtime capability grant/revoke via syscalls
3. **Capability Audit**: Tool to list all granted capabilities per module
4. **Scoped Grants**: Support Thing-scoped grants in boot table (currently only Global)

## References

- Problem Statement: Issue "Kernel: Capability defaults + boot-time grants table"
- Implementation: PR #XXX
- Tests: `crates/kernel/src/boot_grants.rs`, `crates/kernel/src/syscall/host_tests.rs`
