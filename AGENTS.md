# ThingOS — Agent Guidelines

This document defines how agents (human or AI) should work on ThingOS.

An “agent” here means:
* a human contributor,
* a code-generation tool,
* or an autonomous coding assistant.

If you change code in this repository, you are an agent.

## Core Principles

### 1. The Graph Is the System

All system behavior must ultimately be representable as:
* Things
* Links
* Observations
* Reactions

If your change introduces hidden state that cannot be observed or queried, it is probably wrong.

### 2. No Magical Layers

Avoid:
* implicit globals
* invisible side channels
* ad-hoc special cases

If something needs power, give it:
* a Thing
* a capability
* and an explicit place in the graph

### 3. Kernel ≠ Policy

The kernel provides:
* safety
* scheduling
* memory
* graph enforcement

It should not decide:
* UI behavior
* device semantics
* user experience

Push policy into userland.

### 4. Drivers Are Programs

Drivers should:
* live in userland where possible
* communicate via graph operations
* use hardware bridges sparingly

If you’re tempted to “just put it in the kernel,” pause.

## How to Work Safely

### Small, Coherent Changes

Agents should:
* work on one conceptual task at a time
* avoid sweeping refactors unless explicitly asked
* prefer clarity over cleverness

If you can’t explain the change in one paragraph, it’s too big.

### Preserve Architectural Boundaries

Pay attention to directory intent:

* `kernel/`: Core system logic only
* `crates/abi/`: Stable-ish contracts between kernel and userland
* `user/`: Programs, drivers, services
* `arch/`: Architecture-specific glue and bridges

If code “feels homeless,” that’s a smell.

### Respect Existing Shapes

Before introducing:
* a new syscall
* a new graph op
* a new abstraction

Ask:
* “Can this already be expressed using the graph we have?”

The answer is often “yes, but awkwardly.”
That awkwardness is usually instructive.

## For AI Coding Agents (Explicit Rules)

If you are an AI agent generating code:
* Do not invent APIs
* Do not rename subsystems
* Do not delete code you didn’t create
* Do not flatten abstractions for convenience

You may:
* refactor locally with justification
* add comments explaining uncertainty
* leave TODOs instead of guessing

When unsure, stop and ask.

## Commit Discipline

Commits should:
* do one thing
* have descriptive messages
* not mix mechanical cleanup with semantic changes

Bad:
`misc fixes`

Good:
`kernel: make graph link insertion observable`

## Philosophy Check

When in doubt, prefer designs that are:
* inspectable
* reversible
* explicit
* explainable at runtime

ThingOS is not optimized for speed yet.
It is optimized for understanding.

## Final Note

This project values clarity over progress.

A slower system that can explain itself is better than a fast one that cannot.

Build carefully.
