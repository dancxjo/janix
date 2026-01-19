//! Graphable Trait and Standard Payloads.
//!
//! This module defines the `Graphable` trait which binds a Rust type to a
//! language-neutral Schema and WireSafe layout. It also includes the canonical
//! `Edge` and example `Window` payloads.

use crate::errors::{Error, Result};
use crate::packed::{decode_schema, encode_schema};
use crate::wire::{BlobId, KindId, PredicateId, ThingId, WireSafe};
use crate::wire_schema::{schema_hash, Schema};
use core::mem::size_of;

/// A type that can be stored as a Node in the Graph.
///
/// Implementing this implies:
/// 1. The type is `WireSafe` (no pointers, packed layout).
/// 2. The type has a static `SCHEMA` that describes it.
/// 3. The type's `KIND` is derived from the hash of the SCHEMA.
pub trait Graphable: WireSafe + Sized {
    const SCHEMA: Schema;

    // Explicitly declaring the expected wire size helps automated checks.
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn kind() -> KindId {
        schema_hash(&Self::SCHEMA)
    }

    /// Encode self to packed LE bytes using the Schema.
    fn encode(&self, out: &mut [u8]) -> Result<usize> {
        // Safety: We trust that Self is layout-compatible with bound Schema if used correctly.
        // WireSafe ensures it's POD.
        unsafe { encode_schema(self as *const Self as *const u8, &Self::SCHEMA, out) }
    }

    /// Decode from packed LE bytes using the Schema with validation.
    fn decode(bytes: &[u8]) -> Result<Self> {
        let mut val = core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            decode_schema(bytes, &Self::SCHEMA, val.as_mut_ptr() as *mut u8)?;
            let v = val.assume_init();
            v.validate()?;
            Ok(v)
        }
    }

    /// Validate the decoded payload.
    ///
    /// Default implementation checks that the size matches the schema definition.
    /// Overriding implementations can check value ranges, flags validity, etc.
    fn validate(&self) -> Result<()> {
        if size_of::<Self>() != Self::SCHEMA.size() {
            return Err(Error::InvalidDataLength {
                expected: Self::SCHEMA.size(),
                actual: size_of::<Self>(),
            });
        }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Edge Payload
// -----------------------------------------------------------------------------

#[repr(C, packed)]
#[derive(crate::Graphable, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: ThingId,
    pub predicate: PredicateId,
    pub to: ThingId,
    pub flags: u32,
}

// -----------------------------------------------------------------------------
// Window Payload (Example)
// -----------------------------------------------------------------------------

#[repr(C, packed)]
#[derive(crate::Graphable, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Window {
    pub title: BlobId,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

// Note: Automatic derive provides default `validate`.
// If we want custom validation for Window (e.g. w > 0), we need to handle that.
// The macro generates `impl Graphable for Window { const SCHEMA = ...; }`.
// Currently the macro logic doesn't allow overriding `validate` easily unless we make it partial impl or use specialization.
// Or we can manually impl Graphable if we need custom validation.
// For now, let's accept default validation as per "makes Graphable impl automatic" goal.
// If we strictly need validation, we'd need to extend the macro to call a hook or allow manual impl.
// The user prompt said: "impl Graphable for T { const SCHEMA: Schema = ...; }"
// So it seems they want it automatic. I'll stick to automatic for now.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thing::Thing;
    use crate::wire::assert_wire_safe;
    use crate::{edge, thing}; // Test macros
    use core::mem::size_of;

    #[test]
    fn test_wire_safe_markers() {
        assert_wire_safe::<Edge>();
        assert_wire_safe::<Window>();
        assert_wire_safe::<ThingId>();
    }

    #[test]
    fn test_window_payload_roundtrip() {
        let original = Window {
            title: BlobId([0xAB; 16]),
            x: -100,
            y: 200,
            w: 1920,
            h: 1080,
        };

        let mut buf = [0u8; 128]; // ample space
        let written = original.encode(&mut buf).expect("Encode failed");
        assert_eq!(written, size_of::<Window>());

        let decoded = Window::decode(&buf[..written]).expect("Decode failed");
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_thing_macro() {
        let t: Thing<Window> = thing!(Window {
            title: BlobId([1; 16]),
            x: 10,
            y: 20,
            w: 100,
            h: 100,
        });
        let x_val = t.value.x;
        assert_eq!(x_val, 10);
        let k = t.kind;
        assert_eq!(k, Window::kind());
    }

    #[test]
    fn test_edge_macro() {
        let from = ThingId([1; 16]);
        let to = ThingId([2; 16]);
        let pred = PredicateId([3; 16]);

        let e: Thing<Edge> = edge!(from, pred, to, 0u32);

        let e_from = e.value.from;
        let e_to = e.value.to;
        let e_pred = e.value.predicate;

        assert_eq!(e_from, from);
        assert_eq!(e_to, to);
        assert_eq!(e_pred, pred);
    }
}
