# Thing Model Contract

This document defines the canonical "Thing" model, which serves as the heart of ThingOS. It establishes the shared language between the kernel, providers, and userland.

## The Four-Part Definition

A **Thing** is a pure data record consisting of:

1.  **Identity (`ThingId`)**: An immutable, unique 64-bit identifier.
2.  **Kind (`ThingId`)**: A reference to another Thing that acts as its Kind (type).
3.  **Body (`ThingBody`)**: A typed, byte-encoded payload.
4.  **Links**: Edges that connect Things (implemented as Things themselves).

## The Three Axioms

1.  **Edges are Things**: Links are not special database primitives; they are Things with a specific structure (`LinkBody`) and Kind (`LinkKind`).
2.  **Kinds are Things**: Types are not special metadata; they are Things with a specific structure (`KindBody`) and a self-referential Kind.
3.  **Kinds are Schemas**: A Kind-Thing contains a reference to a Schema-Thing, which defines the validation rules for Things of that Kind.

## Serde Contract (The "1:1" Rule)

The system enforces a strict 1:1 mapping between Rust structs and Thing bodies:

*   **Format**: `postcard` (no-std compatible binary serialization).
*   **Contract**: Any Rust struct `T` that implements `Serialize` + `Deserialize` can be stored directly as a `ThingBody`.
*   **Safety**: The kernel treats the body as opaque bytes (`Vec<u8>`). Validation happens via Schema rules, not kernel hardcoding.

### Example

```rust
#[derive(Serialize, Deserialize)]
struct MyData {
    x: u32,
    y: u32,
}

// Encoding
let body = ThingBody::from(&MyData { x: 1, y: 2 })?;

// Decoding
let data: MyData = body.decode()?;
```
