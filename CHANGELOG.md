# Changelog

## Recent Status

The operating system is undergoing a major expansion of its userspace capabilities, with a focus on **structured data management** and **enhanced hardware support**. A new **Graph Database (Phloem)** has been introduced, bringing OpenGQL-subset query capabilities to the system graph. The networking stack has been consolidated into a unified **Network Daemon (Netd)**, and audio support has arrived with the **Virtio Sound** driver.

System orchestration is now handled by **Sprout**, a dedicated supervisor service, while **Cambium** provides a reactive data-binding layer for UI synchronization. The development tooling (`xtask`) has seen significant updates to support **multi-architecture emulation** (x86_64, ARM64, RISC-V, LoongArch) and automated visual verification. In addition, the kernel graph engine now employs **query buffer recycling** to reduce allocations, and the standard UI library has been expanded with new form widgets.

## Recent Changes

### 🌟 New Services

*   **Phloem (Graph DB)**: Introduced a graph query service that allows applications to query and mutate the system graph using an OpenGQL-subset syntax (e.g., `MATCH`, `MERGE`, `LINKS_TO`). It includes a client library and a TCP-based service.
    *   *Artifacts*: `userspace/phloem/`
*   **Netd (Network Daemon)**: A unified network service handling DHCP configuration, DNS resolution, and HTTP requests, consolidating previous network components.
    *   *Artifacts*: `userspace/netd/`
*   **Sprout (Supervisor)**: A new process supervisor that manages the lifecycle of core userspace services like `flytrap` and `blossom`.
    *   *Artifacts*: `userspace/sprout/`
*   **Cambium (Data Binding)**: A reactive data-binding service that synchronizes properties between graph nodes based on `BINDING` definitions, enabling dynamic UI updates without polling.
    *   *Artifacts*: `userspace/cambium/`
*   **Flytrap (Asset Watcher)**: Enhanced the asset watcher service to automatically discover and hash assets from boot modules, ensuring reliable content delivery.
    *   *Artifacts*: `userspace/flytrap/`

### 🔊 Drivers & Hardware

*   **Virtio Sound**: Added a driver for VirtIO sound devices, supporting PCM playback, queue management, and real-time status monitoring (buffer usage, underruns).
    *   *Artifacts*: `userspace/virtio_sound/`
*   **Virtio GPU**: Expanded support for VirtIO GPU devices with a dedicated driver implementation.
    *   *Artifacts*: `userspace/virtio_gpu/`
*   **Bristle (Input)**: Enhanced mouse input processing to handle split packets and improve event normalization.
    *   *Artifacts*: `userspace/bristle/`
*   **Kernel IRQ Handling**: Updated the kernel IRQ handler to prefer LAPIC ISR-reported vectors for more reliable interrupt processing.
    *   *Artifacts*: `kernel/src/arch/x86_64/interrupts.rs`

### 🛠️ Tooling & Infrastructure

*   **Multi-Arch QEMU Support**: The `xtask` tool now supports running QEMU for multiple architectures (`x86_64`, `aarch64`, `riscv64`, `loongarch64`) with configurable BIOS/UEFI modes.
    *   *Artifacts*: `xtask/src/run.rs`
*   **Image Scanning Tool**: Replaced legacy Python scripts with a Rust-based image scanner (`xtask scan`) for verifying visual outputs (e.g., counting red pixels, finding top colors) during tests.
    *   *Artifacts*: `xtask/src/scan.rs`
*   **Phloem Integration Tests**: Added a test harness (`xtask test-phloem`) to verify Graph DB functionality via TCP.
    *   *Artifacts*: `xtask/src/test_phloem.rs`

### 📱 Applications

*   **New Demo Apps**: Introduced `Font Explorer`, `Fortune` (LLM demo), and `Drawlist Demo` to showcase system capabilities.
    *   *Artifacts*: `userspace/font_explorer/`, `userspace/fortune/`, `userspace/drawlist_demo/`

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

*   **Query Buffer Recycling**: Implemented `QueryScratch` to reuse buffers during graph queries, reducing memory allocations in the hot path.
    *   *Artifacts*: `kernel/src/root/query.rs`

### 🎨 Graphics & UI

*   **Framebuffer Stride Heuristics**: The `bulb` crate and `fb_common` library now correctly interpret framebuffer stride as pixels or bytes based on a BPP heuristic. This fixes diagonal shearing issues observed during early boot on certain display backends.
    *   *Artifacts*: `libs/fb_common/src/lib.rs`, `bulb/src/display.rs`

*   **PresentQueue Implementation**: The rendering pipeline has moved from legacy swapchains to a `PresentQueue` model. This improves frame timing and error handling during composition.
    *   *Artifacts*: `userspace/bloom`, `userspace/blossom`

*   **DrawList Demo**: A new `drawlist_demo` application has been added to demonstrate and validate the vector drawing capabilities of the UI graph.
    *   *Artifacts*: `userspace/drawlist_demo/`

*   **Default Wallpaper**: Updated the default system wallpaper to `linen.bmp`, providing a cleaner aesthetic. The `flytrap` service now automatically seeds this wallpaper for the `photosynthesis` desktop environment, and the build system (`xtask`) explicitly allows this asset in the ISO image.
    *   *Artifacts*: `assets/wallpapers/linen.bmp`, `userspace/flytrap/src/main.rs`, `xtask/src/image.rs`

*   **Blossom Widgets**: Added `TextInput` and `ListItem` widgets to the standard UI library (`userspace/blossom`), enabling basic form input and list views.
    *   *Artifacts*: `userspace/blossom/src/graph_ui.rs`

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
