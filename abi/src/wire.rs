//! Wire-safe primitives and IDs for the Graphable contract.
//! 
//! This module defines the fundamental fixed-size IDs used in the Thing-OS
//! data graph and the `WireSafe` trait used to enforce pointer-free,
//! packed layouts for payload structs.

use core::marker::PhantomData;

/// 128-bit unique identifier for a Thing in the graph.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ThingId(pub [u8; 16]);

/// 128-bit content-addressable identifier for a Blob (large binary data).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlobId(pub [u8; 16]);

/// 128-bit identifier for an interned Symbol (string).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SymbolId(pub [u8; 16]);

/// 128-bit identifier for a Schema Kind, derived from the stable hash of the Schema.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct KindId(pub [u8; 16]);

/// 128-bit identifier for a Predicate (edge type).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PredicateId(pub [u8; 16]);

impl ThingId {
    pub fn new() -> Self {
        // TODO: Use true randomness. For now, use a rudimentary counter or zero.
        // Since we are no_std and don't have rand yet.
        Self([0; 16]) 
    }
}

impl SymbolId {
    /// Convert a BlobId to a SymbolId.
    /// 
    /// In this implementation, we simply hash the BlobId to get a SymbolId,
    /// or treating it as distinct type-safe handle.
    /// For now, since they are both 16 bytes, we can map 1:1 if we want,
    /// but let's do a quick mix to ensure they are distinct spaces if needed.
    /// Actually, the prompt suggests "stable hash of the BlobId bytes" or simple mapping.
    /// Let's use blake3 hashing of the BlobId bytes to be consistent with "derived" IDs.
    pub fn from_blob(blob: BlobId) -> SymbolId {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"SymbolId");
        hasher.update(&blob.0);
        let hash = hasher.finalize();
        let mut out = [0u8; 16];
        out.copy_from_slice(&hash.as_bytes()[0..16]);
        SymbolId(out)
    }
}

/// Marker trait for types that are safe to transmit over the wire (pointer-free, packed, fixed-size).
/// 
/// # Safety
/// 
/// Implementing this trait asserts that:
/// 1. The type is `Copy` and `'static`.
/// 2. The type contains NO pointers, references, `Box`, `Vec`, `String`, etc.
/// 3. The type has a stable, platform-independent memory layout (when used with `encode_packed_le`).
///    Ideally `#[repr(C)]` or `#[repr(C, packed)]`.
pub unsafe trait WireSafe: Copy + 'static {}

// Null implementation for PhantomData generic over WireSafe types
unsafe impl<T: WireSafe> WireSafe for PhantomData<T> {}

// Primitive implementations
unsafe impl WireSafe for u8 {}
unsafe impl WireSafe for u16 {}
unsafe impl WireSafe for u32 {}
unsafe impl WireSafe for u64 {}
unsafe impl WireSafe for u128 {}
unsafe impl WireSafe for i8 {}
unsafe impl WireSafe for i16 {}
unsafe impl WireSafe for i32 {}
unsafe impl WireSafe for i64 {}
unsafe impl WireSafe for i128 {}
unsafe impl WireSafe for f32 {}
unsafe impl WireSafe for f64 {}

// IDs
unsafe impl WireSafe for ThingId {}
unsafe impl WireSafe for BlobId {}
unsafe impl WireSafe for SymbolId {}
unsafe impl WireSafe for KindId {}
unsafe impl WireSafe for PredicateId {}

// Arrays
unsafe impl<T: WireSafe, const N: usize> WireSafe for [T; N] {}

/// Assert that a type is WireSafe at compile time.
/// 
/// Used in impl blocks to enforce constraints.
pub const fn assert_wire_safe<T: WireSafe>() {}
