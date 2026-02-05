# Bulb Themes

Boot animation themes implemented as WebAssembly modules.

## Available Themes

| Theme | Description |
|-------|-------------|
| `genie_circles` | Four pulsing circles representing boot phases |

## Building Themes

```bash
cd bulb/themes/genie_circles
cargo build --target wasm32-unknown-unknown --release
```

The `.wasm` file will be at `target/wasm32-unknown-unknown/release/genie_circles.wasm`.

## Theme Contract

Each theme must export:
- `bulb_init(seed: u64) -> u32` - Initialize with random seed
- `bulb_on_boot(state_ptr, state_len)` - Boot state changed
- `bulb_on_tick(now_ms: u64)` - Animation tick (optional)
- `bulb_render()` - Render current frame

Themes emit draw commands via `cmd_submit(ptr, len)`.
