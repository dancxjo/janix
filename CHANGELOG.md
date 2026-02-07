# Changelog

## Recent Status

Current development is focused on **refining the build process and developer ergonomics**. The experimental `json-target-spec` flag has been dropped to simplify the build pipeline, and the userspace library (`stem`) now provides first-class `debug!` and `warn!` macros to aid in application development and troubleshooting.

The operating system's userspace capabilities have expanded significantly with the introduction of a **WASM Driver Host**. This new runtime environment allows drivers to be executed as WebAssembly modules, providing a sandboxed and architecture-independent execution model. Crucially, the host includes a **record-and-replay syscall tracing** system, enabling developers to capture driver interactions and replay them deterministically for debugging.

Complementing this, the **Anther** HTTP server has gained **file upload capabilities**. Clients can now push files directly to the system via a `POST /upload` endpoint, which automatically handles bytespace creation, SHA-256 hashing, and metadata extraction (MIME type, size) to populate `fs.File` nodes in the graph.

On the visual front, the default desktop experience has been refreshed with a new wallpaper (`linen.bmp`), automatically seeded by the `flytrap` asset service.

## Recent Changes

### 🏗️ Infrastructure & Tooling

*   **Simplified Build Flags**: Removed `-Z json-target-spec` from the cargo build invocations in `xtask`. This streamlining reduces reliance on unstable compiler flags for target specification.
    *   *Artifacts*: `xtask/src/image.rs`

### 📚 Libraries & Runtime

*   **Enhanced Logging in Stem**: Added `debug!` and `warn!` macros to the `stem` library. This unifies logging capabilities across the userspace ecosystem, making it easier to instrument code with appropriate log levels.
    *   *Artifacts*: `stem/src/lib.rs`

### 🔌 Userspace & Drivers

*   **WASM Driver Host**: Introduced a runtime for executing WASM-based drivers with `thing.sys` syscall support. It features a tracing mechanism to record and replay syscalls for deterministic debugging.
    *   *Artifacts*: `userspace/driver_wasm_host/`

*   **WASM Driver Prototype**: Added an example driver demonstrating MMIO interactions and logging from within the WASM sandbox.
    *   *Artifacts*: `userspace/proto_driver_wasm/`

*   **Anther File Upload**: Added a `POST /upload` endpoint to the web server. It handles file ingestion by creating bytespaces and file nodes, automatically calculating hashes and setting metadata properties (`FILE_MIME`, `FILE_SIZE`, etc.).
    *   *Artifacts*: `userspace/anther/src/upload.rs`

### 🛠️ Kernel & ABI

*   **Log Entry Eviction**: Implemented a ring-buffer eviction strategy for kernel log entries. This ensures that the graph history doesn't grow indefinitely, evicting the oldest entries when memory limits are reached.
    *   *Artifacts*: `kernel/src/memory/kheap.rs`, `kernel/src/root/graph.rs`

*   **Root Registry Links**: Core system nodes (`scheduler`, `host`, `boot_modules`) are now directly linked from `svc.Root`. This allows for O(1) discovery, bypassing expensive graph traversals for frequently accessed nodes.
    *   *Artifacts*: `kernel/src/root/boot_register.rs`

*   **Schema Standardization**: The content provider schema has been updated to use `fs.Directory` and `fs.File` instead of the legacy `content.Directory` and `content.File`. This change aligns the graph schema with the unified filesystem model.
    *   *Artifacts*: `abi/src/schema.rs`

*   **Watch API Optimization**: The `handle_watch_next` kernel handler now employs a hybrid allocation strategy and a scan limit (`WATCH_SCAN_LIMIT = 64`) to prevent long processing blocks. It returns `-EAGAIN` if the limit is reached, ensuring the kernel remains responsive.
    *   *Artifacts*: `kernel/src/root/handlers/watch.rs`, `kernel/src/root/handlers/watch_payload.rs`

*   **Locale Configuration**: Support for locale configuration has been merged, allowing for better internationalization support in the future.

### 🎨 Graphics & UI

*   **Framebuffer Stride Heuristics**: The `bulb` crate and `fb_common` library now correctly interpret framebuffer stride as pixels or bytes based on a BPP heuristic. This fixes diagonal shearing issues observed during early boot on certain display backends.
    *   *Artifacts*: `libs/fb_common/src/lib.rs`, `bulb/src/display.rs`

*   **PresentQueue Implementation**: The rendering pipeline has moved from legacy swapchains to a `PresentQueue` model. This improves frame timing and error handling during composition.
    *   *Artifacts*: `userspace/bloom`, `userspace/blossom`

*   **DrawList Demo**: A new `drawlist_demo` application has been added to demonstrate and validate the vector drawing capabilities of the UI graph.
    *   *Artifacts*: `userspace/drawlist_demo/`

*   **Default Wallpaper**: Updated the default system wallpaper to `linen.bmp`, providing a cleaner aesthetic. The `flytrap` service now automatically seeds this wallpaper for the `photosynthesis` desktop environment, and the build system (`xtask`) explicitly allows this asset in the ISO image.
    *   *Artifacts*: `assets/wallpapers/linen.bmp`, `userspace/flytrap/src/main.rs`, `xtask/src/image.rs`

### 🏗️ Infrastructure & Tests

*   **Multi-tasking BDD**: Added a new BDD feature file and test steps to verify multi-tasking capabilities, ensuring proper task scheduling and isolation.
    *   *Artifacts*: `docs/behavior/multi_tasking.feature`

*   **ISO9660 Validation**: Added unit tests for `IsoFs::probe` to validate filesystem detection logic without requiring actual ISO images.
    *   *Artifacts*: `userspace/iso9660/src/lib.rs`

*   **Content Provider Schema Fixes**: Updated tests and documentation to match the `fs.Directory` / `fs.File` schema changes, ensuring consistency across the codebase.
    *   *Artifacts*: `abi/src/schema.rs`

*   **Query Chaining Tests**: Added unit tests to verify query chaining behavior and limits in the kernel graph engine.
    *   *Artifacts*: `kernel/src/tests/`

*   **Architecture-Specific Artifacts**: Test artifacts and boot modules are now segregated by architecture (e.g., `docs/behavior/x86_64/`), paving the way for multi-arch support (ARM64, RISC-V).
    *   *Artifacts*: `xtask/src/`, `docs/behavior/`

*   **Watch Payload Tests**: Comprehensive unit tests were added to verify the correctness of watch payload filtering and coalescing, ensuring `Last-Write-Wins` semantics are respected.
    *   *Artifacts*: `kernel/src/root/handlers/watch_payload.rs`
