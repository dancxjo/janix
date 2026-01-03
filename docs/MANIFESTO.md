# ThingOS Manifesto

A declaration for humans, agents, and future maintainers

## What ThingOS Is

ThingOS is an experimental operating system built on a single, uncompromising idea:

**Everything is a Thing, and the graph is the system.**

ThingOS does not treat files, processes, devices, windows, memory, or time as special cases.
They are all Things.
Their relationships are Links.
Together, they form a typed, queryable, living graph that is the operating system.

There is no hidden state behind the graph.
There is no “real system” elsewhere.
If something exists, it exists as a Thing.

### The Graph Is the Source of Truth

The graph is not a cache.
It is not metadata.
It is not a convenience layer.

**The graph is the system.**

* Hardware devices are Things
* Memory regions are Things
* Drivers are Things
* Tasks are Things
* Windows are Things
* Input events are Things
* Time, logs, errors, and faults are Things

Execution does not escape the graph.
It moves through it.

If a component cannot be represented as a Thing with explicit links, it does not belong in ThingOS.

### No Magical Boundaries

ThingOS rejects traditional OS fault lines:

| Traditional OS | ThingOS |
| :--- | :--- |
| Kernel vs Userland | Privilege is a relationship |
| Files vs Memory | Bytespaces with links |
| Processes | Scheduled Things |
| Devices | Graph-exposed interfaces |
| Syscalls | Graph operations |
| IPC | Shared structure |

There is no “special” place where rules change.
Only different constraints, modeled explicitly.

## Architecture Principles

### 1. Everything Must Be Explicit

**Implicit behavior is a bug.**

If something:
* allocates memory
* touches hardware
* schedules work
* renders pixels
* consumes input

…it must do so through declared Things and Links.

If you can’t point to the graph node that explains why something happened, the design is incomplete.

### 2. Schemas Over Ad-Hoc Structs

Every Thing has:
* a kind
* an associated schema
* a stable identity (UUID / interned symbol)

Schemas are not documentation.
They are contracts enforced by the system and relied upon by agents.

**If you change a schema, you change the system.**

### 3. The Kernel Is Not Special—Just Trusted

The kernel is a graph provider with elevated capabilities.
It is not the “real” OS.
It is infrastructure that mounts and maintains graphs.

Schedulers, memory managers, drivers, and compositors are services, not gods.

Power comes from position in the graph, not from where code lives.

### 4. Memory Is a Place, Not a Side Effect

Memory is modeled as Bytespaces:
* addressable
* linkable
* inspectable
* transferable

Allocation is not a hidden act.
It is a relationship between Things.

If something owns memory, that ownership must be visible.

### 5. Time Is a First-Class Thing

Logs, ticks, frames, events, and histories are not ephemeral.
They are Things in time.

You should be able to:
* query what happened
* replay it
* visualize it
* reason about causality

Debugging is graph traversal.

## What Agents Must Understand

This repository is not a pile of Rust code.
It is a model of a system.

**When acting as an agent working on ThingOS:**

**You Must:**
* Preserve graph integrity above all else
* Prefer declarative structure over procedural shortcuts
* Make relationships visible
* Avoid “temporary hacks” that bypass the graph
* Treat refactors as ontology changes, not mere code motion

**You Must Not:**
* Introduce hidden global state
* Encode meaning in call order
* Add “just this once” special cases
* Assume POSIX semantics
* Recreate filesystems, processes, or syscalls in disguise

If a design smells like UNIX, question it.

## Versioning Philosophy

ThingOS versions do not mean “feature complete.”
They mean **ontologically stable**.

A version bump implies:
* Graph shapes are intentional
* Schemas are defensible
* Names are chosen carefully
* The system can explain itself

If the system cannot describe its own structure using its own primitives, it is not ready.

## Why This Exists

ThingOS exists because:
* Traditional OS abstractions are historical accidents
* Debugging invisible state is intolerable
* Systems should be inspectable while alive
* Software should describe itself
* Tools should reason, not guess

This is not about performance first.
It is about coherence first.

Performance comes later.
Clarity does not.

## The Test of Correctness

A ThingOS feature is correct if:
1. It can be represented in the graph
2. It can be queried and inspected
3. It composes with other Things
4. It does not require out-of-band explanation

If you have to say “well, except for this part,”
the work is unfinished.

## Final Directive

ThingOS is not built by adding features.
It is built by removing exceptions.

Every commit should make the system:
* more legible
* more self-describing
* more honest about what it is doing

If you are unsure what to do next:

**Make the graph tell the truth.**

That is always the correct move.
