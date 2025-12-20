#![no_std]

extern crate alloc;

use alloc::vec::Vec;

pub mod graph_kinds;
pub mod resident;
pub mod resident_layout;
pub mod mouse_stream;
pub mod keyboard_stream;
pub mod syscall_defs;
pub mod syscall_numbers;
pub mod syscalls;
pub mod wire;

pub use crate::wire::memory::{MemorySummary, SchedulerSummary, FrameInfo, MapFlags};
pub use crate::wire::buffers::{PixelFormat, SharedBufferInfo};

use crate::wire::common::UserSlice;
use crate::wire::graph::{WireProp, WireSchemaProp, BatchUpdateEntry as WireBatchEntry};

/// Process identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(pub u64);

/// Transaction identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionId(pub u64);

/// Node identifier in the graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

/// Thing identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ThingId(pub u64);

impl ThingId {
    pub const fn new(index: u32, generation: u32) -> Self {
        ThingId((generation as u64) << 32 | (index as u64))
    }

    pub const fn index(self) -> u32 {
        self.0 as u32
    }

    pub const fn generation(self) -> u32 {
        (self.0 >> 32) as u32
    }
}

/// Predicate identifier for a link between Things.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Predicate(pub u64);

/// Canonical link representation inside the graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Link {
    pub id: ThingId,
    pub src: ThingId,
    pub dst: ThingId,
    pub pred: Predicate,
}

pub const USER_HEAP_START: usize = 0x0000_0000_4000_0000;
pub const USER_HEAP_SIZE: usize = 32 * 1024 * 1024;
pub const USER_HEAP_END: usize = USER_HEAP_START + USER_HEAP_SIZE;

pub const USER_RESIDENT_BASE: usize = 0x5000_0000;
pub const USER_RESIDENT_LIMIT: usize = 0x6000_0000;

/// Address Space identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AddressSpaceId(pub u64);

/// Thread identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadId(pub u64);

/// CPU Core identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CpuCoreId(pub u64);

/// Frame identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameId(pub u64);

/// Frame Pool identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FramePoolId(pub u64);

use crate::syscall_defs::SymbolId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchedThreadInfo {
    pub tid: u64,
    pub state: u64,
    pub priority: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaRegistryOutcome {
    Created,
    AlreadyRegisteredSame,
    Conflict,
}

pub mod requests;
pub use requests::{KernelRequest, KernelResponse};

pub const THING_GET_MAX_KIND_LEN: usize = 128;
pub const THING_GET_MAX_STR_LEN: usize = 128;
pub const THING_GET_MAX_PROPS: usize = 8;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThingPropScalarType {
    U64 = 0,
    I64 = 1,
    Bool = 2,
    Str = 3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ThingPropData {
    pub present: u8,
    pub key_len: usize,
    pub key: [u8; THING_GET_MAX_STR_LEN],
    pub value_type: ThingPropScalarType,
    pub value_u64: u64,
    pub value_i64: i64,
    pub value_bool: u8,
    pub value_str_len: usize,
    pub value_str: [u8; THING_GET_MAX_STR_LEN],
}

impl Default for ThingPropData {
    fn default() -> Self {
        Self {
            present: 0,
            key_len: 0,
            key: [0; THING_GET_MAX_STR_LEN],
            value_type: ThingPropScalarType::U64,
            value_u64: 0,
            value_i64: 0,
            value_bool: 0,
            value_str_len: 0,
            value_str: [0; THING_GET_MAX_STR_LEN],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ThingGetSyscallResult {
    pub kind_len: usize,
    pub kind: [u8; THING_GET_MAX_KIND_LEN],
    pub prop_count: usize,
    pub props: [ThingPropData; THING_GET_MAX_PROPS],
}

impl Default for ThingGetSyscallResult {
    fn default() -> Self {
        Self {
            kind_len: 0,
            kind: [0; THING_GET_MAX_KIND_LEN],
            prop_count: 0,
            props: [ThingPropData::default(); THING_GET_MAX_PROPS],
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing_id_packing() {
        let id = ThingId::new(1, 2);
        assert_eq!(id.index(), 1);
        assert_eq!(id.generation(), 2);

        let id_old = ThingId::new(1, 1);
        let id_new = ThingId::new(1, 2);
        assert!(id_new > id_old);

        let id_idx = ThingId::new(2, 1);
        assert!(id_idx > id_old);
        assert!(id_new > id_idx); // Generation dominates
    }

    #[test]
    fn test_thing_id_invariants() {
        // ThingId is a transparent wrapper around u64
        let id_zero = ThingId(0);
        let id_one = ThingId(1);

        assert_eq!(id_zero.0, 0);
        assert_eq!(id_one.0, 1);
        assert!(id_zero < id_one);

        // Verify it implements Copy/Clone/Debug/etc
        let copy_id = id_zero;
        assert_eq!(copy_id, id_zero);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphEvent {
    ThingCreated(ThingId),
    ThingUpdated(ThingId),
    LinkAdded {
        src: ThingId,
        dst: ThingId,
        pred: Predicate,
    },
    BatchUpdateComplete,
}
