# Graph Actualization Model

**(ThingOS Hardware Delegation Architecture)**

## Overview

ThingOS follows a **Graph Actualization** model for hardware interaction, moving away from traditional syscalls, device files, or driver-specific APIs where possible.

There are two flows:

### 1. Input Direction (Hardware → Kernel → Graph)
The kernel captures hardware events (via IRQs, controllers, timers) and records them into the graph as structured Things.

### 2. Output Direction (Userland → Graph → Kernel → Hardware)
Userland describes its intended hardware state by mutating Things in the graph. The kernel's **Actualizer** subsystem observes these changes and updates the physical hardware to match.

The graph becomes the OS’s sole control plane.
The kernel becomes a graph actualizer.

## Goals

*   **No driver-specific syscalls**: Interact with hardware via generic `ThingCreate` / `ThingUpdate` calls.
*   **Declarative Intent**: Userland expresses *what* it wants (state), not *how* to do it (commands).
*   **Kernel as Physics Engine**: The kernel ensures the physical world (hardware) matches the virtual world (graph).
*   **Introspection**: Hardware state is queryable via standard graph tools.

## Input Pipeline

**(Hardware → Kernel → Graph)**

Hardware events are captured by kernel drivers and reified as Things:

*   **Keyboard**: `KeyScanEvent` (via `ps2_keyboard_driver` user pump).
*   **Mouse**: `MousePacket` stream or `PointerEvent`.
*   **Timers**: `TickEvent` (or implicitly via Scheduler).

Note: High-bandwidth inputs (like Audio or Video capture) may use **Resident Memory Streams** (mapped shared memory) for payload data while using Graph Things for signaling control and status, to avoid graph database overhead for bulk data.

## Output Pipeline

**(Userland → Graph → Kernel → Hardware)**

Userland determines what hardware should do by mutating specific Things.

### Examples:

#### Cursor Movement
Userland compositor writes:
```rust
CursorState {
    x: 640,
    y: 480,
    visible: true
}
```
Kernel actualizer:
1.  Observes change to `CursorState`.
2.  Updates hardware cursor registers (or software overlay).

#### Serial Output
Userland writes:
```rust
SerialWrite {
    port: COM1,
    bytes: Blob("Hi"),
    state: Pending
}
```
Kernel actualizer:
1.  Finds `Pending` writes.
2.  Writes data to UART.
3.  Updates Thing to `state: Done`.

#### Display Update
Userland:
1.  Writes pixels to shared memory buffer.
2.  Updates `DisplayBuffer` Thing:
```rust
DisplayBuffer {
    buffer_ref: ThingId(123),
    version: 42
}
```
Kernel actualizer:
1.  Detects version change.
2.  Flushes buffer to physical VRAM (if necessary) or flips pointers.

## Kernel Actualizer Module

The `kernel::graph::actualizer` (and related `reifier` modules) are responsible for this loop.

**Responsibilities:**
1.  **Observe**: Subscribe to graph changes (`GraphEvent::ThingUpdated`).
2.  **Filter**: Identify Things of interest (e.g., `Kind::Schedule`, `Kind::SerialWrite`).
3.  **Actuate**: Perform the physical side-effect (MMIO write, register update).
4.  **Feedback**: Update the graph with results (e.g., `state: Done`, `error: ...`).

## Comparison to Traditional Models

| Traditional OS | ThingOS |
| :--- | :--- |
| Syscalls, ioctls, device files | Graph mutations |
| Kernel drivers interpret semantics | Userland interprets semantics |
| Kernel enforces policy | Userland expresses intent; kernel actualizes |
| Userland calls kernel | Kernel watches userland state |
| Imperative commands | Declarative state description |

## Summary

In ThingOS:
*   **Hardware → Graph** expresses what happened.
*   **Userland → Graph** expresses what should happen.
*   **Kernel Actualizer** makes the physical world match the graph world.

This creates a coherent, declarative, introspective, and elegantly minimal OS model.
