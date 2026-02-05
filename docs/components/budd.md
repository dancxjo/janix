# Budd

**Budd** (Boot Up Display & Diagnostics) is the early-boot display and diagnostics library.

## Role

Budd provides the capability to draw text and basic graphics to the framebuffer *before* the full graphics stack (Bloom/Blossom) is running.

## Usage

*   **Bran**: Uses Budd to display the boot progress, logo, and early logs on the screen.
*   **Kernel**: May use Budd for panic screens or early console output.

## Features

*   **Software Rendering**: Draws directly to the linear framebuffer provided by the bootloader.
*   **Text Console**: Supports a simple text console with scrolling/wrapping (or cyclical logging).
*   **No Dependencies**: Designed to run in a `no_std` environment with minimal requirements.

Budd is the "visual voice" of the system during the critical handover from bootloader to kernel.
