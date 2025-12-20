use crate::{
    ThingId, FrameId, TransactionId, NodeId, Predicate,
    SchedThreadInfo, SchemaRegistryOutcome,
    syscall_defs::{SymbolId, SysError},
    wire::{
        common::UserSlice,
        graph::{WireProp, BatchUpdateEntry as WireBatchEntry, WireSchemaProp},
        memory::{MemorySummary, SchedulerSummary, FrameInfo, MapFlags},
        resident::{ResidentAllocResp, ResidentMapResp, RestResp, ResidentError, ResidentMapPerms, RestPolicy},
        buffers::{PixelFormat, SharedBufferInfo},
    },
};

#[derive(Debug, Clone)]
pub enum KernelRequest {
    GraphQuery {
        node_id: NodeId,
        out: UserSlice<u8>,
    },
    CreateTransaction,
    CommitTransaction {
        tx_id: TransactionId,
    },
    Log {
        message: UserSlice<u8>,
    },
    ThingCreate {
        kind: SymbolId,
        props: UserSlice<WireProp>,
    },
    SpawnProgram {
        boot_program_id: ThingId,
    },
    ThingList {
        kind: SymbolId,
        start_after: ThingId,
    },
    ThingGet {
        id: ThingId,
        out: UserSlice<u8>,
    },
    ThingUpdate {
        id: ThingId,
        props: UserSlice<WireProp>,
    },
    ThingBatchUpdate {
        updates: UserSlice<WireBatchEntry>,
    },
    SchemaRegisterPackage {
        kind: SymbolId,
        description: SymbolId,
        props: UserSlice<WireSchemaProp>,
    },
    SchemaGet {
        kind: SymbolId,
        out: UserSlice<WireSchemaProp>,
    },
    GetMemorySummary,
    GetSchedulerSummary,
    AllocFrame {
        pool_index: u64,
    },
    FreeFrame {
        frame_id: FrameId,
    },
    CreateProcess {
        name: UserSlice<u8>,
    },
    CreateThread {
        pid: u64,
        name: UserSlice<u8>,
        app_id: u64,
        priority: u64,
    },
    SchedulerTick,
    ExitThread,
    AddLink {
        src: ThingId,
        pred: Predicate,
        dst: ThingId,
    },
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
        perms: ResidentMapPerms,
    },
    ResidentUnmap {
        thing_id: ThingId,
    },
    ThingRest {
        thing_id: ThingId,
        policy: RestPolicy,
    },
}

#[derive(Debug, Clone)]
pub enum KernelResponse {
    Success {
        data: Option<u64>,
    },
    Error {
        err: SysError,
    },
    TransactionCreated {
        tx_id: TransactionId,
    },
    NodeData {
        written: u64,
    },
    ThingCreated {
        id: ThingId,
    },
    SchemaRegistered {
        kind: SymbolId,
        outcome: SchemaRegistryOutcome,
    },
    SchemaData {
        written: u64,
        fingerprint: u64,
    },
    MemorySummary {
        summary: MemorySummary,
    },
    SchedulerSummary {
        summary: SchedulerSummary,
    },
    FrameAllocated {
        frame: FrameInfo,
    },
    FrameFreed {
        frame_id: FrameId,
    },
    ProcessCreated {
        pid: u64,
    },
    ThreadCreated {
        tid: u64,
    },
    SchedulerTicked {
        has_current: u32,
        current: SchedThreadInfo,
    },
    LinkTarget {
        found: u32,
        target: ThingId,
    },
    ProgramSpawned {
        process_id: ThingId,
        thread_id: ThingId,
    },
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
        resp: ResidentAllocResp,
    },
    ResidentMapped {
        resp: ResidentMapResp,
    },
    ThingRested {
        resp: RestResp,
    },
    ResidentError(ResidentError),
}
