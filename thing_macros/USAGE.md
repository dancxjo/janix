# Thing Macro Usage

The `Thing` derive macro requires that you provide a description using the `#[thing(description = "...")]` attribute.

## ✅ Correct Usage

```rust
#[derive(Thing)]
#[thing(description = "A counter that tracks the number of operations")]
pub struct Counter {
    pub count: u64,
}
```

## ❌ Incorrect Usage (Will Not Compile)

```rust
#[derive(Thing)]  // Error: Thing derive requires a #[thing(description = "...")] attribute
pub struct Counter {
    pub count: u64,
}
```

## Why This is Required

Descriptions are mandatory to ensure that all Thing types have meaningful documentation. This helps with:

- Self-documenting code
- Graph database introspection
- Runtime schema information
- Developer understanding

## Compile-Time Enforcement

The macro will emit a compile error if the description attribute is missing:

```
error: Thing derive requires a #[thing(description = "...")] attribute
 --> src/lib.rs:8:10
  |
8 | #[derive(Thing)]
  |          ^^^^^
```

This compile-time check prevents the silent use of placeholder text like "No description provided".
