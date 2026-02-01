# Changelog

## Recent Status

The operating system is currently undergoing significant refinements in its core subsystems. Key areas of focus include the **Unified Device Graph**, where schema definitions are being standardized (e.g., `fs.Directory` vs `content.Directory`), and the **Kernel Watch API**, which has received performance optimizations to reduce memory allocations and improve responsiveness during high-throughput graph mutations.

On the user-facing side, the **Graphical Subsystem** is evolving with the introduction of `PresentQueue` replacing swapchains, enabling more robust frame presentation. The `drawlist_demo` application showcases the new vector drawing capabilities.

Infrastructure-wise, the build system is becoming more architecture-aware, with test artifacts now organized by target architecture (e.g., `x86_64`), improving cross-compilation support.

## Recent Changes

### 🛠️ Kernel & ABI

*   **Schema Standardization**: The content provider schema has been updated to use `fs.Directory` and `fs.File` instead of the legacy `content.Directory` and `content.File`. This change aligns the graph schema with the unified filesystem model.
    *   *Artifacts*: `abi/src/schema.rs`

*   **Watch API Optimization**: The `handle_watch_next` kernel handler now employs a hybrid allocation strategy and a scan limit (`WATCH_SCAN_LIMIT = 64`) to prevent long processing blocks. It returns `-EAGAIN` if the limit is reached, ensuring the kernel remains responsive.
    *   *Artifacts*: `kernel/src/root/handlers/watch.rs`, `kernel/src/root/handlers/watch_payload.rs`

*   **Locale Configuration**: Support for locale configuration has been merged, allowing for better internationalization support in the future.

### 🎨 Graphics & UI

*   **PresentQueue Implementation**: The rendering pipeline has moved from legacy swapchains to a `PresentQueue` model. This improves frame timing and error handling during composition.
    *   *Artifacts*: `userspace/bloom`, `userspace/blossom`

*   **DrawList Demo**: A new `drawlist_demo` application has been added to demonstrate and validate the vector drawing capabilities of the UI graph.
    *   *Artifacts*: `userspace/drawlist_demo/`

### 🏗️ Infrastructure & Tests

*   **Architecture-Specific Artifacts**: Test artifacts and boot modules are now segregated by architecture (e.g., `docs/behavior/x86_64/`), paving the way for multi-arch support (ARM64, RISC-V).
    *   *Artifacts*: `xtask/src/`, `docs/behavior/`

*   **Watch Payload Tests**: Comprehensive unit tests were added to verify the correctness of watch payload filtering and coalescing, ensuring `Last-Write-Wins` semantics are respected.
    *   *Artifacts*: `kernel/src/root/handlers/watch_payload.rs`
