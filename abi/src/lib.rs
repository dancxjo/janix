#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub mod graph_kinds;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThingId(pub u64);

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
    GraphQuery { node_id: NodeId },
    /// Create a transaction
    CreateTransaction,
    /// Commit a transaction
    CommitTransaction { tx_id: TransactionId },
    /// Log a message
    Log { message: &'static str },
    /// Create a new Thing
    ThingCreate {
        kind: &'static str,
        props: &'static [(PropKey, PropValue)],
    },
    /// Spawn a program defined by a BootProgram Thing
    SpawnProgram {
        boot_program_id: ThingId,
    },
    /// Get a Thing
    ThingGet { id: ThingId },
    /// Update a Thing
    ThingUpdate {
        id: ThingId,
        props: &'static [(PropKey, PropValue)],
    },
    /// Register a schema
    SchemaRegister {
        kind: &'static str,
        description: &'static str,
        props: &'static [(&'static str, PropType)],
    },
    /// Get a schema
    SchemaGet { kind: &'static str },
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
    FreeFrame { frame_id: FrameId },
    /// Create a process
    CreateProcess { name: &'static str },
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
    /// Add an edge between Things
    AddEdge {
        from: ThingId,
        edge_kind: &'static str,
        to: ThingId,
    },
    /// Fetch the target of the edge at a specific index.
    EdgeAt {
        from: ThingId,
        edge_kind: &'static str,
        index: u64,
    },
}

/// Kernel response to userland
#[derive(Debug, Clone)]
pub enum KernelResponse {
    /// Success with optional data
    Success { data: Option<u64> },
    /// Error with message
    Error { message: &'static str },
    /// Transaction created
    TransactionCreated { tx_id: TransactionId },
    /// Node data
    NodeData { node_id: NodeId, value: u64 },
    /// Thing created
    ThingCreated { id: ThingId },
    /// Thing data
    ThingData {
        id: ThingId,
        kind: &'static str,
        props: &'static [Option<(PropKey, PropValue)>],
    },
    /// Schema registered
    SchemaRegistered { kind: &'static str },
    /// Schema data
    SchemaData {
        kind: &'static str,
        props: &'static [Option<(&'static str, PropType)>],
    },
    /// Memory summary data
    MemorySummary { summary: MemorySummary },
    /// Scheduler summary data
    SchedulerSummary { summary: SchedulerSummary },
    /// Frame allocated
    FrameAllocated { frame: FrameInfo },
    /// Frame freed
    FrameFreed { frame_id: FrameId },
    /// Process created
    ProcessCreated { pid: u64 },
    /// Thread created
    ThreadCreated { tid: u64 },
    /// Scheduler ticked
    SchedulerTicked {
        // snapshot of the thread that just ran (or None)
        current: Option<ThreadInfo>,
    },
    /// Result of querying an edge target.
    EdgeTarget { target: Option<ThingId> },
    /// Program spawn result
    ProgramSpawned {
        process_id: ThingId,
        thread_id: ThingId,
    },
}

/// Syscall numbers for Ring 3 -> Ring 0 communication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum SyscallNumber {
    Yield = 1,
    Log = 2,
    ExitThread = 3,
    GraphQuery = 4,
    CreateTransaction = 5,
    CommitTransaction = 6,
    ThingCreate = 7,
    ThingGet = 8,
    ThingUpdate = 9,
    AllocFrame = 10,
    FreeFrame = 11,
    CreateProcess = 12,
    CreateThread = 13,
    SchemaRegister = 14,
    TimeNow = 15,
    SleepUntil = 16,
    TimeMonotonicNs = 20,
    TimeSystemNs = 21,
    SleepForNs = 22,
    AddEdge = 23,
    EdgeAt = 24,
    SpawnProgram = 25,
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
