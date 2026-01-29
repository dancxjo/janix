//! Generic envelope for things in the graph.

use crate::graphable::Graphable;
use crate::wire::{KindId, ThingId};

/// A generic envelope that provides identity + kind for any T: Graphable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Thing<T: Graphable> {
    pub id: ThingId,
    pub kind: KindId,
    pub value: T,
}

impl<T: Graphable> Thing<T> {
    /// Create a new Thing with a default (zeroed) handle.
    ///
    /// NOTE: In userspace, you should usually not mint your own IDs.
    /// Use this as a sentinel or for temporary storage until the kernel assigns one.
    pub fn new(value: T) -> Self {
        let id = ThingId::default();
        Self {
            id,
            kind: T::kind(),
            value,
        }
    }

    /// Create a new Thing with a specific ID.
    pub fn with_id(id: ThingId, value: T) -> Self {
        Self {
            id,
            kind: T::kind(),
            value,
        }
    }
}
