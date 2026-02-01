# Kernel

The **Kernel** (`/kernel`) is the core logic of ThingOS. It is architecture-agnostic, relying on **Bran** for hardware abstraction during boot.

## Role

The kernel's primary responsibility is to host the **Root** service (the graph database) and provide the environment for userspace processes (like **Sprout**) to run.

Unlike traditional monolithic kernels, ThingOS kernel is relatively minimal. It provides:

*   **Memory Management**: Paging, allocation (building on Bran's memory map).
*   **Task Scheduling**: Preemptive multitasking for kernel threads and userspace processes.
*   **IPC**: Inter-Process Communication, primarily via the Graph (Root) and direct messaging.
*   **Root Hosting**: It runs the Root service as a high-priority kernel thread.

## Relationship with Bran

The kernel receives a `BootRuntime` trait object from Bran at entry. It uses this to interact with the hardware (console, memory map, etc.) without knowing the underlying architecture details.

```rust
pub fn start<R: BootRuntime>(runtime: &'static R) -> ! {
    // ...
}
```

## Startup Sequence

1.  **Entry**: Bran calls `kernel::start(runtime)`.
2.  **Init**: Kernel initializes memory, logging, and tasking.
3.  **Root Spawn**: The kernel spawns the `Root` service thread (`crate::root::init_root_service`).
4.  **Registration**: The kernel registers itself and the hardware inventory into the Root graph.
5.  **Sprout Load**: The kernel looks for the `sprout` boot module (the init process).
6.  **Userland Launch**: The kernel creates a user address space and spawns `sprout`.
7.  **Idle Loop**: The main kernel thread enters the scheduler/idle loop.

## Key Subsystems

*   `root/`: The implementation of the graph database.
*   `task/`: Scheduler and process management.
*   `memory/`: Virtual and physical memory management.
*   `syscall/`: System call handlers (interface for **Stem**).
