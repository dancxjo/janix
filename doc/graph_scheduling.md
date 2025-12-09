# Graph-Driven Scheduling

ThingOS now treats the kernel graph as the source of truth for scheduling. Threads, CPUs, and sleep events are modeled as Things with well-known properties and edges, and mutations emit push-style events that hardware reifiers subscribe to.

## Schema
- Nodes
  - `Thread`: `tid`, `state` (`New`|`Runnable`|`Running`|`Sleeping`|`Blocked`|`Exited`), `priority`, `runtime_ns`, `last_started_ns`
  - `Process`: `pid`
  - `CpuCore`: `index`
  - `SleepEvent`: `wake_at_ns`, `created_at_ns`
- Edges
  - `proc.owns_thread`: `Process -> Thread`
  - `sched.runs_on`: `Thread -> CpuCore`
  - `sched.sleeps_until`: `Thread -> SleepEvent`

## Scheduler
- Entry point: `sched_graph::sched_tick(graph: &mut Graph, cpu: CpuId, now: TimeNs)`
- Behavior:
  - Finds the current thread via `sched.runs_on` edges, accounts runtime, and preempts when the slice expires.
  - Picks the next runnable thread (by priority, then lowest runtime) from the graph.
  - Writes decisions back into the graph (`state`, `last_started_ns`, `sched.runs_on`).
  - Never calls arch/hardware APIs directly.

## Events and Reifiers
- `GraphEvent` is emitted for node/prop/edge changes via fixed listener tables.
- Boot/arch registers listeners (e.g., for `sched.runs_on`) that:
  - Map `CpuCore.index` to a CPU slot.
  - Track the currently running thread per CPU.
  - Invoke platform context-switch hooks when the desired thread changes (placeholder logging today).
- Sleep events use `sched.sleeps_until` edges; helpers are provided to create/clear `SleepEvent` Things.

## Invariants
- A running thread should have exactly one outgoing `sched.runs_on` edge.
- Only one `Thread` should target a given `CpuCore` via `sched.runs_on`.
- `Thread.state = Running` implies `last_started_ns` is set and `runtime_ns` is monotonic.
- Preempted threads transition back to `Runnable` and drop their `sched.runs_on` edge.

## Lifecycle
```
timer interrupt -> sched_tick(graph, cpu, now)
                -> graph mutations (state/edges)
                -> GraphEvent::EdgeAdded("sched.runs_on")
                -> arch reifier compares desired vs current and switches contexts
```

The entire scheduling path is observable by inspecting the graph, and mutations are pushed to interested subsystems immediately.***
