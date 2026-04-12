Rust and LLVM snapshot patches for the vendored `vendor/rust` checkout.

Usage:

```bash
just fetch-rust
just rust-apply-patches
```

Layout:

- `vendor-rust/` applies to the top-level `vendor/rust` repository.
- `llvm-project/` applies to the nested `vendor/rust/src/llvm-project` repository.

The older flat `patches/rust/*.patch` stack in this repo is left untouched here;
`just rust-apply-patches` intentionally replays only the structured snapshots
above.

These snapshots are a replay mechanism for local development. The long-term
source of truth is still the `rust-thingos` fork.
