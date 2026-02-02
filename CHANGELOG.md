# Changelog

## Recent Status

The operating system has entered a new phase of visual and architectural maturity. The **Userspace Driver Ecosystem** is expanding with the introduction of a standalone `virtio-gpu` driver and the `ingestd` asset manager, which treats system resources as first-class graph citizens. Introspection capabilities have taken a leap forward with the **HTTPD Graph Explorer**, allowing developers to visualize the live object graph via a web interface. Meanwhile, the boot experience has been refined with a flicker-free, centered log display.

These changes complement the ongoing work on the **Unified Device Graph** and **Kernel Watch API** optimizations.

## Recent Changes

### 🖥️ Display & UI

*   **Bud Display Overhaul**: The early boot display (`bud`) has been redesigned for clarity and performance. It now renders a vertically-centered, three-line log message (Timestamp, Source, Message) and employs dirty-rectangle tracking to clear only the updated text area, eliminating full-screen clears.
    *   *Artifacts*: `bud/src/display.rs`

*   **HTTPD Graph Explorer**: The HTTP server now includes an interactive Graph Explorer at `/graph.html`. Powered by ELK (Eclipse Layout Kernel) and Cytoscape.js, it provides an automatic, layered layout of the system graph, allowing users to visualize node relationships and inspect properties in real-time.
    *   *Artifacts*: `userspace/httpd/assets/graph/`, `userspace/httpd/README.md`

*   **PresentQueue Implementation**: The rendering pipeline has moved from legacy swapchains to a `PresentQueue` model. This improves frame timing and error handling during composition.
    *   *Artifacts*: `userspace/bloom`, `userspace/blossom`

*   **DrawList Demo**: A new `drawlist_demo` application has been added to demonstrate and validate the vector drawing capabilities of the UI graph.
    *   *Artifacts*: `userspace/drawlist_demo/`

### 📦 System Services

*   **Ingestd (Asset Service)**: A new core service, `ingestd`, has been introduced to manage system assets. It continuously watches boot modules and content sources, computes SHA-256 hashes for deduplication, and publishes canonical `ASSET` nodes to the graph. It also handles automatic seeding of wallpapers and cursors.
    *   *Artifacts*: `userspace/ingestd/`

*   **Virtio-GPU Driver**: A standalone userspace driver for VirtIO GPU has been added (`userspace/virtio_gpu`). It demonstrates 2D resource management, MSI-X interrupt handling, and framebuffer setup entirely from userspace, serving as a reference for future driver implementations.
    *   *Artifacts*: `userspace/virtio_gpu/`

### 🛠️ Kernel & ABI

*   **Schema Standardization**: The content provider schema has been updated to use `fs.Directory` and `fs.File` instead of the legacy `content.Directory` and `content.File`. This change aligns the graph schema with the unified filesystem model.
    *   *Artifacts*: `abi/src/schema.rs`

*   **Watch API Optimization**: The `handle_watch_next` kernel handler now employs a hybrid allocation strategy and a scan limit (`WATCH_SCAN_LIMIT = 64`) to prevent long processing blocks. It returns `-EAGAIN` if the limit is reached, ensuring the kernel remains responsive.
    *   *Artifacts*: `kernel/src/root/handlers/watch.rs`, `kernel/src/root/handlers/watch_payload.rs`

*   **Locale Configuration**: Support for locale configuration has been merged, allowing for better internationalization support in the future.

### 🏗️ Infrastructure & Tests

*   **Architecture-Specific Artifacts**: Test artifacts and boot modules are now segregated by architecture (e.g., `docs/behavior/x86_64/`), paving the way for multi-arch support (ARM64, RISC-V).
    *   *Artifacts*: `xtask/src/`, `docs/behavior/`

*   **Watch Payload Tests**: Comprehensive unit tests were added to verify the correctness of watch payload filtering and coalescing, ensuring `Last-Write-Wins` semantics are respected.
    *   *Artifacts*: `kernel/src/root/handlers/watch_payload.rs`
