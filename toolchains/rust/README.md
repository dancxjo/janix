# ThingOS Rust std Patch Queue (Ramp A)

This directory tracks ThingOS-specific `library/std` changes as patch files, without vendoring the full `rust-lang/rust` repo into this workspace.

## Baseline Upstream Commit

Current patch series was generated from:

- `rust-lang/rust` commit: `286fbe5d84569c718f189122db9e68a16b50eeef`

## Files

- `patches/0001-std-add-initial-ThingOS-PAL-and-minimal-sys-backend.patch`
- `patches/0002-std-add-ThingOS-_start-runtime-entry.patch`
- `scripts/apply.sh`
- `demo/std_demo.rs`

## Usage

1. Clone rust and checkout baseline:

```bash
git clone https://github.com/rust-lang/rust ~/src/rust-thingos
cd ~/src/rust-thingos
git checkout 286fbe5d84569c718f189122db9e68a16b50eeef
git checkout -b thingos-std
```

2. Apply ThingOS patch queue:

```bash
/path/to/thing-os/toolchains/rust/scripts/apply.sh ~/src/rust-thingos
```

3. Build std for ThingOS JSON target:

```bash
cd ~/src/rust-thingos
./x.py build library/std --target /path/to/thing-os/targets/x86_64-unknown-thingos.json
```

4. Compile/link demo proof from ThingOS repo:

```bash
cd /path/to/thing-os
just rust-std-demo /path/to/rust-checkout 0 0
```

## Notes

- `bootstrap.toml` in your rust checkout should disable deny-warnings for custom JSON targets, or invoke `x.py --warnings warn ...`.
- The current minimal profile focuses on: panic-abort, stdio write, time/sleep, and baseline thread hooks. File APIs are intentionally deferred.
