# Input Pipeline Technical Overview

This document traces the path of a keypress from hardware interrupt to userland application event in ThingOS.

## 1. Hardware Interrupt (IRQ 1)
*   **Module**: `kernel/src/interrupts/idt.rs` (IDT entry), `kernel/src/arch/x86_64/interrupts.rs` (ISR).
*   **Implementation**: The kernel IDT routes IRQ 1 to `extern "x86-interrupt" fn keyboard_interrupt_handler`.
*   **Action**: The ISR reads the scancode byte from I/O port `0x60`.
*   **Output**: The byte is pushed into a kernel-space ring buffer (`PS2_KEYBOARD_BUFFER` in `kernel/src/devices/ps2_buffers.rs`). No graph mutations occur here.

## 2. Kernel Input Pump (Hybrid Processing)
*   **Module**: `kernel/src/devices/ps2_buffers.rs` and `user/ps2_keyboard_driver`.
*   **Implementation**: The kernel buffers raw bytes. The userland `ps2_keyboard_driver` acts as the "pump" by polling the kernel buffer.
*   **Action**: `ps2_keyboard_driver` runs a loop that periodically sleeps (`POLL_INTERVAL_NS`) and then calls `syscall_dev_read`.
*   **Output**: Raw byte stream delivered to the userland driver's memory.

## 3. Syscall ABI
*   **Module**: `abi/src/syscall_defs.rs`, `kernel/src/syscall/mod.rs` (dispatch).
*   **Mechanism**: `SYS_DEV_READ` (0x2001).
*   **Action**: The userland driver invokes the syscall on a `DeviceHandle` (obtained via `SYS_DEV_OPEN`). The kernel copies data from the `PS2_KEYBOARD_BUFFER` to the user supplied buffer.
*   **Constraints**: Strictly POD copy; no graph interaction.

## 4. Userland Input Service (Decoder)
*   **Module**: `user/ps2_keyboard_driver/src/lib.rs`.
*   **Implementation**: The driver implements a `KeyboardDecoder`.
*   **Action**:
    *   **Phase 4.1 (Physical)**: Decodes Set 2 scancodes, handling E0 prefixes and break codes.
    *   **Phase 4.2 (Stream)**: Writes decoded `KeyboardEntry` (scancode + utf32 + flags) to a Resident Memory Stream ("KeyboardStream") mapped by the compositor.
    *   **Phase 4.3 (Events)**: Creates `KeyScanEvent` and `InputCharEvent` Things in the graph using `create_thing`.
*   **Output**:
    *   Resident Memory: Stream of `KeyboardEntry` structures.
    *   Graph: Nodes representing input events (`KeyScanEvent`, `InputCharEvent`).

## 5. Input Consumption
*   **Module**: `user/compositor`.
*   **Implementation**:
    *   **Events**: The compositor queries the graph for `KeyScanEvent` and `InputCharEvent` things.
    *   **Stream**: The compositor maps the "KeyboardStream" resident memory to read high-throughput events efficiently.
*   **Action**: The compositor dispatches these events to the active window or handles global shortcuts (like Mode Switching).

## 6. Focus & Routing
*   **Module**: `user/compositor/src/input.rs` and `user/compositor/src/model.rs`.
*   **Implementation**: `process_mouse_packets` and `ensure_window_active_from_layout`.
*   **Action**: The compositor determines the active window based on layout and mouse interaction. It updates `Window` properties (`active`, `z_index`) to reflect focus. Application input routing is currently implicit via the `Window` property state, or apps subscribe to input events directly (NOTE: This potentially broadcasts all key events to all listeners, relying on "Focus" property for filtering).

## Known Gaps / Issues
1.  **Broadcast vs Targeted**: Input events are currently broadcast to the graph or available in a global stream. Secure input routing will eventually require targeted piping to the focused window's queue rather than global graph visibility.
2.  **Latency**: Polling `syscall_dev_read` introduces latency compared to interrupt-driven userspace notifications (planned).
