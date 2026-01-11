2025-01-11 – [Optimization: Direct Graph Lookup]
Learning: Replacing manual iteration of `relationships_from` with `relationships_by_kind` reduces lock contention from O(N) to O(1).
Guardrail: When optimizing graph traversals, verify whether the target is the relationship ID or the target Thing ID. `relationships_by_kind` returns targets, which is non-obvious from the name but convenient.
