# Graph Waitables

Thing-OS now treats graph-originating activity as part of the same wait fabric as ports, task exit, timers, and IRQ readiness.

## Model

There are three public graph-facing handle shapes:

- `GraphWatch`: a persistent stream handle for graph deltas.
- `GraphOpHandle`: a one-shot completion handle for async graph mutations.
- `GraphCondition`: a one-shot condition helper layered on top of a watch plus current-state checks.

Under the hood, watches still use the existing Root watch machinery and async operations still resolve through Root reply cells, but the public `stem` API presents them as graph concepts rather than Root clerk operations.

## Watch Versus Condition

`GraphWatch` and `GraphCondition` are intentionally different.

- A watch is stream semantics. It stays active until closed, becomes readable while unread graph payloads exist, and may report overflow.
- A condition is one-shot semantics. It answers “has this graph fact become true yet?” and disarms after success.

The first implementation of `GraphCondition` is deliberately simple:

- it performs an immediate current-state check when armed
- if the condition is already true, the caller does not need to enter `wait_many`
- otherwise it waits on an underlying graph watch
- when the watch wakes, it drains pending payloads and re-evaluates the condition against current graph state

This keeps condition waits declarative without pretending they are the same thing as event streams.

## Operation Handles

Async graph submission now produces waitable graph operation handles.

- `GraphOpHandle` is valid input to `wait_many` through `WaitKind::GraphOp`
- readiness is reported with `ready::DONE`
- failures are reported as `ready::DONE | ready::ERROR`
- the wait result carries the success value or errno payload

Lifecycle rules are explicit:

- pending handles remain registered until completion or cancellation
- `take_result()` consumes the completion and drops the kernel handle
- dropping or canceling a handle wakes any blocked waiters, which then observe `ENOENT` on the next poll

This makes destruction visible instead of silently sleeping forever.

## Overflow And Backpressure

Graph watches remain persistent and bounded.

- the kernel keeps a bounded commit history
- if a watch falls behind history retention, it reports overflow
- overflow is sticky until the consumer observes it
- condition waits do not queue condition hits; they re-check truth after each wake and after overflow

This means stream consumers can detect loss explicitly, while condition consumers avoid flooding because they only care whether the predicate is true now.

## Current Public API Direction

The long-term API shape should stay graph-native:

- open a graph watch
- submit a graph operation and receive a graph op handle
- arm a graph condition and, if needed, include its wait spec in `wait_many`

The current implementation still maps onto Root syscalls internally, but new userland code does not need to name Root watch opcodes or async reply plumbing directly.
