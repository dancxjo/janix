# Bulb

**Bulb** (Boot Up Liveness Behavior) is the early-boot display and animation library with Wasm plugin support.

## Role

Bulb provides the capability to draw text and basic graphics to the framebuffer *before* the full graphics stack (Bloom/Blossom) is running. It supports **swappable boot animations** via sandboxed WebAssembly modules.

## Architecture

### Host (kernel or early userspace) owns:
- Framebuffer / drawing primitives (via `embedded-graphics`)
- Timing (ticks)
- Boot state (Dormant → KernelLoaded → ServicesLoaded)
- Scheduling / preemption (fuel-based)
- Module loading (from Limine modules)

### Guest (Wasm "bulb module") owns:
- Pure rendering decisions
- Lightweight internal state machine
- Animation behavior (within fuel limits)

## Features

*   **Software Rendering**: Draws directly to the linear framebuffer provided by the bootloader.
*   **Text Console**: Supports a simple text console with scrolling/wrapping (or cyclical logging).
*   **No Dependencies**: Designed to run in a `no_std` environment with minimal requirements.
*   **Wasm Plugin System**: Custom boot animations can be provided as `.wasm` modules.
*   **Fuel Preemption**: Wasm modules run with instruction budgets; misbehaving modules are safely trapped.

## Wasm Module Contract

Guest modules must export:
- `bulb_init(seed: u64) -> u32`
- `bulb_on_boot(state_ptr, state_len)`
- `bulb_render()`
- Optionally: `bulb_on_tick(now_ms: u64)`

The host provides graphics operations via a command buffer (mapped to `embedded-graphics` primitives).

Bulb is the "visual voice" of the system during the critical handover from bootloader to kernel.
