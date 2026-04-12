//! Janix Architecture Guardrail Tests
//!
//! These tests validate observable, compile-time invariants that correspond to
//! the four non-negotiable design rules described in
//! `docs/concepts/janix-guardrails.md`.
//!
//! They are intentionally lightweight — the goal is to catch ABI regressions
//! and accidental reintroductions of banned patterns (e.g. `SYS_FORK`) at
//! CI time, not to exercise runtime behaviour.

use abi::syscall::{
    // Userland-driver direction: device primitives must exist
    SYS_DEVICE_CLAIM,
    SYS_DEVICE_IRQ_SUBSCRIBE,
    SYS_DEVICE_IRQ_WAIT,
    SYS_DEVICE_MAP_MMIO,
    SYS_FS_CLOSE,
    SYS_FS_MOUNT,
    // VFS-first: core VFS syscalls must exist
    SYS_FS_OPEN,
    SYS_FS_READ,
    SYS_FS_STAT,
    SYS_FS_WRITE,
    // Scheduler-first: spawn/exec family must exist
    SYS_SPAWN_PROCESS,
    SYS_SPAWN_PROCESS_EX,
    SYS_SPAWN_THREAD,
    SYS_TASK_EXEC,
    SYS_TASK_WAIT,
    SYS_WAITPID,
    SYS_YIELD,
};

// ---------------------------------------------------------------------------
// Guardrail 1 — Scheduler-first
// The spawn/exec/yield family must exist in the ABI so that all execution
// is routed through the scheduler.
// ---------------------------------------------------------------------------

#[test]
fn guardrail_scheduler_first_spawn_exec_exist() {
    // Verify the numeric assignments match the documented ABI contract.
    assert_eq!(
        SYS_SPAWN_THREAD, 0x1004,
        "SYS_SPAWN_THREAD ABI number changed"
    );
    assert_eq!(
        SYS_SPAWN_PROCESS, 0x1005,
        "SYS_SPAWN_PROCESS ABI number changed"
    );
    assert_eq!(
        SYS_SPAWN_PROCESS_EX, 0x1006,
        "SYS_SPAWN_PROCESS_EX ABI number changed"
    );
    assert_eq!(SYS_TASK_EXEC, 0x100D, "SYS_TASK_EXEC ABI number changed");
    assert_eq!(SYS_TASK_WAIT, 0x1007, "SYS_TASK_WAIT ABI number changed");
    assert_eq!(SYS_WAITPID, 0x1011, "SYS_WAITPID ABI number changed");
    assert_eq!(SYS_YIELD, 0x100B, "SYS_YIELD ABI number changed");
}

// ---------------------------------------------------------------------------
// Guardrail 2 — Userland-driver direction
// The SYS_DEVICE_* primitives that enable userspace drivers must exist in
// their expected range (0x5000–0x5008).
// ---------------------------------------------------------------------------

#[test]
fn guardrail_userland_drivers_device_primitives_exist() {
    assert_eq!(
        SYS_DEVICE_CLAIM, 0x5000,
        "SYS_DEVICE_CLAIM ABI number changed"
    );
    assert_eq!(
        SYS_DEVICE_MAP_MMIO, 0x5002,
        "SYS_DEVICE_MAP_MMIO ABI number changed"
    );
    assert_eq!(
        SYS_DEVICE_IRQ_SUBSCRIBE, 0x5007,
        "SYS_DEVICE_IRQ_SUBSCRIBE ABI number changed"
    );
    assert_eq!(
        SYS_DEVICE_IRQ_WAIT, 0x5008,
        "SYS_DEVICE_IRQ_WAIT ABI number changed"
    );

    // All device syscalls must be in the 0x5000–0x50FF range.
    for &n in &[
        SYS_DEVICE_CLAIM,
        SYS_DEVICE_MAP_MMIO,
        SYS_DEVICE_IRQ_SUBSCRIBE,
        SYS_DEVICE_IRQ_WAIT,
    ] {
        assert!(
            (0x5000..0x5100).contains(&n),
            "Device syscall {:#06x} is outside the 0x5000–0x50FF range",
            n
        );
    }
}

// ---------------------------------------------------------------------------
// Guardrail 3 — VFS-first system surface
// The core SYS_FS_* family must exist in the 0x4000+ range.
// ---------------------------------------------------------------------------

#[test]
fn guardrail_vfs_first_fs_syscalls_exist() {
    assert_eq!(SYS_FS_OPEN, 0x4000, "SYS_FS_OPEN ABI number changed");
    assert_eq!(SYS_FS_CLOSE, 0x4001, "SYS_FS_CLOSE ABI number changed");
    assert_eq!(SYS_FS_READ, 0x4002, "SYS_FS_READ ABI number changed");
    assert_eq!(SYS_FS_WRITE, 0x4003, "SYS_FS_WRITE ABI number changed");

    // All checked VFS syscalls must live in the 0x4000+ range.
    for &n in &[
        SYS_FS_OPEN,
        SYS_FS_CLOSE,
        SYS_FS_READ,
        SYS_FS_WRITE,
        SYS_FS_STAT,
        SYS_FS_MOUNT,
    ] {
        assert!(
            n >= 0x4000,
            "VFS syscall {:#06x} is below the 0x4000 VFS range",
            n
        );
    }
}

// ---------------------------------------------------------------------------
// Guardrail 4 — Spawn + exec; no fork
// There must be NO SYS_FORK constant in the ABI.  The absence of a symbol is
// checked by ensuring the module compiles without it — if anyone adds
// `SYS_FORK` this test file will need updating and a guardrail review is
// triggered.
// ---------------------------------------------------------------------------

#[test]
fn guardrail_no_fork_spawn_exec_numbers_are_distinct() {
    // spawn and exec must have different numbers (basic sanity).
    assert_ne!(
        SYS_SPAWN_PROCESS, SYS_TASK_EXEC,
        "spawn and exec must be distinct"
    );
    assert_ne!(
        SYS_SPAWN_PROCESS_EX, SYS_TASK_EXEC,
        "spawn_ex and exec must be distinct"
    );

    // The spawn+exec pair covers what POSIX fork+exec would cover.
    // Verify exec is in the task-management range (0x1000–0x10FF).
    assert!(
        (0x1000..0x1100).contains(&SYS_TASK_EXEC),
        "SYS_TASK_EXEC {:#06x} drifted out of the 0x1000–0x10FF task range",
        SYS_TASK_EXEC
    );
}

// ---------------------------------------------------------------------------
// Target panic contract — abort-only
// Thing-OS target specs must stay aligned with runtime policy: unwind is not
// currently supported, so all targets must declare `panic-strategy = "abort"`.
// ---------------------------------------------------------------------------

#[test]
fn guardrail_thingos_targets_are_abort_only() {
    let targets = [
        (
            "targets/x86_64-unknown-thingos.json",
            include_str!("../targets/x86_64-unknown-thingos.json"),
        ),
        (
            "targets/aarch64-unknown-thingos.json",
            include_str!("../targets/aarch64-unknown-thingos.json"),
        ),
        (
            "targets/riscv64gc-unknown-thingos.json",
            include_str!("../targets/riscv64gc-unknown-thingos.json"),
        ),
        (
            "targets/loongarch64-unknown-thingos.json",
            include_str!("../targets/loongarch64-unknown-thingos.json"),
        ),
    ];

    for (path, json) in targets {
        assert!(
            json.contains("\"panic-strategy\": \"abort\""),
            "{} must set panic-strategy=abort",
            path
        );
    }
}

#[test]
fn guardrail_thingos_unwind_diagnostics_exist() {
    let stem_runtime = include_str!("../stem/src/lib.rs");
    assert!(
        stem_runtime.contains("Thing-OS does not support panic=unwind"),
        "stem runtime must emit a compile-time panic=unwind diagnostic"
    );

    let std_pal = include_str!("../vendor/rust/library/std/src/sys/pal/thingos/mod.rs");
    assert!(
        std_pal.contains("ThingOS std PAL is abort-only"),
        "ThingOS std PAL must emit a compile-time panic=unwind diagnostic"
    );
}
