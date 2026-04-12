# Rustc on ThingOS: Current Status (April 2026)

The std/bootstrap work is **partially working**.

As of April 12, 2026, `BUILD_RUSTC=1 cargo xtask rustc-thingos` successfully
drives `x.py` through:

- stage1 host compiler build
- stage1 host std/sysroot build
- stage1 ThingOS target std build

The current failure is **outside** the Rust bootstrap itself: the outer
`xtask` wrapper had stale assumptions about where `x.py` writes its artifacts.

## What Was Revalidated

The following command completed the Rust bootstrap successfully:

```bash
BUILD_RUSTC=1 cargo xtask rustc-thingos
```

Observed behavior:

- `rustc_driver` no longer hard-fails as a ThingOS `dylib` build
- the ThingOS target std crates build successfully
- `x.py` finishes with `Build completed successfully`

Relevant outputs from the current fork/bootstrap layout:

```text
build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/rustc-main
build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-*.rlib
build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-thingos/release/deps/*.rlib
build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/...
```

## Current Breakage

The old `xtask` logic expected one of these paths:

```text
vendor/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
vendor/rust/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-thingos/release/rustc
```

That is no longer where the current bootstrap writes the compiler.

The current layout uses the repository-root `build/` directory and emits the
stage1 compiler as:

```text
build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/rustc-main
```

Similarly, the ThingOS target libraries are not promoted into
`stage1/lib/rustlib/x86_64-unknown-thingos`; they remain under:

```text
build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-thingos/release/deps/
```

## What Landed in the Outer Repo

The outer wrapper now:

- accepts the current `x.py` artifact layout
- caches the produced compiler binary under `target/rustc-thingos/rustc`
- assembles a cached `rustlib` tree under `target/rustc-thingos/rustlib`
- copies host rustlib from `stage1/lib/rustlib`
- mirrors ThingOS target `.rlib`/`.rmeta` files from `stage1-std/.../deps`

## Important Limitation

This is still **not** a ThingOS-hosted compiler.

The current bootstrap config keeps:

```toml
[build]
host = ["x86_64-unknown-linux-gnu"]
target = ["x86_64-unknown-linux-gnu", "x86_64-unknown-thingos"]
```

So the produced executable is a **Linux-hosted cross-compiler**, not a ThingOS
ELF. `file` reports:

```text
ELF 64-bit LSB pie executable, x86-64, dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2
```

Because of that, ISO staging is intentionally disabled for now. Shipping that
binary into the ThingOS image would place an unusable Linux executable at
`/bin/rustc`.

## Current Warnings

The std build emits `unexpected_cfgs` warnings for `target_os = "thingos"` in
multiple `library/std/src/sys/...` modules.

These warnings do not currently block the build, but they indicate the fork
still needs one of:

- target registration in the relevant check-cfg paths, or
- explicit `cargo::rustc-check-cfg` coverage where appropriate

## Remaining Work

1. Decide whether the near-term goal is:
   - a Linux-hosted cross-compiler only, or
   - a true ThingOS-hosted compiler staged into the image
2. If the goal is ThingOS-hosted, switch bootstrap `host` to `x86_64-unknown-thingos`
   and revalidate the full compiler build under that configuration.
3. Promote ThingOS target libs into a canonical sysroot layout rather than
   reconstructing them from `stage1-std/.../deps`.
4. Resolve the `unexpected_cfgs` warnings for `target_os = "thingos"`.
5. Re-enable ISO staging only after the produced `rustc` binary is actually
   runnable on ThingOS.

## Related Checkout Issue

Some checkouts currently have `.gitmodules` and `.git/modules/vendor/rust`
present without a `vendor/rust` gitlink recorded in the main worktree index.

In that state, submodule-only `just fetch-rust` logic fails even though the
Rust fork metadata exists locally. The build/docs should treat `vendor/rust` as
supporting both:

- normal submodule mode
- plain checkout mode

---
*Document last updated: April 12, 2026*
*Status: Bootstrap succeeds; wrapper/docs were stale; ThingOS-hosted rustc still pending*
