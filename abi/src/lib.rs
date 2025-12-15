#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub mod graph_kinds;
pub mod resident;
pub mod resident_layout;
pub mod syscall_defs;
pub mod syscall_numbers;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
pub const USER_HEAP_SIZE: usize = 4 * 1024 * 1024;
pub const USER_HEAP_END: usize = USER_HEAP_START + USER_HEAP_SIZE;

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

/// Simple property key
pub type PropKey = &'static str;

/// Simple property value
#[derive(Debug, Clone, PartialEq)]
pub enum PropValue {
    U64(u64),
    I64(i64),
    Bool(bool),
    Str(String),
}

/// Property type for schema validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropType {
    U64,
    I64,
    Bool,
    Str,
}

/// Schema identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaId(pub u64);

/// Memory summary statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySummary {
    pub total_frames: u64,
    pub used_frames: u64,
    pub free_frames: u64,
}

/// Scheduler summary statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulerSummary {
    pub process_count: u64,
    pub thread_count: u64,
    pub runnable_threads: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameInfo {
    pub id: FrameId,
    pub base: u64,
    pub size: u64,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PixelFormat {
    Rgba8888 = 0,
    Bgra8888 = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct MapFlags(pub u64);

impl MapFlags {
    pub const READ: MapFlags = MapFlags(1 << 0);
    pub const WRITE: MapFlags = MapFlags(1 << 1);
    pub const EXECUTE: MapFlags = MapFlags(1 << 2);
    pub const USER: MapFlags = MapFlags(1 << 3);

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn contains(self, other: MapFlags) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn union(self, other: MapFlags) -> MapFlags {
        MapFlags(self.0 | other.0)
    }
}

impl From<u64> for MapFlags {
    fn from(value: u64) -> Self {
        MapFlags(value)
    }
}

impl core::ops::BitOr for MapFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        MapFlags(self.0 | rhs.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SharedBufferInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: PixelFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadInfo {
    pub tid: u64,
    pub state: u64,
    pub priority: u64,
}

#[derive(Debug, Clone)]
pub struct BatchUpdateEntry {
    pub id: ThingId,
    pub props: &'static [(PropKey, PropValue)],
}

/// Kernel request from userland
#[derive(Debug, Clone)]
pub enum KernelRequest {
    /// Query the graph
    GraphQuery {
        node_id: NodeId,
    },
    /// Create a transaction
    CreateTransaction,
    /// Commit a transaction
    CommitTransaction {
        tx_id: TransactionId,
    },
    /// Log a message
    Log {
        message: &'static str,
    },
    /// Create a new Thing
    ThingCreate {
        kind: &'static str,
        props: &'static [(PropKey, PropValue)],
    },
    /// Spawn a program defined by a BootProgram Thing
    SpawnProgram {
        boot_program_id: ThingId,
    },
    /// Enumerate Things of a given kind
    ThingList {
        kind: &'static str,
        start_after: ThingId,
    },
    /// Get a Thing
    ThingGet {
        id: ThingId,
    },
    /// Update a Thing
    ThingUpdate {
        id: ThingId,
        props: &'static [(PropKey, PropValue)],
    },
    /// Batch update multiple Things
    ThingBatchUpdate {
        updates: &'static [BatchUpdateEntry],
    },
    /// Register a schema
    SchemaRegister {
        kind: &'static str,
        description: &'static str,
        props: &'static [(&'static str, PropType)],
    },
    /// Get a schema
    SchemaGet {
        kind: &'static str,
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
        name: &'static str,
    },
    /// Create a thread
    CreateThread {
        pid: u64,
        name: &'static str,
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
        kind: &'static str,
        byte_len: u32,
    },
    ResidentMap {
        thing_id: ThingId,
        perms: crate::resident::ResidentMapPerms,
    },
    ResidentUnmap {
        thing_id: ThingId,
    },
    ThingRest {
        thing_id: ThingId,
        policy: crate::resident::RestPolicy,
    },
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
        message: &'static str,
    },
    /// Transaction created
    TransactionCreated {
        tx_id: TransactionId,
    },
    /// Node data
    NodeData {
        node_id: NodeId,
        value: u64,
    },
    /// Thing created
    ThingCreated {
        id: ThingId,
    },
    /// Thing data
    ThingData {
        id: ThingId,
        kind: &'static str,
        props: &'static [Option<(PropKey, PropValue)>],
    },
    /// Schema registered
    SchemaRegistered {
        kind: &'static str,
    },
    /// Schema data
    SchemaData {
        kind: &'static str,
        props: &'static [Option<(&'static str, PropType)>],
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
        // snapshot of the thread that just ran (or None)
        current: Option<ThreadInfo>,
    },
    /// Result of querying a link target.
    LinkTarget {
        target: Option<ThingId>,
    },
    /// Program spawn result
    ProgramSpawned {
        process_id: ThingId,
        thread_id: ThingId,
    },
    /// Result of Thing enumeration
    ThingListEntry {
        id: Option<ThingId>,
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
        resp: crate::resident::ResidentAllocResp,
    },
    ResidentMapped {
        resp: crate::resident::ResidentMapResp,
    },
    ThingRested {
        resp: crate::resident::RestResp,
    },
    ResidentError(crate::resident::ResidentError),
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

/// Syscall numbers for Ring 3 -> Ring 0 communication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum SyscallNumber {
    // Scheduling and timekeeping
    Yield = 0,
    SleepForNs = 1,
    SleepUntil = 2,
    TimeMonotonicNs = 3,
    TimeSystemNs = 4,
    TimeNow = 5,

    // Diagnostics / thread management
    Log = 6,
    ExitThread = 7,

    // Memory and process management
    AllocFrame = 8,
    FreeFrame = 9,
    CreateProcess = 10,
    CreateThread = 11,
    SpawnProgram = 12,

    // Thing graph operations
    ThingCreate = 13,
    ThingGet = 14,
    ThingUpdate = 15,
    ThingList = 16,
    AddLink = 17,
    LinkAt = 18,
    SchemaRegister = 19,

    // Transactional / query interfaces
    GraphQuery = 20,
    CreateTransaction = 21,
    CommitTransaction = 22,
    MapSharedBuffer = 23,
    CreateSharedBuffer = 24,
    GetSharedBufferInfo = 25,
    ResidentAlloc = 26,
    ResidentMap = 27,
    ResidentUnmap = 28,
    ThingRest = 29,
    // Add others as needed
}

#[repr(C)]
pub struct SpawnProgramResult {
    pub process_id: ThingId,
    pub thread_id: ThingId,
}

pub trait Thing: Sized {
    const KIND: &'static str;
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

    #[test]
    fn test_syscall_number_encoding() {
        // Lock in specific syscall numbers to ensure ABI stability
        assert_eq!(SyscallNumber::Yield as u64, 0);
        assert_eq!(SyscallNumber::SleepForNs as u64, 1);
        assert_eq!(SyscallNumber::SleepUntil as u64, 2);
        assert_eq!(SyscallNumber::TimeMonotonicNs as u64, 3);
        assert_eq!(SyscallNumber::TimeSystemNs as u64, 4);
        assert_eq!(SyscallNumber::TimeNow as u64, 5);
        assert_eq!(SyscallNumber::Log as u64, 6);
        assert_eq!(SyscallNumber::ExitThread as u64, 7);
        assert_eq!(SyscallNumber::AllocFrame as u64, 8);
        assert_eq!(SyscallNumber::FreeFrame as u64, 9);
        assert_eq!(SyscallNumber::CreateProcess as u64, 10);
        assert_eq!(SyscallNumber::CreateThread as u64, 11);
        assert_eq!(SyscallNumber::SpawnProgram as u64, 12);
        assert_eq!(SyscallNumber::ThingCreate as u64, 13);
        assert_eq!(SyscallNumber::ThingGet as u64, 14);
        assert_eq!(SyscallNumber::ThingUpdate as u64, 15);
        assert_eq!(SyscallNumber::ThingList as u64, 16);
        assert_eq!(SyscallNumber::AddLink as u64, 17);
        assert_eq!(SyscallNumber::LinkAt as u64, 18);
        assert_eq!(SyscallNumber::SchemaRegister as u64, 19);
        assert_eq!(SyscallNumber::GraphQuery as u64, 20);
        assert_eq!(SyscallNumber::CreateTransaction as u64, 21);
        assert_eq!(SyscallNumber::CommitTransaction as u64, 22);
        assert_eq!(SyscallNumber::MapSharedBuffer as u64, 23);
        assert_eq!(SyscallNumber::CreateSharedBuffer as u64, 24);
        assert_eq!(SyscallNumber::GetSharedBufferInfo as u64, 25);
        assert_eq!(SyscallNumber::ResidentAlloc as u64, 26);
        assert_eq!(SyscallNumber::ResidentMap as u64, 27);
        assert_eq!(SyscallNumber::ResidentUnmap as u64, 28);
        assert_eq!(SyscallNumber::ThingRest as u64, 29);
    }

    #[test]
    fn test_pixel_format_encoding() {
        assert_eq!(PixelFormat::Rgba8888 as u8, 0);
        assert_eq!(PixelFormat::Bgra8888 as u8, 1);
    }

    #[test]
    fn test_thing_prop_scalar_type_encoding() {
        assert_eq!(ThingPropScalarType::U64 as u8, 0);
        assert_eq!(ThingPropScalarType::I64 as u8, 1);
        assert_eq!(ThingPropScalarType::Bool as u8, 2);
        assert_eq!(ThingPropScalarType::Str as u8, 3);
    }

    #[test]
    fn test_map_flags_invariants() {
        let read = MapFlags::READ;
        let write = MapFlags::WRITE;
        let rw = read.union(write);

        assert_eq!(read.bits(), 1);
        assert_eq!(write.bits(), 2);
        assert_eq!(rw.bits(), 3);
        assert!(rw.contains(read));
        assert!(rw.contains(write));
        assert!(!read.contains(write));
    }
}
pub mod graph_ops;
