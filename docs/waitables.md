# Waitables

Thing-OS exposes a unified wait fabric for all asynchronous readiness events: file
descriptors, ports, task exit, timers, and IRQ delivery. This document describes
the waitable handle shapes and their semantics.

## Model

There are three public waitable handle shapes:

- `FsWatch`: a persistent stream handle for filesystem/device change notifications.
- `OpHandle`: a one-shot completion handle for async operations (e.g., async writes).
- `ReadyCondition`: a one-shot condition helper that arms against a watch and
  re-checks state on each wake.

Internally these map onto the kernel's poll/waiter infrastructure, but userland code
does not need to name internal opcodes or reply-cell plumbing directly.

## Watch Versus Condition

`FsWatch` and `ReadyCondition` are intentionally different.

- A watch is stream semantics. It stays active until closed, becomes readable while
  unread event payloads exist, and may report overflow.
- A condition is one-shot semantics. It answers "has this resource become ready yet?"
  and disarms after success.

The first implementation of `ReadyCondition` is deliberately simple:

- it performs an immediate readiness check when armed
- if the condition is already satisfied, the caller does not need to enter `wait_many`
- otherwise it waits on an underlying `FsWatch`
- when the watch wakes, it drains pending payloads and re-evaluates the condition
  against current resource state

This keeps condition waits declarative without pretending they are the same thing as
event streams.

## Operation Handles

Async operations produce waitable completion handles.

- `OpHandle` is valid input to `wait_many` through `WaitKind::Op`
- readiness is reported with `ready::DONE`
- failures are reported with `ready::DONE | ready::ERROR`
- the wait result carries the success value or errno payload

Lifecycle rules are explicit:

- pending handles remain registered until completion or cancellation
- `take_result()` consumes the completion and drops the kernel handle
- dropping or canceling a handle wakes any blocked waiters, which then observe
  `ENOENT` on the next poll

This makes destruction visible instead of silently sleeping forever.

## Overflow And Backpressure

File watches are persistent and bounded.

- the kernel keeps a bounded event history per watch
- if a watch falls behind the retained history, it reports overflow
- overflow is sticky until the consumer observes it
- condition waits do not queue condition hits; they re-check readiness after each
  wake and after overflow

This means stream consumers can detect loss explicitly, while condition consumers
avoid flooding because they only care whether the resource is ready now.

## Public API Direction

The long-term API shape is filesystem/device-native:

- open a file watch on a path or file descriptor
- submit an async operation and receive an `OpHandle`
- arm a `ReadyCondition` and, if needed, include its wait spec in `wait_many`

New userland code should use these VFS-oriented handle types. Do not introduce
new APIs that reference graph watches, graph ops, or graph-era naming.
