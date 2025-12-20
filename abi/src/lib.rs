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

/// Simple property key
pub type PropKey = alloc::string::String;

/// Simple property value
#[derive(Debug, Clone, PartialEq)]
pub enum PropValue {
    U64(u64),
    I64(i64),
    Bool(bool),
    Str(alloc::string::String),
    Blob(alloc::vec::Vec<u8>),
    Symbol(crate::syscall_defs::SymbolId),
}

/// Property type for schema validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropType {
    U64,
    I64,
    Bool,
    Symbol,
    Str,
    Blob,
}

/// Schema identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadInfo {
    pub tid: u64,
    pub state: u64,
    pub priority: u64,
}

/// Kernel request from userland
#[derive(Debug, Clone)]
pub enum KernelRequest {
    /// Query the graph
    GraphQuery {
        node_id: NodeId,
        out: UserSlice<u8>,
    },
    /// Create a transaction
    CreateTransaction,
    /// Commit a transaction
    CommitTransaction {
        tx_id: TransactionId,
    },
    /// Log a message
    Log {
        message: UserSlice<u8>,
    },
    /// Create a new Thing
    ThingCreate {
        kind: SymbolId,
        props: UserSlice<WireProp>,
    },
    /// Spawn a program defined by a BootProgram Thing
    SpawnProgram {
        boot_program_id: ThingId,
    },
    /// Enumerate Things of a given kind
    ThingList {
        kind: SymbolId,
        start_after: ThingId,
    },
    /// Get a Thing
    ThingGet {
        id: ThingId,
        out: UserSlice<u8>,
    },
    /// Update a Thing
    ThingUpdate {
        id: ThingId,
        props: UserSlice<WireProp>,
    },
    /// Batch update multiple Things
    ThingBatchUpdate {
        updates: UserSlice<WireBatchEntry>,
    },
    /// Register a package schema (scoped to the calling process).
    SchemaRegisterPackage {
        kind: SymbolId,
        description: SymbolId,
        props: UserSlice<WireSchemaProp>,
    },
    /// Get a schema
    SchemaGet {
        kind: SymbolId,
        out: UserSlice<WireSchemaProp>,
    },
    /// Get memory summary
    GetMemorySummary,
    /// Get scheduler summary
    GetSchedulerSummary,
    /// Allocate a frame
    AllocFrame {
        // optional: later we can support multiple pools; for now, use 0
        pool_index: u64,
    },
    /// Free a frame
    FreeFrame {
        frame_id: FrameId,
    },
    /// Create a process
    CreateProcess {
        name: UserSlice<u8>,
    },
    /// Create a thread
    CreateThread {
        pid: u64,
        name: UserSlice<u8>,
        app_id: u64,
        priority: u64,
    },
    /// Advance scheduler tick
    SchedulerTick,
    /// Exit the current thread
    ExitThread,
    /// Add a link between Things
    AddLink {
        src: ThingId,
        pred: Predicate,
        dst: ThingId,
    },
    /// Fetch the target of the link at a specific index.
    LinkAt {
        src: ThingId,
        pred: Predicate,
        idx: usize,
    },
    CreateSharedBuffer {
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
    },
    MapSharedBuffer {
        buffer_id: ThingId,
        flags: MapFlags,
    },
    GetSharedBufferInfo {
        buffer_id: ThingId,
    },
    ResidentAlloc {
        kind: SymbolId,
        byte_len: u32,
        flags: u32,
    },
    ResidentMap {
        id: ThingId,
        perms: crate::wire::resident::ResidentMapPerms,
    },
    ResidentUnmap {
        thing_id: ThingId,
    },
    ThingRest {
        thing_id: ThingId,
        policy: crate::wire::resident::RestPolicy,
    },
}

/// Schema register outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaRegistryOutcome {
    Created,
    AlreadyRegisteredSame,
    Conflict,
}

/// Kernel response to userland
#[derive(Debug, Clone)]
pub enum KernelResponse {
    /// Success with optional data
    Success {
        data: Option<u64>,
    },
    /// Error with message
    Error {
        err: crate::syscall_defs::SysError,
    },
    /// Transaction created
    TransactionCreated {
        tx_id: TransactionId,
    },
    /// Node data
    NodeData {
        written: u64,
    },
    /// Thing created
    ThingCreated {
        id: ThingId,
    },
    /// Schema registered result
    SchemaRegistered {
        kind: SymbolId,
        outcome: SchemaRegistryOutcome,
    },
    /// Schema data
    SchemaData {
        written: u64,
        fingerprint: u64,
    },
    /// Memory summary data
    MemorySummary {
        summary: MemorySummary,
    },
    /// Scheduler summary data
    SchedulerSummary {
        summary: SchedulerSummary,
    },
    /// Frame allocated
    FrameAllocated {
        frame: FrameInfo,
    },
    /// Frame freed
    FrameFreed {
        frame_id: FrameId,
    },
    /// Process created
    ProcessCreated {
        pid: u64,
    },
    /// Thread created
    ThreadCreated {
        tid: u64,
    },
    /// Scheduler ticked
    SchedulerTicked {
        has_current: u32,
        current: ThreadInfo,
    },
    /// Result of querying a link target.
    LinkTarget {
        found: u32,
        target: ThingId,
    },
    /// Program spawn result
    ProgramSpawned {
        process_id: ThingId,
        thread_id: ThingId,
    },
    /// Result of Thing enumeration
    ThingListEntry {
        valid: u32,
        id: ThingId,
    },
    SharedBufferCreated {
        buffer_id: ThingId,
    },
    SharedBufferMapped {
        vaddr: u64,
        size: u64,
    },
    SharedBufferInfoResponse {
        info: SharedBufferInfo,
    },
    ResidentAllocated {
        resp: crate::wire::resident::ResidentAllocResp,
    },
    ResidentMapped {
        resp: crate::wire::resident::ResidentMapResp,
    },
    ThingRested {
        resp: crate::wire::resident::RestResp,
    },
    ResidentError(crate::wire::resident::ResidentError),
}

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

pub trait Thing: Sized {
    const KIND: &'static str; // High level string, wrapper must intern
    const DESCRIPTION: &'static str;

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>);
    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self;

    /// Static schema for this Thing, used for registration.
    fn schema() -> &'static [(&'static str, PropType)];

    /// Get the description for this Thing instance, falling back to the type description.
    /// This can be overridden to check for an instance-specific "description" property.
    fn get_description(&self) -> &'static str {
        Self::DESCRIPTION
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
pub mod graph_ops;
