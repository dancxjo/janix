# Core Ontology v0.2

This document describes the core ontology of ThingOS, defined in `crates/models`.

## ID Ranges

*   **1–99**: Meta-kinds + meta-schemas (Kind, Schema, Link, Intent, Observation, Result)
*   **100–199**: Predicate Kinds + their schemas
*   **200–399**: Core Kinds + their schemas
*   **400–999**: Reserved
*   **1000+**: User/App kinds

## Predicate Kinds (Verbs)

Predicates define the semantics of links between Things.

*   `OWNS` (100): Ownership relationship (e.g., Process owns Thread).
*   `HAS_CAP` (101): Possession of a capability.
*   `HAS_SCHEMA` (102): Links a Kind to its Schema.
*   `MOUNTS` (103): Links a Mount point to a Graph.
*   `BACKED_BY` (104): Links a Graph to a GraphProvider (or implementation detail).

## Core Kinds (Nouns)

Core kinds represent the fundamental system entities.

*   `TimeNow` (200): Represents the current system time.
*   `Process` (201): An execution context (address space + resources). Linking: `OWNS` threads/resources.
*   `Thread` (202): A schedulable unit of execution.
*   `Capability` (203): An unforgeable token of authority.
*   `Graph` (204): A virtual graph node/directory.
*   `Mount` (205): A point where one graph is grafted onto another. Links: `MOUNTS` exactly one Graph.
*   `GraphProvider` (206): A driver or service providing graph content.
*   `Buffer` (207): A block of shared memory.
*   `Stream` (208): A data stream handle.

## Seed Ordering

Deterministic seeding is critical for the kernel to bootstrap the graph without race conditions.

1.  **Meta-Kinds**: `Kind`, `Schema`. (The axioms)
2.  **Meta-Schemas**: Schema for Kind, Schema for Schema.
3.  **Core Meta-Kinds**: `Link`, `Intent`, `Observation`, `Result`.
4.  **Predicate Kinds**: `OWNS`, `HAS_CAP`, etc.
5.  **Core Kinds**: `TimeNow`, `Process`, `Thread`, etc.

## Example Facts

*   `Process(1234) --[OWNS]--> Thread(5678)`
*   `Process(1234) --[HAS_CAP]--> Capability("NetworkAccess")`
*   `Mount("/tmp") --[MOUNTS]--> Graph(TmpFS)`
*   `Kind("Process") --[HAS_SCHEMA]--> Schema("ProcessSchema")`
