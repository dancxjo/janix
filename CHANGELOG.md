# Changelog

## Major Userspace Expansion & Tooling Upgrade

The operating system has undergone a significant architectural expansion, primarily focusing on creating a robust and modular userspace. Key infrastructure components have been introduced to handle graph data management (Phloem), networking (Netd), and service orchestration (Sprout). This shift moves logic out of ad-hoc implementations and into dedicated, queryable services, leveraging the system graph as the central source of truth.

Simultaneously, the developer experience has been upgraded with powerful new tooling in `xtask`. This includes multi-architecture QEMU support (x86_64, ARM64, RISC-V, LoongArch), automated visual verification for UI tests, and comprehensive integration tests for the new graph database. The kernel has also seen targeted optimizations in memory management and interrupt handling to support these higher-level abstractions.

## Detailed Changes

### 🌟 New Services & Core Infrastructure

*   **Phloem (Graph DB)**: A dedicated graph database service implementing an OpenGQL-subset query language. It enables applications to perform complex graph traversals and mutations (e.g., `MATCH`, `MERGE`, `LINKS_TO`) over a TCP socket, moving data logic out of the kernel.
    *   *Artifacts*: `userspace/phloem/`, `xtask/src/test_phloem.rs`

*   **Netd (Network Daemon)**: The networking stack has been consolidated into a unified daemon. It handles DHCP configuration, DNS resolution, and HTTP requests, exposing a clean IPC interface for other applications.
    *   *Artifacts*: `userspace/netd/`, `userspace/nectar/`

*   **Sprout (Service Supervisor)**: A new process supervisor that manages the lifecycle of core userspace services. It handles startup order and dependency management for critical components like `flytrap` (assets) and `blossom` (UI).
    *   *Artifacts*: `userspace/sprout/`

*   **Cambium (Reactive Data Binding)**: Introduces a reactive layer that synchronizes UI properties with graph node state. By defining `BINDING` relationships, UI elements automatically update when underlying data changes, eliminating manual polling.
    *   *Artifacts*: `userspace/cambium/`

*   **Flytrap (Asset Watcher)**: Enhanced to automatically discover and hash assets (like wallpapers and fonts) provided by boot modules. It ensures that content is available in the graph for applications to use.
    *   *Artifacts*: `userspace/flytrap/`

### 🛠️ Developer Tooling & Testing

*   **SysDescribe (LLM System Description)**: A new utility that leverages Large Language Models (LLMs) to generate human-readable descriptions of system resources and graph nodes.
    *   *Artifacts*: `userspace/sysdescribe/`

*   **Scheduler Fairness Test**: A rigorous test suite (`scheduler_fairness`) to verify that the kernel scheduler provides fair CPU time to all runnable threads, detecting starvation scenarios.
    *   *Artifacts*: `userspace/scheduler_fairness/`

*   **Root Watch Tester**: A dedicated tool to validate the `root_watch` syscall and its event delivery mechanism, ensuring that graph mutations (CREATE_NODE, LINK, PROP_SET) are correctly propagated to watchers.
    *   *Artifacts*: `userspace/root_watch_tester/`

*   **Multi-Arch QEMU Support**: The `xtask` build system now supports running QEMU for multiple architectures (`x86_64`, `aarch64`, `riscv64`, `loongarch64`), enabling cross-platform development and testing.
    *   *Artifacts*: `xtask/src/run.rs`

*   **Visual Verification (Scan Tool)**: Replaced legacy Python scripts with a Rust-based image scanner (`xtask scan`) for verifying UI rendering in tests (e.g., counting colored pixels to confirm drawing operations).
    *   *Artifacts*: `xtask/src/scan.rs`

### 🎨 Graphics & UI

*   **Virgl 3D Demo**: A new demo application (`virgl_demo`) showcasing hardware-accelerated 3D graphics using VirtIO GPU. It demonstrates context creation, resource management, and command submission.
    *   *Artifacts*: `userspace/virgl_demo/`

*   **DrawList Demo**: A vector graphics demonstration app verifying the `PaintPipeline`'s ability to render complex shapes and paths.
    *   *Artifacts*: `userspace/drawlist_demo/`, `docs/behavior/features/custom_rendering.feature`

*   **Blossom Widget Library**: Expanded the standard UI library with new widgets like `TextInput` and `ListItem`, enabling form-based applications.
    *   *Artifacts*: `userspace/blossom/src/graph_ui.rs`

*   **PresentQueue Rendering**: Migrated the rendering pipeline to a `PresentQueue` model, improving frame timing and reducing visual artifacts during composition.
    *   *Artifacts*: `userspace/bloom`, `userspace/blossom`

### 🔊 Drivers & Hardware Support

*   **Virtio Sound**: Added a driver for VirtIO sound devices, enabling PCM audio playback and status monitoring.
    *   *Artifacts*: `userspace/virtio_sound/`

*   **Virtio GPU**: Expanded support for VirtIO GPU devices, including the 3D acceleration capabilities demonstrated in `virgl_demo`.
    *   *Artifacts*: `userspace/virtio_gpu/`

*   **Bristle (Input Processing)**: Improved mouse input handling to robustly manage split packets and event normalization.
    *   *Artifacts*: `userspace/bristle/`

### 📱 Applications

*   **Font Explorer**: A utility for browsing and inspecting installed system fonts.
    *   *Artifacts*: `userspace/font_explorer/`

*   **Fortune (LLM Demo)**: A demo application showcasing LLM integration for generating text.
    *   *Artifacts*: `userspace/fortune/`

### ⚙️ Kernel & System Internals

*   **Query Buffer Recycling**: Implemented `QueryScratch` in the kernel to reuse memory buffers during graph queries, significantly reducing heap allocations in the hot path.
    *   *Artifacts*: `kernel/src/root/query.rs`

*   **Watch API Optimization**: The `handle_watch_next` handler now uses a scan limit and hybrid allocation strategy to maintain responsiveness under heavy load.
    *   *Artifacts*: `kernel/src/root/handlers/watch.rs`

*   **Root Registry Links**: Core system nodes (scheduler, host) are now directly linked from the root, allowing O(1) access and bypassing expensive traversals.
    *   *Artifacts*: `kernel/src/root/boot_register.rs`
