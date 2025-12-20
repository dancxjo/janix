# ABI Surface Inventory

## KernelRequest Variants

| Variant | Field Types | Source |
|---|---|---|
| `GraphQuery` | `node_id: NodeId` | `abi/src/lib.rs` |
| `GraphQuery` | `out: UserSlice<u8>` | `abi/src/lib.rs` |
| `CreateTransaction` | Unit | `abi/src/lib.rs` |
| `CommitTransaction` | `tx_id: TransactionId` | `abi/src/lib.rs` |
| `Log` | `message: UserSlice<u8>` | `abi/src/lib.rs` |
| `ThingCreate` | `kind: SymbolId` | `abi/src/lib.rs` |
| `ThingCreate` | `props: UserSlice<WireProp>` | `abi/src/lib.rs` |
| `SpawnProgram` | `boot_program_id: ThingId` | `abi/src/lib.rs` |
| `ThingList` | `kind: SymbolId` | `abi/src/lib.rs` |
| `ThingList` | `start_after: ThingId` | `abi/src/lib.rs` |
| `ThingGet` | `id: ThingId` | `abi/src/lib.rs` |
| `ThingGet` | `out: UserSlice<u8>` | `abi/src/lib.rs` |
| `ThingUpdate` | `id: ThingId` | `abi/src/lib.rs` |
| `ThingUpdate` | `props: UserSlice<WireProp>` | `abi/src/lib.rs` |
| `ThingBatchUpdate` | `updates: UserSlice<WireBatchEntry>` | `abi/src/lib.rs` |
| `SchemaRegisterPackage` | `kind: SymbolId` | `abi/src/lib.rs` |
| `SchemaRegisterPackage` | `description: SymbolId` | `abi/src/lib.rs` |
| `SchemaRegisterPackage` | `props: UserSlice<WireSchemaProp>` | `abi/src/lib.rs` |
| `SchemaGet` | `kind: SymbolId` | `abi/src/lib.rs` |
| `SchemaGet` | `out: UserSlice<WireSchemaProp>` | `abi/src/lib.rs` |
| `GetMemorySummary` | Unit | `abi/src/lib.rs` |
| `GetSchedulerSummary` | Unit | `abi/src/lib.rs` |
| `AllocFrame` | `pool_index: u64` | `abi/src/lib.rs` |
| `FreeFrame` | `frame_id: FrameId` | `abi/src/lib.rs` |
| `CreateProcess` | `name: UserSlice<u8>` | `abi/src/lib.rs` |
| `CreateThread` | `pid: u64` | `abi/src/lib.rs` |
| `CreateThread` | `name: UserSlice<u8>` | `abi/src/lib.rs` |
| `CreateThread` | `app_id: u64` | `abi/src/lib.rs` |
| `CreateThread` | `priority: u64` | `abi/src/lib.rs` |
| `SchedulerTick` | Unit | `abi/src/lib.rs` |
| `ExitThread` | Unit | `abi/src/lib.rs` |
| `AddLink` | `src: ThingId` | `abi/src/lib.rs` |
| `AddLink` | `pred: Predicate` | `abi/src/lib.rs` |
| `AddLink` | `dst: ThingId` | `abi/src/lib.rs` |
| `LinkAt` | `src: ThingId` | `abi/src/lib.rs` |
| `LinkAt` | `pred: Predicate` | `abi/src/lib.rs` |
| `LinkAt` | `idx: usize` | `abi/src/lib.rs` |
| `CreateSharedBuffer` | `width: u32` | `abi/src/lib.rs` |
| `CreateSharedBuffer` | `height: u32` | `abi/src/lib.rs` |
| `CreateSharedBuffer` | `pixel_format: PixelFormat` | `abi/src/lib.rs` |
| `MapSharedBuffer` | `buffer_id: ThingId` | `abi/src/lib.rs` |
| `MapSharedBuffer` | `flags: MapFlags` | `abi/src/lib.rs` |
| `GetSharedBufferInfo` | `buffer_id: ThingId` | `abi/src/lib.rs` |
| `ResidentAlloc` | `kind: SymbolId` | `abi/src/lib.rs` |
| `ResidentAlloc` | `byte_len: u32` | `abi/src/lib.rs` |
| `ResidentAlloc` | `flags: u32` | `abi/src/lib.rs` |
| `ResidentMap` | `id: ThingId` | `abi/src/lib.rs` |
| `ResidentMap` | `perms: crate` | `abi/src/lib.rs` |
| `ResidentUnmap` | `thing_id: ThingId` | `abi/src/lib.rs` |
| `ThingRest` | `thing_id: ThingId` | `abi/src/lib.rs` |
| `ThingRest` | `policy: crate` | `abi/src/lib.rs` |

## KernelResponse Variants

| Variant | Field Types | Source |
|---|---|---|
| `Success` | `data: Option<u64>` | `abi/src/lib.rs` |
| `Error` | `err: crate` | `abi/src/lib.rs` |
| `TransactionCreated` | `tx_id: TransactionId` | `abi/src/lib.rs` |
| `NodeData` | `written: u64` | `abi/src/lib.rs` |
| `ThingCreated` | `id: ThingId` | `abi/src/lib.rs` |
| `SchemaRegistered` | `kind: SymbolId` | `abi/src/lib.rs` |
| `SchemaRegistered` | `outcome: SchemaRegistryOutcome` | `abi/src/lib.rs` |
| `SchemaData` | `written: u64` | `abi/src/lib.rs` |
| `SchemaData` | `fingerprint: u64` | `abi/src/lib.rs` |
| `MemorySummary` | `summary: MemorySummary` | `abi/src/lib.rs` |
| `SchedulerSummary` | `summary: SchedulerSummary` | `abi/src/lib.rs` |
| `FrameAllocated` | `frame: FrameInfo` | `abi/src/lib.rs` |
| `FrameFreed` | `frame_id: FrameId` | `abi/src/lib.rs` |
| `ProcessCreated` | `pid: u64` | `abi/src/lib.rs` |
| `ThreadCreated` | `tid: u64` | `abi/src/lib.rs` |
| `SchedulerTicked` | `has_current: u32` | `abi/src/lib.rs` |
| `SchedulerTicked` | `current: ThreadInfo` | `abi/src/lib.rs` |
| `LinkTarget` | `found: u32` | `abi/src/lib.rs` |
| `LinkTarget` | `target: ThingId` | `abi/src/lib.rs` |
| `ProgramSpawned` | `process_id: ThingId` | `abi/src/lib.rs` |
| `ProgramSpawned` | `thread_id: ThingId` | `abi/src/lib.rs` |
| `ThingListEntry` | `valid: u32` | `abi/src/lib.rs` |
| `ThingListEntry` | `id: ThingId` | `abi/src/lib.rs` |
| `SharedBufferCreated` | `buffer_id: ThingId` | `abi/src/lib.rs` |
| `SharedBufferMapped` | `vaddr: u64` | `abi/src/lib.rs` |
| `SharedBufferMapped` | `size: u64` | `abi/src/lib.rs` |
| `SharedBufferInfoResponse` | `info: SharedBufferInfo` | `abi/src/lib.rs` |
| `ResidentAllocated` | `resp: crate` | `abi/src/lib.rs` |
| `ResidentMapped` | `resp: crate` | `abi/src/lib.rs` |
| `ThingRested` | `resp: crate` | `abi/src/lib.rs` |

## Wire Types (abi/src/wire/**)

| Type Name | Kind | Fields/Types | Source | Compliance |
|---|---|---|---|---|
| `SpawnProgramResult` | struct | `pub process_id: ThingId` | `abi/src/wire/process.rs` | ✅ |
| `SpawnProgramResult` | struct | `pub thread_id: ThingId` | `abi/src/wire/process.rs` | ✅ |
| `DeviceHandle` | struct | `pub raw: u32` | `abi/src/wire/dev.rs` | ✅ |
| `DeviceKind` | enum | `Ps2Keyboard = 1: Unit` | `abi/src/wire/dev.rs` | ✅ |
| `DeviceKind` | enum | `Ps2Mouse = 2: Unit` | `abi/src/wire/dev.rs` | ✅ |
| `DevOpenArgs` | struct | `pub kind: u32` | `abi/src/wire/dev.rs` | ✅ |
| `DevOpenArgs` | struct | `pub index: u32` | `abi/src/wire/dev.rs` | ✅ |
| `DevOpenRet` | struct | `pub handle: DeviceHandle` | `abi/src/wire/dev.rs` | ✅ |
| `DevReadArgs` | struct | `pub handle: DeviceHandle` | `abi/src/wire/dev.rs` | ✅ |
| `DevReadArgs` | struct | `pub out: UserSlice<u8>` | `abi/src/wire/dev.rs` | ✅ |
| `DevReadRet` | struct | `pub bytes_read: u32` | `abi/src/wire/dev.rs` | ✅ |
| `WireValueTag` | enum | `U64 = 0: Unit` | `abi/src/wire/graph.rs` | ✅ |
| `WireValueTag` | enum | `I64 = 1: Unit` | `abi/src/wire/graph.rs` | ✅ |
| `WireValueTag` | enum | `Bool = 2: Unit` | `abi/src/wire/graph.rs` | ✅ |
| `WireValueTag` | enum | `Str = 3: Unit` | `abi/src/wire/graph.rs` | ✅ |
| `WireValueTag` | enum | `Blob = 4: Unit` | `abi/src/wire/graph.rs` | ✅ |
| `ValueFormat` | enum | `Postcard = 1: Unit` | `abi/src/wire/graph.rs` | ✅ |
| `ValueBlobHeader` | struct | `pub format: u8` | `abi/src/wire/graph.rs` | ✅ |
| `ValueBlobHeader` | struct | `pub _pad: [u8; 3]` | `abi/src/wire/graph.rs` | ✅ |
| `ValueBlobHeader` | struct | `pub type_id: SymbolId` | `abi/src/wire/graph.rs` | ✅ |
| `ValueBlobHeader` | struct | `pub len: u32` | `abi/src/wire/graph.rs` | ✅ |
| `WireBlob` | struct | `pub ptr: u64` | `abi/src/wire/graph.rs` | ✅ |
| `WireBlob` | struct | `pub len: u64` | `abi/src/wire/graph.rs` | ✅ |
| `WirePropValue` | struct | `pub tag: u8` | `abi/src/wire/graph.rs` | ✅ |
| `WirePropValue` | struct | `pub _pad: [u8; 7]` | `abi/src/wire/graph.rs` | ✅ |
| `WirePropValue` | struct | `pub data_0: u64` | `abi/src/wire/graph.rs` | ✅ |
| `WirePropValue` | struct | `pub data_1: u64` | `abi/src/wire/graph.rs` | ✅ |
| `WireProp` | struct | `pub key: SymbolId` | `abi/src/wire/graph.rs` | ✅ |
| `WireProp` | struct | `pub _pad: u32` | `abi/src/wire/graph.rs` | ✅ |
| `WireProp` | struct | `pub value: WirePropValue` | `abi/src/wire/graph.rs` | ✅ |
| `WireSchemaProp` | struct | `pub name: SymbolId` | `abi/src/wire/graph.rs` | ✅ |
| `WireSchemaProp` | struct | `pub prop_type: u32` | `abi/src/wire/graph.rs` | ✅ |
| `BatchUpdateEntry` | struct | `pub id: crate` | `abi/src/wire/graph.rs` | ✅ |
| `BatchUpdateEntry` | struct | `pub props_ptr: UserPtr<WireProp>` | `abi/src/wire/graph.rs` | ✅ |
| `BatchUpdateEntry` | struct | `pub props_len: u64` | `abi/src/wire/graph.rs` | ✅ |
| `BatchUpdateReq` | struct | `pub updates_ptr: UserPtr<BatchUpdateEntry>` | `abi/src/wire/graph.rs` | ✅ |
| `BatchUpdateReq` | struct | `pub updates_len: u64` | `abi/src/wire/graph.rs` | ✅ |
| `FrameInfo` | struct | `pub id: FrameId` | `abi/src/wire/memory.rs` | ✅ |
| `FrameInfo` | struct | `pub base: u64` | `abi/src/wire/memory.rs` | ✅ |
| `FrameInfo` | struct | `pub size: u64` | `abi/src/wire/memory.rs` | ✅ |
| `MemorySummary` | struct | `pub total_frames: u64` | `abi/src/wire/memory.rs` | ✅ |
| `MemorySummary` | struct | `pub used_frames: u64` | `abi/src/wire/memory.rs` | ✅ |
| `MemorySummary` | struct | `pub free_frames: u64` | `abi/src/wire/memory.rs` | ✅ |
| `SchedulerSummary` | struct | `pub process_count: u64` | `abi/src/wire/memory.rs` | ✅ |
| `SchedulerSummary` | struct | `pub thread_count: u64` | `abi/src/wire/memory.rs` | ✅ |
| `SchedulerSummary` | struct | `pub runnable_threads: u64` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `pub const READ: MapFlags = MapFlags(1 << 0);` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `pub const WRITE: MapFlags = MapFlags(1 << 1);` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `pub const EXECUTE: MapFlags = MapFlags(1 << 2);` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `pub const USER: MapFlags = MapFlags(1 << 3);` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `pub const fn bits: self` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `self.0: Unit` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `pub const fn contains(self, other: MapFlags) -> bool {` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `: self.0 & other.0` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `pub const fn union(self, other: MapFlags) -> MapFlags {` | `abi/src/wire/memory.rs` | ✅ |
| `MapFlags` | struct | `MapFlags: self.0 | other.0` | `abi/src/wire/memory.rs` | ✅ |
| `UserPtr` | struct | `pub ptr: u64` | `abi/src/wire/common.rs` | ✅ |
| `UserPtr` | struct | `pub _phantom: PhantomData<T>` | `abi/src/wire/common.rs` | ✅ |
| `UserSlice` | struct | `pub ptr: u64` | `abi/src/wire/common.rs` | ✅ |
| `UserSlice` | struct | `pub len: u64` | `abi/src/wire/common.rs` | ✅ |
| `UserSlice` | struct | `pub _phantom: PhantomData<T>` | `abi/src/wire/common.rs` | ✅ |
| `UserStr` | struct | `pub ptr: u64` | `abi/src/wire/common.rs` | ✅ |
| `UserStr` | struct | `pub len: u64` | `abi/src/wire/common.rs` | ✅ |
| `ResidentId` | struct | `pub const READ: Self = Self(1 << 0);` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentId` | struct | `pub const WRITE: Self = Self(1 << 1);` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentMapPerms` | struct | `pub const READ: Self = Self(1 << 0);` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentMapPerms` | struct | `pub const WRITE: Self = Self(1 << 1);` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentAllocArgs` | struct | `pub kind_id: ThingId` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentAllocArgs` | struct | `pub byte_len: u32` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentAllocArgs` | struct | `pub flags: u32` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentAllocResp` | struct | `pub id: ThingId` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentMapArgs` | struct | `pub id: ThingId` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentMapArgs` | struct | `pub perms: ResidentMapPerms` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentMapResp` | struct | `pub user_addr: u64` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentMapResp` | struct | `pub byte_len: u32` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentMapResp` | struct | `pub _pad: u32` | `abi/src/wire/resident.rs` | ✅ |
| `RestPolicy` | enum | `SnapshotKeepResident = 0: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `RestPolicy` | enum | `SnapshotEvictResident = 1: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ArchiveRef` | struct | `pub id: u32` | `abi/src/wire/resident.rs` | ✅ |
| `RestResp` | struct | `pub thing_id: crate` | `abi/src/wire/resident.rs` | ✅ |
| `RestResp` | struct | `pub archived_ref: ArchiveRef` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `#[default]: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `Success = 0: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `Unknown = 1: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `PermissionDenied = 2: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `NotFound = 3: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `BadThing = 4: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `NotResident = 5: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `AlreadyMappedRw = 6: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `SerializeFailed = 7: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `Bounds = 8: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `BadHeader = 9: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `BadKind = 10: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `OutOfMemory = 11: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentErrorCode` | enum | `Internal = 12: Unit` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentError` | struct | `pub code: ResidentErrorCode` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentError` | struct | `pub aux0: u64` | `abi/src/wire/resident.rs` | ✅ |
| `ResidentError` | struct | `pub aux1: u64` | `abi/src/wire/resident.rs` | ✅ |
| `PixelFormat` | enum | `Rgba8888 = 0: Unit` | `abi/src/wire/buffers.rs` | ✅ |
| `PixelFormat` | enum | `Bgra8888 = 1: Unit` | `abi/src/wire/buffers.rs` | ✅ |
| `SharedBufferInfo` | struct | `pub width: u32` | `abi/src/wire/buffers.rs` | ✅ |
| `SharedBufferInfo` | struct | `pub height: u32` | `abi/src/wire/buffers.rs` | ✅ |
| `SharedBufferInfo` | struct | `pub stride: u32` | `abi/src/wire/buffers.rs` | ✅ |
| `SharedBufferInfo` | struct | `pub pixel_format: PixelFormat` | `abi/src/wire/buffers.rs` | ✅ |
