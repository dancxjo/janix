# Links and Predicates

ThingOS represents the system as a directed graph where "edges" are first-class Things called **Links**.

## Link Things

A Link is a Thing with:
*   **Kind**: `THING_LINK_KIND` (System builtin)
*   **Body**: `LinkBody { from: ThingId, to: ThingId, predicate: ThingId }`

Since Links are Things, they have their own `ThingId` and can be the subject of other Links (e.g., an Assertion linking to a Link).

### Creating Links

Use the `link_thing!` macro to create link Things safely and ergonomically:

```rust
let my_link = link_thing!(
    id: ThingId(500),
    from: process_id,
    to: thread_id,
    pred: THING_OWNS_KIND
);
```

## Predicate Kinds

The `predicate` field of a Link describes the *meaning* of the relationship. Predicates are Kinds.

Defined predicates include:
*   `OWNS`: Ownership/Lifecycle dependency.
*   `HAS_CAP`: Authority possession.
*   `BACKED_BY`: Implementation detail or backing store.

### Declaring Predicates

Predicates are declared using the `predicate_kind!` macro:

```rust
predicate_kind! {
    kind Owns {
        id: THING_OWNS_KIND,
        sym: SYM_OWNS,
        version: 1,
        schema_id: THING_OWNS_SCHEMA
    }
}
```

This ensures all predicates share a consistent structure (`PredicateBody`) and are properly registered in the system.
