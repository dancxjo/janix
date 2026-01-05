use crate::ids::{GraphId, RelationshipId, SymbolId, ThingId};

/// Syscall result type (FFI-safe)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SyscallResult {
    pub status: u64,
    pub val0: u64,
    pub val1: u64,
}

impl SyscallResult {
    pub fn new(status: i32, val0: u64, val1: u64) -> Self {
        Self {
            status: status as u64,
            val0,
            val1,
        }
    }
}

/// Syscall dispatch function type
pub type SyscallDispatch = extern "C" fn(u32, u64, u64, u64, u64, u64, u64) -> SyscallResult;

/// Graph operation request
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphOp<'a> {
    /// Get the root graph ID
    GetRootGraph,

    /// Create a new Thing
    ThingCreate {
        kind: SymbolId,
        schema: SymbolId,
        version: u32,
        payload: &'a [u8],
    },

    /// Read a Thing's header and payload
    ThingRead { id: ThingId },

    /// Create a relationship between two Things
    RelationshipCreate {
        from: ThingId,
        to: ThingId,
        predicate: SymbolId,
    },

    /// Find relationships originating from a Thing
    RelationshipsFrom {
        from: ThingId,
        predicate: Option<SymbolId>,
    },

    /// Find things contained within a graph
    ContainedIn { graph: GraphId },
}

/// Graph operation response
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphReply {
    /// Generic success
    Ok,
    /// Returns a ThingId
    ThingId(ThingId),
    /// Returns a RelationshipId
    RelationshipId(RelationshipId),
    /// Returns a list of ThingIds
    IdList(alloc::vec::Vec<ThingId>),
    /// Returns a list of RelationshipIds
    RelList(alloc::vec::Vec<RelationshipId>),
    /// Returns raw bytes
    Bytes(alloc::vec::Vec<u8>),
    /// Operation failed
    Error(i32),
}
