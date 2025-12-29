# Symbol Table v0

## Overview

The Symbol Table provides a mechanism to map strings to 64-bit identifiers (`SymbolId`) and back. Ideally, this allows the system to handle strings as efficient integers (`u64`) while preserving their meaningfulness.

## Design

### Identifier
`SymbolId` is a transparent wrapper around `u64`.
It is generated deterministically using the **FNV-1a 64-bit** hash of the UTF-8 definition of the string.

### Tiers
1.  **Builtins**: A small seed list of symbols (e.g., "Process", "Kind") that are essential for the system. These are guaranteed to exist after boot.
2.  **Dynamic**: Symbols interned at runtime by applications or the kernel.

### Collision Policy
Because we use a 64-bit hash, collisions are statistically improbable ("meteor strike").
However, the system enforces a policy:
- If `intern("foo")` generates ID `X`, and `X` is free, it is inserted.
- If `X` is occupied by "foo", it returns verification (OK).
- If `X` is occupied by "bar", it returns a **Collision Error**.

### Persistence
In the **Hosted** environment, the symbol table is persisted to disk (e.g., `target/thingos/symbols.tsym`) to ensure that IDs remain stable across runs.
- **Format**: Simple binary (Magic, Records: `[u64 id, u32 len, bytes]`).
- On boot, the table loads existing symbols.
- Builtins are then seeded (verifying no conflict).
- New interns are appended.
- The table is saved on intern (or periodically/shutdown).

For **Bare Metal**, the table is currently in-memory only (v0.2).

## Application Access
Apps interact with the symbol table via `sys_graph` (the single system facade).
We added `GraphOp` variants:
- `SymbolIntern { text }` -> `SymbolInterned { id }`
- `SymbolResolve { id }` -> `SymbolResolved { text }`

This avoids creating a separate "syscall zoo" for symbols.
