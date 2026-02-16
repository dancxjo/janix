# Changelog

## SVG Graphics, Concurrency & Networking

This update brings native SVG rendering capabilities, significant improvements to service concurrency, and a more robust network stack.

### 🎨 SVG & Vector Graphics

*   **Native SVG Support**: The `bloom` compositor now supports parsing and rendering SVG content directly. This is powered by a new lightweight XML parser and CSS-subset engine in the `stem` library, enabling scalable vector graphics for UI elements and icons.
    *   *Artifacts*: `userspace/bloom/src/svg/`, `stem/src/xml/`

### ⚡ Concurrency & Stability

*   **Service Threading Model**: The `anther` service (and others) has transitioned to using `stem::thread::spawn` with a trampoline mechanism. This resolves previous instability with standard library threading on the platform, ensuring reliable concurrent handling of HTTP and UI requests.
    *   *Artifacts*: `userspace/anther/src/main.rs`

### 🌐 Networking

*   **Network Stack Robustness**: The `netd` service has been enhanced with improved connection lifecycle management and detailed logging. This includes better handling of TCP states and driver interaction, making network services more resilient.
    *   *Artifacts*: `userspace/netd/`

## System Monitoring, Storage & WASM Drivers

Recent updates have introduced essential system utilities and laid the groundwork for sandboxed drivers.

### 📊 System Monitoring & Desktop Widgets

*   **Taskman**: A reactive task manager that visualizes the system's process tree. It provides real-time updates on task states (Running, Sleeping, Blocked), CPU affinity, and priority, leveraging `root_watch` to reflect graph changes instantly without polling.
    *   *Artifacts*: `userspace/taskman/`

*   **Fetchd**: A desktop widget that monitors network connectivity, displaying the system's IP address in the bottom-left corner. It acts as a visual status indicator for the network stack.
    *   *Artifacts*: `userspace/fetchd/`

*   **Clock**: A digital clock widget that displays the current system time, supporting timezones and locale configuration via `locale.conf`.
    *   *Artifacts*: `userspace/clock/`

*   **Photosynthesis**: A force-directed graph layout engine. It calculates optimal positions for nodes in a 2D space, designed to visualize the complex relationships within the system graph.
    *   *Artifacts*: `userspace/photosynthesis/`

### 💾 Storage & Filesystems

*   **ISO 9660 Parser**: A robust, allocation-free ISO9660 filesystem implementation. It supports Level 1/2 interchange and Rock Ridge extensions, enabling the system to read assets and configuration directly from boot media.
    *   *Artifacts*: `userspace/iso9660/`, `userspace/iso9660d/`

*   **Storage Drivers**: New drivers for AHCI and ATA controllers, along with a disk probing utility, expanding hardware support for storage devices.
    *   *Artifacts*: `userspace/ahci_disk/`, `userspace/ata_disk/`, `userspace/disk_probe/`

### 🛡️ Sandboxed Drivers (WASM)

*   **WASM Driver Host**: A runtime environment for executing drivers compiled to WebAssembly. This experimental host uses `wasmi` to sandbox drivers, with support for recording and replaying device interactions for debugging and regression testing.
    *   *Artifacts*: `userspace/driver_wasm_host/`

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
