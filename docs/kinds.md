# Kinds and Schemas

In ThingOS, the type system is self-describing.

- **Kinds are Things**: A Kind is a Thing with `kind: THING_KIND_KIND`.
- **Kinds are Schemas**: Every Kind has a `schema_id`. That Schema is also a Thing (schema: `THING_SCHEMA_KIND`, or more accurately, schema: `THING_KIND_SCHEMA`).

## Stable Type Tags

To ensure data portability and deterministic behavior without runtime symbol interning, we use stable TypeTags.

A `TypeTag` is a `u64` hash (FNV-1a 64-bit) of a stable string identifier.

Rule: The string used must be a stable identifier like `"thingos.IntentBody.v1"`.

```rust
pub struct TypeTag(pub u64);
```

## declarative `thing_kind!` Macro

We use a declarative macro `thing_kind!` to define Kinds. This ensures that:
1. All IDs are explicit constants.
2. The Kind and Schema definitions are kept in sync.
3. Stable TypeTags are generated automatically.
4. Seeding functions are generated to bootstrap the graph.

### Syntax

```rust
thing_kind! {
    kind Intent {
        id: 1001,
        sym: SYM_INTENT,
        version: 1,
        body: IntentBody,
        type_tag: "thingos.IntentBody.v1",
        schema_id: 2001,

        links {
            predicate THING_RESULT_KIND min 0 max 1;
            predicate THING_OBSERVATION_KIND min 0 max many;
        }
    }
}
```

This generates:
- `THING_INTENT_KIND` (ThingId)
- `THING_INTENT_SCHEMA` (ThingId)
- `INTENT_TYPE_TAG` (TypeTag)
- `seed_intent_kind()` -> Returns the Kind Thing and Schema Thing.

## Seeding

The kernel seeds the graph using a unified registry:

```rust
models::builtins::builtin_seed_things()
```

This function returns all built-in Kinds and Schemas in a deterministic order.
