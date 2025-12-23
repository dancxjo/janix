# Graph-Mirrored Scheduling

ThingOS uses the kernel graph to mirror its internal scheduling state. While the low-level scheduler uses efficient internal structures (run queues, priority vectors) for dispatching, it publishes all state changes to the graph. This allows userland and debug tools to inspect the scheduler state using standard graph queries.

## Schema
- Things
  - `Thread`: `tid`, `state` (`New`|`Runnable`|`Running`|`Sleeping`|`Blocked`|`Exited`), `priority`, `runtime_ns`, `last_started_ns`
  - `Process`: `pid`
  - `CpuCore`: `index`
  - `SleepEvent`: `wake_at_ns`, `created_at_ns`
- Links
  - `proc.owns_thread`: `Process -> Thread`
  - `sched.runs_on`: `Thread -> CpuCore`
  - `sched.sleeps_until`: `Thread -> SleepEvent`

## Scheduler
- Entry point: `sched::schedule_next()`
- Behavior:
  - Maintains an internal `run_queue` (priority-based round-robin).
  - Handles timer interrupts and context switching imperatively.
  - **Mirrors** state changes to the graph when `graph_enabled` is true:
    - Updates `Thread.state` (Running, Runnable, Sleeping).
    - Updates `sched.runs_on` links.
    - Updates `runtime_ns` stats.

## Observation
- The graph serves as a read-only view of the scheduler for the rest of the system.
- Debug tools can watch for `GraphEvent`s to visualize thread switching.

## Invariants
- A running thread should have exactly one outgoing `sched.runs_on` link.
- Only one `Thread` should target a given `CpuCore` via `sched.runs_on`.
- `Thread.state = Running` implies `last_started_ns` is set and `runtime_ns` is monotonic.
- Preempted threads transition back to `Runnable` and drop their `sched.runs_on` link.

## Lifecycle
```
timer interrupt -> sched::scheduler_tick()
                -> internal run_queue update
                -> context switch (arch-specific)
                -> graph update (mirroring state to Things)
```

The scheduling path is observable by inspecting the graph, which reflects the decisions made by the kernel scheduler.
