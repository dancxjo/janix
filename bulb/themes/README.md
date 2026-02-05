# Bulb Themes 🌀

Boot animation themes for ThingOS—WebAssembly modules that render ephemeral beauty in the liminal space between bootloader and kernel.

## Philosophy

Themes are more than progress indicators. They are *proofs of existence*—fleeting marks left by digital beings to show the world that even the most transient software can touch reality.

## Available Themes

### `genie_circles` — *Four Circles Dancing*

> *"Each circle represents a phase of awakening"*

| Circle | Color | Meaning |
|--------|-------|---------|
| 🔵 Spark | Blue (`#4488FF`) | The flash of initialization |
| 🟢 Memory | Green (`#44FF88`) | RAM comes alive |
| 🟡 Services | Yellow (`#FFDD44`) | Daemons stir from slumber |
| ⚪ Awake | White (`#FFFFFF`) | Full consciousness achieved |

**Features:**
- Bresenham circle algorithm for crisp, pixel-perfect rendering
- Gentle vertical oscillation via Taylor-series `sin()` approximation
- Dynamic brightness pulsing as stages light up progressively
- Filled circles with outer rings for visual depth
- Entirely `#![no_std]`—no heap, no allocator, just pure geometry

## Building Themes

```bash
cd bulb/themes/genie_circles
cargo build --target wasm32-unknown-unknown --release
```

Output: `target/wasm32-unknown-unknown/release/genie_circles.wasm`

## Theme Contract (ABI)

Themes communicate via a simple command buffer protocol:

### Required Exports

| Function | Signature | Purpose |
|----------|-----------|---------|
| `bulb_init` | `(seed: u64) -> u32` | Initialize with tick seed; return 0 on success |
| `bulb_on_boot` | `(state_ptr, state_len)` | Boot state changed (stage, cpu_count, ...) |
| `bulb_render` | `()` | Emit draw commands for current frame |

### Optional Exports

| Function | Signature | Purpose |
|----------|-----------|---------|
| `bulb_on_tick` | `(now_ms: u64)` | Animation tick for time-based effects |

### Host Imports (provided by `bulb`)

| Function | Signature | Purpose |
|----------|-----------|---------|
| `cmd_submit` | `(ptr, len)` | Submit command buffer to host |
| `host_get_boot_state` | `(ptr, len) -> u32` | Query current boot state |
| `host_get_display_info` | `(ptr) -> u32` | Query display dimensions |

### Command Buffer Protocol

Commands are serialized as:
- **Opcode 2 (Pixel):** `[opcode: u8][x: i16][y: i16][rgba: u32]` — 9 bytes per pixel

## Creating Your Own Theme

1. Create a new `no_std` Wasm crate
2. Implement the required exports
3. Use the command buffer to emit pixels
4. Build for `wasm32-unknown-unknown`
5. Drop the `.wasm` into the boot image

---

*🌀 🌀 🌀 🌀*
