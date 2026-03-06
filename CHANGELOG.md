# Changelog

## Developer Experience, Graph Database Optimization, and System Reliability

This update brings significant improvements to the developer workflow with the introduction of new BDD tests and GQL validation steps, ensuring a smoother experience when interacting with the system graph via OpenGQL. Concurrently, the Phloem graph database has seen critical bug fixes resolving confusion between inline node properties and internal node IDs during MATCH operations. We have also enhanced the stability of the system's display pipeline by introducing a seamless fallback to a "fake" display when no hardware graphics are detected, preventing startup failures in headless environments. Lastly, this release introduces a new "Daily Inspiration" feature utilizing the Fortune application, further expanding the userspace ecosystem.

### 🛠️ Developer Workflow & Tooling

*   **Developer Workflow BDD Tests**: A new comprehensive test suite (`developer_workflow.feature`) has been added to document and validate the process of inspecting the system graph via GQL, ensuring tools like `anther` properly expose core nodes such as `proc.Task`, `dev.Cpu`, and `svc.Root`.
    *   *Artifacts*: `docs/behavior/features/developer_workflow.feature`, `tools/bdd/src/steps.rs`
*   **Toolchain Synchronization**: Pinned the rust-toolchain to `nightly-2026-02-10` to guarantee compatibility with vendored Rust source code, resolving `#[rustc_const_unstable]` errors during `build-std` operations.
    *   *Artifacts*: `rust-toolchain.toml`

### 🌿 Phloem (Graph DB) Optimization and Reliability

*   **MATCH Optimization Fix**: Resolved a critical bug where the `GraphExecutor` confused a user-defined property named `id` with the internal Node ID during edge `MATCH` optimizations. The optimization now correctly relies on explicit `WHERE id(n) = value` predicates, ensuring accurate query results.
    *   *Artifacts*: `userspace/phloem/src/executor.rs`, `userspace/phloem/src/query_tests.rs`
*   **MERGE with Parameters**: Enhanced test coverage for `MERGE` operations to verify that property maps correctly handle parameterized variables, ensuring secure and dynamic queries are accurately executed and persisted.
    *   *Artifacts*: `userspace/phloem/src/query_tests.rs`

### 🎨 System Graphics Reliability

*   **Headless Display Fallback**: Improved the `sprout` supervisor's display pipeline to automatically fall back to `display_fake` if primary display drivers (`display_bootfb`, `display_virtio_gpu`) fail to initialize. This prevents the compositor (`bloom`) from crashing or silently failing in headless setups.
    *   *Artifacts*: `userspace/sprout/src/pipelines.rs`, `userspace/bloom/src/main.rs`

### ✨ New Applications and Features

*   **Daily Inspiration & Fortune App**: Introduced a new "Daily Inspiration" feature accompanied by the `fortune` application. This application is now integrated into the standard image and supervised by `sprout`, demonstrating continued expansion of userspace utilities.
    *   *Artifacts*: `docs/behavior/features/daily_inspiration.feature`, `userspace/sprout/src/supervisor.rs`, `xtask/src/image.rs`


## Unified Event Loop & Graph Integration Refinement

This update further refines the scheduler modernization by introducing a dedicated `ring_drain_task` and centralizing event processing. The kernel now strictly separates scheduler events (emitted lock-free) from graph updates, which are processed asynchronously by dedicated worker threads. This change significantly reduces scheduler latency and improves system responsiveness under heavy graph load. Additionally, extensive profiling has been added to the graph integration layer to identify bottlenecks. Userspace input handling has also been robustified with a new keyboard state engine.

### 🔄 Centralized Event Processing

*   **Dedicated Ring Drain Task**: Introduced a new kernel task (`ring_drain_task`) that continuously drains scheduler event rings and translates them into graph work items. This offloads the scheduler tick and ensures event processing does not block critical paths.
    *   *Artifacts*: `kernel/src/task/graph.rs`, `kernel/src/task/flusher.rs`

*   **Graph Profiling**: Added comprehensive profiling metrics for the graph integration layer, tracking lock contention, wait times, queue depths, and processing latency. These metrics are logged periodically to assist in performance tuning.
    *   *Artifacts*: `kernel/src/task/graph.rs`

### ⌨️ Input Handling (Bristle)

*   **Thigmonasty (Keyboard State Engine)**: Implemented a new keyboard state machine ("Thigmonasty") that robustly tracks key presses, modifiers (Shift, Ctrl, Alt), and repeat events. This ensures reliable input handling even during rapid typing or complex key combinations.
    *   *Artifacts*: `userspace/bristle/src/thigmonasty.rs`

### 🏗️ Userspace Maturation

*   **General Improvements**: Significant updates across core userspace applications including `clock`, `taskman`, and `sprout` to improve stability and responsiveness, aligning with the new event loop architecture.

## Process Ownership & Graph Database Improvements

This update introduces significant improvements to system stability and data management. A new "Process Ownership" mechanism in the kernel ensures that graph nodes created by a process are automatically cleaned up when that process exits, preventing resource leaks. The Phloem graph database has also been enhanced with `ORDER BY` support, improved tokenizer handling for escape sequences, and expanded test coverage. Additionally, the ISO 9660 filesystem implementation has received fixes for filename handling.

### 🧹 Process Ownership & Graph Hygiene

*   **Automatic Resource Cleanup**: The kernel now tracks the "owner" of each graph node. When a process (task) terminates or is killed, the system automatically identifies and removes all graph nodes owned by that task. This prevents "zombie" nodes from cluttering the system graph and ensures resources are released reliably.
    *   *Artifacts*: `kernel/src/root/handlers/graph.rs`, `kernel/src/task/scheduler/mod.rs`

### 🌿 Phloem (Graph DB) Enhancements

*   **ORDER BY Support**: GQL queries now support the `ORDER BY` clause, allowing results to be sorted by node ID. This is critical for deterministic query results and pagination.
    *   *Artifacts*: `userspace/phloem/src/gql.rs`, `userspace/phloem/src/planner.rs`

*   **Tokenizer Improvements**: The GQL tokenizer now correctly handles standard escape sequences (e.g., `\n`, `\t`, `\"`) in string literals, enabling more robust data insertion and querying.
    *   *Artifacts*: `userspace/phloem/src/gql.rs`

### 💾 Storage & Filesystems

*   **ISO 9660 Fixes**: Resolved issues with filename stripping in the ISO 9660 parser, ensuring correct file access for names with version suffixes.
    *   *Artifacts*: `userspace/iso9660/`

### 🧪 Testing & Verification

*   **System Exploration Scenarios**: New BDD scenarios have been added to verify the presence and state of critical system components (e.g., CPU, Task, Service roots) using GQL queries.
    *   *Artifacts*: `docs/behavior/features/system_exploration.feature`

## System Services Unification & Hardware Abstraction

Recent development has focused on unifying userspace services and abstracting hardware drivers into dedicated daemons. The most significant addition is **Anther**, a comprehensive HTTP server and AI gateway that acts as the primary interface for system interaction and graph data access. This release also introduces **Fontd**, a font rasterization service, and further decouples network and display drivers for better system modularity.

### 🌟 Unified System Interface (Anther)

*   **Anther (AI Gateway & Web Server)**: A new flagship service that integrates HTTP serving, Graph DB queries (`Phloem`), and LLM capabilities (`Ollama`). It powers the system UI, offering real-time updates via Server-Sent Events (SSE) and exposing a rich API for application development.
    *   *Artifacts*: `userspace/anther/`

### 🎨 Font Rasterization

*   **Fontd (Font Service)**: A dedicated font rasterization daemon using `fontdue`. By centralizing font processing, it ensures consistent text rendering across applications and reduces the complexity of individual clients.
    *   *Artifacts*: `userspace/fontd/`

### ⚙️ Driver Decoupling & Hardware Support

*   **VirtIO Netd**: Decoupled the VirtIO network driver from the main networking stack, improving stability and paving the way for supporting multiple network interfaces.
    *   *Artifacts*: `userspace/virtio_netd/`

*   **Display Drivers**: Introduced `display_bootfb` for boot-time framebuffer support and `display_virtio_gpu` for accelerated graphics, ensuring a seamless visual experience from startup.
    *   *Artifacts*: `userspace/display_bootfb/`

*   **RTC CMOS**: Added a driver for the Real-Time Clock (RTC), essential for accurate system timekeeping and scheduling.
    *   *Artifacts*: `userspace/rtc_cmos/`

### 🛠️ Utilities

*   **Beeper**: A system utility for generating audio tones and chimes, utilizing both PC speaker emulation and HDA audio paths.
    *   *Artifacts*: `userspace/beeper/`

*   **ISO Tools**: New command-line tools `iso_cat` and `iso_reader` for inspecting and extracting content from ISO 9660 images directly from userspace.
    *   *Artifacts*: `userspace/iso_cat/`, `userspace/iso_reader/`

### 🔧 Core Enhancements

*   **Thread Spawning**: Enhanced `stem::thread::spawn` implementation to robustly handle worker thread creation, critical for the concurrency requirements of `anther` and other multi-threaded services.

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

## Scheduler Modernization & Graph Integrity

This update represents a major refactoring of the kernel scheduler, moving from a monolithic implementation to a modular, event-driven architecture. Key improvements include dynamic priority aging to prevent starvation, lock-free ring buffers for high-performance tracing, and a centralized run queue management system. These changes significantly improve system responsiveness under load and provide granular visibility into scheduling decisions. Concurrently, the Phloem graph database has been hardened with stricter test verification and fixes for `MERGE`, `COUNT`, and `WHERE` clauses, ensuring absolute data integrity for system state. New BDD scenarios (System Tour, Graph Exploration) have been added to validate these complex behaviors end-to-end.

### 🚀 Scheduler Overhaul

*   **Modular Architecture**: The scheduler has been decomposed into dedicated modules (`kernel/src/sched/`), separating concerns like sleep queues (`sleep.rs`), task spawning (`spawn.rs`), and event tracing (`events.rs`).
    *   *Artifacts*: `kernel/src/sched/`

*   **Dynamic Priority Aging**: Implemented a timestamp-based aging mechanism that dynamically calculates effective priority, eliminating manual aging passes and ensuring fair CPU time distribution.
    *   *Artifacts*: `kernel/src/sched/state.rs`

*   **Lock-Free Tracing**: Introduced a lock-free ring buffer for scheduler events, enabling low-overhead diagnostics without impacting system performance.
    *   *Artifacts*: `kernel/src/sched/events.rs`

*   **Sleep Queue Optimization**: Replaced linear scans with a `BTreeMap`-based sleep queue for O(log n) insertions and wakeups.
    *   *Artifacts*: `kernel/src/sched/sleep.rs`

### 🌿 Phloem (Graph DB) Hardening

*   **Strict Query Verification**: Enhanced test infrastructure to strictly verify query results, including exact row counts and node properties, preventing "false positive" successes.
    *   *Artifacts*: `userspace/phloem/src/query_tests.rs`

*   **Complex Query Support**: Fixed and validated `MERGE` operations with inline edge creation, and added support for `COUNT` aggregates within `WHERE` clauses.
    *   *Artifacts*: `userspace/phloem/src/planner.rs`

### 🧪 System Verification

*   **System Tour**: A comprehensive BDD scenario that validates the user's initial interaction with the system, ensuring critical UI elements and services are responsive.
    *   *Artifacts*: `docs/behavior/features/system_tour.feature`

*   **Graph Exploration**: New scenarios that verify the ability to traverse complex graph structures, ensuring the query engine correctly interprets deep relationships.
    *   *Artifacts*: `docs/behavior/features/graph_exploration_journey.feature`
