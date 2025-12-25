# Bootstrap Process: Meta-Kinds and Graph Seeding

This document explains how ThingOS bootstraps its self-describing graph on day one.

## Why Seed Meta-Kinds?

ThingOS follows the "Everything is a Thing" philosophy. This includes the definition of "Kind" itself. To avoid circular dependency paradoxes (e.g., "What is the Kind of the Kind Thing?"), we seed a minimal set of canonical Things into the graph before schema enforcement is activated.

## The Bootstrap Sequence

The bootstrap process (`seed_builtins`) runs in a special "Seed Mode":

1.  **Seed Insertion**: The kernel inserts built-in Things into the graph storage, bypassing schema validation.
2.  **Enforcement Activation**: Once seeded, the kernel switches to "Enforcement Mode" (future work), where all new writes must satisfy the schemas defined by the seeded Things.

## Built-in Things

All built-in Things have stable, deterministic `ThingId`s in the reserved range `1..=100`.

### 1. Meta-Kinds

*   **Kind (`THING_KIND_KIND`, ID 1)**: The meta-kind. All Kinds have this as their `kind`. Its own `kind` is itself (1).
*   **Schema (`THING_SCHEMA_KIND`, ID 2)**: The kind for Schema Things.

### 2. Core Kinds

*   **Link (`THING_LINK_KIND`, ID 3)**: The kind for all edges in the graph.
*   **Intent (`THING_INTENT_KIND`, ID 4)**: Represents a request for action.
*   **Observation (`THING_OBSERVATION_KIND`, ID 5)**: Represents a sensor reading or state update.
*   **Result (`THING_RESULT_KIND`, ID 6)**: Represents the outcome of an Intent.

### 3. Built-in Schemas

For every built-in Kind, there is a corresponding Schema Thing (IDs 11..16) that defines its structure.

## Determinism Rule

The seed set is deterministic. The `SymbolId`s for known keys (e.g., "name", "version") are compile-time constants, ensuring that the kernel and userland agree on the binary layout of these fundamental types without needing runtime symbol interning negotiation during early boot.
