# Changelog

## Recent Status

The operating system is seeing major improvements in core stability, memory management, and display reliability. A critical update introduces **kernel log eviction**, a mechanism that manages graph history memory usage by evicting old entries when pressure is high, preventing system-wide memory exhaustion. Performance for the **Unified Device Graph** has been significantly boosted by linking core services—such as the Scheduler, Host, and Boot Modules—directly to `svc.Root` for O(1) discovery, bypassing expensive traversals.

On the visual front, the **early boot display (bud)** now incorporates smarter heuristics for framebuffer stride and BPP detection. This eliminates diagonal shearing artifacts previously observed on certain hardware configurations.

Testing infrastructure has also been expanded with new **multi-tasking BDD scenarios** to verify task isolation and scheduling, alongside comprehensive **ISO9660 unit tests** that validate filesystem probing logic without the need for external images.

## Recent Changes

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

*   **Framebuffer Stride Heuristics**: The `bud` crate and `fb_common` library now correctly interpret framebuffer stride as pixels or bytes based on a BPP heuristic. This fixes diagonal shearing issues observed during early boot on certain display backends.
    *   *Artifacts*: `libs/fb_common/src/lib.rs`, `bud/src/display.rs`

*   **PresentQueue Implementation**: The rendering pipeline has moved from legacy swapchains to a `PresentQueue` model. This improves frame timing and error handling during composition.
    *   *Artifacts*: `userspace/bloom`, `userspace/blossom`

*   **DrawList Demo**: A new `drawlist_demo` application has been added to demonstrate and validate the vector drawing capabilities of the UI graph.
    *   *Artifacts*: `userspace/drawlist_demo/`

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
