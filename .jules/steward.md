2024-10-24 – Repeated Graph Lookups in Hot Paths
Learning: `store::find_thing_by_name` is O(log N) but still too expensive for high-frequency operations like logging, especially in interrupt contexts where locking contention matters.
Guardrail: Cache stable ThingIds in `static mut` (or `AtomicU64`) when they are accessed in hot paths. Initialize them lazily or during a dedicated seed phase.
