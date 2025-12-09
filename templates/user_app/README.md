# {{ crate_name }}

ThingOS userland app.

## What this is

This crate is a no_std userland app for ThingOS. It:

- uses `userland::prelude::*` to hide ABI / alloc boilerplate
- exposes a single entry point:

```rust
pub fn run<S: Sys>(sys: &mut S)
```

## Typical editing flow

1. Put your app logic in `run`.
2. Optionally define `Thing` models to store state in the graph.
3. Wire this crate into the kernel’s app launcher so the kernel creates a process + thread for it at boot.
