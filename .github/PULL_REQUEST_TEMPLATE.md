## Description

<!-- What does this PR do and why? -->

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Refactor
- [ ] Documentation
- [ ] Build / tooling

## Janix Architecture Guardrails

> See [`docs/concepts/janix-guardrails.md`](../docs/concepts/janix-guardrails.md) for the full reference.
> Answer each question.  If the answer is **Yes**, add a brief justification below.

### Scheduler-first
- [ ] This change does NOT introduce execution that bypasses the scheduler
      (e.g., no spin-loops that never yield, no direct task state manipulation
      outside `kernel/src/sched/`).

### Userland-driver direction
- [ ] This change does NOT add new in-kernel device logic that could instead
      live in a userspace driver process using `SYS_DEVICE_*` primitives.

### VFS-first system surface
- [ ] This change does NOT require reaching system resources through graph
      nodes, `ThingId` lookups, or `UI_CROWN` — only through VFS paths and
      file descriptors.
- [ ] Boot-critical paths (display, storage, network bring-up) still work with
      only VFS/fd state available — no graph discovery required for first paint
      or first packet.

### Spawn + exec process model
- [ ] This change does NOT introduce `fork`-like semantics or add a
      `SYS_FORK` / copy-on-write address-space duplication syscall.
- [ ] New process creation (if any) goes through `SYS_SPAWN_PROCESS[_EX]`
      followed by `SYS_TASK_EXEC`.

### Platform boundary
- [ ] All new kernel / userspace code uses `#![no_std]` + `stem` (not `std`).
- [ ] No new `std`-only crates are added to runtime (kernel/userspace) code paths.

## Testing

- [ ] Existing tests pass (`just behave`, `cargo test -p kernel`)
- [ ] New behavior is covered by new or updated tests

## Refactor Issues

<!-- If this PR touches a guardrail area, link back to #<guardrail issue>. -->
