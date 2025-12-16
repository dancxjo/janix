# Geographer

Geographer is a read-only, continuously updating visualization of the live ThingOS system graph. It renders the current truth of the system: Things, Kinds, and Links, over a calm, cloud-tiled background.

## Purpose

Geographer serves as a "system map" rather than a dashboard or debugger. It is designed to be:
*   **Passive**: It observes but never mutates the graph.
*   **Calm**: It uses slow, smooth animations and avoids jarring movements.
*   **Resilient**: It continues rendering the background even if graph queries fail.

## Architecture

Geographer runs as a userland service. It:
1.  Creates a full-screen window and surface.
2.  Enters a render loop (target 60 FPS).
3.  Periodically snapshots the system graph (every ~2 seconds).
4.  Renders the snapshot using a deterministic layout.

### Rendering Layers

1.  **Sky**: A scrolling, procedurally generated cloud texture.
2.  **Grid**: Things are grouped by Kind and placed in a deterministic grid.
3.  **Links**: Relationships between Things are drawn as faint lines.
4.  **Overlay**: A minimal HUD shows total counts (Things, Kinds, Links).

## Limitations

*   **No Interaction**: Geographer currently ignores mouse and keyboard input.
*   **No Physics**: Items are placed deterministically based on their ID, not simulated.
*   **Snapshot Accuracy**: The graph view is a snapshot and may lag behind the kernel's actual state by up to 2 seconds.
