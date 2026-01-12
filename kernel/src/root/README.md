# Root

Root is the graph core of ThingOS.

It is the first subsystem where the operating system stops being a collection of mechanisms and starts being a coherent, inspectable system.

If Bran answers “what hardware did we wake up on?”
and the kernel answers “how do we run code safely?”
then Root answers:

“What exists, and how is it related?”

## What Root Is

Root is a kernel-resident graph engine backed by an append-only journal.

It provides:

- A unified representation of everything the system knows
- A single source of truth for system state
- A watch-driven mechanism for reacting to change
- A minimal, explicit syscall surface for interaction

Root is not a service layered on top of the kernel.
Root is part of the kernel’s structural spine.

## The Core Idea

At its heart, Root is built on one idea:

**Everything is a Thing.**

Devices, processes, threads, memory regions, drivers, clocks, windows, files, capabilities — all of them are represented as nodes in a graph, connected by typed edges, evolving over time via a journal.

There is no “special case” state.

If it matters, it belongs in the graph.
If it changes, it goes in the journal.

## The Journal

Root maintains an append-only journal.

Every mutation of the graph is recorded as a journal entry:

- Node creation
- Edge creation or deletion
- Property updates
- Capability grants or revocations

The journal is:

- Authoritative — the graph is derived from it
- Ordered — time matters
- Durable — nothing is silently overwritten
- Auditable — history is not lost

The current graph is a projection of the journal, not the other way around.

This makes Root:

- Debuggable
- Replayable
- Inspectable
- Future-proof

## The Graph

The Root graph is:

- Typed (nodes and edges have kinds)
- Identified (stable IDs, not pointers)
- Explicit (no hidden relationships)
- Live (changes propagate through watches)

The graph is not a filesystem.
It is not an object hierarchy.
It is not an in-memory database clone.

It is a system model.

## Watches

Root introduces watches: declarative subscriptions to graph changes.

A watch says:

“When this kind of change happens here, notify me.”

Watches can trigger:

- Kernel behavior
- Driver behavior
- Userspace notifications
- Deferred work
- Reactive system wiring

This is how ThingOS avoids polling, special hooks, and ad-hoc callbacks.

Root does not do the work — it signals that something changed.

## Root’s Responsibilities

Root is responsible for:

### 🌳 Graph State

- Creating, deleting, and updating Things
- Maintaining relationships
- Enforcing basic graph invariants

### 📜 Journaling

- Recording all mutations
- Providing replay and inspection
- Ensuring atomicity of graph updates

### 👁️ Watches

- Registering interest in changes
- Delivering notifications reliably
- Keeping the system reactive, not procedural

### 🔐 Minimal Authority

- Capability attachment
- Ownership relationships
- Structural permissions (not policy)

Root provides structure, not decisions.

## What Root Explicitly Does Not Do

Root is intentionally constrained.

It does not:

- Schedule threads
- Render UI
- Load drivers directly
- Allocate memory beyond its needs
- Implement high-level policy
- Interpret meaning beyond structure

Root does not “understand” clocks, drivers, or windows.

It merely represents them.

## Root and Syscalls

Root is the first serious syscall surface in ThingOS.

Instead of dozens of unrelated syscalls, Root exposes a small set of graph operations, such as:

- Create a Thing
- Link two Things
- Update a property
- Watch for changes
- Query structure

Everything else builds on top of that.

This is where ThingOS stops being “a kernel with features” and becomes a system with an internal language.

## Relationship to Other Layers

Root sits at the center of ThingOS:

```
Firmware
  ↓
Bootloader
  ↓
Bran
  ↓
Kernel
  ↓
Root  ←── the system becomes self-describing here
  ↓
Sprout   (drivers, early services)
  ↓
Bloom    (UI, compositor)
  ↓
Leaves   (applications)
```

Above Root, nothing should need to invent its own global state.
Below Root, nothing should pretend to understand meaning.

## Design Principles

Root is guided by a few hard rules:

### 🌱 Explicit Over Clever

If a relationship matters, it must be written down.

### 📜 History Over Mutation

You don’t change the system — you append to its story.

### 🧩 Structure Over Policy

Root describes what is, not what should be done.

### 🔍 Inspectability

If the system can’t explain itself, it’s lying.

## In Short

Root is:

- The system’s memory
- The system’s structure
- The system’s nervous system

Or, more simply:

**Root is where ThingOS becomes honest about itself.**
