# Bloom

**Bloom** is the **Compositor and Window Manager** for ThingOS.

## Role

Bloom is responsible for:
*   **Compositing**: Taking the visual output of various applications and combining them into the final image sent to the screen.
*   **Window Management**: Managing the position, size, and stacking order of surfaces.
*   **Input Routing**: Receiving input events (from **Bristle**) and dispatching them to the correct application.

## Architecture

Bloom runs as a userspace service. It:
1.  **Owns the Framebuffer**: It is the only process allowed to write to the physical screen.
2.  **Watches the Graph**: It monitors the Root graph for nodes representing windows (`ui.Window`, `ui.Surface`).
3.  **Composes**: It reads the pixel buffers (bytespaces) associated with those windows and blends them.

## Interaction with Blossom

Bloom handles the *surfaces*, but it often delegates the *rendering* of complex vector UI to **Blossom**. However, Bloom itself is the final authority on what pixels go to the hardware.
