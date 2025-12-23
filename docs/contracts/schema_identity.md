# Schema Identity

Schemas in ThingOS are identified by a deterministic fingerprint, calculated from their canonical definition.

## Fingerprint Calculation

`Fingerprint = Hash(Canonical(SchemaDef))`

Where `Canonical(SchemaDef)` ensures:
1. Properties are sorted by name.
2. Indexed properties are sorted by name.
3. Property types are encoded consistently.

The fingerprint is currently a 64-bit hash (FNV-1a or similar stable hash of the Postcard-encoded canonical definition).

## Idempotency and Conflict

- **Idempotency**: Declaring a schema with the same name and same fingerprint multiple times is a no-op (success).
- **Conflict**: Declaring a schema with the same name but different fingerprint results in a `Conflict` error. This ensures ontology consistency.

## Namespace Policy

See `namespaces.md` for ownership rules.
