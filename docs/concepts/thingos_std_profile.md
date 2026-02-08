# ThingOS Minimum `std` Profile (Ramp A)

This document defines the **first supported `std` surface** for ThingOS while continuing to use JSON targets.

It is intentionally narrow: enough to run `std` programs with logging, allocation, timing, and basic threads, without pretending to support full Unix/Windows semantics.

## Status

- Scope: experimental bring-up profile for local `rust-lang/rust` checkout work.
- Policy: this does **not** replace `stem` or the `no_std` policy for existing runtime crates.
- Goal: make `library/std` build and run for `target_os = "thingos"` with explicit feature boundaries.

## Target/Build Contract

- JSON target must set `"os": "thingos"` (already true in `targets/*-unknown-thingos.json`).
- `std` is built from a local Rust checkout with:
  - `./x.py build compiler/rustc`
  - `./x.py build library/std --target /abs/path/to/targets/x86_64-unknown-thingos.json`
- Panic strategy: **abort-only** for initial profile.
- Unwinding: unsupported.

## Supported `std` Surface (v0)

These are the minimum guarantees for the first successful profile.

1. `std::println!`/`eprintln!` and `std::io::Write` to stdout/stderr.
2. Heap allocation via global allocator (`Box`, `Vec`, `String`, collections).
3. `std::time::Instant` monotonic clock behavior.
4. `std::thread::sleep`.
5. `std::thread::spawn` + `JoinHandle::join` (basic threads, no advanced tuning).
6. `panic!` prints best-effort message and aborts predictably.

## Explicitly Unsupported in v0

These must return a stable unsupported/error path (typically `ENOSYS`-style) rather than half-working behavior.

1. `std::fs` (`File`, directories, metadata, path-based open).
2. `std::net` (TCP/UDP, listeners, DNS).
3. `std::process::Command` and subprocess management.
4. `std::env` process environment (`vars`, `current_dir`, `home_dir`, etc.).
5. `std::os::unix::*`/`std::os::windows::*` specific APIs.
6. Unwinding and `catch_unwind`-dependent recovery semantics.

## PAL-to-ThingOS Mapping (v0)

This is the expected backend behavior for `library/std/src/sys/pal/thingos`.

### Termination + Panic

- `abort`/fatal path -> `SYS_EXIT`.
- Panic reporting -> `SYS_LOG_WRITE` (or debug channel equivalent), then abort.
- No panic unwinding runtime.

### Stdio

- `stdout` and `stderr` write path -> `SYS_LOG_WRITE` for now.
- Reads from `stdin`: unsupported in v0 (return unsupported/error).
- Atomicity is best-effort line/buffer based, not POSIX fd semantics.

### Time

- Monotonic clock -> `SYS_TIME_MONOTONIC`.
- Sleep -> `SYS_SLEEP_NS` (or ms wrapper when needed).
- Wall clock can be absent/unanchored; monotonic is the correctness baseline.

### Threads

- Spawn -> `SYS_SPAWN_THREAD` (with runtime-managed stack contract).
- Join/wait -> `SYS_TASK_WAIT`.
- Yield -> `SYS_YIELD`.
- Priority and advanced controls are out-of-contract for v0 unless required by `std` internals.

### Memory

- Keep Rust allocator + compiler-builtins path as used today for `core/alloc` targets.
- Any virtual memory growth hooks remain internal to ThingOS runtime/allocator glue.

## Behavior Contract for Unsupported Features

When an unsupported `std` API is called:

1. Fail deterministically.
2. Return documented unsupported errors (`ENOSYS`/`Unsupported` class).
3. Never silently noop for APIs that promise effects.
4. Never fake success.

## Bring-Up Milestones

1. **Compile milestone**: `library/std` compiles for `target_os=thingos` using local rust checkout.
2. **Runtime milestone A**: `println!`, allocation, panic-abort demo works on ThingOS.
3. **Runtime milestone B**: `thread::sleep` and monotonic timing correctness checks pass.
4. **Runtime milestone C**: `thread::spawn` + `join` demo works.
5. **Stabilization milestone**: unsupported modules fail cleanly and consistently.

## Demo Program Requirements (v0)

A proof program should exercise:

1. `println!("hello from std")`
2. `let mut v = Vec::new(); v.push(...)`
3. `std::thread::sleep(Duration::from_millis(50))`
4. `let h = std::thread::spawn(...); h.join().unwrap();`
5. `panic!("expected panic path")` in a separate test binary to verify abort behavior

## Non-Goals for Ramp A

1. Upstreaming target into rustc (Ramp B).
2. Full POSIX compatibility.
3. Full `std` parity with Linux.
4. Replacing existing `stem`-first application model.

## Decision Gate to Ramp B

Consider upstream target work only after all are true:

1. `library/std` build is repeatable for all intended ThingOS architectures.
2. v0 supported APIs run reliably in QEMU and hardware smoke tests.
3. Unsupported surfaces are explicit and tested.
4. Maintenance burden of local Rust patch set is understood and documented.
